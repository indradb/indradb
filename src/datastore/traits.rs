use errors::Error;
use traits::Id;
use std::vec::Vec;
use serde_json::value::Value as JsonValue;
use models;
use std::option::Option;
use chrono::{DateTime, UTC};

/// Specifies a datastore implementation.
///
/// Datastores are responsible for managing accounts, and constructing new
/// transactions.
///
/// # Errors
/// All methods may return an error if something unexpected happens - e.g.
/// if there was a problem connecting to the underlying database.
pub trait Datastore<T: Transaction<I>, I: Id> {
    /// Checks if an account exists.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account to check.
    fn has_account(&self, account_id: I) -> Result<bool, Error>;

    /// Creates a new account, returning a tuple of its ID and secret.
    ///
    /// # Arguments
    /// * `email` - The email of the account.
    fn create_account(&self, email: String) -> Result<(I, String), Error>;

    /// Deletes an account.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account to delete.
    ///
    /// # Errors
    /// Returns an error if the account does not exist.
    fn delete_account(&self, account_id: I) -> Result<(), Error>;

    /// Checks account authentication.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account to authenticate.
    /// * `secret` - The account's secret.
    fn auth(&self, account_id: I, secret: String) -> Result<bool, Error>;

    /// Creates a new transaction tied to a given account.
    ///
    /// # Arguments
    /// * `account_id` - The ID of the account that's triggering the
    /// transaction.
    fn transaction(&self, account_id: I) -> Result<T, Error>;
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
pub trait Transaction<I: Id> {
    /// Gets a range of vertices.
    ///
    /// # Arguments
    /// * `start_id` - Only vertices whose ID is greater than or equal to
    ///   this ID will be returned.
    /// * `limit` - The number of vertices to return.
    fn get_vertex_range(&self, start_id: I, limit: u16) -> Result<Vec<models::Vertex<I>>, Error>;

    /// Gets a vertex.
    ///
    /// # Arguments
    /// * `id` - The ID of the vertex.
    ///
    /// # Errors
    /// Returns `Error::VertexNotFound` if the vertex does not exist.
    fn get_vertex(&self, id: I) -> Result<models::Vertex<I>, Error>;

    /// Creates a new vertex.
    ///
    /// # Arguments
    /// * `t` - The type of the vertex.
    fn create_vertex(&self, t: models::Type) -> Result<I, Error>;

    /// Updates an existing vertex.
    ///
    /// # Arguments
    /// * `vertex` - The vertex model to update.
    ///
    /// # Errors
    /// Returns `Error::VertexNotFound` if the vertex does not exist, or
    /// `Error::Unauthorized` if the vertex is not owned by the current
    /// transaction's account.
    fn set_vertex(&self, vertex: models::Vertex<I>) -> Result<(), Error>;

    /// Deletes a vertex.
    ///
    /// # Arguments
    /// * `id` - The ID of the vertex to delete.
    ///
    /// # Errors
    /// Returns `Error::VertexNotFound` if the vertex does not exist, or
    /// `Error::Unauthorized` if the vertex is not owned by the current
    /// transaction's account.
    fn delete_vertex(&self, id: I) -> Result<(), Error>;
    
    /// Gets an edge.
    ///
    /// # Arguments
    /// * `outbound_id` - The ID of the outbound vertex.
    /// * `t` - The edge type.
    /// * `outbound_id` - The ID of the inbound vertex.
    ///
    /// # Errors
    /// Returns `Error::EdgeNotFound` if the edge does not exist.
    fn get_edge(&self, outbound_id: I, t: models::Type, inbound_id: I) -> Result<models::Edge<I>, Error>;

    /// Creates an edge, or updates an existing edge.
    ///
    /// # Arguments
    /// * `edge` - The edge model to create or update.
    ///
    /// # Errors
    /// Returns `Errors::EdgeNotFound` if the edge does not exist, or
    /// `Error::Unauthorized` if the edge's outbound vertex is not owned by
    /// the current transaction's account.
    fn set_edge(&self, models::Edge<I>) -> Result<(), Error>;

    /// Deletes an edge.
    ///
    /// # Arguments
    /// * `outbound_id` - The ID of the outbound vertex.
    /// * `t` - The edge type.
    /// * `outbound_id` - The ID of the inbound vertex.
    ///
    /// # Errors
    /// Returns `Errors::EdgeNotFound` if the edge does not exist, or
    /// `Error::Unauthorized` if the edge's outbound vertex is not owned by
    /// the current transaction's account.
    fn delete_edge(&self, outbound_id: I, t: models::Type, inbound_id: I) -> Result<(), Error>;

    /// Gets the number of outbound edges of a given type.
    ///
    /// Note that this will not error out if the vertex does not exist - it
    /// will just return 0.
    ///
    /// # Arguments
    /// * `outbound_id` - The ID of the outbound vertex.
    /// * `t` - The edge type.
    fn get_edge_count(&self, outbound_id: I, t: Option<models::Type>) -> Result<u64, Error>;

    /// Gets a range of the outbound edges of a given type.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    ///
    /// # Arguments
    /// * `outbound_id` - The ID of the outbound vertex.
    /// * `t` - The edge type.
    /// * `start_inbound_id` - Only edges whose inbound ID is greater than or
    ///   equal to this ID will be returned.
    /// * `limit` - The number of edges to return.
    fn get_edge_range(&self, outbound_id: I, t: Option<models::Type>, start_inbound_id: I, limit: u16) -> Result<Vec<models::Edge<I>>, Error>;

