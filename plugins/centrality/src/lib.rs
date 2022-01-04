use std::collections::{BTreeMap, BTreeSet};
use std::error::Error as StdError;
use std::fmt;
use std::io::{stdout, Write};
use std::mem::take;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use indradb::{EdgeQueryExt, VertexQueryExt};
use indradb_plugin_host as plugin;

const DEFAULT_MAX_ITERATIONS: u16 = 10;
const DEFAULT_MAX_DELTA: f64 = 0.01;
const DEFAULT_CENTRALITY_PROPERTY_NAME: &str = "centrality";
const DEFAULT_CACHE_EDGES: bool = false;

#[derive(Debug)]
pub struct DidNotConvergeError {
    target_delta: f64,
    iterations: u16,
}

impl StdError for DidNotConvergeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl fmt::Display for DidNotConvergeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "did not converge to target delta {:.4} after {} iterations",
            self.target_delta, self.iterations
        )
    }
}

struct EdgeFetcher {
    datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
    vertex_type_filter: Option<indradb::Identifier>,
    edge_type_filter: Option<indradb::Identifier>,
    is_cached: bool,
    cache: Arc<RwLock<BTreeSet<(uuid::Uuid, uuid::Uuid)>>>,
}

impl EdgeFetcher {
    fn new_with_cache(
        datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
        vertex_type_filter: Option<indradb::Identifier>,
        edge_type_filter: Option<indradb::Identifier>,
    ) -> Self {
        Self {
            datastore,
            vertex_type_filter,
            edge_type_filter,
            is_cached: true,
            cache: Arc::new(RwLock::new(BTreeSet::default())),
        }
    }

    fn new(
        datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
        edge_type_filter: Option<indradb::Identifier>,
    ) -> Self {
        Self {
            datastore,
            vertex_type_filter: None,
            edge_type_filter,
            is_cached: false,
            cache: Arc::new(RwLock::new(BTreeSet::default())),
        }
    }

    fn fetch(&self, vertex_id: uuid::Uuid) -> Result<Vec<uuid::Uuid>, plugin::Error> {
        let q = if let Some(ref edge_type_filter) = self.edge_type_filter {
            indradb::SpecificVertexQuery::single(vertex_id)
                .outbound()
                .t(edge_type_filter.clone())
                .inbound()
                .into()
        } else {
            indradb::SpecificVertexQuery::single(vertex_id)
                .outbound()
                .inbound()
                .into()
        };
        Ok(self.datastore.get_vertices(q)?.into_iter().map(|v| v.id).collect())
    }

    fn get(&self, vertex_id: uuid::Uuid) -> Result<Vec<uuid::Uuid>, plugin::Error> {
        if self.is_cached {
            let cache = self.cache.read().unwrap();
            let mut results = Vec::new();
            for (out_id, in_id) in cache.range((vertex_id, uuid::Uuid::default())..) {
                if *out_id != vertex_id {
                    break;
                }
                results.push(*in_id);
            }
            Ok(results)
        } else {
            self.fetch(vertex_id)
        }
    }
}

impl plugin::util::VertexMapper for EdgeFetcher {
    fn t_filter(&self) -> Option<indradb::Identifier> {
        self.vertex_type_filter.clone()
    }

    fn map(&self, vertex: indradb::Vertex) -> Result<(), plugin::Error> {
        let edges = self.fetch(vertex.id)?;
        let mut cache = self.cache.write().unwrap();
        for linked_vertex_id in edges {
            cache.insert((vertex.id, linked_vertex_id));
        }
        Ok(())
    }
}

// TODO: separate mapper for when there's a weight property to pull
struct CentralityMapper {
    prev_centrality_map: BTreeMap<uuid::Uuid, f64>,
    cur_centrality_map: Arc<Mutex<BTreeMap<uuid::Uuid, f64>>>,
    t_filter: Option<indradb::Identifier>,
    edge_fetcher: Arc<EdgeFetcher>,
}

impl CentralityMapper {
    fn new(
        prev_centrality_map: BTreeMap<uuid::Uuid, f64>,
        t_filter: Option<indradb::Identifier>,
        edge_fetcher: Arc<EdgeFetcher>,
    ) -> Self {
        Self {
            prev_centrality_map,
            cur_centrality_map: Arc::new(Mutex::new(BTreeMap::default())),
            t_filter,
            edge_fetcher,
        }
    }

    fn total_delta(&self) -> f64 {
        let cur_centrality_map = self.cur_centrality_map.lock().unwrap();
        let mut delta = 0.0f64;
        for (id, centrality) in &*cur_centrality_map {
            let prev_centrality = self.prev_centrality_map.get(id).unwrap_or(&1.0);
            delta += f64::abs(centrality - prev_centrality);
        }
        delta
    }

    fn unpack(&mut self) -> BTreeMap<uuid::Uuid, f64> {
        take(&mut self.cur_centrality_map.lock().unwrap())
    }
}

impl plugin::util::VertexMapper for CentralityMapper {
    fn t_filter(&self) -> Option<indradb::Identifier> {
        self.t_filter.clone()
    }

