use super::edges::EdgeKey;
use super::vertices::Vertex;
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Vertex),
    Edge(EdgeKey),
    VertexProperty(Uuid, String, JsonValue),
    EdgeProperty(EdgeKey, String, JsonValue),
}
