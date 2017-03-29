use errors::Error;
use std::vec::Vec;
use serde_json::value::Value as JsonValue;
use models;
use uuid::Uuid;
use std::collections::HashMap;
use super::models::*;

/// Specifies a datastore implementation.
///
/// Datastores are responsible for managing accounts, and constructing new
/// transactions.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub trait Datastore<T: Transaction> {
    /// Checks if an account exists.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account to check.
    fn has_account(&self, account_id: Uuid) -> Result<bool, Error>;

    /// Creates a new account, returning a tuple of its ID and secret.
    ///
    /// # Arguments
    /// * `email` - The email of the account.
    fn create_account(&self, email: String) -> Result<(Uuid, String), Error>;

    /// Deletes an account.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account to delete.
    ///
    /// # Errors
    /// Returns an error if the account does not exist.
    fn delete_account(&self, account_id: Uuid) -> Result<(), Error>;

    /// Checks account authentication.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account to authenticate.
    /// * `secret` - The account's secret.
    fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error>;

    /// Creates a new transaction tied to a given account.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account that's triggering the
    /// transaction.
    fn transaction(&self, account_id: Uuid) -> Result<T, Error>;
}

/// Specifies a transaction implementation, which are returned by datastores.
/// Transactions are responsible for managing:
/// 
/// 1. Vertices.
/// 2. Edges, which connect two vertices.
/// 3. Global metadata: metadata that is not owned by anything, and as a
///    result needs to be manually managed.
/// 4. Account metadata: metadata that is owned by an account, and will be
///    automatically deleted when the associated account is deleted.
/// 5. Vertex metadata: metadata that is owned by a vertex, and will be
///    automatically deleted when the associated vertex is deleted.
/// 6. Edge metadata: metadata that is owned by an edge, and will be
///    automatically deleted when the associated edge is deleted.
pub trait Transaction {
    /// Creates a new vertex.
    ///
    /// # Arguments
    /// * `t` - The type of the vertex.
    fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error>;

    /// Gets a range of vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<models::Vertex>, Error>;

    /// Sets the type of existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `t` - The type to set.
    fn set_vertices(&self, q: VertexQuery, t: models::Type) -> Result<(), Error>;

    /// Deletes existing vertices specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error>;

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime and weight. The transaction tied to the
    /// account must own the vertex from which the edge is outbounding from,
    /// but does not need to own the inbounding vertex.
    ///
    /// # Arguments
    /// `edge`: The edge to create.
    ///
    /// # Errors
    /// Return `Error::VertexNotFound` if either of the connected vertices do
    /// not exist. Returns `Error::Unauthorized` if the account tied to the
    /// current transaction does not own the source vertex.
    fn create_edge(&self, edge: models::Edge) -> Result<(), Error>;
    
    /// Gets a range of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<models::Edge>, Error>;

    /// Updates an existing set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn set_edges(&self, q: EdgeQuery, weight: models::Weight) -> Result<(), Error>;

    /// Deletes a set of edges specified by a query.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error>;

    /// Gets the number of edges that match a query.
    ///
    /// # Arguments
    /// * `outbound_id` - The ID of the outbound vertex.
    /// * `t` - The edge type.
    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error>;

    /// Gets a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_global_metadata(&self, name: String) -> Result<JsonValue, Error>;

    /// Sets a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_global_metadata(&self, name: String, value: JsonValue) -> Result<(), Error>;

    /// Deletes a global metadata value.
    ///
    /// # Arguments
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_global_metadata(&self, name: String) -> Result<(), Error>;

    /// Gets an account metadata value.
    ///
    /// # Arguments
    /// * `account_id`: The ID of the account that the metadata is tied to.
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_account_metadata(&self, account_id: Uuid, name: String) -> Result<JsonValue, Error>;

    /// Sets an account metadata value.
    ///
    /// # Arguments
    /// * `account_id`: The ID of the account that the metadata is tied to.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    ///
    /// # Errors
    /// Returns `Error::AccountNotFound` if the specified account ID does not
    /// exist.
    fn set_account_metadata(&self, account_id: Uuid, name: String, value: JsonValue) -> Result<(), Error>;

    /// Deletes an account metadata value.
    ///
    /// # Arguments
    /// * `account_id`: The ID of the account that the metadata is tied to.
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_account_metadata(&self, account_id: Uuid, name: String) -> Result<(), Error>;

    /// Gets a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn get_vertex_metadata(&self, q: VertexQuery, name: String) -> Result<HashMap<Uuid, JsonValue>, Error>;

    /// Sets a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_vertex_metadata(&self, q: VertexQuery, name: String, value: JsonValue) -> Result<(), Error>;

    /// Deletes a vertex metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn delete_vertex_metadata(&self, q: VertexQuery, name: String) -> Result<(), Error>;

    /// Gets an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn get_edge_metadata(&self, q: EdgeQuery, name: String) -> Result<HashMap<(Uuid, Type, Uuid), JsonValue>, Error>;

    /// Sets an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_edge_metadata(&self, q: EdgeQuery, name: String, value: JsonValue) -> Result<(), Error>;

    /// Deletes an edge metadata value.
    ///
    /// # Arguments
    /// * `q` - The query to run.
    /// * `name` - The metadata name.
    fn delete_edge_metadata(&self, q: EdgeQuery, name: String) -> Result<(), Error>;

    /// Commits the transaction.
    fn commit(self) -> Result<(), Error>;

    /// Rolls the transaction back.
    fn rollback(self) -> Result<(), Error>;
}
