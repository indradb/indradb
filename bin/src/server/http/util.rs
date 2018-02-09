use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType, Headers};
use iron::typemap::{Key, TypeMap};
use router::Router;
use indradb::{Datastore, Error, Type, Weight};
use util::SimpleError;
use common::ProxyTransaction;
use std::error::Error as StdError;
use core::str::FromStr;
use iron::modifiers::Header as HeaderModifier;
use iron::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use iron::request::Body;
use std::io;
use std::io::Read;
use serde_json::value::Value as JsonValue;
use serde_json;
use urlencoded::{UrlDecodingError, UrlEncodedQuery};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use statics;
use uuid::Uuid;

lazy_static! {
    static ref DEFAULT_QUERY_PARAMS: HashMap<String, Vec<String>> = HashMap::new();
}

// Need this to avoid orphan rules
pub struct AccountKey {
    pub account_id: Uuid,
}

impl Key for AccountKey {
    type Value = AccountKey;
}

/// Converts an indradb error to an `IronError`. We need to use this strategy
/// rather than a `From` impl because both traits are implemented outside of
/// this crate.
pub fn convert_to_iron_error(err: &Error) -> IronError {
    let status = match *err {
        Error::AccountNotFound
        | Error::VertexNotFound
        | Error::EdgeNotFound
        | Error::MetadataNotFound => status::NotFound,
        Error::OutOfRange(_) => status::BadRequest,
        Error::Unauthorized => status::Unauthorized,
        Error::Unexpected(_) => status::InternalServerError,
    };

    create_iron_error(status, format!("{}", err))
}

/// Constructs an `IronError`
pub fn create_iron_error(status_code: status::Status, err: String) -> IronError {
    let mut o: serde_json::Map<String, JsonValue> = serde_json::Map::new();
    o.insert("error".to_string(), JsonValue::String(err.clone()));
    let body = serde_json::to_string(&o).unwrap();
    let json_content_type_modifier = HeaderModifier(ContentType(get_json_mime()));
    let modifiers = (status_code, json_content_type_modifier, body);
    IronError::new(SimpleError::new(err), modifiers)
}

/// Returns a JSON content type specification
pub fn get_json_mime() -> Mime {
    Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![(Attr::Charset, Value::Utf8)],
    )
}

/// Serializes a given body and status code into a response
pub fn to_response<T: Serialize>(status_code: status::Status, body: &T) -> Response {
    let mut hs = Headers::new();
    hs.set(ContentType(get_json_mime()));

    Response {
        status: Some(status_code),
        headers: hs,
        extensions: TypeMap::new(),
        body: Some(Box::new(serde_json::to_string(&body).unwrap())),
    }
}

/// Converts a URL parameter to a given type
///
/// # Errors
/// Returns an error if the parameter could not be serialized to the given type
pub fn get_url_param<T: FromStr>(req: &Request, name: &str) -> Result<T, IronError> {
    let s = req.extensions.get::<Router>().unwrap().find(name).unwrap();

    T::from_str(s).map_err(|_| {
        create_iron_error(
            status::BadRequest,
            format!("Invalid value for URL param {}", name),
        )
    })
}

/// Gets a JSON string value
///
/// # Errors
/// Returns an `IronError` if the value is missing from the JSON object, or
/// has an unexpected type.
pub fn get_required_json_string_param(
    json: &serde_json::Map<String, JsonValue>,
    name: &str,
) -> Result<String, IronError> {
    match json.get(name) {
        Some(&JsonValue::String(ref val)) => Ok(val.clone()),
        None | Some(&JsonValue::Null) => Err(create_iron_error(
            status::BadRequest,
            format!("Missing `{}`", name),
        )),
        _ => Err(create_iron_error(
            status::BadRequest,
            format!("Invalid type for `{}`", name),
        )),
    }
}

/// Gets a JSON i64 value
///
/// # Errors
/// Returns an `IronError` if the value is missing from the JSON object, or
/// has an unexpected type.
pub fn get_required_json_f64_param(
    json: &serde_json::Map<String, JsonValue>,
    name: &str,
) -> Result<f64, IronError> {
    match json.get(name) {
        Some(&JsonValue::Number(ref val)) => if val.is_f64() {
            Ok(val.as_f64().unwrap())
        } else {
            Err(create_iron_error(
                status::BadRequest,
                format!("Invalid type for `{}`", name),
            ))
        },
        None | Some(&JsonValue::Null) => Err(create_iron_error(
            status::BadRequest,
            format!("Missing `{}`", name),
        )),
        _ => Err(create_iron_error(
            status::BadRequest,
            format!("Invalid type for `{}`", name),
        )),
    }
}

/// Gets a JSON object value that represents an edge key
///
/// # Errors
/// Returns an `IronError` if the value is missing from the JSON object, or
/// has an unexpected type.
pub fn get_required_json_obj_param<T>(
    json: &serde_json::Map<String, JsonValue>,
    name: &str,
) -> Result<T, IronError>
where
    for<'a> T: Deserialize<'a>,
{
    if let Some(obj) = json.get(name) {
        Ok(serde_json::from_value::<T>(obj.clone()).map_err(|_| {
            create_iron_error(
                status::BadRequest,
                format!("Invalid type for `{}`", name),
            )
        })?)
    } else {
        Err(create_iron_error(
            status::BadRequest,
            format!("Missing `{}`", name),
        ))
    }
}

/// Gets a JSON string value that represents a type
///
/// # Errors
/// Returns an `IronError` if the value is missing from the JSON object, or
/// has an unexpected type.
pub fn get_required_json_type_param(
    json: &serde_json::Map<String, JsonValue>,
    name: &str,
) -> Result<Type, IronError> {
    let s = get_required_json_string_param(json, name)?;

    Ok(Type::from_str(&s[..]).map_err(|_| {
        create_iron_error(
            status::BadRequest,
            format!("Invalid type format for `{}`", name),
        )
    })?)
}

