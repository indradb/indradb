use crate::{Edge, Identifier, Vertex};

use uuid::Uuid;

/// An item to insert, as part of a bulk insert request.
#[derive(Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    /// A vertex to insert.
    Vertex(Vertex),
    /// An edge to insert.
    Edge(Edge),
    /// A vertex property to insert.
    VertexProperty(Uuid, Identifier, serde_json::Value),
    /// An edge property to insert.
    EdgeProperty(Edge, Identifier, serde_json::Value),
}
