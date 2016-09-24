use super::models;
use super::datastore::Id;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Response<I: Id> {
	#[serde(rename="id")]
	VertexId(I),
	#[serde(rename="vertex")]
	Vertex(models::Vertex<I>),
	#[serde(rename="edge")]
	Edge(models::Edge<I>),
	#[serde(rename="count")]
	Count(i64),
	#[serde(rename="edges")]
	Edges(Vec<models::Edge<I>>),
	#[serde(rename="ok")]
	Ok
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ErrorResponse<I: Id> {
	#[serde(rename="unexpected")]
	Unexpected(String),
	#[serde(rename="vertex_does_not_exist")]
	VertexDoesNotExist(I),
	#[serde(rename="edge_does_not_exist")]
	EdgeDoesNotExist(I, String, I),
	#[serde(rename="weight_out_of_range")]
	WeightOutOfRange,
	#[serde(rename="offset_out_of_range")]
	OffsetOutOfRange,
	#[serde(rename="limit_out_of_range")]
	LimitOutOfRange,
}
