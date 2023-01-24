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
    fn call<'a>(
        &self,
        txn: &mut (dyn indradb::Transaction<'a> + 'a),
        _arg: indradb::Json,
    ) -> Result<indradb::Json, plugin::Error> {
        let mapper = Arc::new(NaiveVertexCountMapper {
            count: AtomicU64::new(0),
        });
        plugin::util::map(txn, mapper.clone())?;
        let count = mapper.count.load(Ordering::Relaxed);
        Ok(indradb::Json::new(count.into()))
    }
}

plugin::register_plugins!(1, "naive_vertex_count", || Box::new(crate::NaiveVertexCountPlugin {}));
