use iron::prelude::*;
use std::i32;
use iron::status;
use iron::headers::{Headers, ContentType, Authorization, Basic};
use iron::typemap::{Key, TypeMap};
use iron::middleware::BeforeMiddleware;
use router::Router;
use nutrino::{Vertex, Edge, Transaction, Datastore, SimpleError, PostgresTransaction, PostgresDatastore};
use nutrino::Request as DatastoreRequest;
use nutrino::ErrorResponse as DatastoreErrorResponse;
use std::collections::BTreeMap;
use std::error::Error;
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

header! { (WWWAuthenticate, "WWW-Authenticate") => [String] }

const MAX_RETURNABLE_EDGES: i32 = 1000;

// -- Public function for starting the server
pub fn start(port: u16, datastore: PostgresDatastore) {
	let mut router = Router::new();

	router.post("/", on_create_vertex);
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
	chain.link_before(BasicAuthMiddleware::new(datastore));
	Iron::new(chain).http(&*binding).unwrap();
}

// -- Basic http auth middleware
struct BasicAuthMiddleware {
	datastore: PostgresDatastore
}

impl BasicAuthMiddleware {
	fn new(datastore: PostgresDatastore) -> BasicAuthMiddleware {
		BasicAuthMiddleware {
			datastore: datastore
		}
	}

	fn get_user_id(&self, auth: Option<&Authorization<Basic>>) -> Option<i64> {
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
		let user_id = self.get_user_id(auth);
		let secret = self.get_secret(auth);
		let mut error_message = "Authentication failed".to_string();

		if user_id.is_some() && secret.is_some() && self.datastore.auth(user_id.unwrap(), secret.unwrap()).unwrap_or(false) {
			match self.datastore.transaction(user_id.unwrap()) {
				Ok(transaction) => {
					req.extensions.insert::<AccountKey>(AccountKey {
						user_id: user_id.unwrap()
					});

					req.extensions.insert::<PostgresTransactionKey>(PostgresTransactionKey {
						transaction: transaction
					});

					return Ok(());
				},
				Err(err) => {
					error_message = err.description().to_string();
				}
			}
		}

		let mut d: BTreeMap<String, String> = BTreeMap::new();
		d.insert("error".to_string(), error_message.clone());
		let body = serde_json::to_string(&d).unwrap();

		let www_authenticate_header = WWWAuthenticate("Basic realm=\"main\"".to_owned());
		let www_authenticate_modifier = HeaderModifier(www_authenticate_header);
		let json_content_type_modifier = HeaderModifier(get_json_content_type());

		let modifiers = (status::Unauthorized, json_content_type_modifier, www_authenticate_modifier, body);
		Err(IronError::new(SimpleError::new_from_string(error_message), modifiers))
	}
}

// Need this to avoid orphan rules
struct AccountKey {
	user_id: i64
}

impl Key for AccountKey {
	type Value = AccountKey;
}

struct PostgresTransactionKey {
	transaction: PostgresTransaction
}

impl Key for PostgresTransactionKey {
	type Value = PostgresTransactionKey;
}

// -- Convenience functions
fn create_iron_error<E: Error + Send + 'static>(status_code: status::Status, err: E) -> IronError {
	let mut d: BTreeMap<String, String> = BTreeMap::new();
	d.insert("error".to_string(), err.description().to_string());
	let body = serde_json::to_string(&d).unwrap();
	let json_content_type_modifier = HeaderModifier(get_json_content_type());
	let modifiers = (status_code, json_content_type_modifier, body);
	IronError::new(err, modifiers)
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

fn single_transaction_to_result(trans: &mut PostgresTransaction) -> IronResult<Response> {
	match trans.commit() {
		Ok(res) => {
			let item = res.get(0).unwrap();

			let status = match *item {
				Err(DatastoreErrorResponse::Unexpected(_)) => status::InternalServerError,
				Err(DatastoreErrorResponse::VertexDoesNotExist(_)) | Err(DatastoreErrorResponse::EdgeDoesNotExist(_, _, _)) => status::NotFound,
				Err(DatastoreErrorResponse::WeightOutOfRange) => status::BadRequest,
				_ => status::Ok
			};

			match *item {
				Ok(ref res_item) => Ok(to_response(status, res_item)),
				Err(ref err_res) => Ok(to_response(status, err_res))
			}
		},
		Err(err) => Err(create_iron_error(status::InternalServerError, err))
	}
}

