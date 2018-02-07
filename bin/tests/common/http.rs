use hyper::header::{Authorization, Basic};
use hyper::client::Client as HyperClient;
use hyper::{Method, Request, Uri, StatusCode, Error as HyperError};
use url::Url;
use uuid::Uuid;
use futures::future;
use serde_json;
use serde_json::value::Value as JsonValue;
use std::str::FromStr;
use std::str;
use serde::Deserialize;
use futures::future::Future;
use futures::Stream;
use tokio_core::reactor::Core;
use std::thread::spawn;
use crossbeam_channel::{bounded, Sender, Receiver};
use std::sync::Mutex;
use std::thread;

pub struct Client {
    lock: Mutex<()>,
    in_sender: Sender<Request>,
    out_receiver: Receiver<Result<(StatusCode, JsonValue), HyperError>>
}

impl Client {
    fn default() -> Self {
        let (in_sender, in_receiver) = bounded::<Request>(1);
        let (out_sender, out_receiver) = bounded::<Result<(StatusCode, JsonValue), HyperError>>(1);

        spawn(move || {
            let mut event_loop = Core::new().expect("Expected to create a new event loop");
            let handle = event_loop.handle();
            let client = HyperClient::new(&handle);
            
            loop {
                let request = in_receiver.recv().unwrap();
                let response_future = client.request(request);

                match event_loop.run(response_future) {
                    Ok(response) => {
                        let status = response.status();

                        let body_future = response.body().fold(Vec::new(), |mut v, chunk| {
                            v.extend(&chunk[..]);
                            future::ok::<_, HyperError>(v)
                        }).and_then(|chunks| {
                            let s = String::from_utf8(chunks).expect("Expected to be able to convert the body to a utf-8 string");
                            let v: JsonValue = serde_json::from_str(&s).expect("Expected to be able to convert the body to JSON");
                            future::ok::<_, HyperError>(v)
                        });

                        let body = event_loop.run(body_future).expect("Expected a response body");
                        out_sender.send(Ok((status, body))).unwrap();
                    },
                    Err(err) => {
                        out_sender.send(Err(err)).unwrap();
                    }
                }
            }
        });

        Self {
            lock: Mutex::new(()),
            in_sender: in_sender,
            out_receiver: out_receiver
        }
    }

    pub fn call(
        &self,
        port: usize,
        account_id: Uuid,
        secret: String,
        method_str: &str,
        path: &str,
        query_params: Vec<(&str, String)>,
        body: Option<JsonValue>
    ) -> Result<(StatusCode, JsonValue), HyperError> {
        let method = Method::from_str(method_str).expect("Expected a valid HTTP method");

        let mut url = Url::parse(&format!("http://localhost:{}{}", port, path)[..]).expect("Expected to generate a valid request URL");

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
            request.set_body(serde_json::to_string(&body).expect("Expected a valid request JSON body"));
        }

        let _ = self.lock.lock().unwrap();
        self.in_sender.send(request).unwrap();
        let x = self.out_receiver.recv().unwrap();
        x
    }
}

pub fn from_result<T>(result: Result<(StatusCode, JsonValue), HyperError>) -> Result<T, String> where for<'a> T: Deserialize<'a>  {
    match result {
        Ok((status, json)) => {
            if status == StatusCode::Ok {
                let v: T = serde_json::from_value(json).expect("Expected to be able to serialize from JSON to the custom type");
                Ok(v)
            } else {
                if let JsonValue::Object(obj) = json.clone() {
                    if let Some(&JsonValue::String(ref err)) = obj.get("error") {
                        return Err(err.clone());
                    }
                }

                panic!("Unexpected error response body: {}", json);
            }
        },
        Err(err) => {
            panic!("Connection error: {}", err);
        }
    }
}

lazy_static! {
    pub static ref CLIENT: Client = Client::default();
}
