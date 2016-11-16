use iron::prelude::*;
use std::i64;
use iron::status;
use iron::headers::{Headers, ContentType, Authorization, Basic};
use iron::typemap::{Key, TypeMap};
use iron::middleware::BeforeMiddleware;
use router::Router;
use util::SimpleError;
use nutrino::{Vertex, Edge, Transaction, Datastore, Error};
use common::ProxyTransaction;
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
use std::u16;
use datastore::DATASTORE;
use uuid::Uuid;

lazy_static! {
	static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[\w-_]+(\.lua)?$").unwrap();
	static ref SCRIPT_ROOT: String = match env::var("NUTRINO_SCRIPT_ROOT") {
		Ok(s) => s,
		Err(_) => Path::new(".").join("scripts").to_str().unwrap().to_string()
	};
}

header! { (WWWAuthenticate, "WWW-Authenticate") => [String] }

const MAX_RETURNABLE_EDGES: u16 = 1000;

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

	router.post("/script/:name", on_script);

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

	fn get_account_id(&self, auth: Option<&Authorization<Basic>>) -> Option<Uuid> {
		if let Some(auth) = auth {
			if let Ok(val) = Uuid::from_str(&auth.username[..]) {
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
	account_id: Uuid
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

macro_rules! create_required_json_param_fn {
	($func: ident, $enum_val: path, $ty: ty) => {
		fn $func(json: &BTreeMap<String, JsonValue>, name: &str) -> Result<$ty, IronError> {
			match json.get(name) {
				Some(&$enum_val(ref val)) => Ok(val.clone()),
				None | Some(&JsonValue::Null) => {
					Err(create_iron_error(status::BadRequest, format!("Missing `{}`", name)))
				},
				_ => {
					Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
				}
			}
		}
	}
}

create_required_json_param_fn!(get_required_json_string_param, JsonValue::String, String);
create_required_json_param_fn!(get_required_json_f64_param, JsonValue::F64, f64);

fn json_u64_to_i64(name: &str, val: u64) -> Result<i64, IronError> {
	if val > i64::MAX as u64 {
		Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
	} else {
		Ok(val as i64)
	}
}

fn get_required_json_uuid_param(json: &BTreeMap<String, JsonValue>, name: &str) -> Result<Uuid, IronError> {
	let s = try!(get_required_json_string_param(json, name));

	match Uuid::from_str(&s[..]) {
		Ok(u) => Ok(u),
		Err(_) => Err(create_iron_error(status::BadRequest, format!("Invalid uuid format for `{}`", name)))
	}
}

fn get_optional_json_i64_param(json: &BTreeMap<String, JsonValue>, name: &str) -> Result<Option<i64>, IronError> {
	match json.get(name) {
		Some(&JsonValue::I64(ref val)) => Ok(Some(val.clone())),
		Some(&JsonValue::U64(ref val)) => Ok(Some(try!(json_u64_to_i64(name, val.clone())))),
		None | Some(&JsonValue::Null) => Ok(None),
		_ => {
			Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
		}
	}
}

fn get_optional_json_u64_param(json: &BTreeMap<String, JsonValue>, name: &str) -> Result<Option<u64>, IronError> {
	match json.get(name) {
		Some(&JsonValue::I64(ref val)) => {
			if *val < 0 {
				Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
			} else {
				Ok(Some(val.clone() as u64))
			}
		}
		Some(&JsonValue::U64(ref val)) => Ok(Some(val.clone())),
		None | Some(&JsonValue::Null) => Ok(None),
		_ => {
			Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name)))
		}
	}
}

fn get_optional_json_u16_param(json: &BTreeMap<String, JsonValue>, name: &str) -> Result<Option<u16>, IronError> {
	match try!(get_optional_json_u64_param(json, name)) {
		Some(val) if val > u16::MAX as u64 => Err(create_iron_error(status::BadRequest, format!("Invalid type for `{}`", name))),
		Some(val) => Ok(Some(val as u16)),
		None => Ok(None)
	}
}

