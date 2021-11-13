use std::str::FromStr;
use std::u32;

use crate::{errors, EdgeKey, JsonValue, Type};

use chrono::offset::Utc;
use chrono::DateTime;
use uuid::Uuid;

macro_rules! vertex_query_type {
    ($name:ident, $variant:ident) => {
        impl VertexQueryExt for $name {}

        // we don't want to impl From since the reverse operation isn't allowed
        #[allow(clippy::from_over_into)]
        impl Into<VertexQuery> for $name {
            fn into(self) -> VertexQuery {
                VertexQuery::$variant(self)
            }
        }
    };
}

macro_rules! edge_query_type {
    ($name:ident, $variant:ident) => {
        impl EdgeQueryExt for $name {}

        // we don't want to impl From since the reverse operation isn't allowed
        #[allow(clippy::from_over_into)]
        impl Into<EdgeQuery> for $name {
            fn into(self) -> EdgeQuery {
                EdgeQuery::$variant(self)
            }
        }
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

    PropertyPresence(PropertyPresenceVertexQuery),
    PropertyValue(PropertyValueVertexQuery),

    PipePropertyPresence(PipePropertyPresenceVertexQuery),
    PipePropertyValue(PipePropertyValueVertexQuery),
}

/// Extension trait with methods available in all vertex queries.
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
    /// * `name`: The name of the property to get.
    fn property<T: Into<Type>>(self, name: T) -> VertexPropertyQuery {
        VertexPropertyQuery::new(self.into(), name)
    }

    /// Gets vertices with a property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    fn with_property<T: Into<Type>>(self, name: T) -> PipePropertyPresenceVertexQuery {
        PipePropertyPresenceVertexQuery::new(Box::new(self.into()), name, true)
    }

    /// Gets vertices without a property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    fn without_property<T: Into<Type>>(self, name: T) -> PipePropertyPresenceVertexQuery {
        PipePropertyPresenceVertexQuery::new(Box::new(self.into()), name, false)
    }

    /// Gets vertices with a property equal to a given value.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    fn with_property_equal_to<T: Into<Type>>(self, name: T, value: JsonValue) -> PipePropertyValueVertexQuery {
        PipePropertyValueVertexQuery::new(Box::new(self.into()), name, value, true)
    }

    /// Gets vertices with a property not equal to a given value.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    fn with_property_not_equal_to<T: Into<Type>>(self, name: T, value: JsonValue) -> PipePropertyValueVertexQuery {
        PipePropertyValueVertexQuery::new(Box::new(self.into()), name, value, false)
    }
}

/// Gets vertices with a property.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropertyPresenceVertexQuery {
    /// The name of the property.
    pub name: Type,
}

vertex_query_type!(PropertyPresenceVertexQuery, PropertyPresence);

impl PropertyPresenceVertexQuery {
    /// Creates a new vertex query for getting vertices with a property.
    ///
    /// Arguments
    /// * `name`: The name of the property.
    pub fn new<T: Into<Type>>(name: T) -> Self {
        Self { name: name.into() }
    }
}

/// Gets vertices with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropertyValueVertexQuery {
    /// The name of the property.
    pub name: Type,
    /// The value of the property.
    pub value: JsonValue,
}

vertex_query_type!(PropertyValueVertexQuery, PropertyValue);

impl PropertyValueVertexQuery {
    /// Creates a new vertex query for getting vertices with a property with a
    /// given value.
    ///
    /// Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    pub fn new<T: Into<Type>>(name: T, value: JsonValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

/// Gets vertices with a property.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropertyPresenceVertexQuery {
    /// The query to filter.
    pub inner: Box<VertexQuery>,
    /// The name of the property.
    pub name: Type,
    /// Whether we should look for property presence or lack thereof.
    pub exists: bool,
}

vertex_query_type!(PipePropertyPresenceVertexQuery, PipePropertyPresence);

impl PipePropertyPresenceVertexQuery {
    /// Gets vertices with a property.
    ///
    /// Arguments
    /// * `inner`: The query to filter.
    /// * `name`: The name of the property.
    /// * `exists`: Whether we should look for property presence or lack thereof.
    pub fn new<T: Into<Type>>(inner: Box<VertexQuery>, name: T, exists: bool) -> Self {
        Self {
            inner,
            name: name.into(),
            exists,
        }
    }
}

/// Gets vertices with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropertyValueVertexQuery {
    /// The query to filter.
    pub inner: Box<VertexQuery>,
    /// The name of the property.
    pub name: Type,
    /// The value of the property.
    pub value: JsonValue,
    /// Whether we should look for property equality or non-equality.
    pub equal: bool,
}

