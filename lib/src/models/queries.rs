use super::edges::EdgeKey;
use super::types::Type;
use chrono::offset::Utc;
use chrono::DateTime;
use errors;
use std::str::FromStr;
use uuid::Uuid;
use serde_json::Value as JsonValue;

/// Specifies what kind of items should be piped from one type of query to
/// another.
///
/// Edge and vertex queries can build off of one another via pipes - e.g. you
/// can get the outbound edges of a set of vertices by piping from a vertex
/// query to an edge query. `EdgeDirection`s are used to specify which
/// end of things you want to pipe - either the outbound items or the inbound
/// items.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Copy)]
pub enum EdgeDirection {
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "inbound")]
    Inbound,
}

impl FromStr for EdgeDirection {
    type Err = errors::ValidationError;

    fn from_str(s: &str) -> Result<EdgeDirection, Self::Err> {
        match s {
            "outbound" => Ok(EdgeDirection::Outbound),
            "inbound" => Ok(EdgeDirection::Inbound),
            _ => Err("invalid value".into()),
        }
    }
}

impl From<EdgeDirection> for String {
    fn from(d: EdgeDirection) -> Self {
        match d {
            EdgeDirection::Outbound => "outbound".to_string(),
            EdgeDirection::Inbound => "inbound".to_string(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PropertyQuery {
    pub name: String,
    pub value: JsonValue
}

impl PropertyQuery {
    fn new(name: String, value: JsonValue) -> Self {
        Self { name, value }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct TypeQuery {
    pub t: Type
}

impl TypeQuery {
    fn new(t: Type) -> Self {
        Self { t }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum VertexQuery {
    Intersection(IntersectionVertexQuery),
    Union(UnionVertexQuery),
    Range(RangeVertexQuery),
    Id(IdVertexQuery),
    Pipe(PipeVertexQuery),
    Property(PropertyQuery),
    Type(TypeQuery)
}

impl From<IntersectionVertexQuery> for VertexQuery {
    fn from(query: IntersectionVertexQuery) -> Self {
        VertexQuery::Intersection(query)
    }
}

impl From<UnionVertexQuery> for VertexQuery {
    fn from(query: UnionVertexQuery) -> Self {
        VertexQuery::Union(query)
    }
}

impl From<RangeVertexQuery> for VertexQuery {
    fn from(query: RangeVertexQuery) -> Self {
        VertexQuery::Range(query)
    }
}

impl From<IdVertexQuery> for VertexQuery {
    fn from(query: IdVertexQuery) -> Self {
        VertexQuery::Id(query)
    }
}

impl From<PipeVertexQuery> for VertexQuery {
    fn from(query: PipeVertexQuery) -> Self {
        VertexQuery::Pipe(query)
    }
}

impl From<PropertyQuery> for VertexQuery {
    fn from(query: PropertyQuery) -> Self {
        VertexQuery::Property(query)
    }
}

impl From<TypeQuery> for VertexQuery {
    fn from(query: TypeQuery) -> Self {
        VertexQuery::Type(query)
    }
}

pub trait VertexQueryExt: Into<VertexQuery> {
    fn outbound(self, limit: u32) -> PipeEdgeQuery {
        PipeEdgeQuery::new(Box::new(self.into()), EdgeDirection::Outbound, limit)
    }

    fn inbound(self, limit: u32) -> PipeEdgeQuery {
        PipeEdgeQuery::new(Box::new(self.into()), EdgeDirection::Inbound, limit)
    }

    fn property<S: Into<String>>(self, name: S) -> VertexPropertyQuery {
        VertexPropertyQuery::new(self.into(), name)
    }

    fn and_with_property<S: Into<String>>(self, name: S, value: JsonValue) -> IntersectionVertexQuery {
        IntersectionVertexQuery::new(Box::new(self.into()), Box::new(PropertyQuery::new(name.into(), value).into()))
    }

    fn or_with_property<S: Into<String>>(self, name: S, value: JsonValue) -> UnionVertexQuery {
        UnionVertexQuery::new(Box::new(self.into()), Box::new(PropertyQuery::new(name.into(), value).into()))
    }

    fn and_with_t(self, t: Type) -> IntersectionVertexQuery {
        IntersectionVertexQuery::new(Box::new(self.into()), Box::new(TypeQuery::new(t).into()))
    }

    fn or_with_t(self, t: Type) -> UnionVertexQuery {
        UnionVertexQuery::new(Box::new(self.into()), Box::new(TypeQuery::new(t).into()))
    }

    fn and_with_id(self, id: Uuid) -> IntersectionVertexQuery {
        IntersectionVertexQuery::new(Box::new(self.into()), Box::new(IdVertexQuery::new(id).into()))
    }

    fn or_with_id(self, id: Uuid) -> UnionVertexQuery {
        UnionVertexQuery::new(Box::new(self.into()), Box::new(IdVertexQuery::new(id).into()))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct IntersectionVertexQuery {
    first: Box<VertexQuery>,
    second: Box<VertexQuery>
}

impl IntersectionVertexQuery {
    fn new(first: Box<VertexQuery>, second: Box<VertexQuery>) -> Self {
        Self { first, second }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct UnionVertexQuery {
    first: Box<VertexQuery>,
    second: Box<VertexQuery>
}

impl UnionVertexQuery {
    fn new(first: Box<VertexQuery>, second: Box<VertexQuery>) -> Self {
        Self { first, second }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct RangeVertexQuery {
    pub limit: u32,
    pub start_id: Option<Uuid>,
}

impl VertexQueryExt for RangeVertexQuery {}

impl RangeVertexQuery {
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            start_id: None,
        }
    }

    pub fn start_id(self, start_id: Uuid) -> Self {
        Self {
            limit: self.limit,
            start_id: Some(start_id),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct IdVertexQuery {
    pub id: Uuid
}

impl VertexQueryExt for IdVertexQuery {}

impl IdVertexQuery {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PipeVertexQuery {
    pub inner: Box<EdgeQuery>,
    pub direction: EdgeDirection,
    pub limit: u32,
}

impl VertexQueryExt for PipeVertexQuery {}

impl PipeVertexQuery {
    pub fn new(inner: Box<EdgeQuery>, direction: EdgeDirection, limit: u32) -> Self {
        Self { inner, direction, limit }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct VertexPropertyQuery {
    pub inner: VertexQuery,
    pub name: String
}

impl VertexPropertyQuery {
    pub fn new<S: Into<String>>(inner: VertexQuery, name: S) -> Self {
        Self { inner, name: name.into() }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum EdgeQuery {
    Intersection(IntersectionEdgeQuery),
    Union(UnionEdgeQuery),
    Key(KeyEdgeQuery),
    Pipe(PipeEdgeQuery),
    Property(PropertyQuery),
    Type(TypeQuery)
}

impl From<IntersectionEdgeQuery> for EdgeQuery {
    fn from(query: IntersectionEdgeQuery) -> Self {
        EdgeQuery::Intersection(query)
    }
}

impl From<UnionEdgeQuery> for EdgeQuery {
    fn from(query: UnionEdgeQuery) -> Self {
        EdgeQuery::Union(query)
    }
}

impl From<KeyEdgeQuery> for EdgeQuery {
    fn from(query: KeyEdgeQuery) -> Self {
        EdgeQuery::Key(query)
    }
}

impl From<PipeEdgeQuery> for EdgeQuery {
    fn from(query: PipeEdgeQuery) -> Self {
        EdgeQuery::Pipe(query)
    }
}

impl From<PropertyQuery> for EdgeQuery {
    fn from(query: PropertyQuery) -> Self {
        EdgeQuery::Property(query)
    }
}

impl From<TypeQuery> for EdgeQuery {
    fn from(query: TypeQuery) -> Self {
        EdgeQuery::Type(query)
    }
}

pub trait EdgeQueryExt: Into<EdgeQuery> {
    fn outbound(self, limit: u32) -> PipeVertexQuery {
        PipeVertexQuery::new(Box::new(self.into()), EdgeDirection::Outbound, limit)
    }

    fn inbound(self, limit: u32) -> PipeVertexQuery {
        PipeVertexQuery::new(Box::new(self.into()), EdgeDirection::Inbound, limit)
    }

    fn property<S: Into<String>>(self, name: S) -> EdgePropertyQuery {
        EdgePropertyQuery::new(self.into(), name)
    }

    fn and_with_property<S: Into<String>>(self, name: S, value: JsonValue) -> IntersectionEdgeQuery {
        IntersectionEdgeQuery::new(Box::new(self.into()), Box::new(PropertyQuery::new(name.into(), value).into()))
    }

    fn or_with_property<S: Into<String>>(self, name: S, value: JsonValue) -> UnionEdgeQuery {
        UnionEdgeQuery::new(Box::new(self.into()), Box::new(PropertyQuery::new(name.into(), value).into()))
    }

    fn and_with_t(self, t: Type) -> IntersectionEdgeQuery {
        IntersectionEdgeQuery::new(Box::new(self.into()), Box::new(TypeQuery::new(t).into()))
    }

    fn or_with_t(self, t: Type) -> UnionEdgeQuery {
        UnionEdgeQuery::new(Box::new(self.into()), Box::new(TypeQuery::new(t).into()))
    }

    fn and_with_key(self, key: EdgeKey) -> IntersectionEdgeQuery {
        IntersectionEdgeQuery::new(Box::new(self.into()), Box::new(KeyEdgeQuery::new(key).into()))
    }

    fn or_with_key(self, key: EdgeKey) -> UnionEdgeQuery {
        UnionEdgeQuery::new(Box::new(self.into()), Box::new(KeyEdgeQuery::new(key).into()))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct IntersectionEdgeQuery {
    first: Box<EdgeQuery>,
    second: Box<EdgeQuery>
}

impl IntersectionEdgeQuery {
    fn new(first: Box<EdgeQuery>, second: Box<EdgeQuery>) -> Self {
        Self { first, second }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct UnionEdgeQuery {
    first: Box<EdgeQuery>,
    second: Box<EdgeQuery>
}

impl UnionEdgeQuery {
    fn new(first: Box<EdgeQuery>, second: Box<EdgeQuery>) -> Self {
        Self { first, second }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct KeyEdgeQuery {
    pub key: EdgeKey
}

impl EdgeQueryExt for KeyEdgeQuery {}

impl KeyEdgeQuery {
    pub fn new(key: EdgeKey) -> Self {
        Self { key }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PipeEdgeQuery {
    pub inner: Box<VertexQuery>,
    pub direction: EdgeDirection,
    pub limit: u32,
    pub high: Option<DateTime<Utc>>,
    pub low: Option<DateTime<Utc>>,
}

impl EdgeQueryExt for PipeEdgeQuery {}

impl PipeEdgeQuery {
    pub fn new(inner: Box<VertexQuery>, direction: EdgeDirection, limit: u32) -> Self {
        Self {
            inner,
            direction,
            limit,
            high: None,
            low: None,
        }
    }

    pub fn high(self, high: DateTime<Utc>) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            high: Some(high),
            low: self.low,
        }
    }

    pub fn low(self, low: DateTime<Utc>) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            high: self.high,
            low: Some(low),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct EdgePropertyQuery {
    pub inner: EdgeQuery,
    pub name: String
}

impl EdgePropertyQuery {
    pub fn new<S: Into<String>>(inner: EdgeQuery, name: S) -> Self {
        Self { inner, name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::EdgeDirection;
    use std::str::FromStr;

    #[test]
    fn should_convert_str_to_edge_direction() {
        assert_eq!(EdgeDirection::from_str("outbound").unwrap(), EdgeDirection::Outbound);
        assert_eq!(EdgeDirection::from_str("inbound").unwrap(), EdgeDirection::Inbound);
        assert!(EdgeDirection::from_str("foo").is_err());
    }

    #[test]
    fn should_convert_edge_direction_to_string() {
        let s: String = EdgeDirection::Outbound.into();
        assert_eq!(s, "outbound".to_string());
        let s: String = EdgeDirection::Inbound.into();
        assert_eq!(s, "inbound".to_string());
    }
}
