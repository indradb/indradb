use std::str::FromStr;
use std::u32;

use crate::{errors, Edge, Identifier};

use uuid::Uuid;

macro_rules! into_query {
    ($name:ident, $variant:ident) => {
        // we don't want to impl From since the reverse operation isn't allowed
        #[allow(clippy::from_over_into)]
        impl Into<Query> for $name {
            fn into(self) -> Query {
                Query::$variant(self)
            }
        }
    };
}

macro_rules! query_type {
    ($name:ident, $variant:ident) => {
        impl QueryExt for $name {}
        into_query!($name, $variant);
    };
}

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

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Query {
    AllVertex(AllVertexQuery),
    RangeVertex(RangeVertexQuery),
    SpecificVertex(SpecificVertexQuery),
    VertexWithPropertyPresence(VertexWithPropertyPresenceQuery),
    VertexWithPropertyValue(VertexWithPropertyValueQuery),

    AllEdge(AllEdgeQuery),
    SpecificEdge(SpecificEdgeQuery),
    EdgeWithPropertyPresence(EdgeWithPropertyPresenceQuery),
    EdgeWithPropertyValue(EdgeWithPropertyValueQuery),

    Pipe(PipeQuery),
    PipeProperty(PipePropertyQuery),
    PipeWithPropertyPresence(PipeWithPropertyPresenceQuery),
    PipeWithPropertyValue(PipeWithPropertyValueQuery),

    Include(IncludeQuery),
    Count(CountQuery),
}

impl Query {
    pub(crate) fn output_len(&self) -> usize {
        match self {
            Query::AllVertex(_)
            | Query::RangeVertex(_)
            | Query::SpecificVertex(_)
            | Query::VertexWithPropertyPresence(_)
            | Query::VertexWithPropertyValue(_)
            | Query::AllEdge(_)
            | Query::SpecificEdge(_)
            | Query::EdgeWithPropertyPresence(_)
            | Query::EdgeWithPropertyValue(_)
            | Query::Count(_) => 1,
            Query::Pipe(q) => q.inner.output_len(),
            Query::PipeProperty(q) => q.inner.output_len(),
            Query::PipeWithPropertyPresence(q) => q.inner.output_len(),
            Query::PipeWithPropertyValue(q) => q.inner.output_len(),
            Query::Include(q) => 1 + q.inner.output_len(),
        }
    }

    pub(crate) fn output_type(&self) -> errors::ValidationResult<QueryOutputValue> {
        match self {
            Query::AllVertex(_)
            | Query::RangeVertex(_)
            | Query::SpecificVertex(_)
            | Query::VertexWithPropertyPresence(_)
            | Query::VertexWithPropertyValue(_) => Ok(QueryOutputValue::Vertices(Vec::default())),
            Query::AllEdge(_)
            | Query::SpecificEdge(_)
            | Query::EdgeWithPropertyPresence(_)
            | Query::EdgeWithPropertyValue(_) => Ok(QueryOutputValue::Edges(Vec::default())),
            Query::Count(_) => Ok(QueryOutputValue::Count(0)),
            Query::Pipe(q) => q.inner.output_type(),
            Query::PipeProperty(q) => match q.inner.output_type()? {
                QueryOutputValue::Vertices(_) => Ok(QueryOutputValue::VertexProperties(Vec::default())),
                QueryOutputValue::Edges(_) => Ok(QueryOutputValue::EdgeProperties(Vec::default())),
                _ => Err(errors::ValidationError::InnerQuery),
            },
            Query::PipeWithPropertyPresence(q) => q.inner.output_type(),
            Query::PipeWithPropertyValue(q) => q.inner.output_type(),
            Query::Include(q) => q.inner.output_type(),
        }
    }
}

pub trait QueryExt: Into<Query> {
    fn outbound(self) -> errors::ValidationResult<PipeQuery> {
        PipeQuery::new(Box::new(self.into()), EdgeDirection::Outbound)
    }

    fn inbound(self) -> errors::ValidationResult<PipeQuery> {
        PipeQuery::new(Box::new(self.into()), EdgeDirection::Inbound)
    }