vertex_query_type!(PipePropertyValueVertexQuery, PipePropertyValue);

impl PipePropertyValueVertexQuery {
    /// Creates a new vertex query for getting vertices with a property with a
    /// given value.
    ///
    /// Arguments
    /// * `inner`: The query to filter.
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    /// * `equal`: Whether we should look for property equality or non-equality.
    pub fn new<T: Into<Type>>(inner: Box<VertexQuery>, name: T, value: JsonValue, equal: bool) -> Self {
        Self {
            inner,
            name: name.into(),
            value,
            equal,
        }
    }
}

/// Gets a range of vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct RangeVertexQuery {
    /// Limits the number of vertices to get.
    pub limit: u32,

    /// Filters the type of vertices returned.
    pub t: Option<Type>,

    /// Sets the lowest vertex ID to return.
    pub start_id: Option<Uuid>,
}

vertex_query_type!(RangeVertexQuery, Range);

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
    pub ids: Vec<Uuid>,
}

vertex_query_type!(SpecificVertexQuery, Specific);

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
    pub limit: u32,

    /// Filters the type of vertices returned.
    pub t: Option<Type>,
}

vertex_query_type!(PipeVertexQuery, Pipe);

impl PipeVertexQuery {
    /// Creates a new pipe vertex query.
    ///
    /// Arguments
    /// * `inner`: The edge query to build off of.
    /// * `direction`: Whether to get outbound or inbound vertices on the
    ///   edges.
    pub fn new(inner: Box<EdgeQuery>, direction: EdgeDirection) -> Self {
        Self {
            inner,
            direction,
            limit: u32::max_value(),
            t: None,
        }
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
    pub fn t(self, t: Type) -> Self {
        Self {
            inner: self.inner,
            direction: self.direction,
            limit: self.limit,
            t: Some(t),
        }
    }
}

/// Gets property values associated with vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct VertexPropertyQuery {
    /// The vertex query to build off of.
    pub inner: VertexQuery,

    /// The name of the property to get.
    pub name: Type,
}

impl VertexPropertyQuery {
    /// Creates a new vertex property query.
    ///
    /// Arguments
    /// * `inner`: The vertex query to build off of.
    /// * `name`: The name of the property to get.
    pub fn new<T: Into<Type>>(inner: VertexQuery, name: T) -> Self {
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

    PropertyPresence(PropertyPresenceEdgeQuery),
    PropertyValue(PropertyValueEdgeQuery),

    PipePropertyPresence(PipePropertyPresenceEdgeQuery),
    PipePropertyValue(PipePropertyValueEdgeQuery),
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
    /// * `name`: The name of the property to get.
    fn property<T: Into<Type>>(self, name: T) -> EdgePropertyQuery {
        EdgePropertyQuery::new(self.into(), name)
    }

    /// Gets edges with a property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    fn with_property<T: Into<Type>>(self, name: T) -> PipePropertyPresenceEdgeQuery {
        PipePropertyPresenceEdgeQuery::new(Box::new(self.into()), name, true)
    }

    /// Gets edges without a property.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    fn without_property<T: Into<Type>>(self, name: T) -> PipePropertyPresenceEdgeQuery {
        PipePropertyPresenceEdgeQuery::new(Box::new(self.into()), name, false)
    }

    /// Gets edges with a property equal to a given value.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    fn with_property_equal_to<T: Into<Type>>(self, name: T, value: JsonValue) -> PipePropertyValueEdgeQuery {
        PipePropertyValueEdgeQuery::new(Box::new(self.into()), name, value, true)
    }

    /// Gets edges with a property not equal to a given value.
    ///
    /// # Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    fn with_property_not_equal_to<T: Into<Type>>(self, name: T, value: JsonValue) -> PipePropertyValueEdgeQuery {
        PipePropertyValueEdgeQuery::new(Box::new(self.into()), name, value, false)
    }
}

/// Gets edges with a property.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropertyPresenceEdgeQuery {
    /// The name of the property.
    pub name: Type,
}

edge_query_type!(PropertyPresenceEdgeQuery, PropertyPresence);

impl PropertyPresenceEdgeQuery {
    /// Creates a new edge query for getting edges with a property.
    ///
    /// Arguments
    /// * `name`: The name of the property.
    pub fn new<T: Into<Type>>(name: T) -> Self {
        Self { name: name.into() }
    }
}

/// Gets edges with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropertyValueEdgeQuery {
    /// The name of the property.
    pub name: Type,
    /// The value of the property.
    pub value: JsonValue,
}

edge_query_type!(PropertyValueEdgeQuery, PropertyValue);