fn parse_limit(val: Option<u16>) -> u16 {
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
				Error::AccountNotFound | Error::VertexNotFound | Error::EdgeNotFound | Error::MetadataNotFound => status::NotFound,
				Error::OutOfRange(_) => status::BadRequest,
				Error::Unexpected(_) => status::InternalServerError
			};

			Err(create_iron_error(status, format!("{}", err)))
		}
	}
}

fn get_account_id(req: &Request) -> Uuid {
	let ext = &(*req.extensions.get::<AccountKey>().unwrap());
	ext.account_id
}

fn get_transaction(req: &Request) -> Result<ProxyTransaction, IronError> {
	let account_id = get_account_id(req);
	match DATASTORE.transaction(account_id) {
		Ok(val) => Ok(val),
		Err(err) => Err(create_iron_error(status::InternalServerError, format!("Could not create datastore transaction: {}", err)))
	}
}

fn read_optional_json(body: &mut Body) -> Result<Option<JsonValue>, IronError> {
	let mut payload = String::new();
	let read_result: Result<usize, io::Error> = body.read_to_string(&mut payload);

	if let Err(err) = read_result {
		return Err(create_iron_error(status::BadRequest, format!("Could not read JSON body: {}", err)))
	}

	if payload.len() == 0 {
		Ok(None)
	} else {
		match serde_json::from_str(&payload[..]) {
			Ok(json) => Ok(Some(json)),
			Err(err) => Err(create_iron_error(status::BadRequest, format!("Could not parse JSON payload: {}", err.description())))
		}
	}	
}

fn read_required_json(mut body: &mut Body) -> Result<JsonValue, IronError> {
	match try!(read_optional_json(&mut body)) {
		Some(value) => Ok(value),
		None => Err(create_iron_error(status::BadRequest, "Missing JSON payload".to_string())),
	}
}

