use super::models;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Response {
	#[serde(rename="id")]
	VertexId(i64),
	#[serde(rename="vertex")]
	Vertex(models::Vertex),
	#[serde(rename="edge")]
	Edge(models::Edge),
	#[serde(rename="count")]
	Count(i64),
	#[serde(rename="edges")]
	Edges(Vec<models::Edge>),
	#[serde(rename="ok")]
	Ok
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ErrorResponse {
	#[serde(rename="unexpected")]
	Unexpected(String),
	#[serde(rename="vertex_does_not_exist")]
	VertexDoesNotExist(i64),
	#[serde(rename="edge_does_not_exist")]
	EdgeDoesNotExist(i64, String, i64),
	#[serde(rename="weight_out_of_range")]
	WeightOutOfRange,
	#[serde(rename="offset_out_of_range")]
	OffsetOutOfRange,
	#[serde(rename="limit_out_of_range")]
	LimitOutOfRange,
}
