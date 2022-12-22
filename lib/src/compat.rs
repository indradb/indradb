use crate::errors::{Error, Result};
use crate::models;
use crate::models::QueryExt;
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;

fn expect_count(mut output: Vec<models::QueryOutputValue>) -> Result<u64> {
    if let Some(models::QueryOutputValue::Count(count)) = output.pop() {
        Ok(count)
    } else {
        unreachable!()
    }
}

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

/// A property.
#[derive(Clone, Debug, PartialEq)]
pub struct NamedProperty {
    /// The id of the vertex.
    pub name: models::Identifier,

    /// The property value.
    pub value: serde_json::Value,
}

impl NamedProperty {
    /// Creates a new vertex property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The property value.
    pub fn new(name: models::Identifier, value: serde_json::Value) -> Self {
        Self { name, value }
    }
}

/// A vertex with properties.
#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperties {
    /// The vertex.
    pub vertex: models::Vertex,
    /// All of the vertex's properties.
    pub props: Vec<NamedProperty>,
}

impl VertexProperties {
    /// Creates new properties for a given vertex.
    ///
    /// # Arguments
    /// * `vertex`: The vertex information
    /// * `props`: The properties
    pub fn new(vertex: models::Vertex, props: Vec<NamedProperty>) -> Self {
        VertexProperties { vertex, props }
    }
}

/// An edge with properties.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProperties {
    /// The edge.
    pub edge: models::Edge,
    /// All of the edge's properties.
    pub props: Vec<NamedProperty>,
}

impl EdgeProperties {
    /// Creates a new edge properties for a given edge.
    ///
    /// # Arguments
    /// * `edge`: The edge information
    /// * `props`: The properties
    pub fn new(edge: models::Edge, props: Vec<NamedProperty>) -> Self {
        EdgeProperties { edge, props }
    }
}

/// Represents an edge property.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProperty {
    /// The key to the edge.
    pub key: models::EdgeKey,

    /// The property value.
    pub value: serde_json::Value,
}

impl EdgeProperty {
    /// Creates a new edge property.
    ///
    /// # Arguments
    /// * `key`: The key to the edge.
    /// * `value`: The property value.
    pub fn new(key: models::EdgeKey, value: serde_json::Value) -> Self {
        Self { key, value }
    }
}

// Functions designed to closely emulate those defined in IndraDB versions < 4.
pub trait DatastoreV3CompatExt: crate::Datastore {
    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_vertices(&self, q: models::Query) -> Result<Vec<models::Vertex>> {
        if let Some(models::QueryOutputValue::Vertices(vertices)) = self.get(q)?.pop() {
            Ok(vertices)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    fn delete_vertices(&self, q: models::Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-vertices
        self.delete(q)
    }

    /// Gets the number of vertices in the datastore.
    #[deprecated(since = "4.0.0", note = "use `get` with a count query")]
    fn get_vertex_count(&self) -> Result<u64> {
        expect_count(self.get(models::AllVerticesQuery.count().into())?)
    }

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_edges(&self, q: models::Query) -> Result<Vec<models::Edge>> {
        if let Some(models::QueryOutputValue::Edges(edges)) = self.get(q)?.pop() {
            Ok(edges)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    fn delete_edges(&self, q: models::Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-edges
        self.delete(q)
    }

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    #[deprecated(since = "4.0.0", note = "use `get` with a count query")]
    fn get_edge_count(
        &self,
        id: Uuid,
        t: Option<&models::Identifier>,
        direction: models::EdgeDirection,
    ) -> Result<u64> {
        let q = models::SpecificVertexQuery::single(id);

        let q = match direction {
            models::EdgeDirection::Outbound => q.outbound(),
            models::EdgeDirection::Inbound => q.inbound(),
        };

        let q: models::Query = if let Some(t) = t {
            q.t(t.clone()).into()
        } else {
            q.into()
        };

        expect_count(self.get(q)?)
    }

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_vertex_properties(&self, q: models::Query) -> Result<Vec<VertexProperty>> {
        if let Some(models::QueryOutputValue::VertexProperties(props)) = self.get(q)?.pop() {
            let iter = props
                .into_iter()
                .map(|(vertex, _prop_name, prop_value)| VertexProperty::new(vertex.id, prop_value));
            Ok(iter.collect())
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Gets all vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_all_vertex_properties(&self, q: impl models::QueryExt) -> Result<Vec<VertexProperties>> {
        if let Some(models::QueryOutputValue::VertexProperties(props)) = self.get(q.properties().into())?.pop() {
            let mut props_by_vertex = HashMap::new();
            for (vertex, prop_name, prop_value) in props.into_iter() {
                props_by_vertex
                    .entry(vertex)
                    .or_insert_with(Vec::new)
                    .push(NamedProperty::new(prop_name, prop_value));
            }
            let mut grouped_properties = Vec::with_capacity(props_by_vertex.len());
            for (vertex, named_properties) in props_by_vertex.drain() {
                grouped_properties.push(VertexProperties::new(vertex, named_properties));
            }
            Ok(grouped_properties)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    #[deprecated(since = "4.0.0", note = "use `set_properties`")]
    fn set_vertex_properties(&self, q: models::PipePropertyQuery, value: serde_json::Value) -> Result<()> {
        if let Some(name) = q.name {
            self.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(Error::Unsupported)
        }
    }

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    fn delete_vertex_properties(&self, q: models::Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-vertex properties
        self.delete(q)
    }

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_edge_properties(&self, q: models::Query) -> Result<Vec<EdgeProperty>> {
        if let Some(models::QueryOutputValue::EdgeProperties(props)) = self.get(q)?.pop() {
            let iter = props
                .into_iter()
                .map(|(edge, _prop_name, prop_value)| EdgeProperty::new(edge.key, prop_value));
            Ok(iter.collect())
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Gets all edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `get`")]
    fn get_all_edge_properties(&self, q: impl models::QueryExt) -> Result<Vec<EdgeProperties>> {
        if let Some(models::QueryOutputValue::EdgeProperties(props)) = self.get(q.properties().into())?.pop() {
            let mut props_by_edge = HashMap::new();
            let mut edges_by_key = HashMap::new();
            for (edge, prop_name, prop_value) in props.into_iter() {
                props_by_edge
                    .entry(edge.key.clone())
                    .or_insert_with(Vec::new)
                    .push(NamedProperty::new(prop_name, prop_value));
                edges_by_key.entry(edge.key.clone()).or_insert(edge);
            }
            let mut grouped_properties = Vec::with_capacity(props_by_edge.len());
            for (key, named_properties) in props_by_edge.drain() {
                grouped_properties.push(EdgeProperties::new(edges_by_key[&key].clone(), named_properties));
            }
            Ok(grouped_properties)
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    #[deprecated(since = "4.0.0", note = "use `set_properties`")]
    fn set_edge_properties(&self, q: models::PipePropertyQuery, value: serde_json::Value) -> Result<()> {
        if let Some(name) = q.name {
            self.set_properties(*q.inner, name, value)
        } else {
            // Name must be specified for this compat fn to work
            Err(Error::Unsupported)
        }
    }

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    #[deprecated(since = "4.0.0", note = "use `delete`")]
    fn delete_edge_properties(&self, q: models::Query) -> Result<()> {
        // NOTE: this runs the risk of deleting non-edge properties
        self.delete(q)
    }
}
