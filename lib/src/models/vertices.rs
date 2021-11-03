use std::hash::{Hash, Hasher};

use crate::{util::generate_uuid_v1, Type};
use uuid::Uuid;

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug)]
pub struct Vertex {
    /// The id of the vertex.
    pub id: Uuid,

    /// The type of the vertex.
    pub t: Type,
}

impl Vertex {
    /// Creates a new vertex with an ID generated via UUIDv1. These vertex IDs
    /// are trivially guessable and consequently less secure, but index
    /// better. This method is suggested unless you need vertex IDs to not be
    /// trivially guessable.
    ///
    /// # Arguments
    ///
    /// * `t`: The type of the vertex.
    pub fn new(t: Type) -> Self {
        Self::with_id(generate_uuid_v1(), t)
    }

    /// Creates a new vertex with a specified id.
    ///
    /// # Arguments
    ///
    /// * `id`: The id of the vertex.
    /// * `t`: The type of the vertex.
    pub fn with_id(id: Uuid, t: Type) -> Self {
        Vertex { id, t }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id
    }
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Vertex {}
