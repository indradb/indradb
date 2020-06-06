use crate::{Edge, Type};
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Type),
    Edge(Edge),
    VertexProperty(u64, String, JsonValue),
    EdgeProperty(Edge, String, JsonValue),
}

#[derive(Clone, Debug, PartialEq)]
pub struct BulkInsertResult {
    pub id_range: Option<(u64, u64)>,
}
