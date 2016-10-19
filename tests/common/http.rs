use hyper::header::{Authorization, Basic};
use hyper::client::{Client, RequestBuilder};
use hyper::method::Method;
use hyper::client::response::Response;
use hyper::status::StatusCode;

use serde_json;
use serde_json::value::Value as JsonValue;
use serde::Deserialize;

use std::str::FromStr;
use std::io::Read;
use std::str;
use std::collections::BTreeMap;

use nutrino::*;

pub fn request<'a>(client: &'a Client, port: i32, account_id: i64, secret: String, method_str: &str, path: String) -> RequestBuilder<'a> {
    let method = Method::from_str(method_str).unwrap();
    let url = format!("http://localhost:{}{}", port, path);

    let auth = Authorization(
        Basic {
            username: account_id.to_string(),
            password: Some(secret)
        }
    );

    client.request(method, &url[..]).header(auth)
}

pub fn response_to_error_message(res: &mut Response) -> String {
	let mut payload = String::new();
	res.read_to_string(&mut payload).unwrap();
	let o: BTreeMap<String, JsonValue> = serde_json::from_str(&payload[..]).unwrap();

	match o.get("error") {
		Some(&JsonValue::String(ref error)) => error.clone(),
		_ => panic!("Could not unpack error message")
	}
}

pub fn response_to_obj<T: Deserialize>(res: &mut Response) -> Result<T, Error> {
	match res.status {
		StatusCode::Ok => {
			let mut payload = String::new();
			res.read_to_string(&mut payload).unwrap();
			let v: T = serde_json::from_str(&payload[..]).unwrap();
			Ok(v)
		},
		StatusCode::NotFound => {
			let msg = response_to_error_message(res);

			match &msg[..] {
				"Account not found" => Err(Error::AccountNotFound),
				"Vertex does not exist" => Err(Error::VertexDoesNotExist),
				"Edge does not exist" => Err(Error::EdgeDoesNotExist),
				"Metadata does not exist" => Err(Error::MetadataDoesNotExist),
				_ => Err(Error::Unexpected(format!("Unexpected error message: {}", msg)))
			}
		},
		StatusCode::BadRequest => {
			let msg = response_to_error_message(res);

			match &msg[..] {
				"Weight out of range" => Err(Error::WeightOutOfRange),
				"Limit out of range" => Err(Error::LimitOutOfRange),
				"Offset out of range" => Err(Error::OffsetOutOfRange),
				_ => Err(Error::Unexpected(format!("Unexpected error message: {}", msg)))
			}
		},
		_ => Err(Error::Unexpected(format!("Unexpected return status code: {}", res.status)))
	}
}
