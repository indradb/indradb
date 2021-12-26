use std::sync::Arc;

use serde_json::json;

struct MapReduceVertexCountDriver {
    t_filter: Option<indradb::Identifier>,
}

impl indradb_plugin_map_reduce::MapReduceDriver for MapReduceVertexCountDriver {
    fn t_filter(&self) -> Option<indradb::Identifier> {
        self.t_filter.clone()
    }

    fn map(&self, _vertex: indradb::Vertex) -> Result<serde_json::Value, indradb::Error> {
        Ok(json!(1))
    }

    fn reduce(&self, values: Vec<serde_json::Value>) -> Result<serde_json::Value, indradb::Error> {
        let mut sum = 0u64;
        for value in &values {
            sum += value.as_u64().unwrap();
        }
        Ok(json!(sum))
    }
}

pub struct MapReduceVertexCountPlugin {}

impl indradb_plugin_host::Plugin for MapReduceVertexCountPlugin {
    fn call(
        &self,
        trans: Box<dyn indradb::Transaction + Send>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, indradb::Error> {
        let t_filter = arg
            .get("t_filter")
            .map(|t_filter| indradb::Identifier::new(t_filter.as_str().unwrap()).unwrap());
        let driver = MapReduceVertexCountDriver { t_filter };
        indradb_plugin_map_reduce::map_reduce(Arc::new(driver), trans)
    }
}

indradb_plugin_host::register_plugins!(0, "vertex_count", Box::new(crate::MapReduceVertexCountPlugin {}));