    /// Gets a range of the outbound edges of a given type, optionally bounded
    /// by the specified update/creation datetime upper and lower bounds.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    ///
    /// # Arguments
    /// * `outbound_id` - The ID of the outbound vertex.
    /// * `t` - The edge type.
    /// * `high` - The maximum allowed edge update datetime.
    /// * `low` - The minimum allowed edge update datetime.
    /// * `limit` - The number of edges to return.
    fn get_edge_time_range(&self,
                           outbound_id: I,
                           t: Option<models::Type>,
                           high: Option<DateTime<UTC>>,
                           low: Option<DateTime<UTC>>,
                           limit: u16)
                           -> Result<Vec<models::Edge<I>>, Error>;

    /// Gets the number of inbound edges of a given type.
    ///
    /// Note that this will not error out if the vertex does not exist - it
    /// will just return 0.
    ///
    /// # Arguments
    /// * `inbound_id` - The ID of the inbound vertex.
    /// * `t` - The edge type.
    fn get_reversed_edge_count(&self, inbound_id: I, t: Option<models::Type>) -> Result<u64, Error>;

    /// Gets a range of the inbound edges of a given type.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    ///
    /// # Arguments
    /// * `inbound_id` - The ID of the inbound vertex.
    /// * `t` - The edge type.
    /// * `start_outbound_id` - Only edges whose outbound ID is greater than
    ///   or equal to this ID will be returned.
    /// * `limit` - The number of edges to return.
    fn get_reversed_edge_range(&self,
                               inbound_id: I,
                               t: Option<models::Type>,
                               start_outbound_id: I,
                               limit: u16)
                               -> Result<Vec<models::Edge<I>>, Error>;

    /// Gets a range of the inbound edges of a given type, optionally bounded
    /// by the specified update/creation datetime upper and lower bounds.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    ///
    /// # Arguments
    /// * `inbound_id` - The ID of the inbound vertex.
    /// * `t` - The edge type.
    /// * `high` - The maximum allowed edge update datetime.
    /// * `low` - The minimum allowed edge update datetime.
    /// * `limit` - The number of edges to return.
    fn get_reversed_edge_time_range(&self,
                                    inbound_id: I,
                                    t: Option<models::Type>,
                                    high: Option<DateTime<UTC>>,
                                    low: Option<DateTime<UTC>>,
                                    limit: u16)
                                    -> Result<Vec<models::Edge<I>>, Error>;

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
    fn get_account_metadata(&self, account_id: I, name: String) -> Result<JsonValue, Error>;

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
    fn set_account_metadata(&self, account_id: I, name: String, value: JsonValue) -> Result<(), Error>;

    /// Deletes an account metadata value.
    ///
    /// # Arguments
    /// * `account_id`: The ID of the account that the metadata is tied to.
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_account_metadata(&self, account_id: I, name: String) -> Result<(), Error>;

    /// Gets a vertex metadata value.
    ///
    /// # Arguments
    /// * `vertex_id`: The ID of the vertex that the metadata is tied to.
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Return `Error::VertexNotFound` if the specified vertex does not
    /// exist. Returns `Error::MetadataNotFound` if the metadata does not
    /// exist.
    fn get_vertex_metadata(&self, vertex_id: I, name: String) -> Result<JsonValue, Error>;

    /// Sets a vertex metadata value.
    ///
    /// # Arguments
    /// * `vertex_id`: The ID of the vertex that the metadata is tied to.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    ///
    /// # Errors
    /// Return `Error::VertexNotFound` if the specified vertex does not
    /// exist.
    fn set_vertex_metadata(&self, vertex_id: I, name: String, value: JsonValue) -> Result<(), Error>;

    /// Deletes a vertex metadata value.
    ///
    /// # Arguments
    /// * `vertex_id`: The ID of the vertex that the metadata is tied to.
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_vertex_metadata(&self, vertex_id: I, name: String) -> Result<(), Error>;

    /// Gets an edge metadata value.
    ///
    /// # Arguments
    /// * `edge_outbound_id`: The outbound vertex ID of the edge that the
    ///   metadata is tied to.
    /// * `edge_t`: The type the edge that the metadata is tied to.
    /// * `edge_inbound_id`: The inbound vertex ID of the edge that the
    ///   metadata is tied to.
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::EdgeNotFound` if the specified edge does not exist.
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_edge_metadata(&self, edge_outbound_id: I, edge_t: models::Type, edge_inbound_id: I, name: String) -> Result<JsonValue, Error>;

    /// Sets an edge metadata value.
    ///
    /// # Arguments
    /// * `edge_outbound_id`: The outbound vertex ID of the edge that the
    ///   metadata is tied to.
    /// * `edge_t`: The type the edge that the metadata is tied to.
    /// * `edge_inbound_id`: The inbound vertex ID of the edge that the
    ///   metadata is tied to.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    ///
    /// # Errors
    /// Returns `Error::EdgeNotFound` if the specified edge does not exist.
    fn set_edge_metadata(&self, outbound_id: I, t: models::Type, inbound_id: I, name: String, value: JsonValue) -> Result<(), Error>;

    /// Deletes an edge metadata value.
    ///
    /// # Arguments
    /// * `edge_outbound_id`: The outbound vertex ID of the edge that the
    ///   metadata is tied to.
    /// * `edge_t`: The type the edge that the metadata is tied to.
    /// * `edge_inbound_id`: The inbound vertex ID of the edge that the
    ///   metadata is tied to.
    /// * `name` - The metadata name.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_edge_metadata(&self, outbound_id: I, t: models::Type, inbound_id: I, name: String) -> Result<(), Error>;

    /// Commits the transaction.
    fn commit(self) -> Result<(), Error>;

    /// Rolls the transaction back.
    fn rollback(self) -> Result<(), Error>;
}
