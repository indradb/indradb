use crate::Identifier;

use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::Utc;
use lazy_static::lazy_static;

lazy_static! {
    static ref INIT_TIMESTAMP: u64 = Utc::now().timestamp_nanos() as u64;
}

static OFFSET: AtomicU64 = AtomicU64::new(1);

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
    /// * `t`: The type of the vertex.
    pub fn new(t: Identifier) -> Self {
        let id = *INIT_TIMESTAMP + OFFSET.fetch_add(1, Ordering::Relaxed);
        Self::with_id(id, t)
    }

    /// Creates a new vertex with a specified id.
    ///
    /// # Arguments
    /// * `id`: The id of the vertex.
    /// * `t`: The type of the vertex.
    pub fn with_id(id: u64, t: Identifier) -> Self {
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
            HashSet::from([Vertex::with_id(0, Identifier::new("foo").unwrap())]),
            HashSet::from([Vertex::with_id(0, Identifier::new("foo").unwrap())])
        );
    }
}
