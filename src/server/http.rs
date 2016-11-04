use iron::prelude::*;
use std::i32;
use std::i64;
use iron::status;
use iron::headers::{Headers, ContentType, Authorization, Basic};
use iron::typemap::{Key, TypeMap};
use iron::middleware::BeforeMiddleware;
use router::Router;
use util::SimpleError;
use nutrino::{Vertex, Edge, Transaction, Datastore, PostgresTransaction, Error};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use core::str::FromStr;
use iron::modifiers::Header as HeaderModifier;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::request::Body;
use std::io;
use std::io::Read;
use chrono::naive::datetime::NaiveDateTime;
use serde_json::value::Value as JsonValue;
use serde_json;
use urlencoded::UrlEncodedQuery;
use serde::ser::Serialize;
use std::collections::HashMap;
use scripts;
use std::env;
use std::path::Path;
use regex;
use std::fs::File;
use std::cmp::min;
use datastore::DATASTORE;

header! { (WWWAuthenticate, "WWW-Authenticate") => [String] }

const MAX_RETURNABLE_EDGES: i32 = 1000;

// -- Public function for starting the server
pub fn start(port: u16) {
	let mut router = Router::new();

	router.post("/vertex", on_create_vertex);
	router.post("/transaction", on_transaction);

	router.get("/edge/:outbound_id/:type/:inbound_id", on_get_edge);
	router.put("/edge/:outbound_id/:type/:inbound_id", on_set_edge);
	router.delete("/edge/:outbound_id/:type/:inbound_id", on_delete_edge);
	router.get("/edge/:outbound_id/:type", on_get_edge_range);

	router.get("/vertex/:id", on_get_vertex);
	router.put("/vertex/:id", on_set_vertex);
	router.delete("/vertex/:id", on_delete_vertex);

	router.post("/script", on_input_script);
	router.post("/script/:name", on_named_script);

	let binding = format!("0.0.0.0:{}", port);
	println!("Listening on {}", binding);

	let mut chain = Chain::new(router);
	chain.link_before(BasicAuthMiddleware::new());
	Iron::new(chain).http(&*binding).unwrap();
}

// -- Basic http auth middleware
struct BasicAuthMiddleware {
}

impl BasicAuthMiddleware {
	fn new() -> BasicAuthMiddleware {
		BasicAuthMiddleware {}
	}

	fn get_account_id(&self, auth: Option<&Authorization<Basic>>) -> Option<i64> {
		if let Some(auth) = auth {
			if let Ok(val) = auth.username.parse::<i64>() {
				return Some(val);
			}
		}

		None
	}

	fn get_secret(&self, auth: Option<&Authorization<Basic>>) -> Option<String> {
		if let Some(auth) = auth {
			return auth.password.clone();
		}

		None
	}
}

impl BeforeMiddleware for BasicAuthMiddleware {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		let auth = req.headers.get::<Authorization<Basic>>();
		let account_id = self.get_account_id(auth);
		let secret = self.get_secret(auth);

		if account_id.is_some() && secret.is_some() && DATASTORE.auth(account_id.unwrap(), secret.unwrap()).unwrap_or(false) {
			req.extensions.insert::<AccountKey>(AccountKey {
				account_id: account_id.unwrap()
			});

			return Ok(());
		}

		let error_message = "Authentication failed".to_string();

		let mut d: BTreeMap<String, String> = BTreeMap::new();
		d.insert("error".to_string(), error_message.clone());
		let body = serde_json::to_string(&d).unwrap();

		let www_authenticate_header = WWWAuthenticate("Basic realm=\"main\"".to_owned());
		let www_authenticate_modifier = HeaderModifier(www_authenticate_header);
		let json_content_type_modifier = HeaderModifier(get_json_content_type());

		let modifiers = (status::Unauthorized, json_content_type_modifier, www_authenticate_modifier, body);
		Err(IronError::new(SimpleError::new(error_message), modifiers))
	}
}

// Need this to avoid orphan rules
struct AccountKey {
	account_id: i64
}

impl Key for AccountKey {
	type Value = AccountKey;
}

// -- Convenience functions
fn create_iron_error(status_code: status::Status, err: String) -> IronError {
	let mut d: BTreeMap<String, String> = BTreeMap::new();
	d.insert("error".to_string(), err.clone());
	let body = serde_json::to_string(&d).unwrap();
	let json_content_type_modifier = HeaderModifier(get_json_content_type());
	let modifiers = (status_code, json_content_type_modifier, body);
	IronError::new(SimpleError::new(err), modifiers)
}

fn get_json_content_type() -> ContentType {
	ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))
}

