use std::collections::BTreeMap;
use serde_json::value::Value as JsonValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
	pub id: i64,
	#[serde(rename="type")]
	pub t: String,
	pub properties: BTreeMap<String, JsonValue>
}

impl Vertex {
	pub fn new(id: i64, t: String) -> Vertex {
		Vertex::new_with_properties(id, t, BTreeMap::new())
	}

	pub fn new_with_properties(id: i64, t: String, properties: BTreeMap<String, JsonValue>) -> Vertex {
		Vertex {
			id: id,
			t: t,
			properties: properties
		}
	}
}

impl PartialEq for Vertex {
	fn eq(&self, other: &Vertex) -> bool {
		self.id == other.id
	}
}

impl Eq for Vertex{}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Type {
	pub id: i64,
	pub value: String
}

impl Type {
	pub fn new(id: i64, value: String) -> Type {
		return Type {
			id: id,
			value: value
		}
	}
}

impl PartialEq for Type {
	fn eq(&self, other: &Type) -> bool {
		self.id == other.id
	}
}

impl Eq for Type{}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
	pub outbound_id: i64,
	#[serde(rename="type")]
	pub t: String,
	pub inbound_id: i64,
	pub weight: f32,
	pub properties: BTreeMap<String, JsonValue>
}

impl Edge {
	pub fn new(outbound_id: i64, t: String, inbound_id: i64, weight: f32) -> Edge {
		Edge::new_with_properties(outbound_id, t, inbound_id, weight, BTreeMap::new())
	}

	pub fn new_with_properties(outbound_id: i64, t: String, inbound_id: i64, weight: f32, properties: BTreeMap<String, JsonValue>) -> Edge {
		Edge {
			outbound_id: outbound_id,
			t: t,
			inbound_id: inbound_id,
			weight: weight,
			properties: properties
		}
	}
}

impl PartialEq for Edge {
	fn eq(&self, other: &Edge) -> bool {
		self.outbound_id == other.outbound_id && self.t == other.t && self.inbound_id == other.inbound_id
	}
}

impl Eq for Edge{}
