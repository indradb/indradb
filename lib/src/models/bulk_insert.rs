use crate::{Edge, Identifier, Vertex};

/// An item to insert, as part of a bulk insert request.
#[derive(Clone, Debug, PartialEq)]
pub enum BulkInsertItem {
    /// A vertex to insert.
    Vertex(Vertex),
    /// An edge to insert.
    Edge(Edge),
    /// A vertex property to insert.
    VertexProperty(u64, Identifier, serde_json::Value),
    /// An edge property to insert.
    EdgeProperty(Edge, Identifier, serde_json::Value),
}