    fn map(&self, vertex: indradb::Vertex) -> Result<(), plugin::Error> {
        let linked_ids = self.edge_fetcher.get(vertex.id)?;
        let centrality = *self.prev_centrality_map.get(&vertex.id).unwrap_or(&1.0);
        let vote_weight = centrality / (linked_ids.len() as f64);
        let mut cur_centrality_map = self.cur_centrality_map.lock().unwrap();
        *cur_centrality_map.entry(vertex.id).or_insert(0.0) += 1.0;
        for linked_id in linked_ids {
            *cur_centrality_map.entry(linked_id).or_insert(0.0) += vote_weight;
        }
        Ok(())
    }
}

pub struct CentralityPlugin {}

impl plugin::Plugin for CentralityPlugin {
    fn call(
        &self,
        datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, plugin::Error> {
        let vertex_count = datastore.get_vertex_count()?;
        let vertex_type_filter = parse_identifier(&arg, "vertex_type_filter")?;
        let edge_type_filter = parse_identifier(&arg, "edge_type_filter")?;
        let centrality_property_name = parse_identifier(&arg, "centrality_property_name")?
            .unwrap_or_else(|| indradb::Identifier::new(DEFAULT_CENTRALITY_PROPERTY_NAME).unwrap());
        let max_iterations = parse_max_iterations(&arg)?;
        let max_delta = parse_max_delta(&arg)?;
        let cache_edges = parse_cache_edges(&arg)?;

        let edge_fetcher = if cache_edges {
            print!("centrality plugin: caching edges");
            stdout().flush().unwrap();
            let edge_fetcher = Arc::new(EdgeFetcher::new_with_cache(
                datastore.clone(),
                vertex_type_filter.clone(),
                edge_type_filter,
            ));
            plugin::util::map(edge_fetcher.clone(), datastore.clone())?;
            println!("\rcentrality plugin: caching edges: done");
            edge_fetcher
        } else {
            Arc::new(EdgeFetcher::new(datastore.clone(), edge_type_filter))
        };

        let mut prev_centrality_map = BTreeMap::default();

        for i in 0..max_iterations {
            let start_time = Instant::now();
            print!("centrality plugin: iteration {}", i);
            stdout().flush().unwrap();

            let mut mapper = Arc::new(CentralityMapper::new(
                prev_centrality_map,
                vertex_type_filter.clone(),
                edge_fetcher.clone(),
            ));
            plugin::util::map(mapper.clone(), datastore.clone())?;

            let delta = mapper.total_delta() / (vertex_count as f64);
            println!(
                "\rcentrality plugin: iteration {}: delta={}, runtime={:?}",
                i,
                delta,
                start_time.elapsed()
            );

            prev_centrality_map = Arc::get_mut(&mut mapper).unwrap().unpack();

            if delta < max_delta {
                let properties: Vec<indradb::BulkInsertItem> = prev_centrality_map
                    .into_iter()
                    .map(|(id, centrality)| {
                        indradb::BulkInsertItem::VertexProperty(
                            id,
                            centrality_property_name.clone(),
                            centrality.into(),
                        )
                    })
                    .collect();
                datastore.bulk_insert(properties)?;

                return Ok(delta.into());
            }
        }

        Err(plugin::Error::Other(Box::new(DidNotConvergeError {
            target_delta: max_delta,
            iterations: max_iterations,
        })))
    }
}

fn parse_identifier(arg: &serde_json::Value, name: &str) -> Result<Option<indradb::Identifier>, plugin::Error> {
    if let Some(value) = arg.get(name) {
        if let serde_json::Value::String(s) = value {
            let ident = indradb::Identifier::new(s).map_err(|err| {
                plugin::Error::InvalidArgument(format!("'{}' is not a valid identifier: {}", name, err))
            })?;
            return Ok(Some(ident));
        }
        Err(plugin::Error::InvalidArgument(format!("'{}' is not a string", name)))
    } else {
        Ok(None)
    }
}

fn parse_max_iterations(arg: &serde_json::Value) -> Result<u16, plugin::Error> {
    if let Some(value) = arg.get("max_iterations") {
        if let serde_json::Value::Number(num) = value {
            if let Some(num_u64) = num.as_u64() {
                if num_u64 < u16::max_value() as u64 {
                    return Ok(num_u64 as u16);
                }
            }
        }
        Err(plugin::Error::InvalidArgument(
            "'max_iterations' is not a u16".to_string(),
        ))
    } else {
        Ok(DEFAULT_MAX_ITERATIONS)
    }
}

fn parse_max_delta(arg: &serde_json::Value) -> Result<f64, plugin::Error> {
    match arg.get("max_delta") {
        Some(value) if value.is_f64() => Ok(value.as_f64().unwrap()),
        Some(_) => Err(plugin::Error::InvalidArgument("'max_delta' is not an f64".to_string())),
        None => Ok(DEFAULT_MAX_DELTA),
    }
}

fn parse_cache_edges(arg: &serde_json::Value) -> Result<bool, plugin::Error> {
    match arg.get("cache_edges") {
        Some(value) if value.is_boolean() => Ok(value.as_bool().unwrap()),
        Some(_) => Err(plugin::Error::InvalidArgument(
            "'cache_edges' is not a bool".to_string(),
        )),
        None => Ok(DEFAULT_CACHE_EDGES),
    }
}

plugin::register_plugins!(0, "centrality", Box::new(crate::CentralityPlugin {}));
