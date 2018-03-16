use super::edges::EdgeKey;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Represents vertex metadata.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VertexMetadata {
    /// The id of the vertex
    pub id: Uuid,

    /// The metadata value.
    pub value: JsonValue,
}

impl VertexMetadata {
    /// Creates a new vertex metadata.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the vertex.
    /// * `value` - The metadata value.
    pub fn new(id: Uuid, value: JsonValue) -> Self {
        Self {
            id: id,
            value: value,
        }
    }
}

/// Represents edge metadata.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EdgeMetadata {
    /// The key to the edge.
    pub key: EdgeKey,

    /// The metadata value.
    pub value: JsonValue,
}

impl EdgeMetadata {
    /// Creates a new vertex metadata.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to the edge.
    /// * `value` - The metadata value.
    pub fn new(key: EdgeKey, value: JsonValue) -> Self {
        Self {
            key: key,
            value: value,
        }
    }
}
