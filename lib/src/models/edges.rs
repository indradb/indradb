use super::types::Type;
use uuid::Uuid;

/// Represents a uniquely identifiable key to an edge.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge {
    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The type of the edge.
    pub t: Type,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,
}

impl Edge {
    /// Creates a new edge key.
    ///
    /// # Arguments
    ///
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    pub fn new(outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Edge {
        Edge {
            outbound_id,
            t,
            inbound_id,
        }
    }
}
