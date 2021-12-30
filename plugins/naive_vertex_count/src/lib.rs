use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

struct NaiveVertexCountMapper {
    count: AtomicU64,
    t_filter: Option<indradb::Identifier>,
}

impl indradb_plugin_map_util::VertexMapper for NaiveVertexCountMapper {
    fn t_filter(&self) -> Option<indradb::Identifier> {
        self.t_filter.clone()
    }

    fn map(&self, _vertex: indradb::Vertex) -> Result<(), indradb::Error> {
        self.count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

pub struct NaiveVertexCountPlugin {}

impl indradb_plugin_host::Plugin for NaiveVertexCountPlugin {
    fn call(
        &self,
        trans: Box<dyn indradb::Transaction + Send>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, indradb::Error> {
        let mapper = Arc::new(NaiveVertexCountMapper {
            count: AtomicU64::new(0),
            t_filter: arg
                .get("t_filter")
                .map(|t_filter| indradb::Identifier::new(t_filter.as_str().unwrap()).unwrap()),
        });

        indradb_plugin_map_util::map(mapper.clone(), trans)?;
        let count = mapper.count.load(Ordering::Relaxed);
        Ok(count.into())
    }
}

indradb_plugin_host::register_plugins!(0, "naive_vertex_count", Box::new(crate::NaiveVertexCountPlugin {}));
