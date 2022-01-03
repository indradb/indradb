use crate::errors::{Error, Result};
use crate::models;
use crate::models::QueryExt;
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

    /// Gets the number of vertices in the datastore.
    fn get_vertex_count(&self) -> Result<u64>;

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `key`: The edge to create.
    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool>;

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    fn get_edge_count(&self, id: Uuid, t: Option<&models::Identifier>, direction: models::EdgeDirection)
        -> Result<u64>;

    fn get(&self, q: models::Query) -> Result<Vec<(models::Query, models::QueryOutputValue)>>;
    fn delete(&self, q: models::Query) -> Result<()>;

    fn get_all_properties(&self, q: models::Query) -> Result<Vec<(models::Query, Vec<(models::Identifier, serde_json::Value)>)>>;

    fn get_properties(&self, q: models::Query, name: models::Identifier) -> Result<Vec<(models::Query, serde_json::Value)>>;

    /// Sets properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    fn set_properties(&self, q: models::Query, name: models::Identifier, value: serde_json::Value) -> Result<()>;

    fn delete_properties(&self, q: models::Query, name: models::Identifier) -> Result<()>;

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
}