fn get_url_param<T: FromStr>(req: &Request, name: &str) -> Result<T, IronError> {
	let s = req.extensions.get::<Router>().unwrap().find(name).unwrap();

	match T::from_str(s) {
		Ok(val) => Ok(val),
		Err(_) => Err(create_iron_error(status::BadRequest, SimpleError::new_from_string(format!("Invalid value for URL param {}", name))))
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
						Err(create_iron_error(status::BadRequest, SimpleError::new_from_string(format!("Missing `{}`", name))))
					}
				}
				_ => {
					Err(create_iron_error(status::BadRequest, SimpleError::new_from_string(format!("Invalid type for `{}`", name))))
				}
			}
		}
	}
}

create_json_param_fn!(get_json_string_param, JsonValue::String, String);
create_json_param_fn!(get_json_u64_param, JsonValue::U64, u64);
create_json_param_fn!(get_json_f64_param, JsonValue::F64, f64);
create_json_param_fn!(get_json_object_param, JsonValue::Object, BTreeMap<String, JsonValue>);

fn get_transaction(req: &mut Request) -> PostgresTransaction {
	let transaction = &(*req.extensions.get::<PostgresTransactionKey>().unwrap());
	transaction.transaction.clone()
}

fn read_json(body: &mut Body) -> Result<JsonValue, IronError> {
	let mut payload = String::new();
	let read_result: Result<usize, io::Error> = body.read_to_string(&mut payload);

	if read_result.is_err() {
	    return Err(create_iron_error(status::BadRequest, read_result.unwrap_err()))
	}

	match serde_json::from_str(&payload[..]) {
	    Ok(json) => Ok(json),
	    Err(err) => Err(create_iron_error(status::BadRequest, SimpleError::new_from_string(format!("Could not parse JSON payload: {}", err.description()))))
	}
}

fn read_json_object(body: &mut Body) -> Result<BTreeMap<String, JsonValue>, IronError> {
	match try!(read_json(body)) {
		JsonValue::Object(obj) => Ok(obj),
		_ => Err(create_iron_error(status::BadRequest, SimpleError::new("Bad payload: expected object")))
	}
}

fn get_query_params<'a>(req: &'a mut Request) -> Result<&'a HashMap<String, Vec<String>>, IronError> {
	match req.get_ref::<UrlEncodedQuery>() {
        Ok(map) => Ok(map),
        Err(_) => Err(create_iron_error(status::BadRequest, SimpleError::new("Could not parse query parameters")))
    }
}

fn get_query_param<T: FromStr>(params: &HashMap<String, Vec<String>>, key: String, required: bool) -> Result<Option<T>, IronError> {
	if let Some(values) = params.get(&key) {
		if let Some(first_value) = values.get(0) {
			match first_value.parse::<T>() {
				Ok(value) => return Ok(Some(value)),
				Err(_) => return Err(create_iron_error(status::BadRequest, SimpleError::new_from_string(format!("Could not parse query parameter `{}`", key))))
			}
		}
	}

	if required {
		Err(create_iron_error(status::BadRequest, SimpleError::new_from_string(format!("Missing required query parameter `{}`", key))))
	} else {
		Ok(None)
	}
}

// -- Handlers
fn on_get_vertex(req: &mut Request) -> IronResult<Response> {
	let id: i64 = try!(get_url_param(&req, "id"));
	let mut trans = get_transaction(req);
	trans.request(DatastoreRequest::GetVertex(id));
	single_transaction_to_result(&mut trans)
}

fn on_create_vertex(req: &mut Request) -> IronResult<Response> {
	let mut trans = get_transaction(req);
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_json_string_param(&obj, "type", false));
	let properties = try!(get_json_object_param(&obj, "properties", true));

	if properties.is_some() {
		trans.request(DatastoreRequest::CreateVertex(t.unwrap(), properties.unwrap()));
	} else {
		trans.request(DatastoreRequest::CreateVertex(t.unwrap(), BTreeMap::new()));
	}

	single_transaction_to_result(&mut trans)
}

