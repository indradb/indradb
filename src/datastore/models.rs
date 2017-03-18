use uuid::Uuid;
use models;
use chrono::{UTC, DateTime};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum QueryTypeConverter {
    Outbound,
    Inbound
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum VertexQuery {
    All(Uuid, u32),
    Vertex(Uuid),
    Vertices(Vec<Uuid>),
    Pipe(Box<EdgeQuery>, QueryTypeConverter, Option<u32>)
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash)]
pub enum EdgeQuery {
    All(EdgeQueryFilter),
    Edge(Uuid, models::Type, Uuid),
    Edges(Vec<(Uuid, models::Type, Uuid)>),
    Pipe(Box<VertexQuery>, QueryTypeConverter, EdgeQueryFilter)
}

pub type EdgeQueryFilter = (Option<models::Type>, Option<DateTime<UTC>>, Option<DateTime<UTC>>, Option<u32>);