impl PropertyValueEdgeQuery {
    /// Creates a new edge query for getting edges with a property with a
    /// given value.
    ///
    /// Arguments
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    pub fn new<T: Into<Type>>(name: T, value: JsonValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

/// Gets edges with a property.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropertyPresenceEdgeQuery {
    /// The query to filter.
    pub inner: Box<EdgeQuery>,
    /// The name of the property.
    pub name: Type,
    /// Whether we should look for property presence or lack thereof.
    pub exists: bool,
}

edge_query_type!(PipePropertyPresenceEdgeQuery, PipePropertyPresence);

impl PipePropertyPresenceEdgeQuery {
    /// Gets edges with a property.
    ///
    /// Arguments
    /// * `inner`: The query to filter.
    /// * `name`: The name of the property.
    /// * `exists`: Whether we should look for property presence or lack thereof.
    pub fn new<T: Into<Type>>(inner: Box<EdgeQuery>, name: T, exists: bool) -> Self {
        Self {
            inner,
            name: name.into(),
            exists,
        }
    }
}

/// Gets edges with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropertyValueEdgeQuery {
    /// The query to filter.
    pub inner: Box<EdgeQuery>,
    /// The name of the property.
    pub name: Type,
    /// The value of the property.
    pub value: JsonValue,
    /// Whether we should look for property equality or non-equality.
    pub equal: bool,
}

edge_query_type!(PipePropertyValueEdgeQuery, PipePropertyValue);

impl PipePropertyValueEdgeQuery {
    /// Creates a new edge query for getting edges with a property with a
    /// given value.
    ///
    /// Arguments
    /// * `inner`: The query to filter.
    /// * `name`: The name of the property.
    /// * `value`: The value of the property.
    /// * `equal`: Whether we should look for property equality or non-equality.
    pub fn new<T: Into<Type>>(inner: Box<EdgeQuery>, name: T, value: JsonValue, equal: bool) -> Self {
        Self {
            inner,
            name: name.into(),
            value,
            equal,
        }
    }
}

/// Gets a specific set of edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpecificEdgeQuery {
    /// The keys of the edges to get.
    pub keys: Vec<EdgeKey>,
}

edge_query_type!(SpecificEdgeQuery, Specific);

impl SpecificEdgeQuery {
    /// Creates a new edge query for getting a list of edges by their
    /// keys.
    ///
    /// Arguments
    /// * `keys`: The keys of the edges to get.
    pub fn new(keys: Vec<EdgeKey>) -> Self {
        Self { keys }
    }

    /// Creates a new edge query for getting a single edge.
    ///
    /// Arguments
    /// * `key`: The key of the edge to get.
    pub fn single(key: EdgeKey) -> Self {
        Self { keys: vec![key] }
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
    pub limit: u32,

    /// Filters the type of edges returned.
    pub t: Option<Type>,

    /// Specifies the newest update datetime for returned edges.
    pub high: Option<DateTime<Utc>>,

    /// Specifies the oldest update datetime for returned edges.
    pub low: Option<DateTime<Utc>>,
}

edge_query_type!(PipeEdgeQuery, Pipe);

impl PipeEdgeQuery {
    /// Creates a new pipe edge query.
    ///
    /// Arguments
    /// * `inner`: The edge query to build off of.
    /// * `direction`: Whether to get outbound or inbound edges on the
    ///   vertices.
    /// * `limit`: Limits the number of edges to get.
    pub fn new(inner: Box<VertexQuery>, direction: EdgeDirection) -> Self {
        Self {
            inner,
            direction,
            limit: u32::max_value(),
            t: None,
            high: None,
            low: None,
        }
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
            high: self.high,
            low: self.low,
        }
    }

    /// Filter the type of edges returned.
    ///
    /// # Arguments
    /// * `t`: Sets the type filter.
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

    /// Filter the update datetime of the edges returned.
    ///
    /// # Arguments
    /// * `high`: The newest update datetime for the edges returned.
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

    /// Filter the update datetime of the edges returned.
    ///
    /// # Arguments
    /// * `low`: The oldest update datetime for the edges returned.
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

/// Gets property values associated with edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EdgePropertyQuery {
    /// The edge query to build off of.
    pub inner: EdgeQuery,

    /// The name of the property to get.
    pub name: Type,
}

impl EdgePropertyQuery {
    /// Creates a new edge property query.
    ///
    /// Arguments
    /// * `inner`: The edge query to build off of.
    /// * `name`: The name of the property to get.
    pub fn new<T: Into<Type>>(inner: EdgeQuery, name: T) -> Self {
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