fn read_json_object(body: &mut Body) -> Result<BTreeMap<String, JsonValue>, IronError> {
	match try!(read_required_json(body)) {
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
	let id: Uuid = try!(get_url_param(req, "id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.get_vertex(id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_create_vertex(req: &mut Request) -> IronResult<Response> {
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_required_json_string_param(&obj, "type"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.create_vertex(t)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_set_vertex(req: &mut Request) -> IronResult<Response> {
	let id: Uuid = try!(get_url_param(req, "id"));
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_required_json_string_param(&obj, "type"));
	let v = Vertex::new(id, t);
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.set_vertex(v)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_delete_vertex(req: &mut Request) -> IronResult<Response> {
	let id: Uuid = try!(get_url_param(req, "id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.delete_vertex(id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_get_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));
	let inbound_id: Uuid = try!(get_url_param(req, "inbound_id"));
	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.get_edge(outbound_id, t, inbound_id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_set_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));
	let inbound_id: Uuid = try!(get_url_param(req, "inbound_id"));
	let obj = try!(read_json_object(&mut req.body));
	let weight = try!(get_required_json_f64_param(&obj, "weight"));
	let e = Edge::new(outbound_id, t, inbound_id, weight as f32);

	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.set_edge(e)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_delete_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));
	let inbound_id: Uuid = try!(get_url_param(req, "inbound_id"));

	let trans = try!(get_transaction(req));
	let result = try!(datastore_request(trans.delete_edge(outbound_id, t, inbound_id)));
	try!(datastore_request(trans.commit()));
	Ok(to_response(status::Ok, &result))
}

fn on_get_edge_range(req: &mut Request) -> IronResult<Response> {
	let outbound_id: Uuid = try!(get_url_param(req, "outbound_id"));
	let t: String = try!(get_url_param(req, "type"));

	let trans = try!(get_transaction(req));

	let query_params = try!(get_query_params(req));
	let action = &try!(get_query_param::<String>(query_params, "action".to_string(), true)).unwrap()[..];

	// TODO: Right now we check for `count` separately to avoid a type error, since it returns an
	// i64, vs the others which return Vec<Edge<Uuid>>. There's probably a cleaner way to do this
	// in rust.
	if action == "count" {
		let result = try!(datastore_request(trans.get_edge_count(outbound_id, t)));
		try!(datastore_request(trans.commit()));
		Ok(to_response(status::Ok, &result))
	} else {
		let result = match action {
			"time" => {
				let limit = parse_limit(try!(get_query_param::<u16>(query_params, "limit".to_string(), false)));
				let high = parse_datetime(try!(get_query_param::<i64>(query_params, "high".to_string(), false)));
				let low = parse_datetime(try!(get_query_param::<i64>(query_params, "low".to_string(), false)));
				try!(datastore_request(trans.get_edge_time_range(outbound_id, t, high, low, limit)))
			},
			"position" => {
				let limit = parse_limit(try!(get_query_param::<u16>(query_params, "limit".to_string(), false)));
				let offset = try!(get_query_param::<u64>(query_params, "offset".to_string(), false)).unwrap_or(0);
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

fn on_script(req: &mut Request) -> IronResult<Response> {
	let script_name: String = try!(get_url_param(req, "name"));

	if !SCRIPT_NAME_VALIDATOR.is_match(&script_name[..]) {
		return Err(create_iron_error(status::BadRequest, "Invalid script name".to_string()));
	}

	let arg = match try!(read_optional_json(&mut req.body)) {
		Some(val) => val,
		None => JsonValue::Null
	};

	let path = Path::new(&SCRIPT_ROOT[..]).join(script_name);

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

	match scripts::run(trans, account_id, &payload[..], arg) {
		Ok(val) => Ok(to_response(status::Ok, &val)),
		Err(err) => Err(create_iron_error(status::InternalServerError, format!("Script failed: {:?}", err)))
	}
}

fn on_transaction(req: &mut Request) -> IronResult<Response> {
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
	let t = try!(get_required_json_string_param(item, "type"));
	execute_trans_item(trans.create_vertex(t))
}

fn trans_set_vertex(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_required_json_uuid_param(item, "id"));
	let t = try!(get_required_json_string_param(item, "type"));
	execute_trans_item(trans.set_vertex(Vertex::new(id, t)))
}

fn trans_delete_vertex(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let id = try!(get_required_json_uuid_param(item, "id"));
	execute_trans_item(trans.delete_vertex(id))
}

fn trans_get_edge(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_string_param(item, "type"));
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	execute_trans_item(trans.get_edge(outbound_id, t, inbound_id))
}

fn trans_set_edge(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_string_param(item, "type"));
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	let weight = try!(get_required_json_f64_param(item, "weight"));
	execute_trans_item(trans.set_edge(Edge::new(outbound_id, t, inbound_id, weight as f32)))
}

fn trans_delete_edge(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_string_param(item, "type"));
	let inbound_id = try!(get_required_json_uuid_param(item, "inbound_id"));
	execute_trans_item(trans.delete_edge(outbound_id, t, inbound_id))
}

fn trans_get_edge_count(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_string_param(item, "type"));
	execute_trans_item(trans.get_edge_count(outbound_id, t))
}

fn trans_get_edge_range(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_string_param(item, "type"));
	let limit = parse_limit(try!(get_optional_json_u16_param(item, "limit")));
	let offset = try!(get_optional_json_u64_param(item, "offset")).unwrap_or(0);
	execute_trans_item(trans.get_edge_range(outbound_id, t, offset, limit))
}

fn trans_get_edge_time_range(trans: &ProxyTransaction, item: &BTreeMap<String, JsonValue>) -> Result<JsonValue, IronError> {
	let outbound_id = try!(get_required_json_uuid_param(item, "outbound_id"));
	let t = try!(get_required_json_string_param(item, "type"));
	let limit = parse_limit(try!(get_optional_json_u16_param(item, "limit")));
	let high = parse_datetime(try!(get_optional_json_i64_param(item, "high")));
	let low = parse_datetime(try!(get_optional_json_i64_param(item, "low")));
	execute_trans_item(trans.get_edge_time_range(outbound_id, t, high, low, limit))
}

fn execute_trans_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
	let result = try!(datastore_request(result));
	Ok(serde_json::to_value(&result))
}
