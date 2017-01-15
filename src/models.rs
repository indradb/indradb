use traits::Id;
use regex::Regex;
use errors::ValidationError;
use core::str::FromStr;

lazy_static! {
	static ref TYPE_VALIDATOR: Regex = Regex::new("^[a-zA-Z0-9-_]+$").unwrap();
}

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex<I: Id> {
    pub id: I,
    #[serde(rename="type")]
    pub t: Type,
}

impl<I: Id> Vertex<I> {
    pub fn new(id: I, t: Type) -> Vertex<I> {
        Vertex { id: id, t: t }
    }
}

impl<I: Id> PartialEq for Vertex<I> {
    fn eq(&self, other: &Vertex<I>) -> bool {
        self.id == other.id
    }
}

impl<I: Id> Eq for Vertex<I> {}

/// An edge.
///
/// Edges are how you would represent a verb or a relationship in the
/// datastore. An example might be "liked" or "reviewed". Edges are typed,
/// weighted and directed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge<I: Id> {
    pub outbound_id: I,
    #[serde(rename="type")]
    pub t: Type,
    pub inbound_id: I,
    pub weight: Weight,
}

impl<I: Id> Edge<I> {
    pub fn new(outbound_id: I, t: Type, inbound_id: I, weight: Weight) -> Edge<I> {
        Edge {
            outbound_id: outbound_id,
            t: t,
            inbound_id: inbound_id,
            weight: weight,
        }
    }
}

impl<I: Id> PartialEq for Edge<I> {
    fn eq(&self, other: &Edge<I>) -> bool {
        self.outbound_id == other.outbound_id && self.t == other.t &&
        self.inbound_id == other.inbound_id
    }
}

impl<I: Id> Eq for Edge<I> {}

/// An edge weight.
///
/// Edge weights must be between -1.0 and 1.0.
#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Weight(pub f32);

impl Weight {
    /// Constructs a new edge weight.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the weight is below -1.0 or above 1.0.
    pub fn new(w: f32) -> Result<Self, ValidationError> {
        if w < -1.0 || w > 1.0 {
            Err(ValidationError::new("Weight out of range".to_string()))
        } else {
            Ok(Weight(w))
        }
    }
}

/// An edge or vertex type.
///
/// Types must be less than 256 characters long, and can only contain letters,
/// numbers, dashes and underscores.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub struct Type(pub String);

impl Type {
    /// Constructs a new type.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the type is longer than 255 characters,
    /// or has invalid characters.
    pub fn new(t: String) -> Result<Self, ValidationError> {
        if t.len() > 255 {
            Err(ValidationError::new("Type is too long".to_string()))
        } else if !TYPE_VALIDATOR.is_match(&t[..]) {
            Err(ValidationError::new("Invalid type".to_string()))
        } else {
            Ok(Type(t))
        }
    }
}

impl FromStr for Type {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string())?)
    }
}
