use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType, Headers};
use iron::typemap::TypeMap;
use router::Router;
use indradb::{Datastore, Error};
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
use serde::{Deserialize, Serialize};
use statics;

/// Converts an indradb error to an `IronError`. We need to use this strategy
/// rather than a `From` impl because both traits are implemented outside of
/// this crate.
pub fn convert_to_iron_error(err: &Error) -> IronError {
    let status = match *err {
        Error::VertexNotFound | Error::EdgeNotFound | Error::MetadataNotFound => status::NotFound,
        Error::OutOfRange(_) => status::BadRequest,
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

/// Gets a JSON object value.
///
/// # Errors
/// Returns an `IronError` if the value is missing from the JSON object, or
/// has an unexpected type.
pub fn get_json_obj_value<T>(
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

/// Parses a response from the datastore into a specified type
///
/// # Errors
/// Returns an `IronError` if the `Result` from the datastore is an error.
pub fn datastore_request<T>(result: Result<T, Error>) -> Result<T, IronError> {
    Ok(result.map_err(|err| convert_to_iron_error(&err))?)
}

/// Gets a new transaction.
///
/// # Errors
/// Returns an `IronError` if it was not possible to create a transaction.
pub fn get_transaction() -> Result<ProxyTransaction, IronError> {
    Ok(statics::DATASTORE.transaction().map_err(|err| {
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
