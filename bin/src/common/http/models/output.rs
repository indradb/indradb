use chrono::DateTime;
use chrono::offset::Utc;
use indradb;
use juniper::ID;

#[derive(Clone, Debug)]
pub enum OutputItem {
    Edge(OutputEdge),
    Vertex(OutputVertex),
    EdgeMetadata(OutputEdgeMetadata),
    VertexMetadata(OutputVertexMetadata),
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

#[graphql(description = "Represents a uniquely identifiable key to an edge.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdgeKey {
    #[graphql(description = "The id of the outbound vertex.")]
    pub outbound_id: ID,

    #[graphql(description = "The type of the edge.")]
    pub t: String,

    #[graphql(description = "The id of the inbound vertex.")]
    pub inbound_id: ID,
}

impl From<indradb::EdgeKey> for OutputEdgeKey {
    fn from(key: indradb::EdgeKey) -> OutputEdgeKey {
        OutputEdgeKey {
            outbound_id: ID::from(key.outbound_id.hyphenated().to_string()),
            t: key.t.0,
            inbound_id: ID::from(key.inbound_id.hyphenated().to_string()),
        }
    }
}

#[graphql(description = "An edge.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdge {
    #[graphql(description = "The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description = "When the edge was created.")]
    pub created_datetime: DateTime<Utc>,
}

impl From<indradb::Edge> for OutputEdge {
    fn from(e: indradb::Edge) -> OutputEdge {
        OutputEdge {
            key: OutputEdgeKey::from(e.key),
            created_datetime: e.created_datetime,
        }
    }
}

#[graphql(description = "Represents vertex metadata.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputVertexMetadata {
    #[graphql(description = "The id of the vertex")]
    pub id: ID,

    #[graphql(description = "The metadata value as stringified JSON.")]
    pub value: String,
}

impl From<indradb::VertexMetadata> for OutputVertexMetadata {
    fn from(v: indradb::VertexMetadata) -> OutputVertexMetadata {
        OutputVertexMetadata {
            id: ID::from(v.id.hyphenated().to_string()),
            value: v.value.to_string(),
        }
    }
}

#[graphql(description = "Represents edge metadata.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdgeMetadata {
    #[graphql(description = "The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description = "The metadata value as stringified JSON.")]
    pub value: String,
}

impl From<indradb::EdgeMetadata> for OutputEdgeMetadata {
    fn from(e: indradb::EdgeMetadata) -> OutputEdgeMetadata {
        OutputEdgeMetadata {
            key: OutputEdgeKey::from(e.key),
            value: e.value.to_string(),
        }
    }
}

#[graphql(description = "A vertex.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputVertex {
    #[graphql(description = "The id of the vertex.")]
    pub id: ID,

    #[graphql(description = "The type of the vertex.")]
    pub t: String,
}

impl From<indradb::Vertex> for OutputVertex {
    fn from(v: indradb::Vertex) -> OutputVertex {
        OutputVertex {
            id: ID::from(v.id.hyphenated().to_string()),
            t: v.t.0,
        }
    }
}
