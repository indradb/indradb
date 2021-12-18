use crate::errors::{Error, Result};
use crate::models;
use crate::models::{EdgeQueryExt, VertexQueryExt};
use std::vec::Vec;
use uuid::Uuid;

/// Specifies a datastore implementation.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub trait Datastore {
    type Trans: Transaction;

    /// Syncs persisted content. Depending on the datastore implementation,
    /// this has different meanings - including potentially being a no-op.
    fn sync(&self) -> Result<()> {
        Err(Error::Unsupported)
    }

    /// Creates a new transaction.
    fn transaction(&self) -> Result<Self::Trans>;

    /// Bulk inserts many vertices, edges, and/or properties.
    ///
    /// # Arguments
    /// * `items`: The items to insert.
    fn bulk_insert(&self, items: Vec<models::BulkInsertItem>) -> Result<()> {
        let trans = self.transaction()?;
        trans.bulk_insert(items)
    }

    // Enables indexing on a specified property. When indexing is enabled on a
    // property, it's possible to query on its presence and values.
    //
    // # Arguments
    // * `name`: The name of the property to index.
    fn index_property(&self, name: models::Identifier) -> Result<()> {
        let trans = self.transaction()?;
        trans.index_property(name)
    }
}

/// Specifies a transaction implementation, which are provided by datastores.
///
/// All datastore manipulations are done through transactions. Datastore
/// implementations carry different guarantees. Depending on the
/// implementation, it may not be possible to rollback the changes on error.
/// See the documentation of individual implementations for details.
///
/// Note that this trait and its members purposefully do not employ any
/// generic arguments. While that would improve ergonomics, it would remove
/// object safety, which we want in order to be able to pass
/// `Box<dyn Transaction>` around (e.g. for plugins.)
pub trait Transaction {
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

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn get_vertices(&self, q: models::VertexQuery) -> Result<Vec<models::Vertex>>;

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn delete_vertices(&self, q: models::VertexQuery) -> Result<()>;

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

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn get_edges(&self, q: models::EdgeQuery) -> Result<Vec<models::Edge>>;

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn delete_edges(&self, q: models::EdgeQuery) -> Result<()>;

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    fn get_edge_count(&self, id: Uuid, t: Option<&models::Identifier>, direction: models::EdgeDirection)
        -> Result<u64>;

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn get_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<Vec<models::VertexProperty>>;

    /// Gets all vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn get_all_vertex_properties(&self, q: models::VertexQuery) -> Result<Vec<models::VertexProperties>>;

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    fn set_vertex_properties(&self, q: models::VertexPropertyQuery, value: serde_json::Value) -> Result<()>;

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn delete_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<()>;

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn get_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>>;

    /// Gets all edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn get_all_edge_properties(&self, q: models::EdgeQuery) -> Result<Vec<models::EdgeProperties>>;

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    /// * `value`: The property value.
    fn set_edge_properties(&self, q: models::EdgePropertyQuery, value: serde_json::Value) -> Result<()>;

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q`: The query to run.
    fn delete_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<()>;

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
                    let query = models::SpecificVertexQuery::single(id).property(name);
                    self.set_vertex_properties(query, value)?;
                }
                models::BulkInsertItem::EdgeProperty(edge_key, name, value) => {
                    let query = models::SpecificEdgeQuery::single(edge_key).property(name);
                    self.set_edge_properties(query, value)?;
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
