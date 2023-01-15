//! A demonstration plugin that uses the vertex mapping infrastructure to
//! count vertices. You wouldn't actually want to use this since IndraDB has
//! much faster built-in functionality for counting vertices.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use indradb_plugin_host as plugin;

struct NaiveVertexCountMapper {
    /// The number of vertices.
    count: AtomicU64,
}

impl plugin::util::VertexMapper for NaiveVertexCountMapper {
    fn map(&self, _vertex: indradb::Vertex) -> Result<(), plugin::Error> {
        self.count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

pub struct NaiveVertexCountPlugin {}

impl plugin::Plugin for NaiveVertexCountPlugin {
    fn call(
        &self,
        txn: plugin::DynTransaction<'_>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, plugin::Error> {
        let mapper = NaiveVertexCountMapper {
            count: AtomicU64::new(0),
        };
        plugin::util::map(txn, mapper.map, None)?;
        let count = mapper.count.load(Ordering::Relaxed);
        Ok(count.into())
    }
}

plugin::register_plugins!(1, "naive_vertex_count", || Box::new(crate::NaiveVertexCountPlugin {
    db
}));
