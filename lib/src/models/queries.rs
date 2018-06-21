use super::edges::EdgeKey;
use super::types::Type;
use chrono::offset::Utc;
use chrono::DateTime;
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

/// A query for vertices.
///
/// This is used by transactions to get, set and delete vertices and vertex
/// metadata.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VertexQuery {
    All {
        start_id: Option<Uuid>,
        limit: u32,
    },
    Vertices {
        ids: Vec<Uuid>,
    },
    Pipe {
        edge_query: Box<EdgeQuery>,
        converter: EdgeDirection,
        limit: u32,
    },
}

impl VertexQuery {
    pub fn outbound_edges(
        self,
        type_filter: Option<Type>,
        high_filter: Option<DateTime<Utc>>,
        low_filter: Option<DateTime<Utc>>,
        limit: u32,
    ) -> EdgeQuery {
        EdgeQuery::Pipe {
            vertex_query: Box::new(self),
            converter: EdgeDirection::Outbound,
            type_filter,
            high_filter,
            low_filter,
            limit,
        }
    }

    pub fn inbound_edges(
        self,
        type_filter: Option<Type>,
        high_filter: Option<DateTime<Utc>>,
        low_filter: Option<DateTime<Utc>>,
        limit: u32,
    ) -> EdgeQuery {
        EdgeQuery::Pipe {
            vertex_query: Box::new(self),
            converter: EdgeDirection::Inbound,
            type_filter,
            high_filter,
            low_filter,
            limit,
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
    Edges {
        keys: Vec<EdgeKey>,
    },
    Pipe {
        vertex_query: Box<VertexQuery>,
        converter: EdgeDirection,
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
            converter: EdgeDirection::Outbound,
            limit,
        }
    }

    pub fn inbound_vertices(self, limit: u32) -> VertexQuery {
        VertexQuery::Pipe {
            edge_query: Box::new(self),
            converter: EdgeDirection::Inbound,
            limit,
        }
    }
}
