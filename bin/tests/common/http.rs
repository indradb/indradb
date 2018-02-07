use hyper::header::{Authorization, Basic};
use hyper::client::FutureResponse;
use hyper::{Method, Request, Response, Uri, StatusCode, Error as HyperError};
use url::Url;
use uuid::Uuid;
use futures::future;
use serde_json;
use serde_json::value::Value as JsonValue;
use std::str::FromStr;
use std::str;
use std::collections::BTreeMap;
use serde::Deserialize;
use futures::future::Future;
use futures::Stream;
use tokio_core::reactor::Core;

pub fn request(
    port: usize,
    account_id: Uuid,
    secret: String,
    method_str: &str,
    path: &str,
    query_params: Vec<(&str, String)>,
    body: Option<JsonValue>,
) -> Request {
    let method = Method::from_str(method_str).unwrap();

    let mut url = Url::parse(&format!("http://localhost:{}{}", port, path)[..]).unwrap();

    if !query_params.is_empty() {
        let mut query_pairs_builder = url.query_pairs_mut();

        for (key, value) in query_params {
            query_pairs_builder.append_pair(key, &value[..]);
        }
    }

    let mut request = Request::new(method, Uri::from_str(&url.into_string()).unwrap());

    request.headers_mut().set(Authorization(Basic {
        username: account_id.hyphenated().to_string(),
        password: Some(secret),
    }));

    if let Some(body) = body {
        request.set_body(serde_json::to_string(&body).unwrap());
    }

    request
}

pub fn handle_response<T>(future: FutureResponse, mut event_loop: Core) -> Result<T, String>
where
    for<'a> T: Deserialize<'a>
{
    let res = event_loop.run(future).unwrap();

    if res.status() == StatusCode::Ok {
        let body: T = response_to_json(res);
        Ok(body)
    } else {
        let body: BTreeMap<String, JsonValue> = response_to_json(res);

        match body.get("error") {
            Some(&JsonValue::String(ref error)) => Err(error.clone()),
            _ => panic!("Could not unpack error message")
        }
    }
}

fn response_to_json<T>(res: Response) -> T
where
    for<'a> T: Deserialize<'a>
{
    let body_future = res.body().fold(Vec::new(), |mut v, chunk| {
        v.extend(&chunk[..]);
        future::ok::<_, HyperError>(v)
    }).and_then(|chunks| {
        let s = String::from_utf8(chunks).unwrap();
        let v: T = serde_json::from_str(&s).unwrap();
        future::ok::<_, HyperError>(v)
    });

    body_future.wait().expect("Expect a response body")
}
