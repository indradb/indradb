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
    trans: Arc<Box<dyn indradb::Transaction + Send + Sync + 'static>>,
    prev_centrality_map: BTreeMap<uuid::Uuid, f64>,
    cur_centrality_map: Arc<Mutex<BTreeMap<uuid::Uuid, f64>>>,
    t_filter: Option<indradb::Identifier>,
}

impl CentralityMapper {
    fn new(
        trans: Arc<Box<dyn indradb::Transaction + Send + Sync + 'static>>,
        prev_centrality_map: BTreeMap<uuid::Uuid, f64>,
        t_filter: Option<indradb::Identifier>,
    ) -> Self {
        Self {
            trans,
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
        let linked_vertices = self.trans.get_vertices(q)?;
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
        trans: Box<dyn indradb::Transaction + Send + Sync + 'static>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, plugin::Error> {
        let trans = Arc::new(trans);
        let t_filter = arg
            .get("t_filter")
            .map(|t_filter| indradb::Identifier::new(t_filter.as_str().unwrap()).unwrap());
        // TODO: pull max iterations and max delta as configs

        let mut prev_centrality_map = BTreeMap::default();
        let mut deltas = Vec::new();
        for _ in 0..DEFAULT_MAX_ITERATIONS {
            let mut mapper = Arc::new(CentralityMapper::new(
                trans.clone(),
                prev_centrality_map.clone(),
                t_filter.clone(),
            ));
            plugin::util::map(mapper.clone(), trans.clone())?;
            let cur_centrality_map = Arc::get_mut(&mut mapper).unwrap().unpack();
            let mut delta = 0.0f64;
            for (id, centrality) in &cur_centrality_map {
                let prev_centrality = *prev_centrality_map.get(id).unwrap_or(&1.0);
                delta += f64::abs(centrality - prev_centrality);
            }
            if delta < DEFAULT_MAX_DELTA {
                return Ok(delta.into());
            }
            prev_centrality_map = cur_centrality_map;
            deltas.push(delta);
        }

        // TODO: persist

        Err(plugin::Error::Other(Box::new(DidNotConvergeError {
            target_delta: DEFAULT_MAX_DELTA,
            iterations: DEFAULT_MAX_ITERATIONS,
            deltas,
        })))
    }
}

plugin::register_plugins!(0, "centrality", Box::new(crate::CentralityPlugin {}));
