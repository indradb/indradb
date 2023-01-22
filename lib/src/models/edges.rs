use super::Identifier;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An edge.
///
/// Edges are how you would represent a verb or a relationship in the
/// datastore. An example might be "liked" or "reviewed". Edges are typed and
/// directed.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Edge {
    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The type of the edge.
    pub t: Identifier,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,
}

impl Edge {
    /// Creates a new edge key.
    ///
    /// # Arguments
    ///
    /// * `outbound_id`: The id of the outbound vertex.
    /// * `t`: The type of the edge.
    /// * `inbound_id`: The id of the inbound vertex.
    pub fn new(outbound_id: Uuid, t: Identifier, inbound_id: Uuid) -> Edge {
        Edge {
            outbound_id,
            t,
            inbound_id,
        }
    }

    /// Produces a new edge key that is a reversed version of this one; i.e.
    /// it has the same type, but the outbound and inbound IDs are flipped.
    pub fn reversed(&self) -> Edge {
        Edge::new(self.inbound_id, self.t, self.outbound_id)
    }
}
