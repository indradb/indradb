use uuid::Uuid;
use models;
use chrono::{UTC, DateTime};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum QueryTypeConverter {
    #[serde(rename="outbound")]
    Outbound,
    #[serde(rename="inbound")]
    Inbound
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum VertexQuery {
    #[serde(rename="all")]
    All(Uuid, u32),
    #[serde(rename="vertex")]
    Vertex(Uuid),
    #[serde(rename="vertices")]
    Vertices(Vec<Uuid>),
    #[serde(rename="pipe")]
    Pipe(Box<EdgeQuery>, QueryTypeConverter, u32)
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum EdgeQuery {
    #[serde(rename="all")]
    All(Option<models::Type>, Option<DateTime<UTC>>, Option<DateTime<UTC>>, u32),
    #[serde(rename="edge")]
    Edge(Uuid, models::Type, Uuid),
    #[serde(rename="edges")]
    Edges(Vec<(Uuid, models::Type, Uuid)>),
    #[serde(rename="pipe")]
    Pipe(Box<VertexQuery>, QueryTypeConverter, Option<models::Type>, Option<DateTime<UTC>>, Option<DateTime<UTC>>, u32)
}
