use super::edges::Edge;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Represents a vertex property.
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProperty {
    /// The edge that this property is associated with.
    pub edge: Edge,

    /// The property value.
    pub value: JsonValue,
}

impl EdgeProperty {
    /// Creates a new edge property.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge that this property is associated with.
    /// * `value` - The property value.
    pub fn new(edge: Edge, value: JsonValue) -> Self {
        Self { edge, value }
    }
}
