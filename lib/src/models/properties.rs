use super::edges::EdgeKey;
use super::ids::Id;
use serde_json::Value as JsonValue;

/// Represents a vertex property.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperty {
    /// The id of the vertex
    pub id: Id,

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
    pub fn new(id: Id, value: JsonValue) -> Self {
        Self { id, value }
    }
}

/// Represents an edge property.
#[derive(Clone, Debug, PartialEq)]
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
