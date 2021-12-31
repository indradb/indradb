use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use indradb_plugin_host as plugin;

struct NaiveVertexCountMapper {
    count: AtomicU64,
    t_filter: Option<indradb::Identifier>,
}

impl plugin::util::VertexMapper for NaiveVertexCountMapper {
    fn t_filter(&self) -> Option<indradb::Identifier> {
        self.t_filter.clone()
    }

    fn map(&self, _vertex: indradb::Vertex) -> Result<(), plugin::Error> {
        self.count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

pub struct NaiveVertexCountPlugin {}

impl plugin::Plugin for NaiveVertexCountPlugin {
    fn call(
        &self,
        trans: Box<dyn indradb::Transaction + Send + Sync + 'static>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, plugin::Error> {
        let mapper = Arc::new(NaiveVertexCountMapper {
            count: AtomicU64::new(0),
            t_filter: arg
                .get("t_filter")
                .map(|t_filter| indradb::Identifier::new(t_filter.as_str().unwrap()).unwrap()),
        });

        plugin::util::map(mapper.clone(), Arc::new(trans))?;
        let count = mapper.count.load(Ordering::Relaxed);
        Ok(count.into())
    }
}

plugin::register_plugins!(0, "naive_vertex_count", Box::new(crate::NaiveVertexCountPlugin {}));