    /// Gets vertices with a property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    fn with_property<T: Into<Identifier>>(self, name: T) -> errors::ValidationResult<PipeWithPropertyPresenceQuery> {
        PipeWithPropertyPresenceQuery::new(Box::new(self.into()), name, true)
    }

    /// Gets vertices without a property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    fn without_property<T: Into<Identifier>>(self, name: T) -> errors::ValidationResult<PipeWithPropertyPresenceQuery> {
        PipeWithPropertyPresenceQuery::new(Box::new(self.into()), name, false)
    }

    /// Gets vertices with a property equal to a given value.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    fn with_property_equal_to<T: Into<Identifier>>(
        self,
        name: T,
        value: serde_json::Value,
    ) -> errors::ValidationResult<PipeWithPropertyValueQuery> {
        PipeWithPropertyValueQuery::new(Box::new(self.into()), name, value, true)
    }

    /// Gets vertices with a property not equal to a given value.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    fn with_property_not_equal_to<T: Into<Identifier>>(
        self,
        name: T,
        value: serde_json::Value,
    ) -> errors::ValidationResult<PipeWithPropertyValueQuery> {
        PipeWithPropertyValueQuery::new(Box::new(self.into()), name, value, false)
    }

    fn properties(self) -> errors::ValidationResult<PipePropertyQuery> {
        PipePropertyQuery::new(Box::new(self.into()))
    }

    #[deprecated(since = "4.0.0", note = "use `.properties().name(...)`")]
    fn property(self, name: Identifier) -> errors::ValidationResult<PipePropertyQuery> {
        Ok(self.properties()?.name(name))
    }

    fn include(self) -> IncludeQuery {
        IncludeQuery::new(Box::new(self.into()))
    }

    fn count(self) -> errors::ValidationResult<CountQuery> {
        CountQuery::new(Box::new(self.into()))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct VertexWithPropertyPresenceQuery {
    /// The name of the property.
    pub(crate) name: Identifier,
}

query_type!(VertexWithPropertyPresenceQuery, VertexWithPropertyPresence);

impl VertexWithPropertyPresenceQuery {
    pub fn new<T: Into<Identifier>>(name: T) -> Self {
        Self { name: name.into() }
    }
}

/// Gets vertices with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct VertexWithPropertyValueQuery {
    /// The name of the property.
    pub(crate) name: Identifier,
    /// The value of the property.
    pub(crate) value: serde_json::Value,
}

query_type!(VertexWithPropertyValueQuery, VertexWithPropertyValue);

impl VertexWithPropertyValueQuery {
    pub fn new<T: Into<Identifier>>(name: T, value: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EdgeWithPropertyPresenceQuery {
    /// The name of the property.
    pub(crate) name: Identifier,
}

query_type!(EdgeWithPropertyPresenceQuery, EdgeWithPropertyPresence);

impl EdgeWithPropertyPresenceQuery {
    pub fn new<T: Into<Identifier>>(name: T) -> Self {
        Self { name: name.into() }
    }
}

/// Gets vertices with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EdgeWithPropertyValueQuery {
    /// The name of the property.
    pub(crate) name: Identifier,
    /// The value of the property.
    pub(crate) value: serde_json::Value,
}

query_type!(EdgeWithPropertyValueQuery, EdgeWithPropertyValue);

impl EdgeWithPropertyValueQuery {
    pub fn new<T: Into<Identifier>>(name: T, value: serde_json::Value) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

/// Gets vertices with a property.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipeWithPropertyPresenceQuery {
    /// The query to filter.
    pub(crate) inner: Box<Query>,
    /// The name of the property.
    pub(crate) name: Identifier,
    /// Whether we should look for property presence or lack thereof.
    pub(crate) exists: bool,
}

query_type!(PipeWithPropertyPresenceQuery, PipeWithPropertyPresence);

impl PipeWithPropertyPresenceQuery {
    /// Gets vertices with a property.
    ///
    /// Arguments
    /// * `inner`: The query to filter.
    /// * `name`: The name of the property.
    /// * `exists`: Whether we should look for property presence or lack thereof.
    pub fn new<T: Into<Identifier>>(inner: Box<Query>, name: T, exists: bool) -> errors::ValidationResult<Self> {
        match inner.output_type()? {
            QueryOutputValue::Vertices(_) | QueryOutputValue::Edges(_) => {}
            _ => return Err(errors::ValidationError::InnerQuery),
        }
        Ok(Self {
            inner,
            name: name.into(),
            exists,
        })
    }
}

/// Gets vertices with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipeWithPropertyValueQuery {
    /// The query to filter.
    pub(crate) inner: Box<Query>,
    /// The name of the property.
    pub(crate) name: Identifier,
    /// The value of the property.
    pub(crate) value: serde_json::Value,
    /// Whether we should look for property equality or non-equality.
    pub(crate) equal: bool,
}

