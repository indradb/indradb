use super::types::Type;
use super::ids::Id;
use util::generate_uuid_v1;

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    /// The id of the vertex.
    pub id: Id,

    /// The type of the vertex.
    #[serde(rename = "type")]
    pub t: Type,
}

impl Vertex {
    /// Creates a new vertex with an ID generated via UUIDv1. These vertex IDs
    /// are trivially guessable and consequently less secure, but likely index
    /// better depending on the datastore. This method is suggested unless you
    /// need vertex IDs to not be trivially guessable.
    ///
    /// # Arguments
    ///
    /// * `t` - The type of the vertex.
    pub fn new(t: Type) -> Self {
        let id = Id::new(generate_uuid_v1().to_hyphenated().to_string()).unwrap();
        Self::with_id(id, t)
    }

    /// Creates a new vertex with a specified id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the vertex.
    /// * `t` - The type of the vertex.
    pub fn with_id(id: Id, t: Type) -> Self {
        Vertex { id, t }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.id == other.id
    }
}

impl Eq for Vertex {}