fn to_response<T: Serialize>(status_code: status::Status, body: &T) -> Response {
	let mut hs = Headers::new();
	hs.set(get_json_content_type());

	Response {
		status: Some(status_code),
		headers: hs,
		extensions: TypeMap::new(),
		body: Some(Box::new(serde_json::to_string(&body).unwrap()))
	}
}

fn get_url_param<T: FromStr>(req: &Request, name: &str) -> Result<T, IronError> {
	let s = req.extensions.get::<Router>().unwrap().find(name).unwrap();

	match T::from_str(s) {
		Ok(val) => Ok(val),
		Err(_) => Err(create_iron_error(status::BadRequest, format!("Invalid value for URL param {}", name)))
	}
}

macro_rules! create_json_param_fn {
	($func: ident, $enum_val: path, $ty: ty) => {
		fn $func(json: &BTreeMap<String, JsonValue>, name: &str, optional: bool) -> Result<Option<$ty>, IronError> {
			match json.get(name) {
				Some(&$enum_val(ref val)) => Ok(Some(val.clone())),
				None | Some(&JsonValue::Null) => {
					if optional {
						Ok(None)
					} else {
						Err(create_iron_error(status::BadRequest, format!("Missing `{}`", name)))
					}
				}
				_ => {
					Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
				}
			}
		}
	}
}

create_json_param_fn!(get_json_string_param, JsonValue::String, String);
create_json_param_fn!(get_json_f64_param, JsonValue::F64, f64);

fn get_json_i64_param(json: &BTreeMap<String, JsonValue>, name: &str, optional: bool) -> Result<Option<i64>, IronError> {
	match json.get(name) {
		Some(&JsonValue::I64(ref val)) => Ok(Some(val.clone())),
		Some(&JsonValue::U64(ref val)) => {
			let val = val.clone();
			if val > i64::MAX as u64 {
				Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
			} else {
				Ok(Some(val as i64))
			}
		},
		None | Some(&JsonValue::Null) => {
			if optional {
				Ok(None)
			} else {
				Err(create_iron_error(status::BadRequest, format!("Missing `{}`", name)))
			}
		}
		_ => {
			Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
		}
	}
}

fn get_json_i32_param(json: &BTreeMap<String, JsonValue>, name: &str, optional: bool) -> Result<Option<i32>, IronError> {
	match try!(get_json_i64_param(json, name, optional)) {
		Some(val) => {
			if val > i32::MAX as i64 {
				Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
			} else {
				Ok(Some(val as i32))
			}
		},
		None => Ok(None)
	}
}

fn parse_limit(val: Option<i32>) -> i32 {
	match val {
		Some(val) => min(val, MAX_RETURNABLE_EDGES),
		_ => MAX_RETURNABLE_EDGES
	}
}

fn parse_datetime(val: Option<i64>) -> Option<NaiveDateTime> {
	match val {
		Some(val) => Some(NaiveDateTime::from_timestamp(val, 0)),
		_ => None
	}
}

fn datastore_request<T>(result: Result<T, Error>) -> Result<T, IronError> {
	match result {
		Ok(result) => Ok(result),
		Err(err) => {
			let status = match err {
				Error::AccountNotFound | Error::VertexDoesNotExist | Error::EdgeDoesNotExist | Error::MetadataDoesNotExist => status::NotFound,
				Error::LimitOutOfRange | Error::OffsetOutOfRange | Error::WeightOutOfRange => status::BadRequest,
				Error::Unexpected(_) => status::InternalServerError
			};

			Err(create_iron_error(status, format!("{}", err)))
		}
	}
}

fn get_account_id(req: &Request) -> i64 {
	let ext = &(*req.extensions.get::<AccountKey>().unwrap());
	ext.account_id
}

fn get_transaction(req: &Request) -> Result<PostgresTransaction, IronError> {
	let account_id = get_account_id(req);
	match DATASTORE.transaction(account_id) {
		Ok(val) => Ok(val),
		Err(err) => Err(create_iron_error(status::InternalServerError, format!("Could not create datastore transaction: {}", err)))
	}
}

fn read_json(body: &mut Body) -> Result<JsonValue, IronError> {
	let mut payload = String::new();
	let read_result: Result<usize, io::Error> = body.read_to_string(&mut payload);

	if let Err(err) = read_result {
		return Err(create_iron_error(status::BadRequest, format!("Could not read JSON body: {}", err)))
	}

	match serde_json::from_str(&payload[..]) {
	    Ok(json) => Ok(json),
	    Err(err) => Err(create_iron_error(status::BadRequest, format!("Could not parse JSON payload: {}", err.description())))
	}
}

fn read_json_object(body: &mut Body) -> Result<BTreeMap<String, JsonValue>, IronError> {
	match try!(read_json(body)) {
		JsonValue::Object(obj) => Ok(obj),
		_ => Err(create_iron_error(status::BadRequest, "Bad payload: expected object".to_string()))
	}
}

