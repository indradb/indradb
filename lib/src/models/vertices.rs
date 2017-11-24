use uuid::Uuid;
use super::types::Type;

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    /// The id of the vertex.
    pub id: Uuid,

    /// The type of the vertex.
    #[serde(rename = "type")]
    pub t: Type,
}

impl Vertex {
    /// Creates a new vertex.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the vertex.
    /// * `t` - The type of the vertex.
    pub fn new(id: Uuid, t: Type) -> Vertex {
        Vertex { id: id, t: t }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id
    }
}

impl Eq for Vertex {}
