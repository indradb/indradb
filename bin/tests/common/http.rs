use hyper::header::{Authorization, Basic};
use hyper::client::Client as HyperClient;
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
use std::thread::spawn;
use crossbeam_channel::{bounded, Sender, Receiver};

pub struct Client {
    in_sender: Sender<Request>,
    out_receiver: Receiver<Result<Response, HyperError>>
}

impl Client {
    fn default() -> Self {
        let (in_sender, in_receiver) = bounded::<Request>(1);
        let (out_sender, out_receiver) = bounded::<Result<Response, HyperError>>(1);

        spawn(move || {
            let mut event_loop = Core::new().unwrap();
            let handle = event_loop.handle();
            let client = HyperClient::new(&handle);
            
            loop {
                let request = in_receiver.recv().unwrap();
                let response_future = client.request(request);
                let response = event_loop.run(response_future);
                out_sender.send(response).unwrap();
            }
        });

        Self {
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
    ) -> Result<Response, HyperError> {
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

        self.in_sender.send(request).unwrap();
        self.out_receiver.recv().unwrap()
    }
}

pub fn from_response<T>(response: Response) -> Result<T, String> where for<'a> T: Deserialize<'a>  {
    if response.status() == StatusCode::Ok {
        let body: T = response_to_json(response);
        Ok(body)
    } else {
        let body: BTreeMap<String, JsonValue> = response_to_json(response);

        match body.get("error") {
            Some(&JsonValue::String(ref error)) => Err(error.clone()),
            _ => panic!("Could not unpack error message")
        }
    }
}

fn response_to_json<T>(response: Response) -> T
where
    for<'a> T: Deserialize<'a>
{
    let body_future = response.body().fold(Vec::new(), |mut v, chunk| {
        v.extend(&chunk[..]);
        future::ok::<_, HyperError>(v)
    }).and_then(|chunks| {
        let s = String::from_utf8(chunks).unwrap();
        let v: T = serde_json::from_str(&s).unwrap();
        future::ok::<_, HyperError>(v)
    });

    body_future.wait().expect("Expect a response body")
}

lazy_static! {
    pub static ref CLIENT: Client = Client::default();
}