fn get_query_params<'a>(req: &'a mut Request) -> Result<&'a HashMap<String, Vec<String>>, IronError> {
	match req.get_ref::<UrlEncodedQuery>() {
        Ok(map) => Ok(map),
        Err(_) => Err(create_iron_error(status::BadRequest, "Could not parse query parameters".to_string()))
    }
}

fn get_query_param<T: FromStr>(params: &HashMap<String, Vec<String>>, key: String, required: bool) -> Result<Option<T>, IronError> {
	if let Some(values) = params.get(&key) {
		if let Some(first_value) = values.get(0) {
			match first_value.parse::<T>() {
				Ok(value) => return Ok(Some(value)),
				Err(_) => return Err(create_iron_error(status::BadRequest, format!("Could not parse query parameter `{}`", key)))
			}
		}
	}

	if required {
		Err(create_iron_error(status::BadRequest, format!("Missing required query parameter `{}`", key)))
	} else {
		Ok(None)
	}
}

// -- Handlers
fn on_get_vertex(req: &mut Request) -> IronResult<Response> {
	let id: i64 = try!(get_url_param(req, "id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.get_vertex(id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_create_vertex(req: &mut Request) -> IronResult<Response> {
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_json_string_param(&obj, "type", false));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.create_vertex(t.unwrap())));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_set_vertex(req: &mut Request) -> IronResult<Response> {
	let id: i64 = try!(get_url_param(req, "id"));
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_json_string_param(&obj, "type", false));
	let v = Vertex::new(id, t.unwrap());
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.set_vertex(v)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_delete_vertex(req: &mut Request) -> IronResult<Response> {
	let id: i64 = try!(get_url_param(req, "id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.delete_vertex(id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_get_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));
	let inbound_id: i64 = try!(get_url_param(req, "inbound_id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.get_edge(outbound_id, t, inbound_id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_set_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));
	let inbound_id: i64 = try!(get_url_param(req, "inbound_id"));
	let obj = try!(read_json_object(&mut req.body));
	let weight = try!(get_json_f64_param(&obj, "weight", false));
	let e = Edge::new(outbound_id, t, inbound_id, weight.unwrap() as f32);

	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.set_edge(e)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_delete_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));
	let inbound_id: i64 = try!(get_url_param(req, "inbound_id"));

	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.delete_edge(outbound_id, t, inbound_id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_get_edge_range(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));

	let trans = try!(get_transaction(req));

	let query_params = try!(get_query_params(req));
	let action = &try!(get_query_param::<String>(query_params, "action".to_string(), true)).unwrap()[..];

	// TODO: Right now we check for `count` separately to avoid a type error, since it returns an
	// i64, vs the others which return Vec<Edge<i64>>. There's probably a cleaner way to do this
	// in rust.
	if action == "count" {
		let result = try!(datastore_request(trans.get_edge_count(outbound_id, t)));
		try!(datastore_request(trans.commit()));
		Ok(to_response(status::Ok, &result))
	} else {
		let result = match action {
			"time" => {
				let limit = parse_limit(try!(get_query_param::<i32>(query_params, "limit".to_string(), false)));
				let high = parse_datetime(try!(get_query_param::<i64>(query_params, "high".to_string(), false)));
				let low = parse_datetime(try!(get_query_param::<i64>(query_params, "low".to_string(), false)));
				try!(datastore_request(trans.get_edge_time_range(outbound_id, t, high, low, limit)))
			},
			"position" => {
				let limit = parse_limit(try!(get_query_param::<i32>(query_params, "limit".to_string(), false)));
				let offset = try!(get_query_param::<i64>(query_params, "offset".to_string(), false)).unwrap_or(0);
				try!(datastore_request(trans.get_edge_range(outbound_id, t, offset, limit)))
			},
			_ => {
				return Err(create_iron_error(status::BadRequest, "Invalid `action`".to_string()))
			}
		};

		try!(datastore_request(trans.commit()));
		Ok(to_response(status::Ok, &result))
	}
}

fn on_input_script(req: &mut Request) -> IronResult<Response> {
	let mut payload = String::new();
	let read_result: Result<usize, io::Error> = req.body.read_to_string(&mut payload);

	if let Err(err) = read_result {
		return Err(create_iron_error(status::BadRequest, format!("Could not read script contents: {}", err)))
	}

	let account_id = get_account_id(req);
	let trans = try!(get_transaction(req));
	execute_script(trans, account_id, &payload[..], JsonValue::Null)
}

fn on_named_script(req: &mut Request) -> IronResult<Response> {
	let script_name_validator = regex::Regex::new(r"^[\w-_]+(\.lua)?$").unwrap();
	let script_name: String = try!(get_url_param(req, "name"));

	if !script_name_validator.is_match(&script_name[..]) {
		return Err(create_iron_error(status::BadRequest, "Invalid script name".to_string()));
	}

	let arg = try!(read_json(&mut req.body));

	let script_root = match env::var("NUTRINO_SCRIPT_ROOT") {
		Ok(s) => s,
		Err(_) => Path::new(".").join("scripts").to_str().unwrap().to_string()
	};

	let path = Path::new(&script_root[..]).join(script_name);

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
	execute_script(trans, account_id, &payload[..], arg)
}

fn execute_script(trans: PostgresTransaction, account_id: i64, payload: &str, arg: JsonValue) -> IronResult<Response> {
	match scripts::run(trans, account_id, &payload[..], arg) {
		Ok(val) => Ok(to_response(status::Ok, &val)),
		Err(err) => Err(create_iron_error(status::InternalServerError, format!("Script failed: {:?}", err)))
	}
}

fn on_transaction(req: &mut Request) -> IronResult<Response> {
	let trans = try!(get_transaction(req));
	let mut idx: u16 = 0;
	let mut jsonable_res: Vec<JsonValue> = Vec::new();

	match read_json(&mut req.body) {
		Ok(JsonValue::Array(items)) => {
			for item in items {
				match item {
					JsonValue::Object(obj) => {
						let action = try!(get_json_string_param(&obj, "action", false));

						let result: Result<JsonValue, IronError> = match &action.unwrap()[..] {
							"get_vertex" => get_vertex_item(&trans, &obj),
							"create_vertex" => create_vertex_item(&trans, &obj),
							"set_vertex" => set_vertex_item(&trans, &obj),
							"delete_vertex" => delete_vertex_item(&trans, &obj),
							"get_edge" => get_edge_item(&trans, &obj),
							"set_edge" => set_edge_item(&trans, &obj),
							"delete_edge" => delete_edge_item(&trans, &obj),
							"get_edge_count" => get_edge_count_item(&trans, &obj),
							"get_edge_range" => get_edge_range_item(&trans, &obj),
							"get_edge_time_range" => get_edge_time_range_item(&trans, &obj),
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

fn get_vertex_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_json_i64_param(item, "id", false));
	transaction_item(trans.get_vertex(id.unwrap()))
}

fn create_vertex_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let t = try!(get_json_string_param(item, "type", false));
	transaction_item(trans.create_vertex(t.unwrap()))
}

fn set_vertex_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_json_i64_param(item, "id", false));
	let t = try!(get_json_string_param(item, "type", false));
	transaction_item(trans.set_vertex(Vertex::new(id.unwrap(), t.unwrap())))
}

fn delete_vertex_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_json_i64_param(item, "id", false));
	transaction_item(trans.delete_vertex(id.unwrap()))
}

fn get_edge_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_json_i64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let inbound_id = try!(get_json_i64_param(item, "inbound_id", false));
	transaction_item(trans.get_edge(outbound_id.unwrap(), t.unwrap(), inbound_id.unwrap()))
}

fn set_edge_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_json_i64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let inbound_id = try!(get_json_i64_param(item, "inbound_id", false));
	let weight = try!(get_json_f64_param(item, "weight", false));
	transaction_item(trans.set_edge(Edge::new(outbound_id.unwrap(), t.unwrap(), inbound_id.unwrap(), weight.unwrap() as f32)))
}

fn delete_edge_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_json_i64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let inbound_id = try!(get_json_i64_param(item, "inbound_id", false));
	transaction_item(trans.delete_edge(outbound_id.unwrap(), t.unwrap(), inbound_id.unwrap()))
}

fn get_edge_count_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_json_i64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	transaction_item(trans.get_edge_count(outbound_id.unwrap(), t.unwrap()))
}

fn get_edge_range_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_json_i64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let limit = parse_limit(try!(get_json_i32_param(item, "limit", true)));
	let offset = try!(get_json_i64_param(item, "offset", true)).unwrap_or(0);
	transaction_item(trans.get_edge_range(outbound_id.unwrap(), t.unwrap(), offset, limit))
}

fn get_edge_time_range_item(trans: &PostgresTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_json_i64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let limit = parse_limit(try!(get_json_i32_param(item, "limit", true)));
	let high = parse_datetime(try!(get_json_i64_param(item, "high", true)));
	let low = parse_datetime(try!(get_json_i64_param(item, "low", true)));
	transaction_item(trans.get_edge_time_range(outbound_id.unwrap(), t.unwrap(), high, low, limit))
}

fn transaction_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
	let result = try!(datastore_request(result));
	Ok(serde_json::to_value(&result))
}
