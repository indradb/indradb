use hyper::header::{Authorization, Basic};
use hyper::client::{Client, RequestBuilder};
use hyper::method::Method;
use hyper::client::response::Response;

use serde_json;
use serde_json::value::Value as JsonValue;

use std::str::FromStr;
use std::io::Read;
use std::str;
use std::collections::BTreeMap;

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
