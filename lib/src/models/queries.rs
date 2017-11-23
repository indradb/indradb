use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;
use super::types::Type;
use super::edges::EdgeKey;

/// Specifies what kind of items should be piped from one type of query to
/// another.
///
/// Edge and vertex queries can build off of one another via pipes - e.g. you
/// can get the outbound edges of a set of vertices by piping from a vertex
/// query to an edge query. `QueryTypeConverter`s are used to specify which
/// end of things you want to pipe - either the outbound items or the inbound
/// items.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum QueryTypeConverter {
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "inbound")]
    Inbound,
}

/// A query for vertices.
///
/// This is used by transactions to get, set and delete vertices and vertex
/// metadata.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VertexQuery {
    All { start_id: Option<Uuid>, limit: u32 },
    Vertex { id: Uuid },
    Vertices { ids: Vec<Uuid> },
    Pipe {
        edge_query: Box<EdgeQuery>,
        converter: QueryTypeConverter,
        limit: u32,
    },
}

impl VertexQuery {
    pub fn outbound_edges(
        self,
        t: Option<Type>,
        high: Option<DateTime<Utc>>,
        low: Option<DateTime<Utc>>,
        limit: u32,
    ) -> EdgeQuery {
        EdgeQuery::Pipe {
            vertex_query: Box::new(self),
            converter: QueryTypeConverter::Outbound,
            type_filter: t,
            high_filter: high,
            low_filter: low,
            limit: limit,
        }
    }

    pub fn inbound_edges(
        self,
        t: Option<Type>,
        high: Option<DateTime<Utc>>,
        low: Option<DateTime<Utc>>,
        limit: u32,
    ) -> EdgeQuery {
        EdgeQuery::Pipe {
            vertex_query: Box::new(self),
            converter: QueryTypeConverter::Inbound,
            type_filter: t,
            high_filter: high,
            low_filter: low,
            limit: limit,
        }
    }
}

/// A query for edges.
///
/// This is used by transactions to get, set and delete edges and edge
/// metadata.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EdgeQuery {
    Edge { key: EdgeKey },
    Edges { keys: Vec<EdgeKey> },
    Pipe {
        vertex_query: Box<VertexQuery>,
        converter: QueryTypeConverter,
        type_filter: Option<Type>,
        high_filter: Option<DateTime<Utc>>,
        low_filter: Option<DateTime<Utc>>,
        limit: u32,
    },
}

impl EdgeQuery {
    pub fn outbound_vertices(self, limit: u32) -> VertexQuery {
        VertexQuery::Pipe {
            edge_query: Box::new(self),
            converter: QueryTypeConverter::Outbound,
            limit: limit,
        }
    }

    pub fn inbound_vertices(self, limit: u32) -> VertexQuery {
        VertexQuery::Pipe {
            edge_query: Box::new(self),
            converter: QueryTypeConverter::Inbound,
            limit: limit,
        }
    }
}
