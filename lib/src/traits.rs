use errors::Result;
use models;
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
    type VertexIterator: VertexIterator;

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

    /// Creates a new edge. If the edge already exists, this will update it
    /// with a new update datetime. Returns whether the edge was successfully
    /// created - if this is false, it's because one of the specified vertices
    /// is missing.
    ///
    /// # Arguments
    /// * `key`: The edge to create.
    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool>;

    /// Sets a vertex metadata value.
    ///
    /// # Arguments
    /// * `id` - The vertex ID.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_vertex_metadata(&self, id: Uuid, name: &str, value: &JsonValue) -> Result<()>;

    /// Sets an edge metadata value.
    ///
    /// # Arguments
    /// * `key` - The edge key.
    /// * `name` - The metadata name.
    /// * `value` - The metadata value.
    fn set_edge_metadata(&self, key: &models::EdgeKey, name: &str, value: &JsonValue) -> Result<()>;

    fn vertices(&self, offset: Option<Uuid>) -> Self::VertexIterator;
    fn vertex(&self, id: Uuid) -> Self::VertexIterator;
}

pub trait VertexIterator {
    type EdgeIterator: EdgeIterator;
    type VertexMetadataIterator: VertexMetadataIterator;
    type Iterator: Iterator<Item=models::Vertex>;

    fn t(self, t: models::Type) -> Self;

    fn metadata(self, name: String) -> Self::VertexMetadataIterator;
    fn outbound(self) -> Self::EdgeIterator;
    fn inbound(self) -> Self::EdgeIterator;

    fn get(&self) -> Result<Self::Iterator>;
    fn delete(&self) -> Result<()>;
}

pub trait VertexMetadataIterator {
    type Iterator: Iterator<Item=models::VertexMetadata>;

    fn get(&self) -> Result<Self::Iterator>;
    fn delete(&self) -> Result<()>;
}

pub trait EdgeIterator {
    type VertexIterator: VertexIterator;
    type EdgeMetadataIterator: EdgeMetadataIterator;
    type Iterator: Iterator<Item=models::Edge>;

    fn t(self, t: models::Type) -> Self;
    fn high(self, dt: DateTime<Utc>) -> Self;
    fn low(self, dt: DateTime<Utc>) -> Self;

    fn metadata(self, name: String) -> Self::EdgeMetadataIterator;
    fn outbound(self) -> Self::VertexIterator;
    fn inbound(self) -> Self::VertexIterator;

    fn get(&self) -> Result<Self::Iterator>;
    fn delete(&self) -> Result<()>;
}

pub trait EdgeMetadataIterator {
    type Iterator: Iterator<Item=models::EdgeMetadata>;

    fn get(&self) -> Result<Self::Iterator>;
    fn delete(&self) -> Result<()>;
}
