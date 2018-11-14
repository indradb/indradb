use super::edges::EdgeKey;
use super::vertices::Vertex;
use super::ids::Id;
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BulkInsertItem {
    Vertex(Vertex),
    Edge(EdgeKey),
    VertexProperty(Id, String, JsonValue),
    EdgeProperty(EdgeKey, String, JsonValue),
}
