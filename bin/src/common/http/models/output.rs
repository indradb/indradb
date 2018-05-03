use chrono::DateTime;
use chrono::offset::Utc;
use juniper::{ID, Value};

#[graphql(description="Represents something that could be returned by a query - a vertex, edge, or metadata.")]
#[derive(Clone, Debug)]
pub enum OutputItem {
    Edge(OutputEdge),
    Vertex(OutputVertex),
    EdgeMetadata(OutputEdgeMetadata),
    VertexMetadata(OutputVertexMetadata)
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

#[graphql(description="An edge.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdge {
    #[graphql(description="The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description="When the edge was created.")]
    pub created_datetime: DateTime<Utc>,
}

#[graphql(description="Represents vertex metadata.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputVertexMetadata {
    #[graphql(description="The id of the vertex")]
    pub id: ID,

    #[graphql(description="The metadata value as stringified JSON.")]
    pub value: String,
}

#[graphql(description="Represents edge metadata.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputEdgeMetadata {
    #[graphql(description="The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description="The metadata value as stringified JSON.")]
    pub value: String,
}

#[graphql(description="An edge or vertex type.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputType {
    value: String
}

#[graphql(description="A vertex.")]
#[derive(Clone, Debug, GraphQLObject)]
pub struct OutputVertex {
    #[graphql(description="The id of the vertex.")]
    pub id: ID,

    #[graphql(description="The type of the vertex.")]
    pub t: OutputType,
}
