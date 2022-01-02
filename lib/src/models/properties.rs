use crate::EdgeKey;

use uuid::Uuid;

/// Represents a vertex property.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperty {
    /// The id of the vertex.
    pub id: Uuid,

    /// The property value.
    pub value: serde_json::Value,
}

impl VertexProperty {
    /// Creates a new vertex property.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `value`: The property value.
    pub fn new(id: Uuid, value: serde_json::Value) -> Self {
        Self { id, value }
    }
}

/// Represents an edge property.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProperty {
    /// The key to the edge.
    pub key: EdgeKey,

    /// The property value.
    pub value: serde_json::Value,
}

impl EdgeProperty {
    /// Creates a new edge property.
    ///
    /// # Arguments
    /// * `key`: The key to the edge.
    /// * `value`: The property value.
    pub fn new(key: EdgeKey, value: serde_json::Value) -> Self {
        Self { key, value }
    }
}
