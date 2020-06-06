use super::types::Type;
use std::hash::{Hash, Hasher};

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug)]
pub struct Vertex {
    /// The id of the vertex.
    pub id: u64,

    /// The type of the vertex.
    pub t: Type,
}

impl Vertex {
    /// Creates a new vertex.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the vertex.
    /// * `t` - The type of the vertex.
    pub fn new(id: u64, t: Type) -> Self {
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
