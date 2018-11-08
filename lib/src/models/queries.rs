use super::edges::EdgeKey;
use super::types::Type;
use chrono::offset::Utc;
use chrono::DateTime;
use errors;
use std::str::FromStr;
use uuid::Uuid;

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
pub enum VertexQuery {
    Range(RangeVertexQuery),
    Specific(SpecificVertexQuery),
    Pipe(PipeVertexQuery),
}

impl From<RangeVertexQuery> for VertexQuery {
    fn from(query: RangeVertexQuery) -> Self {
        VertexQuery::Range(query)
    }
}

impl From<SpecificVertexQuery> for VertexQuery {
    fn from(query: SpecificVertexQuery) -> Self {
        VertexQuery::Specific(query)
    }
}

impl From<PipeVertexQuery> for VertexQuery {
    fn from(query: PipeVertexQuery) -> Self {
        VertexQuery::Pipe(query)
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
}

#[derive(PartialEq, Clone, Debug)]
pub struct RangeVertexQuery {
    pub limit: u32,
    pub t: Option<Type>,
    pub start_id: Option<Uuid>,
}

impl VertexQueryExt for RangeVertexQuery {}

impl RangeVertexQuery {
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            t: None,
            start_id: None,
        }
    }

    pub fn t(self, t: Type) -> Self {
        Self {
            limit: self.limit,
            t: Some(t),
            start_id: self.start_id,
        }
    }

    pub fn start_id(self, start_id: Uuid) -> Self {
        Self {
            limit: self.limit,
            t: self.t,
            start_id: Some(start_id),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SpecificVertexQuery {
    pub ids: Vec<Uuid>,
}

impl VertexQueryExt for SpecificVertexQuery {}

impl SpecificVertexQuery {
    pub fn new(ids: Vec<Uuid>) -> Self {
        Self { ids }
    }

    pub fn single(id: Uuid) -> Self {
        Self { ids: vec![id] }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PipeVertexQuery {
    pub inner: Box<EdgeQuery>,
    pub direction: EdgeDirection,
    pub limit: u32,
    pub t: Option<Type>,
}

impl VertexQueryExt for PipeVertexQuery {}

impl PipeVertexQuery {
    pub fn new(inner: Box<EdgeQuery>, direction: EdgeDirection, limit: u32) -> Self {
        Self {
            inner,
            direction,
            limit,
            t: None,
        }
    }

    pub fn t(self, t: Type) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: Some(t),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct VertexPropertyQuery {
    pub inner: VertexQuery,
    pub name: String,
}

impl VertexPropertyQuery {
    pub fn new<S: Into<String>>(inner: VertexQuery, name: S) -> Self {
        Self {
            inner,
            name: name.into(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum EdgeQuery {
    Specific(SpecificEdgeQuery),
    Pipe(PipeEdgeQuery),
}

impl From<SpecificEdgeQuery> for EdgeQuery {
    fn from(query: SpecificEdgeQuery) -> Self {
        EdgeQuery::Specific(query)
    }
}

impl From<PipeEdgeQuery> for EdgeQuery {
    fn from(query: PipeEdgeQuery) -> Self {
        EdgeQuery::Pipe(query)
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
}

#[derive(PartialEq, Clone, Debug)]
pub struct SpecificEdgeQuery {
    pub keys: Vec<EdgeKey>,
}

impl EdgeQueryExt for SpecificEdgeQuery {}

impl SpecificEdgeQuery {
    pub fn new(keys: Vec<EdgeKey>) -> Self {
        Self { keys }
    }

    pub fn single(key: EdgeKey) -> Self {
        Self { keys: vec![key] }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PipeEdgeQuery {
    pub inner: Box<VertexQuery>,
    pub direction: EdgeDirection,
    pub limit: u32,
    pub t: Option<Type>,
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
            t: None,
            high: None,
            low: None,
        }
    }

    pub fn t(self, t: Type) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: Some(t),
            high: self.high,
            low: self.low,
        }
    }

    pub fn high(self, high: DateTime<Utc>) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: self.t,
            high: Some(high),
            low: self.low,
        }
    }

    pub fn low(self, low: DateTime<Utc>) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: self.t,
            high: self.high,
            low: Some(low),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct EdgePropertyQuery {
    pub inner: EdgeQuery,
    pub name: String,
}

impl EdgePropertyQuery {
    pub fn new<S: Into<String>>(inner: EdgeQuery, name: S) -> Self {
        Self {
            inner,
            name: name.into(),
        }
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
