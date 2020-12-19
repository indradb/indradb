use super::edges::Edge;
use super::vertices::Vertex;
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Vertex),
    Edge(Edge),
    VertexProperty(Uuid, String, JsonValue),
    EdgeProperty(Edge, String, JsonValue),
}
