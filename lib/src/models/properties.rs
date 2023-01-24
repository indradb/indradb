use crate::{Edge, Identifier, Json, Vertex};

use uuid::Uuid;

/// Represents a vertex property.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperty {
    /// The id of the vertex.
    pub id: Uuid,

    /// The property value.
    pub value: Json,
}

impl VertexProperty {
    /// Creates a new vertex property.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `value`: The property value.
    pub fn new(id: Uuid, value: Json) -> Self {
        Self { id, value }
    }
}

/// A property.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedProperty {
    /// The property name.
    pub name: Identifier,

    /// The property value.
    pub value: Json,
}

impl NamedProperty {
    /// Creates a new vertex property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The property value.
    pub fn new(name: Identifier, value: Json) -> Self {
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
    /// The edge.
    pub edge: Edge,

    /// The property value.
    pub value: Json,
}

impl EdgeProperty {
    /// Creates a new edge property.
    ///
    /// # Arguments
    /// * `edge`: The edge.
    /// * `value`: The property value.
    pub fn new(edge: Edge, value: Json) -> Self {
        Self { edge, value }
    }
}
