use iron::prelude::*;
use iron::status;
use nutrino::{Vertex, Edge, Transaction, Error};
use common::ProxyTransaction;
use std::collections::BTreeMap;
use serde_json::value::Value as JsonValue;
use serde_json;
use serde::ser::Serialize;
use std::u16;
use super::util::*;

pub fn transaction(req: &mut Request) -> IronResult<Response> {
	let trans = try!(get_transaction(req));
	let mut idx: u16 = 0;
	let mut jsonable_res: Vec<JsonValue> = Vec::new();

	match read_required_json(&mut req.body) {
		Ok(JsonValue::Array(items)) => {
			for item in items {
				match item {
					JsonValue::Object(obj) => {
						let action = try!(get_required_json_string_param(&obj, "action"));

						let result: Result<JsonValue, IronError> = match &action[..] {
							"get_vertex" => trans_get_vertex(&trans, &obj),
							"create_vertex" => trans_create_vertex(&trans, &obj),
							"set_vertex" => trans_set_vertex(&trans, &obj),
							"delete_vertex" => trans_delete_vertex(&trans, &obj),
							
							"get_edge" => trans_get_edge(&trans, &obj),
							"set_edge" => trans_set_edge(&trans, &obj),
							"delete_edge" => trans_delete_edge(&trans, &obj),
							
							"get_edge_count" => trans_get_edge_count(&trans, &obj),
							"get_edge_range" => trans_get_edge_range(&trans, &obj),
							"get_edge_time_range" => trans_get_edge_time_range(&trans, &obj),
							
							"get_reversed_edge_count" => trans_get_reversed_edge_count(&trans, &obj),
							"get_reversed_edge_range" => trans_get_reversed_edge_range(&trans, &obj),
							"get_reversed_edge_time_range" => trans_get_reversed_edge_time_range(&trans, &obj),
							
							_ => {
								return Err(create_iron_error(status::BadRequest, "Unknown action".to_string()))
							}
						};

						match result {
							Err(err) => {
								let message = format!("Item #{}: {}", idx, err);
								return Err(create_iron_error(status::BadRequest, message));
							}
							Ok(value) => {
								jsonable_res.push(value);
							}
						}

						idx += 1;
					},
					_ => {
						return Err(create_iron_error(status::BadRequest, format!("Item #{}: Invalid type", idx)))
					}
				}
			}
		},
		_ => return Err(create_iron_error(status::BadRequest, "Request body should be an array".to_string()))
	}

	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &jsonable_res))
}

fn trans_get_vertex(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_required_json_uuid_param(item, "id"));
	execute_trans_item(trans.get_vertex(id))
}

fn trans_create_vertex(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let t = try!(get_required_json_type_param(item, "type"));
	execute_trans_item(trans.create_vertex(t))
}

fn trans_set_vertex(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_required_json_uuid_param(item, "id"));
	let t = try!(get_required_json_type_param(item, "type"));
	execute_trans_item(trans.set_vertex(Vertex::new(id, t)))
}

fn trans_delete_vertex(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_required_json_uuid_param(item, "id"));
	execute_trans_item(trans.delete_vertex(id))
}

fn trans_get_edge(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	execute_trans_item(trans.get_edge(outbound_id, t, inbound_id))
}

fn trans_set_edge(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	let weight = try!(get_required_json_weight_param(item, "weight"));
	execute_trans_item(trans.set_edge(Edge::new(outbound_id, t, inbound_id, weight)))
}

fn trans_delete_edge(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	execute_trans_item(trans.delete_edge(outbound_id, t, inbound_id))
}

fn trans_get_edge_count(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	execute_trans_item(trans.get_edge_count(outbound_id, t))
}

fn trans_get_edge_range(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	let limit = parse_limit(try!(get_optional_json_u16_param(item, "limit")));
	let offset = try!(get_optional_json_u64_param(item, "offset")).unwrap_or(0);
	execute_trans_item(trans.get_edge_range(outbound_id, t, offset, limit))
}

fn trans_get_edge_time_range(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	let limit = parse_limit(try!(get_optional_json_u16_param(item, "limit")));
	let high = parse_datetime(try!(get_optional_json_i64_param(item, "high")));
	let low = parse_datetime(try!(get_optional_json_i64_param(item, "low")));
	execute_trans_item(trans.get_edge_time_range(outbound_id, t, high, low, limit))
}

fn trans_get_reversed_edge_count(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	execute_trans_item(trans.get_reversed_edge_count(inbound_id, t))
}

fn trans_get_reversed_edge_range(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	let limit = parse_limit(try!(get_optional_json_u16_param(item, "limit")));
	let offset = try!(get_optional_json_u64_param(item, "offset")).unwrap_or(0);
	execute_trans_item(trans.get_reversed_edge_range(inbound_id, t, offset, limit))
}

fn trans_get_reversed_edge_time_range(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	let t = try!(get_required_json_type_param(item, "type"));
	let limit = parse_limit(try!(get_optional_json_u16_param(item, "limit")));
	let high = parse_datetime(try!(get_optional_json_i64_param(item, "high")));
	let low = parse_datetime(try!(get_optional_json_i64_param(item, "low")));
	execute_trans_item(trans.get_reversed_edge_time_range(inbound_id, t, high, low, limit))
}

fn execute_trans_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
	let result = try!(datastore_request(result));
	Ok(serde_json::to_value(&result))
}
