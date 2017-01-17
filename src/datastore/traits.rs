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
    fn has_account(&self, account_id: I) -> Result<bool, Error>;

    /// Creates a new account, returning a tuple of its ID and secret.
    fn create_account(&self, email: String) -> Result<(I, String), Error>;

    /// Deletes an account.
    ///
    /// # Errors
    /// Returns an error if the account does not exist.
    fn delete_account(&self, account_id: I) -> Result<(), Error>;

    /// Checks account authentication.
    fn auth(&self, account_id: I, secret: String) -> Result<bool, Error>;

    /// Creates a new transaction tied to a given account.
    fn transaction(&self, account_id: I) -> Result<T, Error>;
}

/// Specifies a transaction implementation, which are returned by datastores.
/// Transactions are responsible for managing the following kinds of data:
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
    /// Gets a vertex.
    ///
    /// # Errors
    /// Returns `Error::VertexNotFound` if the vertex does not exist.
    fn get_vertex(&self, I) -> Result<models::Vertex<I>, Error>;

    /// Creates a new vertex.
    fn create_vertex(&self, models::Type) -> Result<I, Error>;

    /// Updates an existing vertex.
    ///
    /// # Errors
    /// Returns `Error::VertexNotFound` if the vertex does not exist, or
    /// `Error::Unauthorized` if the vertex is not owned by the current
    /// transaction's account.
    fn set_vertex(&self, models::Vertex<I>) -> Result<(), Error>;

    /// Deletes a vertex.
    ///
    /// # Errors
    /// Returns `Error::VertexNotFound` if the vertex does not exist, or
    /// `Error::Unauthorized` if the vertex is not owned by the current
    /// transaction's account.
    fn delete_vertex(&self, I) -> Result<(), Error>;

    /// Gets an edge.
    ///
    /// # Errors
    /// Returns `Error::EdgeNotFound` if the edge does not exist.
    fn get_edge(&self, I, models::Type, I) -> Result<models::Edge<I>, Error>;

    /// Creates an edge, or updates an existing edge.
    ///
    /// # Errors
    /// Returns `Errors::EdgeNotFound` if the edge does not exist, or
    /// `Error::Unauthorized` if the edge's outbound vertex is not owned by
    /// the current transaction's account.
    fn set_edge(&self, models::Edge<I>) -> Result<(), Error>;

    /// Deletes an edge.
    /// Returns `Errors::EdgeNotFound` if the edge does not exist, or
    /// `Error::Unauthorized` if the edge's outbound vertex is not owned by
    /// the current transaction's account.
    fn delete_edge(&self, I, models::Type, I) -> Result<(), Error>;

    /// Gets the number of outbound edges of a given type.
    ///
    /// Note that this will not error out if the vertex does not exist - it
    /// will just return 0.
    fn get_edge_count(&self, I, models::Type) -> Result<u64, Error>;

    /// Gets a range of the outbound edges of a given type.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    fn get_edge_range(&self, I, models::Type, u64, u16) -> Result<Vec<models::Edge<I>>, Error>;

    /// Gets a range of the outbound edges of a given type, optionally bounded
    /// by the specified update/creation datetime upper and lower bounds.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    fn get_edge_time_range(&self,
                           I,
                           models::Type,
                           Option<DateTime<UTC>>,
                           Option<DateTime<UTC>>,
                           u16)
                           -> Result<Vec<models::Edge<I>>, Error>;

    /// Gets the number of inbound edges of a given type.
    ///
    /// Note that this will not error out if the vertex does not exist - it
    /// will just return 0.
    fn get_reversed_edge_count(&self, I, models::Type) -> Result<u64, Error>;

    /// Gets a range of the inbound edges of a given type.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    fn get_reversed_edge_range(&self,
                               I,
                               models::Type,
                               u64,
                               u16)
                               -> Result<Vec<models::Edge<I>>, Error>;

    /// Gets a range of the inbound edges of a given type, optionally bounded
    /// by the specified update/creation datetime upper and lower bounds.
    ///
    /// Edges should generally be ordered by their update/creation datetime
    /// (descending), but order may not be exact depending on the
    /// implementation. Note that this will not error out if the vertex does
    /// not exist - it will just return an empty edge range.
    fn get_reversed_edge_time_range(&self,
                                    I,
                                    models::Type,
                                    Option<DateTime<UTC>>,
                                    Option<DateTime<UTC>>,
                                    u16)
                                    -> Result<Vec<models::Edge<I>>, Error>;

    /// Gets a global metadata value.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_global_metadata(&self, String) -> Result<JsonValue, Error>;

    /// Sets a global metadata value.
    fn set_global_metadata(&self, String, JsonValue) -> Result<(), Error>;

    /// Deletes a global metadata value.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_global_metadata(&self, String) -> Result<(), Error>;

    /// Gets an account metadata value.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_account_metadata(&self, I, String) -> Result<JsonValue, Error>;

    /// Sets an account metadata value.
    ///
    /// # Errors
    /// Returns `Error::AccountNotFound` if the specified account ID does not
    /// exist.
    fn set_account_metadata(&self, I, String, JsonValue) -> Result<(), Error>;

    /// Deletes an account metadata value.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_account_metadata(&self, I, String) -> Result<(), Error>;

    /// Gets a vertex metadata value.
    ///
    /// # Errors
    /// Return `Error::VertexNotFound` if the specified vertex does not
    /// exist. Returns `Error::MetadataNotFound` if the metadata does not
    /// exist.
    fn get_vertex_metadata(&self, I, String) -> Result<JsonValue, Error>;

    /// Sets a vertex metadata value.
    ///
    /// # Errors
    /// Return `Error::VertexNotFound` if the specified vertex does not
    /// exist.
    fn set_vertex_metadata(&self, I, String, JsonValue) -> Result<(), Error>;

    /// Deletes a vertex metadata value.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_vertex_metadata(&self, I, String) -> Result<(), Error>;

    /// Gets an edge metadata value.
    ///
    /// # Errors
    /// Returns `Error::EdgeNotFound` if the specified edge does not exist.
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn get_edge_metadata(&self, I, models::Type, I, String) -> Result<JsonValue, Error>;

    /// Sets an edge metadata value.
    ///
    /// # Errors
    /// Returns `Error::EdgeNotFound` if the specified edge does not exist.
    fn set_edge_metadata(&self, I, models::Type, I, String, JsonValue) -> Result<(), Error>;

    /// Deletes an edge metadata value.
    ///
    /// # Errors
    /// Returns `Error::MetadataNotFound` if the metadata does not exist.
    fn delete_edge_metadata(&self, I, models::Type, I, String) -> Result<(), Error>;

    /// Commits the transaction.
    fn commit(self) -> Result<(), Error>;

    /// Rolls the transaction back.
    fn rollback(self) -> Result<(), Error>;
}
