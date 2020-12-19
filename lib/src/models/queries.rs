use super::edges::Edge;
use super::types::Type;
use crate::errors;
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
#[derive(Eq, PartialEq, Clone, Debug, Hash, Copy)]
pub enum EdgeDirection {
    Outbound,
    Inbound,
}

impl FromStr for EdgeDirection {
    type Err = errors::ValidationError;

    fn from_str(s: &str) -> Result<EdgeDirection, Self::Err> {
        match s {
            "outbound" => Ok(EdgeDirection::Outbound),
            "inbound" => Ok(EdgeDirection::Inbound),
            _ => Err(errors::ValidationError::InvalidValue),
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

/// A query for vertices.
///
/// Generally you shouldn't need to instantiate a `VertexQuery` directly, but
/// rather one of the vertex query structs, and then call `.into()` on it to
/// convert it to a `VertexQuery`.
#[derive(Eq, PartialEq, Clone, Debug)]
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

/// Extension trait that specifies methods exposed by all vertex queries.
pub trait VertexQueryExt: Into<VertexQuery> {
    /// Gets the outbound edges associated with the vertices.
    fn outbound(self) -> PipeEdgeQuery {
        PipeEdgeQuery::new(Box::new(self.into()), EdgeDirection::Outbound)
    }

    /// Gets the inbound edges associated with the vertices.
    fn inbound(self) -> PipeEdgeQuery {
        PipeEdgeQuery::new(Box::new(self.into()), EdgeDirection::Inbound)
    }

    /// Gets a property associated with the vertices.
    ///
    /// # Arguments
    /// * `name` - The name of the property to get.
    fn property<S: Into<String>>(self, name: S) -> VertexPropertyQuery {
        VertexPropertyQuery::new(self.into(), name)
    }
}

/// Gets a range of vertices.
#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct RangeVertexQuery {
    /// Limits the number of vertices to get.
    pub limit: Option<usize>,

    /// Filters the type of vertices returned.
    pub t: Option<Type>,

    /// Sets the lowest vertex ID to return.
    pub start_id: Option<Uuid>,
}

impl VertexQueryExt for RangeVertexQuery {}

impl RangeVertexQuery {
    /// Filter the type of vertices returned.
    ///
    /// # Arguments
    /// * `t` - Sets the type filter.
    pub fn t(self, t: Type) -> Self {
        Self {
            limit: self.limit,
            t: Some(t),
            start_id: self.start_id,
        }
    }

    /// Sets the lowest vertex ID to return.
    ///
    /// # Arguments
    /// * `start_id` - The lowest vertex ID to return.
    pub fn start_id(self, start_id: Uuid) -> Self {
        Self {
            limit: self.limit,
            t: self.t,
            start_id: Some(start_id),
        }
    }

    /// Limits the number of returned results.
    ///
    /// # Arguments
    /// * `limit` - Limits the number of returned results.
    pub fn limit(self, limit: usize) -> Self {
        Self {
            limit: Some(limit),
            t: self.t,
            start_id: self.start_id,
        }
    }
}

/// Gets a specific set of vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpecificVertexQuery {
    /// The IDs of the vertices to get.
    pub ids: Vec<Uuid>,
}

impl VertexQueryExt for SpecificVertexQuery {}

impl SpecificVertexQuery {
    /// Creates a new vertex query for getting a list of vertices by their
    /// IDs.
    ///
    /// Arguments
    /// * `ids` - The IDs of the vertices to get.
    pub fn new(ids: Vec<Uuid>) -> Self {
        Self { ids }
    }

    /// Creates a new vertex query for getting a single vertex.
    ///
    /// Arguments
    /// * `id` - The ID of the vertex to get.
    pub fn single(id: Uuid) -> Self {
        Self { ids: vec![id] }
    }
}

/// Gets the vertices associated with edges.
///
/// Generally, you shouldn't need to construct this directly, but rather call
/// `.outbound()` or `.inbound()` on an edge query.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipeVertexQuery {
    /// The edge query to build off of.
    pub inner: Box<EdgeQuery>,

    /// Whether to get outbound or inbound vertices on the edges.
    pub direction: EdgeDirection,

    /// Limits the number of vertices to get.
    pub limit: Option<usize>,

    /// Filters the type of vertices returned.
    pub t: Option<Type>,
}

impl VertexQueryExt for PipeVertexQuery {}

impl PipeVertexQuery {
    /// Creates a new pipe vertex query.
    ///
    /// Arguments
    /// * `inner` - The edge query to build off of.
    /// * `direction` - Whether to get outbound or inbound vertices on the
    ///   edges.
    pub fn new(inner: Box<EdgeQuery>, direction: EdgeDirection) -> Self {
        Self {
            inner,
            direction,
            limit: None,
            t: None,
        }
    }

