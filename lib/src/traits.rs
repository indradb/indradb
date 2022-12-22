use crate::errors::{Error, Result};
use crate::models;
use crate::models::QueryExt;
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;

/// Specifies a datastore implementation.
///
/// Note that this trait and its members purposefully do not employ any
/// generic arguments. While that would improve ergonomics, it would remove
/// object safety, which we need for plugins.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub trait Datastore {
    /// Syncs persisted content. Depending on the datastore implementation,
    /// this has different meanings - including potentially being a no-op.
    fn sync(&self) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Creates a new transaction. Some datastore implementations do not
    /// support transactional updates, in which case this will return an
    /// error.
    fn transaction(&self) -> Result<Self>
    where
        Self: Sized,
    {
        Err(Error::Unsupported)
    }

    /// Creates a new vertex. Returns whether the vertex was successfully
    /// created - if this is false, it's because a vertex with the same UUID
    /// already exists.
    ///
    /// # Arguments
    /// * `vertex`: The vertex to create.
    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool>;

    /// Creates a new vertex with just a type specification. As opposed to
    /// `create_vertex`, this is used when you do not want to manually specify
    /// the vertex's UUID. Returns the new vertex's UUID.
    ///
    /// # Arguments
    /// * `t`: The type of the vertex to create.
    fn create_vertex_from_type(&self, t: models::Identifier) -> Result<Uuid> {
        let v = models::Vertex::new(t);

        if !self.create_vertex(&v)? {
            Err(Error::UuidTaken)
        } else {
            Ok(v.id)
        }
    }

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `key`: The edge to create.
    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool>;

    fn get(&self, q: models::Query) -> Result<Vec<models::QueryOutputValue>>;
    fn delete(&self, q: models::Query) -> Result<()>;

    /// Sets properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `name`: The property name.
    /// * `value`: The property value.
    fn set_properties(&self, q: models::Query, name: models::Identifier, value: serde_json::Value) -> Result<()>;

    /// Bulk inserts many vertices, edges, and/or properties.
    ///
    /// # Arguments
    /// * `items`: The items to insert.
    fn bulk_insert(&self, items: Vec<models::BulkInsertItem>) -> Result<()> {
        for item in items {
            match item {
                models::BulkInsertItem::Vertex(vertex) => {
                    self.create_vertex(&vertex)?;
                }
                models::BulkInsertItem::Edge(edge_key) => {
                    self.create_edge(&edge_key)?;
                }
                models::BulkInsertItem::VertexProperty(id, name, value) => {
                    let query = models::SpecificVertexQuery::single(id);
                    self.set_properties(query.into(), name, value)?;
                }
                models::BulkInsertItem::EdgeProperty(edge_key, name, value) => {
                    let query = models::SpecificEdgeQuery::single(edge_key);
                    self.set_properties(query.into(), name, value)?;
                }
            }
        }

        Ok(())
    }

    // Enables indexing on a specified property. When indexing is enabled on a
    // property, it's possible to query on its presence and values.
    //
    // # Arguments
    // * `name`: The name of the property to index.
    fn index_property(&self, name: models::Identifier) -> Result<()>;

    //////////////////////////////////////////////////////////////////////////
    // All functions after this are kept to increase backward compatibility
    // with version < 4.
    //////////////////////////////////////////////////////////////////////////

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
    fn get_vertex_properties(&self, q: models::Query) -> Result<Vec<models::VertexProperty>> {
        if let Some(models::QueryOutputValue::VertexProperties(props)) = self.get(q)?.pop() {
            let iter = props
                .into_iter()
                .map(|(vertex, _prop_name, prop_value)| models::VertexProperty::new(vertex.id, prop_value));
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
    fn get_all_vertex_properties(&self, q: impl models::QueryExt) -> Result<Vec<models::VertexProperties>> {
        if let Some(models::QueryOutputValue::VertexProperties(props)) = self.get(q.properties().into())?.pop() {
            let mut props_by_vertex = HashMap::new();
            for (vertex, prop_name, prop_value) in props.into_iter() {
                props_by_vertex
                    .entry(vertex)
                    .or_insert_with(Vec::new)
                    .push(models::NamedProperty::new(prop_name, prop_value));
            }
            let mut grouped_properties = Vec::with_capacity(props_by_vertex.len());
            for (vertex, named_properties) in props_by_vertex.drain() {
                grouped_properties.push(models::VertexProperties::new(vertex, named_properties));
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
    fn get_edge_properties(&self, q: models::Query) -> Result<Vec<models::EdgeProperty>> {
        if let Some(models::QueryOutputValue::EdgeProperties(props)) = self.get(q)?.pop() {
            let iter = props
                .into_iter()
                .map(|(edge, _prop_name, prop_value)| models::EdgeProperty::new(edge.key, prop_value));
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
    fn get_all_edge_properties(&self, q: impl models::QueryExt) -> Result<Vec<models::EdgeProperties>> {
        if let Some(models::QueryOutputValue::EdgeProperties(props)) = self.get(q.properties().into())?.pop() {
            let mut props_by_edge = HashMap::new();
            let mut edges_by_key = HashMap::new();
            for (edge, prop_name, prop_value) in props.into_iter() {
                props_by_edge
                    .entry(edge.key.clone())
                    .or_insert_with(Vec::new)
                    .push(models::NamedProperty::new(prop_name, prop_value));
                edges_by_key.entry(edge.key.clone()).or_insert(edge);
            }
            let mut grouped_properties = Vec::with_capacity(props_by_edge.len());
            for (key, named_properties) in props_by_edge.drain() {
                grouped_properties.push(models::EdgeProperties::new(
                    edges_by_key[&key].clone(),
                    named_properties,
                ));
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

fn expect_count(mut output: Vec<models::QueryOutputValue>) -> Result<u64> {
    if let Some(models::QueryOutputValue::Count(count)) = output.pop() {
        Ok(count)
    } else {
        unreachable!()
    }
}
