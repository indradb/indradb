use iron::prelude::*;
use std::i64;
use iron::status;
use nutrino::{Vertex, Edge, Transaction, Type};
use std::io::Read;
use serde_json::value::Value as JsonValue;
use script;
use std::path::Path;
use regex;
use std::fs::File;
use std::u16;
use uuid::Uuid;
use super::util::*;
use statics;

lazy_static! {
	static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[\w-_]+(\.lua)?$").unwrap();
}

pub fn get_vertex(req: &mut Request) -> IronResult<Response> {
	let id: Uuid = try!(get_url_param(req, "id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.get_vertex(id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

pub fn create_vertex(req: &mut Request) -> IronResult<Response> {
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_required_json_type_param(&obj, "type"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.create_vertex(t)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

pub fn set_vertex(req: &mut Request) -> IronResult<Response> {
	let id: Uuid = try!(get_url_param(req, "id"));
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_required_json_type_param(&obj, "type"));
	let v = Vertex::new(id, t);
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.set_vertex(v)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

pub fn delete_vertex(req: &mut Request) -> IronResult<Response> {
	let id: Uuid = try!(get_url_param(req, "id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.delete_vertex(id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

pub fn get_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: Type = try!(get_url_param(req, "type"));
	let inbound_id: Uuid = try!(get_url_param(req, "inbound_id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.get_edge(outbound_id, t, inbound_id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

pub fn set_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: Type = try!(get_url_param(req, "type"));
	let inbound_id: Uuid = try!(get_url_param(req, "inbound_id"));
	let obj = try!(read_json_object(&mut req.body));
	let weight = try!(get_required_json_weight_param(&obj, "weight"));
	let e = Edge::new(outbound_id, t, inbound_id, weight);

	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.set_edge(e)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

pub fn delete_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: Type = try!(get_url_param(req, "type"));
	let inbound_id: Uuid = try!(get_url_param(req, "inbound_id"));

	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.delete_edge(outbound_id, t, inbound_id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

pub fn get_edge_range(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: Type = try!(get_url_param(req, "type"));
	let trans = try!(get_transaction(req));
	let query_params = try!(get_query_params(req));
	let action = &try!(get_query_param::<String>(query_params, "action".to_string(), true)).unwrap()[..];

	match action {
		"count" => {
			let result = try!(datastore_request(trans.get_edge_count(outbound_id, t)));
			try!(datastore_request(trans.commit()));
			Ok(to_response(status::Ok, &result))
		},
		"time" => {
			let limit = parse_limit(try!(get_query_param::<u16>(query_params, "limit".to_string(), false)));
			let high = parse_datetime(try!(get_query_param::<i64>(query_params, "high".to_string(), false)));
			let low = parse_datetime(try!(get_query_param::<i64>(query_params, "low".to_string(), false)));
			let result = try!(datastore_request(trans.get_edge_time_range(outbound_id, t, high, low, limit)));
			try!(datastore_request(trans.commit()));
			Ok(to_response(status::Ok, &result))
		},
		"position" => {
			let limit = parse_limit(try!(get_query_param::<u16>(query_params, "limit".to_string(), false)));
			let offset = try!(get_query_param::<u64>(query_params, "offset".to_string(), false)).unwrap_or(0);
			let result = try!(datastore_request(trans.get_edge_range(outbound_id, t, offset, limit)));
			try!(datastore_request(trans.commit()));
			Ok(to_response(status::Ok, &result))
		},
		_ => {
			Err(create_iron_error(status::BadRequest, "Invalid `action`".to_string()))
		}
	}
}

pub fn get_reversed_edge_range(req: &mut Request) -> IronResult<Response> {
	let inbound_id: Uuid = try!(get_url_param(req, "inbound_id"));
	let t: Type = try!(get_url_param(req, "type"));
	let trans = try!(get_transaction(req));
	let query_params = try!(get_query_params(req));
	let action = &try!(get_query_param::<String>(query_params, "action".to_string(), true)).unwrap()[..];

	match action {
		"count" => {
			let result = try!(datastore_request(trans.get_reversed_edge_count(inbound_id, t)));
			try!(datastore_request(trans.commit()));
			Ok(to_response(status::Ok, &result))
		},
		"time" => {
			let limit = parse_limit(try!(get_query_param::<u16>(query_params, "limit".to_string(), false)));
			let high = parse_datetime(try!(get_query_param::<i64>(query_params, "high".to_string(), false)));
			let low = parse_datetime(try!(get_query_param::<i64>(query_params, "low".to_string(), false)));
			let result = try!(datastore_request(trans.get_reversed_edge_time_range(inbound_id, t, high, low, limit)));
			try!(datastore_request(trans.commit()));
			Ok(to_response(status::Ok, &result))
		},
		"position" => {
			let limit = parse_limit(try!(get_query_param::<u16>(query_params, "limit".to_string(), false)));
			let offset = try!(get_query_param::<u64>(query_params, "offset".to_string(), false)).unwrap_or(0);
			let result = try!(datastore_request(trans.get_reversed_edge_range(inbound_id, t, offset, limit)));
			try!(datastore_request(trans.commit()));
			Ok(to_response(status::Ok, &result))
		},
		_ => {
			Err(create_iron_error(status::BadRequest, "Invalid `action`".to_string()))
		}
	}
}

pub fn script(req: &mut Request) -> IronResult<Response> {
	let script_name: String = try!(get_url_param(req, "name"));

	if !SCRIPT_NAME_VALIDATOR.is_match(&script_name[..]) {
		return Err(create_iron_error(status::BadRequest, "Invalid script name".to_string()));
	}

	let arg = match try!(read_optional_json(&mut req.body)) {
		Some(val) => val,
		None => JsonValue::Null
	};

	let path = Path::new(&statics::SCRIPT_ROOT[..]).join(script_name);

	let mut f = match File::open(path) {
		Ok(f) => f,
		Err(_) => return Err(create_iron_error(status::NotFound, "Could not load script".to_string()))
	};

	let mut payload = String::new();

	if let Err(err) = f.read_to_string(&mut payload) {
		return Err(create_iron_error(status::InternalServerError, format!("Could not read script contents: {}", err)));
	}

	let account_id = get_account_id(req);
	let trans = try!(get_transaction(req));

	match script::run(trans, account_id, &payload[..], arg) {
		Ok(val) => Ok(to_response(status::Ok, &val)),
		Err(err) => Err(create_iron_error(status::InternalServerError, format!("Script failed: {:?}", err)))
	}
}