use uuid::Uuid;
use serde_json::value::Value as JsonValue;
use std::str;
use serde::Deserialize;
use reqwest::{Response, Client, Method, Url, Error as ReqwestError, StatusCode};

pub fn request(
    port: usize,
    account_id: Uuid,
    secret: String,
    method: Method,
    path: &str,
    query_params: &Vec<(&str, &str)>,
    body: Option<JsonValue>) -> Result<Response, ReqwestError> {
    let url = Url::parse_with_params(&format!("http://localhost:{}{}", port, path), query_params).expect("Expected to be able to construct a URL");
    let client = Client::new();
    let mut request = client.request(method, url);

    request.basic_auth(account_id.hyphenated().to_string(), Some(secret));

    if let Some(body) = body {
        request.json(&body);
    }

    request.send()
}

pub fn from_result<T>(result: Result<Response, ReqwestError>) -> Result<T, String> where for<'a> T: Deserialize<'a>  {
    match result {
        Ok(mut response) => {
            if response.status() == StatusCode::Ok {
                let v: T = response.json().expect("Could not deserialize response to custom type");
                Ok(v)
            } else {
                let v: JsonValue = response.json().expect("Could not deserialize response to object");

                if let JsonValue::Object(ref obj) = v {
                    if let Some(&JsonValue::String(ref err)) = obj.get("error") {
                        return Err(err.clone());
                    }
                }

                panic!("Unexpected error response object: {}", v);
            }
        },
        Err(err) => {
            panic!("Request error: {}", err);
        }
    }
}
