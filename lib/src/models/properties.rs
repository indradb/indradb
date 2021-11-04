use crate::{Edge, EdgeKey, JsonValue, Vertex};

use uuid::Uuid;

/// Represents a vertex property.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperty {
    /// The id of the vertex.
    pub id: Uuid,

    /// The property value.
    pub value: JsonValue,
}

impl VertexProperty {
    /// Creates a new vertex property.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `value`: The property value.
    pub fn new(id: Uuid, value: JsonValue) -> Self {
        Self { id, value }
    }
}

/// A property.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedProperty {
    /// The id of the vertex.
    pub name: String,

    /// The property value.
    pub value: JsonValue,
}

impl NamedProperty {
    /// Creates a new vertex property.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `value`: The property value.
    pub fn new(name: String, value: JsonValue) -> Self {
        Self { name, value }
    }
}

/// A vertex with properties.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperties {
    /// The vertex.
    pub vertex: Vertex,
    /// All of the vertex's properties.
    pub props: Vec<NamedProperty>,
}

impl VertexProperties {
    /// Creates new properties for a given vertex.
    ///
    /// # Arguments
    /// * `vertex`: The vertex information
    /// * `props`: The properties
    pub fn new(vertex: Vertex, props: Vec<NamedProperty>) -> Self {
        VertexProperties { vertex, props }
    }
}

/// An edge with properties.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProperties {
    /// The edge.
    pub edge: Edge,
    /// All of the edge's properties.
    pub props: Vec<NamedProperty>,
}

impl EdgeProperties {
    /// Creates a new edge properties for a given edge.
    ///
    /// # Arguments
    /// * `edge`: The edge information
    /// * `props`: The properties
    pub fn new(edge: Edge, props: Vec<NamedProperty>) -> Self {
        EdgeProperties { edge, props }
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
    /// * `key`: The key to the edge.
    /// * `value`: The property value.
    pub fn new(key: EdgeKey, value: JsonValue) -> Self {
        Self { key, value }
    }
}
