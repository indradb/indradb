use regex::Regex;
use errors::ValidationError;
use core::str::FromStr;
use uuid::Uuid;
use chrono::{UTC, DateTime};

lazy_static! {
	static ref TYPE_VALIDATOR: Regex = Regex::new("^[a-zA-Z0-9-_]+$").unwrap();
}

/// A vertex.
///
/// Vertices are how you would represent nouns in the datastore. An example
/// might be a user, or a movie. All vertices have a unique ID and a type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    /// The id of the vertex.
    pub id: Uuid,

    /// The type of the vertex.
    #[serde(rename="type")]
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

/// An edge.
///
/// Edges are how you would represent a verb or a relationship in the
/// datastore. An example might be "liked" or "reviewed". Edges are typed,
/// weighted and directed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    /// The id of the outbound vertex.
    pub outbound_id: Uuid,
    #[serde(rename="type")]

    /// The type of the edge.
    pub t: Type,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,

    /// The weight of the edge.
    pub weight: Weight,

    /// When the edge was last updated.
    pub update_datetime: DateTime<UTC>
}

impl Edge {
    /// Creates a new edge with the current datetime in UTC.
    ///
    /// # Arguments
    /// 
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    /// * `weight` - The weight of the edge.
    pub fn new_with_current_datetime(outbound_id: Uuid, t: Type, inbound_id: Uuid, weight: Weight) -> Edge {
        Self::new(outbound_id, t, inbound_id, weight, UTC::now())
    }

    /// Creates a new edge with a specified datetime.
    ///
    /// # Arguments
    /// 
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    /// * `weight` - The weight of the edge.
    /// * `update_datetime` - When the edge was last updated.
    pub fn new(outbound_id: Uuid, t: Type, inbound_id: Uuid, weight: Weight, update_datetime: DateTime<UTC>) -> Edge {
        Edge {
            outbound_id: outbound_id,
            t: t,
            inbound_id: inbound_id,
            weight: weight,
            update_datetime: update_datetime
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.outbound_id == other.outbound_id && self.t == other.t &&
        self.inbound_id == other.inbound_id
    }
}

impl Eq for Edge {}

/// An edge weight.
///
/// Edge weights must be between -1.0 and 1.0.
#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Weight(pub f32);

impl Weight {
    /// Constructs a new edge weight.
    ///
    /// # Arguments
    /// 
    /// * `weight` - The weight, between -1.0 and 1.0.
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
    /// # Arguments
    /// 
    /// * `t` - The type, which must be less than 256 characters long.
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

/// Returns a new ID that can be used by vertices.
pub fn id() -> Uuid {
    loop {
        let id = Uuid::new_v4();

        if id != Uuid::default() {
            return id;
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum QueryTypeConverter {
    #[serde(rename="outbound")]
    Outbound,
    #[serde(rename="inbound")]
    Inbound
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum VertexQuery {
    #[serde(rename="all")]
    All(Option<Uuid>, u32),
    #[serde(rename="vertex")]
    Vertex(Uuid),
    #[serde(rename="vertices")]
    Vertices(Vec<Uuid>),
    #[serde(rename="pipe")]
    Pipe(Box<EdgeQuery>, QueryTypeConverter, u32)
}

impl VertexQuery {
    pub fn outbound_edges(self, t: Option<Type>, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u32) -> EdgeQuery {
        EdgeQuery::Pipe(Box::new(self), QueryTypeConverter::Outbound, t, high, low, limit)
    }

    pub fn inbound_edges(self, t: Option<Type>, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u32) -> EdgeQuery {
        EdgeQuery::Pipe(Box::new(self), QueryTypeConverter::Inbound, t, high, low, limit)
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum EdgeQuery {
    #[serde(rename="all")]
    All(Option<Type>, Option<DateTime<UTC>>, Option<DateTime<UTC>>, u32),
    #[serde(rename="edge")]
    Edge(Uuid, Type, Uuid),
    #[serde(rename="edges")]
    Edges(Vec<(Uuid, Type, Uuid)>),
    #[serde(rename="pipe")]
    Pipe(Box<VertexQuery>, QueryTypeConverter, Option<Type>, Option<DateTime<UTC>>, Option<DateTime<UTC>>, u32)
}

impl EdgeQuery {
    pub fn outbound_vertices(self, limit: u32) -> VertexQuery {
        VertexQuery::Pipe(Box::new(self), QueryTypeConverter::Outbound, limit)
    }

    pub fn inbound_vertices(self, limit: u32) -> VertexQuery {
        VertexQuery::Pipe(Box::new(self), QueryTypeConverter::Inbound, limit)
    }
}