use crate::Identifier;

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
    pub t: Identifier,
}

impl Vertex {
    /// Creates a new vertex.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: The type of the vertex.
    pub fn new(id: u64, t: Identifier) -> Self {
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

    #[test]
    fn should_hash() {
        assert_eq!(
            HashSet::from([Vertex::new(0, Identifier::new("foo").unwrap())]),
            HashSet::from([Vertex::new(0, Identifier::new("foo").unwrap())])
        );
    }
}
