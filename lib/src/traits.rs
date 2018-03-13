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
/// All datastore manipulations are done through transactions. Despite the
/// name, different datastore implementations carry different guarantees.
/// Depending on the implementation, it may not be possible to rollback the
/// changes on error. See the documentation of individual implementations for
/// details. Transactions are automatically committed on drop. Transactions
/// should be designed to not fail on commit; i.e. errors should occur when a
/// method is actually called instead.
pub trait Transaction {
    /// Creates a new vertex.
    ///
    /// # Arguments
    /// * `vertex`: The vertex to create.
    fn create_vertex(&self, vertex: &models::Vertex) -> Result<()>;

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_vertices(&self, q: &models::VertexQuery) -> Result<Vec<models::Vertex>>;

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_vertices(&self, q: &models::VertexQuery) -> Result<()>;

    /// Gets the number of vertices in the datastore..
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
    /// * `q` - The query to run.
    fn get_edges(&self, q: &models::EdgeQuery) -> Result<Vec<models::Edge>>;

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_edges(&self, q: &models::EdgeQuery) -> Result<()>;

    /// Gets the number of edges associated with a vertex.
    ///
    /// # Arguments
    /// * `id` - The id of the vertex.
    /// * `type_filter` - Only get the count for a specified edge type.
    /// * `direction`: The direction of edges to get.
    fn get_edge_count(
        &self,
        id: Uuid,
        type_filter: Option<&models::Type>,
        direction: models::EdgeDirection,
    ) -> Result<u64>;

    /// Gets a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_global_metadata(&self, name: &str) -> Result<Option<JsonValue>>;

    /// Sets a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_global_metadata(&self, name: &str, value: &JsonValue) -> Result<()>;

    /// Deletes a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    fn delete_global_metadata(&self, name: &str) -> Result<()>;

    /// Gets a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn get_vertex_metadata(
        &self,
        q: &models::VertexQuery,
        name: &str,
    ) -> Result<Vec<models::VertexMetadata>>;

    /// Sets a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_vertex_metadata(
        &self,
        q: &models::VertexQuery,
        name: &str,
        value: &JsonValue,
    ) -> Result<()>;

    /// Deletes a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn delete_vertex_metadata(&self, q: &models::VertexQuery, name: &str) -> Result<()>;

    /// Gets an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn get_edge_metadata(
        &self,
        q: &models::EdgeQuery,
        name: &str,
    ) -> Result<Vec<models::EdgeMetadata>>;

    /// Sets an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_edge_metadata(&self, q: &models::EdgeQuery, name: &str, value: &JsonValue)
        -> Result<()>;

    /// Deletes an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn delete_edge_metadata(&self, q: &models::EdgeQuery, name: &str) -> Result<()>;
}
