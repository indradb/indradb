use super::models;
use super::datastore::Id;
use std::option::Option;
use chrono::naive::datetime::NaiveDateTime;
use std::collections::BTreeMap;
use serde_json::value::Value as JsonValue;

#[derive(Clone, Debug)]
pub enum Request<I: Id> {
	GetVertex(I),
	CreateVertex(String, BTreeMap<String, JsonValue>),
	SetVertex(models::Vertex<I>),
	DeleteVertex(I),
	GetEdge(I, String, I),
	SetEdge(models::Edge<I>),
	DeleteEdge(I, String, I),
	GetEdgeCount(I, String),
	GetEdgeRange(I, String, i64, i32),
	GetEdgeTimeRange(I, String, Option<NaiveDateTime>, Option<NaiveDateTime>, i32)
}