query_type!(PipeWithPropertyValueQuery, PipeWithPropertyValue);

impl PipeWithPropertyValueQuery {
    /// Constructs a new pipe with property value query.
    ///
    /// # Arguments
    /// * `inner`: The inner query.
    /// * `name`: The property name to filter.
    /// * `value`: The property value to filter.
    /// * `equal`: Whether the value should be equal, or not equal.
    pub fn new<T: Into<Identifier>>(
        inner: Box<Query>,
        name: T,
        value: serde_json::Value,
        equal: bool,
    ) -> errors::ValidationResult<Self> {
        match inner.output_type()? {
            QueryOutputValue::Vertices(_) | QueryOutputValue::Edges(_) => {}
            _ => return Err(errors::ValidationError::InnerQuery),
        }
        Ok(Self {
            inner,
            name: name.into(),
            value,
            equal,
        })
    }
}

/// Gets all vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct AllVertexQuery;
query_type!(AllVertexQuery, AllVertex);

/// Gets a range of vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct RangeVertexQuery {
    /// Limits the number of vertices to get.
    pub(crate) limit: u32,

    /// Filters the type of vertices returned.
    pub(crate) t: Option<Identifier>,

    /// Sets the lowest vertex ID to return.
    pub(crate) start_id: Option<Uuid>,
}

query_type!(RangeVertexQuery, RangeVertex);

impl Default for RangeVertexQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeVertexQuery {
    /// Creates a new vertex range query.
    pub fn new() -> Self {
        Self {
            limit: u32::max_value(),
            t: None,
            start_id: None,
        }
    }

    /// Sets the limit.
    ///
    /// # Arguments
    /// * `limit`: Limits the number of returned results.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            limit,
            t: self.t,
            start_id: self.start_id,
        }
    }

    /// Filter the type of vertices returned.
    ///
    /// # Arguments
    /// * `t`: Sets the type filter.
    pub fn t(self, t: Identifier) -> Self {
        Self {
            limit: self.limit,
            t: Some(t),
            start_id: self.start_id,
        }
    }

    /// Sets the lowest vertex ID to return.
    ///
    /// # Arguments
    /// * `start_id`: The lowest vertex ID to return.
    pub fn start_id(self, start_id: Uuid) -> Self {
        Self {
            limit: self.limit,
            t: self.t,
            start_id: Some(start_id),
        }
    }
}

/// Gets a specific set of vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpecificVertexQuery {
    /// The IDs of the vertices to get.
    pub(crate) ids: Vec<Uuid>,
}

query_type!(SpecificVertexQuery, SpecificVertex);

impl SpecificVertexQuery {
    /// Creates a new vertex query for getting a list of vertices by their
    /// IDs.
    ///
    /// Arguments
    /// * `ids`: The IDs of the vertices to get.
    pub fn new(ids: Vec<Uuid>) -> Self {
        Self { ids }
    }

    /// Creates a new vertex query for getting a single vertex.
    ///
    /// Arguments
    /// * `id`: The ID of the vertex to get.
    pub fn single(id: Uuid) -> Self {
        Self { ids: vec![id] }
    }
}

/// Gets the vertices associated with edges, or edges associated with
/// vertices.
///
/// Generally, you shouldn't need to construct this directly, but rather call
/// `.outbound()` or `.inbound()`.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipeQuery {
    /// The edge query to build off of.
    pub(crate) inner: Box<Query>,

    /// Whether to get outbound or inbound vertices on the edges.
    pub(crate) direction: EdgeDirection,

    /// Limits the number of vertices to get.
    pub(crate) limit: u32,

    /// Filters the type of vertices returned.
    pub(crate) t: Option<Identifier>,
}

query_type!(PipeQuery, Pipe);

impl PipeQuery {
    /// Constructs a new pipe query.
    ///
    /// # Arguments
    /// * `inner`: The inner query.
    /// * `direction`: Which direction to pipe from.
    pub fn new(inner: Box<Query>, direction: EdgeDirection) -> errors::ValidationResult<Self> {
        match inner.output_type()? {
            QueryOutputValue::Vertices(_) | QueryOutputValue::Edges(_) => {}
            _ => return Err(errors::ValidationError::InnerQuery),
        }

        Ok(Self {
            inner,
            direction,
            limit: u32::max_value(),
            t: None,
        })
    }

