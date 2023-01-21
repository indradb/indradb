use crate::{util::generate_uuid_v1, Identifier};
use std::hash::{Hash, Hasher};
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
    pub t: Identifier,
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
    pub fn new(t: Identifier) -> Self {
        Self::with_id(generate_uuid_v1(), t)
    }

    /// Creates a new vertex with a specified id.
    ///
    /// # Arguments
    ///
    /// * `id`: The id of the vertex.
    /// * `t`: The type of the vertex.
    pub fn with_id(id: Uuid, t: Identifier) -> Self {
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

#[cfg(test)]
mod tests {
    use super::Vertex;
    use crate::Identifier;
    use std::collections::HashSet;
    use uuid::Uuid;

    #[test]
    fn should_hash() {
        assert_eq!(
            HashSet::from([Vertex::with_id(Uuid::default(), Identifier::new("foo").unwrap())]),
            HashSet::from([Vertex::with_id(Uuid::default(), Identifier::new("foo").unwrap())])
        );
    }
}
