use errors::Result;
use std::vec::Vec;
use serde_json::value::Value as JsonValue;
use models;
use uuid::Uuid;

/// Specifies a datastore implementation.
///
/// Datastores are responsible for providing transactions.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub trait Datastore<T: Transaction> {
    /// Creates a new transaction.
    fn transaction(&self) -> Result<T>;
}

/// Specifies a transaction implementation, which are returned by datastores.
/// Transactions are responsible for managing:
///
/// 1. Vertices.
/// 2. Edges, which connect two vertices.
/// 3. Global metadata: metadata that is not owned by anything, and as a
///    result needs to be manually managed.
/// 4. Vertex metadata: metadata that is owned by a vertex, and will be
///    automatically deleted when the associated vertex is deleted.
/// 5. Edge metadata: metadata that is owned by an edge, and will be
///    automatically deleted when the associated edge is deleted.
pub trait Transaction {
    /// Creates a new vertex.
    ///
    /// # Arguments
    /// * `t` - The type of the vertex.
    fn create_vertex(&self, t: models::Type) -> Result<Uuid>;

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_vertices(&self, q: models::VertexQuery) -> Result<Vec<models::Vertex>>;

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_vertices(&self, q: models::VertexQuery) -> Result<()>;

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `key`: The edge to create.
    fn create_edge(&self, key: models::EdgeKey) -> Result<bool>;

    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_edges(&self, q: models::EdgeQuery) -> Result<Vec<models::Edge>>;

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_edges(&self, q: models::EdgeQuery) -> Result<()>;

    /// Gets the number of edges that match a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_edge_count(&self, q: models::EdgeQuery) -> Result<u64>;

    /// Gets a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_global_metadata(&self, name: String) -> Result<Option<JsonValue>>;

    /// Sets a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_global_metadata(&self, name: String, value: JsonValue) -> Result<()>;

    /// Deletes a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    fn delete_global_metadata(&self, name: String) -> Result<()>;

    /// Gets a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn get_vertex_metadata(
        &self,
        q: models::VertexQuery,
        name: String,
    ) -> Result<Vec<models::VertexMetadata>>;

    /// Sets a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_vertex_metadata(
        &self,
        q: models::VertexQuery,
        name: String,
        value: JsonValue,
    ) -> Result<()>;

    /// Deletes a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn delete_vertex_metadata(&self, q: models::VertexQuery, name: String) -> Result<()>;

    /// Gets an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn get_edge_metadata(
        &self,
        q: models::EdgeQuery,
        name: String,
    ) -> Result<Vec<models::EdgeMetadata>>;

    /// Sets an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_edge_metadata(&self, q: models::EdgeQuery, name: String, value: JsonValue)
        -> Result<()>;

    /// Deletes an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn delete_edge_metadata(&self, q: models::EdgeQuery, name: String) -> Result<()>;

    /// Commits the transaction.
    fn commit(self) -> Result<()>;

    /// Rolls the transaction back.
    fn rollback(self) -> Result<()>;
}