    /// Sets the limit.
    ///
    /// # Arguments
    /// * `limit`: Limits the number of returned results.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit,
            t: self.t,
        }
    }

    /// Filter the type of vertices returned.
    ///
    /// # Arguments
    /// * `t`: Sets the type filter.
    pub fn t(self, t: Identifier) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: Some(t),
        }
    }
}

/// Gets all edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct AllEdgeQuery;
query_type!(AllEdgeQuery, AllEdge);

/// Gets a specific set of edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpecificEdgeQuery {
    /// The edges to get.
    pub(crate) edges: Vec<Edge>,
}

query_type!(SpecificEdgeQuery, SpecificEdge);

impl SpecificEdgeQuery {
    /// Creates a new edge query for getting a list of specific edges.
    ///
    /// Arguments
    /// * `edges`: The edges to get.
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    /// Creates a new edge query for getting a single edge.
    ///
    /// Arguments
    /// * `edge`: The edge to get.
    pub fn single(edge: Edge) -> Self {
        Self { edges: vec![edge] }
    }
}

/// Includes the results of a query in output.
///
/// The outermost part of a query will always be explicitly included. This
/// allows you to also output an intermediate result.
///
/// # Examples
/// ```
/// use indradb::{AllVertexQuery, QueryExt};
/// // A query to return all edges in the database, which are implicitly
/// // included as the outermost results.
/// let q = AllVertexQuery.outbound();
/// // A query to return all vertices and all edges in the database, because
/// // vertices are explicitly included as intermediate results.
/// let q = AllVertexQuery.include().outbound();
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct IncludeQuery {
    /// The query to export.
    pub(crate) inner: Box<Query>,
}

query_type!(IncludeQuery, Include);

impl IncludeQuery {
    /// Marks a query as exported.
    ///
    /// Arguments
    /// * `inner`: The query to export.
    pub fn new(inner: Box<Query>) -> Self {
        Self { inner }
    }
}

/// Counts the number of items returned from a query.
///
/// # Examples
/// ```
/// use indradb::{AllVertexQuery, QueryExt};
/// // A query to return the total number of vertices in the database.
/// let q = AllVertexQuery.count();
/// ```
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CountQuery {
    /// The query to export.
    pub(crate) inner: Box<Query>,
}

into_query!(CountQuery, Count);

impl CountQuery {
    /// Marks a query as exported.
    ///
    /// Arguments
    /// * `inner`: The query to export.
    pub fn new(inner: Box<Query>) -> errors::ValidationResult<Self> {
        match inner.output_type()? {
            QueryOutputValue::Vertices(_)
            | QueryOutputValue::Edges(_)
            | QueryOutputValue::VertexProperties(_)
            | QueryOutputValue::EdgeProperties(_) => {}
            _ => return Err(errors::ValidationError::InnerQuery),
        }
        Ok(Self { inner })
    }
}

/// Returns the properties associated with a vertex or edge.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropertyQuery {
    /// The inner query.
    pub(crate) inner: Box<Query>,
    /// The property name to get. If `None`, all properties will be fetched.
    pub(crate) name: Option<Identifier>,
}

into_query!(PipePropertyQuery, PipeProperty);

impl PipePropertyQuery {
    /// Creates a new pipe property query.
    ///
    /// # Arguments
    /// * `inner`: The query to pipe.
    pub fn new(inner: Box<Query>) -> errors::ValidationResult<Self> {
        match inner.output_type()? {
            QueryOutputValue::Vertices(_) | QueryOutputValue::Edges(_) => {}
            _ => return Err(errors::ValidationError::InnerQuery),
        }
        Ok(Self { inner, name: None })
    }

    /// Only include properties with a given name.
    ///
    /// # Arguments
    /// * `name`: The name filter.
    pub fn name(self, name: Identifier) -> Self {
        Self {
            inner: self.inner,
            name: Some(name),
        }
    }
}

/// Value(s) returned from a query.
#[derive(Clone, Debug)]
pub enum QueryOutputValue {
    /// Vertices.
    Vertices(Vec<crate::Vertex>),
    /// Edges.
    Edges(Vec<crate::Edge>),
    /// A Count.
    Count(u64),
    /// Vertex properties.
    VertexProperties(Vec<(crate::Vertex, crate::Identifier, serde_json::Value)>),
    /// Edge properties.
    EdgeProperties(Vec<(crate::Edge, crate::Identifier, serde_json::Value)>),
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
