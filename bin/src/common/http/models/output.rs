use chrono::DateTime;
use chrono::offset::Utc;
use juniper::{ID, Value};
use indradb;

#[graphql(description="Represents something that could be returned by a query - a vertex, edge, or metadata.")]
#[derive(Clone, Debug)]
pub enum OutputItem {
    Edge(OutputEdge),
    Vertex(OutputVertex),
    EdgeMetadata(OutputEdgeMetadata),
    VertexMetadata(OutputVertexMetadata)
}

impl From<indradb::Edge> for OutputItem {
    fn from(e: indradb::Edge) -> OutputItem {
        OutputItem::Edge(OutputEdge::from(e))
    }
}

impl From<indradb::Vertex> for OutputItem {
    fn from(v: indradb::Vertex) -> OutputItem {
        OutputItem::Vertex(OutputVertex::from(v))
    }
}

impl From<indradb::EdgeMetadata> for OutputItem {
    fn from(e: indradb::EdgeMetadata) -> OutputItem {
        OutputItem::EdgeMetadata(OutputEdgeMetadata::from(e))
    }
}

impl From<indradb::VertexMetadata> for OutputItem {
    fn from(v: indradb::VertexMetadata) -> OutputItem {
        OutputItem::VertexMetadata(OutputVertexMetadata::from(v))
    }
}

graphql_union!(OutputItem: () |&self| {
    instance_resolvers: |_| {
        &OutputEdge => match *self {
            OutputItem::Edge(ref e) => Some(e),
            _ => None
        },
        &OutputVertex => match *self {
            OutputItem::Vertex(ref v) => Some(v),
            _ => None
        },
        &OutputEdgeMetadata => match *self {
            OutputItem::EdgeMetadata(ref e) => Some(e),
            _ => None
        },
        &OutputVertexMetadata => match *self {
            OutputItem::VertexMetadata(ref v) => Some(v),
            _ => None
        }
    }
});

#[graphql(description="Represents a uniquely identifiable key to an edge.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdgeKey {
    #[graphql(description="The id of the outbound vertex.")]
    pub outbound_id: ID,

    #[graphql(description="The type of the edge.")]
    pub t: OutputType,

    #[graphql(description="The id of the inbound vertex.")]
    pub inbound_id: ID,
}



impl From<indradb::EdgeKey> for OutputEdgeKey {
    fn from(key: indradb::EdgeKey) -> OutputEdgeKey {
        OutputEdgeKey {
            outbound_id: ID::from(key.outbound_id.hyphenated().to_string()),
            t: OutputType::from(key.t),
            inbound_id: ID::from(key.inbound_id.hyphenated().to_string()),
        }
    }
}

#[graphql(description="An edge.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdge {
    #[graphql(description="The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description="When the edge was created.")]
    pub created_datetime: DateTime<Utc>,
}

impl From<indradb::Edge> for OutputEdge {
    fn from(e: indradb::Edge) -> OutputEdge {
        OutputEdge {
            key: OutputEdgeKey::from(e.key),
            created_datetime: e.created_datetime
        }
    }
}

#[graphql(description="Represents vertex metadata.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputVertexMetadata {
    #[graphql(description="The id of the vertex")]
    pub id: ID,

    #[graphql(description="The metadata value as stringified JSON.")]
    pub value: String,
}

impl From<indradb::VertexMetadata> for OutputVertexMetadata {
    fn from(v: indradb::VertexMetadata) -> OutputVertexMetadata {
        OutputVertexMetadata {
            id: ID::from(v.id.hyphenated().to_string()),
            value: v.value.to_string()
        }
    }
}

#[graphql(description="Represents edge metadata.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdgeMetadata {
    #[graphql(description="The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description="The metadata value as stringified JSON.")]
    pub value: String,
}

impl From<indradb::EdgeMetadata> for OutputEdgeMetadata {
    fn from(e: indradb::EdgeMetadata) -> OutputEdgeMetadata {
        OutputEdgeMetadata {
            key: OutputEdgeKey::from(e.key),
            value: e.value.to_string()
        }
    }
}

#[graphql(description="An edge or vertex type.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputType {
    value: String
}

impl From<indradb::Type> for OutputType {
    fn from(t: indradb::Type) -> OutputType {
        OutputType {
            value: t.0
        }
    }
}

#[graphql(description="A vertex.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputVertex {
    #[graphql(description="The id of the vertex.")]
    pub id: ID,

    #[graphql(description="The type of the vertex.")]
    pub t: OutputType,
}

impl From<indradb::Vertex> for OutputVertex {
    fn from(v: indradb::Vertex) -> OutputVertex {
        OutputVertex {
            id: ID::from(v.id.hyphenated().to_string()),
            t: OutputType::from(v.t)
        }
    }
}
