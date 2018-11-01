use super::vertices::Vertex;
use super::edges::EdgeKey;
use uuid::Uuid;
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Vertex),
    Edge(EdgeKey),
    VertexProperty(Uuid, String, JsonValue),
    EdgeProperty(EdgeKey, String, JsonValue),
}
