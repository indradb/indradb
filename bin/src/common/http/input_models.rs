use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use serde_json::Value as JsonValue;
use core::str::FromStr;
use regex::Regex;
use util::generate_uuid_v1;

#[graphql(description="Represents a uniquely identifiable key to an edge.")]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Ord, PartialOrd, GraphQLInputObject)]
pub struct InputEdgeKey {
    #[graphql(description="The id of the outbound vertex.")]
    pub outbound_id: Uuid,

    #[graphql(description="The type of the edge.")]
    pub t: InputType,

    #[graphql(description="The id of the inbound vertex.")]
    pub inbound_id: Uuid,
}

#[graphql(description="Specifies what kind of items should be piped from one type of query to another.")]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Copy, GraphQLEnum)]
pub enum InputEdgeDirection {
    Outbound,
    Inbound,
}

#[graphql(description="A query for vertices.")]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, GraphQLInputObject)]
pub enum InputVertexQuery {
    All {
        start_id: Option<Uuid>,
        limit: i32,
    },
    Vertices {
        ids: Vec<Uuid>,
    },
    Pipe {
        edge_query: Box<InputEdgeQuery>,
        converter: InputEdgeDirection,
        limit: i32,
    },
}

impl VertexQuery {
    pub fn outbound_edges(
        self,
        t: Option<InputType>,
        high: Option<DateTime<Utc>>,
        low: Option<DateTime<Utc>>,
        limit: i32,
    ) -> EdgeQuery {
        EdgeQuery::Pipe {
            vertex_query: Box::new(self),
            converter: InputEdgeDirection::Outbound,
            type_filter: t,
            high_filter: high,
            low_filter: low,
            limit: limit,
        }
    }

    pub fn inbound_edges(
        self,
        t: Option<InputType>,
        high: Option<DateTime<Utc>>,
        low: Option<DateTime<Utc>>,
        limit: i32,
    ) -> EdgeQuery {
        EdgeQuery::Pipe {
            vertex_query: Box::new(self),
            converter: InputEdgeDirection::Inbound,
            type_filter: t,
            high_filter: high,
            low_filter: low,
            limit: limit,
        }
    }
}

#[graphql(description="A query for edges.")]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, GraphQLInputObject)]
pub enum InputEdgeQuery {
    Edges {
        keys: Vec<InputEdgeKey>,
    },
    Pipe {
        vertex_query: Box<InputVertexQuery>,
        converter: InputEdgeDirection,
        type_filter: Option<Type>,
        high_filter: Option<DateTime<Utc>>,
        low_filter: Option<DateTime<Utc>>,
        limit: i32,
    },
}

impl EdgeQuery {
    pub fn outbound_vertices(self, limit: i32) -> VertexQuery {
        VertexQuery::Pipe {
            edge_query: Box::new(self),
            converter: InputEdgeDirection::Outbound,
            limit: limit,
        }
    }

    pub fn inbound_vertices(self, limit: i32) -> VertexQuery {
        VertexQuery::Pipe {
            edge_query: Box::new(self),
            converter: InputEdgeDirection::Inbound,
            limit: limit,
        }
    }
}

#[graphql(description="An edge or vertex type.")]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Ord, PartialOrd, GraphQLInputObject)]
pub struct InputType {
    value: String
}

#[graphql(description="A vertex.")]
#[derive(Clone, Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct InputVertex {
    #[graphql(description="The id of the vertex.")]
    pub id: Uuid,

    #[graphql(description="The type of the vertex.")]
    pub t: InputType,
}