fn on_set_vertex(req: &mut Request) -> IronResult<Response> {
	let id: i64 = try!(get_url_param(&req, "id"));
	let mut trans = get_transaction(req);
	let obj = try!(read_json_object(&mut req.body));
	let t = try!(get_json_string_param(&obj, "type", false));
	let properties = try!(get_json_object_param(&obj, "properties", true));

	let vertex = if properties.is_some() {
		Vertex::new_with_properties(id, t.unwrap(), properties.unwrap())
	} else {
		Vertex::new(id, t.unwrap())
	};

	trans.request(DatastoreRequest::SetVertex(vertex));
	single_transaction_to_result(&mut trans)
}

fn on_delete_vertex(req: &mut Request) -> IronResult<Response> {
	let id: i64 = try!(get_url_param(&req, "id"));
	let mut trans = get_transaction(req);
	trans.request(DatastoreRequest::DeleteVertex(id));
	single_transaction_to_result(&mut trans)
}

fn on_get_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(&req, "outbound_id"));
	let t: String = try!(get_url_param(&req, "type"));
	let inbound_id: i64 = try!(get_url_param(&req, "inbound_id"));
	let mut trans = get_transaction(req);
	trans.request(DatastoreRequest::GetEdge(outbound_id, t, inbound_id));
	single_transaction_to_result(&mut trans)
}

fn on_set_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(&req, "outbound_id"));
	let t: String = try!(get_url_param(&req, "type"));
	let inbound_id: i64 = try!(get_url_param(&req, "inbound_id"));
	let mut trans = get_transaction(req);
	let obj = try!(read_json_object(&mut req.body));
	let weight = try!(get_json_f64_param(&obj, "weight", false));
	let properties = try!(get_json_object_param(&obj, "properties", true));

	let edge = if properties.is_some() {
		Edge::new_with_properties(outbound_id, t, inbound_id, weight.unwrap() as f32, properties.unwrap())
	} else {
		Edge::new(outbound_id, t, inbound_id, weight.unwrap() as f32)
	};

	trans.request(DatastoreRequest::SetEdge(edge));
	single_transaction_to_result(&mut trans)
}

fn on_delete_edge(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(&req, "outbound_id"));
	let t: String = try!(get_url_param(&req, "type"));
	let inbound_id: i64 = try!(get_url_param(&req, "inbound_id"));
	let mut trans = get_transaction(req);
	trans.request(DatastoreRequest::DeleteEdge(outbound_id, t, inbound_id));
	single_transaction_to_result(&mut trans)
}

fn on_get_edge_range(req: &mut Request) -> IronResult<Response> {
	let outbound_id: i64 = try!(get_url_param(&req, "outbound_id"));
	let t: String = try!(get_url_param(&req, "type"));
	let mut trans = get_transaction(req);

	let query_params = try!(get_query_params(req));
	let action = &try!(get_query_param::<String>(query_params, "action".to_string(), true)).unwrap()[..];

	match action {
		"time" => {
			let mut limit = try!(get_query_param::<i32>(query_params, "limit".to_string(), false)).unwrap_or(0);

			if limit <= 0 || limit > MAX_RETURNABLE_EDGES {
				limit = MAX_RETURNABLE_EDGES;
			}

			let high = match try!(get_query_param::<i64>(query_params, "high".to_string(), false)) {
				Some(val) => Some(NaiveDateTime::from_timestamp(val, 0)),
				_ => None
			};

			let low = match try!(get_query_param::<i64>(query_params, "low".to_string(), false)) {
				Some(val) => Some(NaiveDateTime::from_timestamp(val, 0)),
				_ => None
			};

			trans.request(DatastoreRequest::GetEdgeTimeRange(outbound_id, t, high, low, limit));
		},
		"position" => {
			let mut limit = try!(get_query_param::<i32>(query_params, "limit".to_string(), false)).unwrap_or(0);

			if limit <= 0 || limit > MAX_RETURNABLE_EDGES {
				limit = MAX_RETURNABLE_EDGES;
			}

			let mut offset = try!(get_query_param::<i64>(query_params, "offset".to_string(), false)).unwrap_or(0);

			if offset < 0 {
				offset = 0;
			}

			trans.request(DatastoreRequest::GetEdgeRange(outbound_id, t, offset, limit));
		},
		_ => {
			return Err(create_iron_error(status::BadRequest, SimpleError::new("Invalid `action`")))
		}
	}

	single_transaction_to_result(&mut trans)
}

