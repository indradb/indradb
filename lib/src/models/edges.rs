use super::types::Type;

/// An edge.
///
/// Edges are how you would represent a verb or a relationship in the
/// datastore. An example might be "liked" or "reviewed". Edges are typed
/// and directed.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    /// The id of the outbound vertex.
    pub outbound_id: u64,

    /// The type of the edge.
    pub t: Type,

    /// The id of the inbound vertex.
    pub inbound_id: u64,
}

impl Edge {
    /// Creates a new edge key.
    ///
    /// # Arguments
    ///
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    pub fn new(outbound_id: u64, t: Type, inbound_id: u64) -> Edge {
        Edge {
            outbound_id,
            t,
            inbound_id,
        }
    }
}
