use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use serde_json::Value as JsonValue;
use core::str::FromStr;
use regex::Regex;
use util::generate_uuid_v1;

#[graphql(description="Represents a uniquely identifiable key to an edge.")]
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Ord, PartialOrd, GraphQLObject)]
pub struct OutputEdgeKey {
    #[graphql(description="The id of the outbound vertex.")]
    pub outbound_id: Uuid,

    #[graphql(description="The type of the edge.")]
    pub t: OutputType,

    #[graphql(description="The id of the inbound vertex.")]
    pub inbound_id: Uuid,
}

#[graphql(description="An edge.")]
#[derive(Clone, Debug, Serialize, Deserialize, GraphQLObject)]
pub struct OutputEdge {
    #[graphql(description="The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description="When the edge was created.")]
    pub created_datetime: DateTime<Utc>,
}

#[graphql(description="Represents vertex metadata.")]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, GraphQLObject)]
pub struct OutputVertexMetadata {
    #[graphql(description="The id of the vertex")]
    pub id: Uuid,

    #[graphql(description="The metadata value.")]
    pub value: JsonValue,
}

#[graphql(description="Represents edge metadata.")]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, GraphQLObject)]
pub struct OutputEdgeMetadata {
    #[graphql(description="The key to the edge.")]
    pub key: OutputEdgeKey,

    #[graphql(description="The metadata value.")]
    pub value: JsonValue,
}

#[graphql(description="An edge or vertex type.")]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Ord, PartialOrd, GraphQLObject)]
pub struct OutputType {
    value: String
}

#[graphql(description="A vertex.")]
#[derive(Clone, Debug, Serialize, Deserialize, GraphQLObject)]
pub struct OutputVertex {
    #[graphql(description="The id of the vertex.")]
    pub id: Uuid,

    #[graphql(description="The type of the vertex.")]
    pub t: OutputType,
}
