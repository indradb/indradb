use super::types::Type;
use crate::util::next_uuid;
use crate::errors::ValidationResult;
use uuid::Uuid;

/// Represents an edge.
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
    /// Creates a new edge.
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

    /// Produces a new edge that is a reversed version of this one; i.e.
    /// it has the same type, but the outbound and inbound IDs are flipped.
    pub fn reversed(&self) -> Edge {
        Edge::new(self.inbound_id, self.t.clone(), self.outbound_id)
    }

    // Produces the "next" edge from the current one. This is useful when
    // iterating over ranges of edges.
    pub fn next(&self) -> ValidationResult<Edge> {
        let next_outbound_id = next_uuid(self.outbound_id)?;
        Ok(Edge::new(self.inbound_id, self.t.clone(), next_outbound_id))
    }
}