    /// Filter the type of vertices returned.
    ///
    /// # Arguments
    /// * `t` - Sets the type filter.
    pub fn t(self, t: Type) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: Some(t),
        }
    }

    /// Limits the number of returned results.
    ///
    /// # Arguments
    /// * `limit` - Limits the number of returned results.
    pub fn limit(self, limit: usize) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: Some(limit),
            t: self.t,
        }
    }
}

/// Gets property values associated with vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct VertexPropertyQuery {
    /// The vertex query to build off of.
    pub inner: VertexQuery,

    /// The name of the property to get.
    pub name: String,
}

impl VertexPropertyQuery {
    /// Creates a new vertex property query.
    ///
    /// Arguments
    /// * `inner` - The vertex query to build off of.
    /// * `name` - The name of the property to get.
    pub fn new<S: Into<String>>(inner: VertexQuery, name: S) -> Self {
        Self {
            inner,
            name: name.into(),
        }
    }
}

/// A query for edges.
///
/// Generally you shouldn't need to instantiate an `EdgeQuery` directly, but
/// rather one of the edge query structs, and then call `.into()` on it to
/// convert it to an `EdgeQuery`.
#[derive(Eq, PartialEq, Clone, Debug)]
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

/// Extension trait that specifies methods exposed by all edge queries.
pub trait EdgeQueryExt: Into<EdgeQuery> {
    /// Gets the vertices associated with the outbound end of the edges.
    fn outbound(self) -> PipeVertexQuery {
        PipeVertexQuery::new(Box::new(self.into()), EdgeDirection::Outbound)
    }

    /// Gets the vertices associated with the inbound end of the edges.
    fn inbound(self) -> PipeVertexQuery {
        PipeVertexQuery::new(Box::new(self.into()), EdgeDirection::Inbound)
    }

    /// Gets a property associated with the edges.
    ///
    /// # Arguments
    /// * `name` - The name of the property to get.
    fn property<S: Into<String>>(self, name: S) -> EdgePropertyQuery {
        EdgePropertyQuery::new(self.into(), name)
    }
}

/// Gets a specific set of edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpecificEdgeQuery {
    /// The edges to get.
    pub edges: Vec<Edge>,
}

impl EdgeQueryExt for SpecificEdgeQuery {}

impl SpecificEdgeQuery {
    /// Creates a new edge query.
    ///
    /// Arguments
    /// * `edges` - The edges to get.
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    /// Creates a new edge query for getting a single edge.
    ///
    /// Arguments
    /// * `edge` - The edge to get.
    pub fn single(edge: Edge) -> Self {
        Self { edges: vec![edge] }
    }
}

/// Gets the edges associated with vertices.
///
/// Generally, you shouldn't need to construct this directly, but rather call
/// `.outbound()` or `.inbound()` on a vertex query.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipeEdgeQuery {
    /// The vertex query to build off of.
    pub inner: Box<VertexQuery>,

    /// Whether to get outbound or inbound edges on the vertices.
    pub direction: EdgeDirection,

    /// Limits the number of edges to get.
    pub limit: Option<usize>,

    /// Filters the type of edges returned.
    pub t: Option<Type>,

    /// Offsets into the range.
    pub offset: usize,
}

impl EdgeQueryExt for PipeEdgeQuery {}

impl PipeEdgeQuery {
    /// Creates a new pipe edge query.
    ///
    /// Arguments
    /// * `inner` - The edge query to build off of.
    /// * `direction` - Whether to get outbound or inbound edges on the
    ///   vertices.
    pub fn new(inner: Box<VertexQuery>, direction: EdgeDirection) -> Self {
        Self {
            inner,
            direction,
            limit: None,
            t: None,
            offset: 0,
        }
    }

    /// Filter the type of edges returned.
    ///
    /// # Arguments
    /// * `t` - Sets the type filter.
    pub fn t(self, t: Type) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: Some(t),
            offset: self.offset,
        }
    }

    /// Sets the offset into the range.
    ///
    /// # Arguments
    /// * `offset` - The offset into the range.
    pub fn offset(self, offset: usize) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: self.t,
            offset,
        }
    }

    /// Limits the number of returned results.
    ///
    /// # Arguments
    /// * `limit` - Limits the number of returned results.
    pub fn limit(self, limit: usize) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: Some(limit),
            t: self.t,
            offset: self.offset,
        }
    }
}

/// Gets property values associated with edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EdgePropertyQuery {
    /// The edge query to build off of.
    pub inner: EdgeQuery,

    /// The name of the property to get.
    pub name: String,
}

impl EdgePropertyQuery {
    /// Creates a new edge property query.
    ///
    /// Arguments
    /// * `inner` - The edge query to build off of.
    /// * `name` - The name of the property to get.
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