fn on_input_script(req: &mut Request) -> IronResult<Response> {
	let mut payload = String::new();
	let read_result: Result<usize, io::Error> = req.body.read_to_string(&mut payload);

	if read_result.is_err() {
	    return Err(create_iron_error(status::BadRequest, read_result.unwrap_err()))
	}

	let trans = get_transaction(req);

	let user_id = {
		let ext = &(*req.extensions.get::<AccountKey>().unwrap());
		ext.user_id
	};

	execute_script(trans, user_id, &payload[..], JsonValue::Null)
}

fn on_named_script(req: &mut Request) -> IronResult<Response> {
	let script_name_validator = regex::Regex::new(r"^[\w-_]+(\.lua)?$").unwrap();
	let script_name: String = try!(get_url_param(&req, "name"));

	if !script_name_validator.is_match(&script_name[..]) {
		return Err(create_iron_error(status::BadRequest, SimpleError::new("Invalid script name")));
	}

	let arg = try!(read_json(&mut req.body));

	let script_root = match env::var("NUTRINO_SCRIPT_ROOT") {
		Ok(s) => s,
		Err(_) => Path::new(".").join("scripts").to_str().unwrap().to_string()
	};

	let path = Path::new(&script_root[..]).join(script_name);

	let mut f = match File::open(path) {
		Ok(f) => f,
		Err(_) => return Err(create_iron_error(status::NotFound, SimpleError::new("Could not load script")))
	};

	let mut payload = String::new();

	if f.read_to_string(&mut payload).is_err() {
		return Err(create_iron_error(status::InternalServerError, SimpleError::new("Could not read script contents")));
	}

	let user_id = req.extensions.get::<AccountKey>().unwrap().user_id;
	let trans = get_transaction(req);
	execute_script(trans, user_id, &payload[..], arg)
}

fn execute_script(trans: PostgresTransaction, user_id: i64, payload: &str, arg: JsonValue) -> IronResult<Response> {
	match scripts::run(trans, user_id, &payload[..], arg) {
		Ok(val) => Ok(to_response(status::Ok, &val)),
		Err(err) => Err(create_iron_error(status::InternalServerError, SimpleError::new_from_string(format!("Script failed: {:?}", err))))
	}
}

fn on_transaction(req: &mut Request) -> IronResult<Response> {
	let mut trans = get_transaction(req);
	let mut idx: u16 = 0;

	match read_json(&mut req.body) {
		Ok(JsonValue::Array(items)) => {
			for item in items {
				match item {
					JsonValue::Object(obj) => {
						let action = try!(get_json_string_param(&obj, "action", false));

						let req = match &action.unwrap()[..] {
							"get_vertex" => {
								get_vertex_item(&obj)
							},
							"set_vertex" => {
								set_vertex_item(&obj)
							},
							"delete_vertex" => {
								delete_vertex(&obj)
							},
							"get_edge" => {
								get_edge_item(&obj)
							},
							"set_edge" => {
								set_edge_item(&obj)
							},
							"delete_edge" => {
								delete_edge_item(&obj)
							},
							"get_edge_count" => {
								get_edge_count_item(&obj)
							}
							"get_edge_range" => {
								get_edge_range_item(&obj)
							},
							"get_edge_time_range" => {
								get_edge_time_range_item(&obj)
							}
							_ => {
								Err(create_iron_error(status::BadRequest, SimpleError::new("Unknown action")))
							}
						};

						if req.is_err() {
							let err = SimpleError::new_from_string(format!("Item #{}: {}", idx, req.unwrap_err().description()));
							return Err(create_iron_error(status::BadRequest, err))
						} else {
							trans.request(req.unwrap());
						}

						idx += 1;
					},
					_ => {
						let err = SimpleError::new_from_string(format!("Item #{}: Invalid type", idx));
						return Err(create_iron_error(status::BadRequest, err))
					}
				}
			}
		},
		_ => return Err(create_iron_error(status::BadRequest, SimpleError::new("Request body should be an array")))
	}

	match trans.commit() {
		Ok(res) => {
			let jsonable_res: Vec<JsonValue> = res.iter().map(|item| {
				match *item {
					Ok(ref res_item) => serde_json::to_value(res_item),
					Err(ref err_item) => serde_json::to_value(err_item)
				}
			}).collect();

			Ok(to_response(status::Ok, &jsonable_res))
		},
		Err(err) => Err(create_iron_error(status::InternalServerError, err))
	}
}

fn get_vertex_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let id = try!(get_json_u64_param(item, "id", false));
	Ok(DatastoreRequest::GetVertex(id.unwrap() as i64))
}

