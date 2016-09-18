use super::models;
use std::option::Option;
use chrono::naive::datetime::NaiveDateTime;
use std::collections::BTreeMap;
use serde_json::value::Value as JsonValue;

#[derive(Clone, Debug)]
pub enum Request {
	GetVertex(i64),
	CreateVertex(String, BTreeMap<String, JsonValue>),
	SetVertex(models::Vertex),
	DeleteVertex(i64),
	GetEdge(i64, String, i64),
	SetEdge(models::Edge),
	DeleteEdge(i64, String, i64),
	GetEdgeCount(i64, String),
	GetEdgeRange(i64, String, i64, i64),
	GetEdgeTimeRange(i64, String, Option<NaiveDateTime>, Option<NaiveDateTime>, i64)
}
