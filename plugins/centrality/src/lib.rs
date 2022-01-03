use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt;
use std::mem::take;
use std::sync::{Arc, Mutex};

use indradb::{EdgeQueryExt, VertexQueryExt};
use indradb_plugin_host as plugin;

const DEFAULT_MAX_ITERATIONS: u16 = 10;
const DEFAULT_MAX_DELTA: f64 = 1.0;

#[derive(Debug)]
pub struct DidNotConvergeError {
    target_delta: f64,
    iterations: u16,
    deltas: Vec<f64>,
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
            "did not converge to target delta {:.4} after {} iterations, deltas: ",
            self.target_delta, self.iterations
        )?;
        let mut is_first = false;
        for delta in &self.deltas {
            if !is_first {
                write!(f, ", ")?;
            }
            write!(f, "{:.4}", delta)?;
            is_first = true;
        }
        Ok(())
    }
}

// TODO: separate mapper for when there's a weight property to pull
struct CentralityMapper {
    datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
    prev_centrality_map: BTreeMap<uuid::Uuid, f64>,
    cur_centrality_map: Arc<Mutex<BTreeMap<uuid::Uuid, f64>>>,
    t_filter: Option<indradb::Identifier>,
}

impl CentralityMapper {
    fn new(
        datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
        prev_centrality_map: BTreeMap<uuid::Uuid, f64>,
        t_filter: Option<indradb::Identifier>,
    ) -> Self {
        Self {
            datastore,
            prev_centrality_map,
            cur_centrality_map: Arc::new(Mutex::new(BTreeMap::default())),
            t_filter,
        }
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
        let centrality = *self.prev_centrality_map.get(&vertex.id).unwrap_or(&1.0);
        let q = indradb::SpecificVertexQuery::single(vertex.id)
            .outbound()
            .inbound()
            .into();
        let linked_vertices = self.datastore.get_vertices(q)?;
        let vote_weight = centrality / (linked_vertices.len() as f64);
        let mut map = self.cur_centrality_map.lock().unwrap();
        for vertex in &linked_vertices {
            *map.entry(vertex.id).or_insert(0.0) += vote_weight;
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
        let t_filter = parse_t_filter(&arg)?;
        let max_iterations = parse_max_iterations(&arg)?;
        let max_delta = parse_max_delta(&arg)?;

        let mut prev_centrality_map = BTreeMap::default();
        let mut deltas = Vec::new();
        for _ in 0..max_iterations {
            let mut mapper = Arc::new(CentralityMapper::new(
                datastore.clone(),
                prev_centrality_map.clone(),
                t_filter.clone(),
            ));
            plugin::util::map(mapper.clone(), datastore.clone())?;
            let cur_centrality_map = Arc::get_mut(&mut mapper).unwrap().unpack();
            let mut delta = 0.0f64;
            for (id, centrality) in &cur_centrality_map {
                let prev_centrality = *prev_centrality_map.get(id).unwrap_or(&1.0);
                delta += f64::abs(centrality - prev_centrality);
            }
            delta /= vertex_count as f64;
            if delta < max_delta {
                return Ok(delta.into());
            }
            prev_centrality_map = cur_centrality_map;
            deltas.push(delta);
        }

        // TODO: persist

        Err(plugin::Error::Other(Box::new(DidNotConvergeError {
            target_delta: max_delta,
            iterations: max_iterations,
            deltas,
        })))
    }
}

fn parse_t_filter(arg: &serde_json::Value) -> Result<Option<indradb::Identifier>, plugin::Error> {
    if let Some(value) = arg.get("t_filter") {
        if let serde_json::Value::String(s) = value {
            let ident = indradb::Identifier::new(s).map_err(|err| plugin::Error::Other(Box::new(err)))?;
            return Ok(Some(ident));
        }
        Err(plugin::Error::InvalidArgument("'t_filter' is not a string".to_string()))
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
        Err(plugin::Error::InvalidArgument("'max_iterations' is not a u16".to_string()))
    } else {
        Ok(DEFAULT_MAX_ITERATIONS)
    }
}

fn parse_max_delta(arg: &serde_json::Value) -> Result<f64, plugin::Error> {
    match arg.get("max_delta") {
        Some(value) if value.is_f64() => Ok(value.as_f64().unwrap()),
        Some(_) => Err(plugin::Error::InvalidArgument("'max_delta' is not an f64".to_string())),
        None => Ok(DEFAULT_MAX_DELTA)
    }
}

plugin::register_plugins!(0, "centrality", Box::new(crate::CentralityPlugin {}));
