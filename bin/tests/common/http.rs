use hyper::client::{Client, RequestBuilder};
use hyper::Url;
use hyper::method::Method;
use hyper::client::response::Response;

use serde_json;
use serde_json::value::Value as JsonValue;

use std::str::FromStr;
use std::io::Read;
use std::str;
use std::collections::BTreeMap;

pub fn request<'a>(
    client: &'a Client,
    port: usize,
    method_str: &str,
    path: &str,
    query_params: Vec<(&str, String)>,
) -> RequestBuilder<'a> {
    let method = Method::from_str(method_str).unwrap();

    let mut url = Url::parse(&format!("http://localhost:{}{}", port, path)[..]).unwrap();

    if !query_params.is_empty() {
        let mut query_pairs_builder = url.query_pairs_mut();

        for (key, value) in query_params {
            query_pairs_builder.append_pair(key, &value[..]);
        }
    }

    client.request(method, url)
}

pub fn response_to_error_message(res: &mut Response) -> String {
    let mut payload = String::new();
    res.read_to_string(&mut payload).unwrap();
    let o: BTreeMap<String, JsonValue> = serde_json::from_str(&payload[..]).unwrap();

    match o.get("error") {
        Some(&JsonValue::String(ref error)) => error.clone(),
        _ => panic!("Could not unpack error message"),
    }
}
