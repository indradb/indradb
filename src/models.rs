use std::collections::BTreeMap;
use serde_json::value::Value as JsonValue;
use traits::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex<I: Id> {
	pub id: I,
	#[serde(rename="type")]
	pub t: String,
	pub properties: BTreeMap<String, JsonValue>
}

impl<I: Id> Vertex<I> {
	pub fn new(id: I, t: String) -> Vertex<I> {
		Vertex::new_with_properties(id, t, BTreeMap::new())
	}

	pub fn new_with_properties(id: I, t: String, properties: BTreeMap<String, JsonValue>) -> Vertex<I> {
		Vertex {
			id: id,
			t: t,
			properties: properties
		}
	}
}

impl<I: Id> PartialEq for Vertex<I> {
	fn eq(&self, other: &Vertex<I>) -> bool {
		self.id == other.id
	}
}

impl<I: Id> Eq for Vertex<I>{}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge<I: Id> {
	pub outbound_id: I,
	#[serde(rename="type")]
	pub t: String,
	pub inbound_id: I,
	pub weight: f32,
	pub properties: BTreeMap<String, JsonValue>
}

impl<I: Id> Edge<I> {
	pub fn new(outbound_id: I, t: String, inbound_id: I, weight: f32) -> Edge<I> {
		Edge::new_with_properties(outbound_id, t, inbound_id, weight, BTreeMap::new())
	}

	pub fn new_with_properties(outbound_id: I, t: String, inbound_id: I, weight: f32, properties: BTreeMap<String, JsonValue>) -> Edge<I> {
		Edge {
			outbound_id: outbound_id,
			t: t,
			inbound_id: inbound_id,
			weight: weight,
			properties: properties
		}
	}
}

impl<I: Id> PartialEq for Edge<I> {
	fn eq(&self, other: &Edge<I>) -> bool {
		self.outbound_id == other.outbound_id && self.t == other.t && self.inbound_id == other.inbound_id
	}
}

impl<I: Id> Eq for Edge<I>{}