fn set_vertex_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let id = try!(get_json_u64_param(item, "id", true));
	let t = try!(get_json_string_param(item, "type", false));
	let properties = try!(get_json_object_param(item, "properties", true));

	Ok(match (id, properties) {
		(Some(id), Some(properties)) => DatastoreRequest::SetVertex(Vertex::new_with_properties(id as i64, t.unwrap(), properties)),
		(Some(id), None) => DatastoreRequest::SetVertex(Vertex::new(id as i64, t.unwrap())),
		(None, Some(properties)) => DatastoreRequest::CreateVertex(t.unwrap(), properties),
		(None, None) => DatastoreRequest::CreateVertex(t.unwrap(), BTreeMap::new())
	})
}

fn delete_vertex(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let id = try!(get_json_u64_param(item, "id", false));
	Ok(DatastoreRequest::DeleteVertex(id.unwrap() as i64))
}

fn get_edge_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let outbound_id = try!(get_json_u64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let inbound_id = try!(get_json_u64_param(item, "inbound_id", false));
	Ok(DatastoreRequest::GetEdge(outbound_id.unwrap() as i64, t.unwrap(), inbound_id.unwrap() as i64))
}

fn set_edge_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let outbound_id = try!(get_json_u64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let inbound_id = try!(get_json_u64_param(item, "inbound_id", false));
	let weight = try!(get_json_f64_param(item, "weight", false));
	let properties = try!(get_json_object_param(item, "properties", true));

	let edge = if properties.is_some() {
		Edge::new_with_properties(outbound_id.unwrap() as i64, t.unwrap(), inbound_id.unwrap() as i64, weight.unwrap() as f32, properties.unwrap())
	} else {
		Edge::new(outbound_id.unwrap() as i64, t.unwrap(), inbound_id.unwrap() as i64, weight.unwrap() as f32)
	};

	Ok(DatastoreRequest::SetEdge(edge))
}

fn delete_edge_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let outbound_id = try!(get_json_u64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let inbound_id = try!(get_json_u64_param(item, "inbound_id", false));
	Ok(DatastoreRequest::DeleteEdge(outbound_id.unwrap() as i64, t.unwrap(), inbound_id.unwrap() as i64))
}

fn get_edge_count_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let outbound_id = try!(get_json_u64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	Ok(DatastoreRequest::GetEdgeCount(outbound_id.unwrap() as i64, t.unwrap()))
}

fn get_json_limit_param(item: &BTreeMap<String, JsonValue>) -> Result<i32, IronError> {
	let limit = match try!(get_json_u64_param(item, "limit", true)) {
		Some(val) => {
			if val <= i32::MAX as u64 {
				val as i32
			} else {
				MAX_RETURNABLE_EDGES
			}
		},
	 	_ => MAX_RETURNABLE_EDGES
	};

	if limit <= 0 || limit > MAX_RETURNABLE_EDGES {
		Ok(MAX_RETURNABLE_EDGES)
	} else {
		Ok(limit)
	}
}

fn get_json_timestamp_param(item: &BTreeMap<String, JsonValue>, name: &str) -> Result<Option<NaiveDateTime>, IronError> {
	match try!(get_json_u64_param(item, name, true)) {
		Some(val) => Ok(Some(NaiveDateTime::from_timestamp(val as i64, 0))),
		None => Ok(None)
	}
}

fn get_edge_range_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let outbound_id = try!(get_json_u64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let limit = try!(get_json_limit_param(item));
	let offset = match try!(get_json_u64_param(item, "offset", true)) {
		Some(val) => val as i64,
		None => 0
	};

	Ok(DatastoreRequest::GetEdgeRange(outbound_id.unwrap() as i64, t.unwrap(), offset, limit))
}

fn get_edge_time_range_item(item: &BTreeMap<String, JsonValue>) -> Result<DatastoreRequest<i64>, IronError> {
	let outbound_id = try!(get_json_u64_param(item, "outbound_id", false));
	let t = try!(get_json_string_param(item, "type", false));
	let limit = try!(get_json_limit_param(item));
	let high = try!(get_json_timestamp_param(item, "high"));
	let low = try!(get_json_timestamp_param(item, "low"));
	Ok(DatastoreRequest::GetEdgeTimeRange(outbound_id.unwrap() as i64, t.unwrap(), high, low, limit))
}