// Gets a JSON float value that represents a weight
///
/// # Errors
/// Returns an `IronError` if the value is missing from the JSON object, or
/// has an unexpected type.
pub fn get_required_json_weight_param(
    json: &serde_json::Map<String, JsonValue>,
    name: &str,
) -> Result<Weight, IronError> {
    let w = get_required_json_f64_param(json, name)?;

    Ok(Weight::new(w as f32).map_err(|_| {
        create_iron_error(
            status::BadRequest,
            format!(
                "Invalid weight format for `{}`: it should be a float between -1.0 and 1.0 inclusive.",
                name
            ),
        )
    })?)
}

/// Parses a response from the datastore into a specified type
///
/// # Errors
/// Returns an `IronError` if the `Result` from the datastore is an error.
pub fn datastore_request<T>(result: Result<T, Error>) -> Result<T, IronError> {
    Ok(result.map_err(|err| convert_to_iron_error(&err))?)
}

/// Gets the account UUID from the iron request typemap
pub fn get_account_id(req: &Request) -> Uuid {
    let ext = &(*req.extensions.get::<AccountKey>().unwrap());
    ext.account_id
}

/// Gets a new transaction, tied to the request's account UUID
///
/// # Errors
/// Returns an `IronError` if it was not possible to create a transaction.
pub fn get_transaction(req: &Request) -> Result<ProxyTransaction, IronError> {
    let account_id = get_account_id(req);
    Ok(statics::DATASTORE.transaction(account_id).map_err(|err| {
        create_iron_error(
            status::InternalServerError,
            format!("Could not create datastore transaction: {}", err),
        )
    })?)
}

// Reads the request body into an optional `JsonValue`
///
/// # Errors
/// Returns an `IronError` if the body could not be read, or if a body was
/// specified but is not valid JSON.
pub fn read_optional_json(body: &mut Body) -> Result<Option<JsonValue>, IronError> {
    let mut payload = String::new();
    let read_result: Result<usize, io::Error> = body.read_to_string(&mut payload);

    if let Err(err) = read_result {
        return Err(create_iron_error(
            status::BadRequest,
            format!("Could not read JSON body: {}", err),
        ));
    }

    if payload.is_empty() {
        Ok(None)
    } else {
        Ok(Some(serde_json::from_str(&payload[..]).map_err(|err| {
            create_iron_error(
                status::BadRequest,
                format!("Could not parse JSON payload: {}", err.description()),
            )
        })?))
    }
}

/// Reads the request body into a `JsonValue`.
///
/// # Errors
/// Returns an `IronError` if the body could not be read, or is not valid JSON.
pub fn read_required_json(mut body: &mut Body) -> Result<JsonValue, IronError> {
    match read_optional_json(&mut body)? {
        Some(value) => Ok(value),
        None => Err(create_iron_error(
            status::BadRequest,
            "Missing JSON payload".to_string(),
        )),
    }
}

/// Parses the and returns the request query parameters.
///
/// # Errors
/// Returns an `IronError` if the query parameters could not be parsed.
pub fn get_query_params<'a>(
    req: &'a mut Request,
) -> Result<&'a HashMap<String, Vec<String>>, IronError> {
    match req.get_ref::<UrlEncodedQuery>() {
        Ok(map) => Ok(map),
        Err(UrlDecodingError::EmptyQuery) => Ok(&DEFAULT_QUERY_PARAMS),
        Err(_) => Err(create_iron_error(
            status::BadRequest,
            "Could not parse query parameters".to_string(),
        )),
    }
}

/// Gets a query parameter value and serializes it to the specified type.
///
/// # Errors
/// Returns an `IronError` if the body could not be read, or is not a valid JSON object.
pub fn get_query_param<T: FromStr>(
    params: &HashMap<String, Vec<String>>,
    key: &str,
    required: bool,
) -> Result<Option<T>, IronError> {
    if let Some(values) = params.get(key) {
        if let Some(first_value) = values.get(0) {
            match first_value.parse::<T>() {
                Ok(value) => return Ok(Some(value)),
                Err(_) => {
                    return Err(create_iron_error(
                        status::BadRequest,
                        format!("Could not parse query parameter `{}`", key),
                    ))
                }
            }
        }
    }

    if required {
        Err(create_iron_error(
            status::BadRequest,
            format!("Missing required query parameter `{}`", key),
        ))
    } else {
        Ok(None)
    }
}

/// Gets a required object from the query parameters.
///
/// # Errors
/// Returns an `IronError` if the query could be parsed, or was not specified.
pub fn get_obj_query_param<T>(query_params: &HashMap<String, Vec<String>>) -> Result<T, IronError>
where
    for<'a> T: Deserialize<'a>,
{
    let q_json = get_query_param::<JsonValue>(query_params, "q", true)?.unwrap();

    Ok(serde_json::from_value::<T>(q_json).map_err(|_| {
        create_iron_error(
            status::BadRequest,
            "Invalid type for `q`: expected edge query".to_string(),
        )
    })?)
}

/// Gets a required weight value from the query parameters.
///
/// # Errors
/// Returns an `IronError` if the weight could be parsed, or was not specified.
pub fn get_weight_query_param(
    query_params: &HashMap<String, Vec<String>>,
) -> Result<Weight, IronError> {
    let weight_f32 = get_query_param::<f32>(query_params, "weight", true)?.unwrap();

    Ok(Weight::new(weight_f32).map_err(|_| {
        create_iron_error(
            status::BadRequest,
            "Invalid type for `weight`: expected float between -1.0 and 1.0".to_string(),
        )
    })?)
}
