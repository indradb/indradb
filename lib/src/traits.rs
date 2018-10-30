use errors::Result;
use models;
use serde_json::value::Value as JsonValue;
use std::vec::Vec;
use uuid::Uuid;

/// Specifies a datastore implementation.
///
/// Datastores are responsible for providing transactions.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub trait Datastore {
    type Trans: Transaction;

    /// Creates a new transaction.
    fn transaction(&self) -> Result<Self::Trans>;
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
    fn create_vertex_from_type(&self, t: models::Type) -> Result<Uuid> {
        let v = models::Vertex::new(t);

        if !self.create_vertex(&v)? {
            Err("UUID already taken".into())
        } else {
            Ok(v.id)
        }
    }

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

    /// Gets vertex properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn get_vertex_properties(&self, q: &models::VertexQuery, name: &str) -> Result<Vec<models::VertexProperty>>;

    /// Sets a vertex properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    /// * `value` - The property value.
    fn set_vertex_properties(&self, q: &models::VertexQuery, name: &str, value: &JsonValue) -> Result<()>;

    /// Deletes vertex properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn delete_vertex_properties(&self, q: &models::VertexQuery, name: &str) -> Result<()>;

    /// Gets edge properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn get_edge_properties(&self, q: &models::EdgeQuery, name: &str) -> Result<Vec<models::EdgeProperty>>;

    /// Sets edge properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    /// * `value` - The property value.
    fn set_edge_properties(&self, q: &models::EdgeQuery, name: &str, value: &JsonValue) -> Result<()>;

    /// Deletes edge properties.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The property name.
    fn delete_edge_properties(&self, q: &models::EdgeQuery, name: &str) -> Result<()>;
}
