use super::edges::EdgeKey;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Represents a vertex property.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VertexProperty {
    /// The id of the vertex
    pub id: Uuid,

    /// The property value.
    pub value: JsonValue,
}

impl VertexProperty {
    /// Creates a new vertex property.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the vertex.
    /// * `value` - The property value.
    pub fn new(id: Uuid, value: JsonValue) -> Self {
        Self { id, value }
    }
}

/// Represents an edge property.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EdgeProperty {
    /// The key to the edge.
    pub key: EdgeKey,

    /// The property value.
    pub value: JsonValue,
}

impl EdgeProperty {
    /// Creates a new edge property.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to the edge.
    /// * `value` - The property value.
    pub fn new(key: EdgeKey, value: JsonValue) -> Self {
        Self { key, value }
    }
}
