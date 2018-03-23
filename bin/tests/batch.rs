extern crate chrono;
#[macro_use]
extern crate indradb;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate uuid;

pub use indradb::{Datastore, Edge, EdgeDirection, EdgeKey, EdgeMetadata, EdgeQuery, Error, Transaction, Type, Vertex,
                  VertexMetadata, VertexQuery};
pub use indradb::tests;
pub use regex::Regex;
use reqwest::{Client, Error as ReqwestError, Method, Response, StatusCode, Url};
use serde::Deserialize;
use serde_json::value::Value as JsonValue;
use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

const START_PORT: usize = 1024;

lazy_static! {
    static ref ITEM_ERROR_MESSAGE_PATTERN: Regex = Regex::new(r"Item #0: (.+)").unwrap();
    static ref PORT: AtomicUsize = AtomicUsize::new(START_PORT);
}

fn request(port: usize, body: &JsonValue) -> Result<Response, ReqwestError> {
    let client = Client::new();
    let url =
        Url::parse(&format!("http://localhost:{}/transaction", port)).expect("Expected to be able to construct a URL");
    let mut request = client.request(Method::Post, url);
    request.json(body);
    request.send()
}

#[derive(Debug)]
pub struct BatchDatastore {
    port: usize,
    server: Child,
}

impl BatchDatastore {
    fn default() -> Self {
        let port = PORT.fetch_add(1, Ordering::SeqCst);

        let mut envs = HashMap::new();
        envs.insert("PORT", port.to_string());

        let server = Command::new("../target/debug/indradb-server")
            .envs(envs)
            .spawn()
            .expect("Server failed to start");

        for _ in 0..5 {
            if let Ok(response) = request(port, &json!([])) {
                if response.status() == StatusCode::Ok {
                    return Self {
                        port: port,
                        server: server,
                    };
                }
            }

            sleep(Duration::from_secs(1));
        }

        panic!("Server failed to initialize after a few seconds");
    }
}

impl Drop for BatchDatastore {
    fn drop(&mut self) {
        if let Err(err) = self.server.kill() {
            panic!(format!("Could not kill server instance: {}", err))
        }
    }
}

impl Datastore<BatchTransaction> for BatchDatastore {
    fn transaction(&self) -> Result<BatchTransaction, Error> {
        Ok(BatchTransaction::new(self.port))
    }
}

pub struct BatchTransaction {
    port: usize,
}

impl BatchTransaction {
    fn new(port: usize) -> Self {
        BatchTransaction { port: port }
    }

    fn request<T>(&self, body: &JsonValue) -> Result<T, Error>
    where
        for<'a> T: Deserialize<'a>,
    {
        let mut parts = match request(self.port, &json!([body])) {
            Ok(mut response) => {
                if response.status() == StatusCode::Ok {
                    let v: Vec<T> = response
                        .json()
                        .expect("Could not deserialize response to custom type");
                    v
                } else {
                    let v: JsonValue = response
                        .json()
                        .expect("Could not deserialize response to object");

                    if let JsonValue::Object(ref obj) = v {
                        if let Some(&JsonValue::String(ref err)) = obj.get("error") {
                            if let Some(cap) = ITEM_ERROR_MESSAGE_PATTERN.captures(err) {
                                return Err(cap.get(1).unwrap().as_str().into());
                            } else {
                                panic!(format!("Unexpected error received: {}", err));
                            }
                        }
                    }

                    panic!("Unexpected error response object: {}", v)
                }
            }
            Err(err) => panic!("Request error: {}", err),
        };

        assert!(parts.len() == 1, "Invalid number of items returned");
        Ok(parts.pop().unwrap())
    }
}

impl Transaction for BatchTransaction {
    fn create_vertex(&self, v: &Vertex) -> Result<bool, Error> {
        self.request(&json!({
            "action": "create_vertex",
            "vertex": v
        }))
    }

    fn create_vertex_from_type(&self, t: Type) -> Result<Uuid, Error> {
        self.request(&json!({
            "action": "create_vertex_from_type",
            "type": t
        }))
    }

    fn get_vertices(&self, q: &VertexQuery) -> Result<Vec<Vertex>, Error> {
        self.request(&json!({
            "action": "get_vertices",
            "query": q
        }))
    }

    fn delete_vertices(&self, q: &VertexQuery) -> Result<(), Error> {
        self.request(&json!({
            "action": "delete_vertices",
            "query": q
        }))
    }

    fn get_vertex_count(&self) -> Result<u64, Error> {
        self.request(&json!({
            "action": "get_vertex_count"
        }))
    }

    fn create_edge(&self, e: &EdgeKey) -> Result<bool, Error> {
        self.request(&json!({
            "action": "create_edge",
            "key": e,
        }))
    }

    fn get_edges(&self, q: &EdgeQuery) -> Result<Vec<Edge>, Error> {
        self.request(&json!({
            "action": "get_edges",
            "query": q
        }))
    }

    fn delete_edges(&self, q: &EdgeQuery) -> Result<(), Error> {
        self.request(&json!({
            "action": "delete_edges",
            "query": q
        }))
    }

    fn get_edge_count(&self, id: Uuid, type_filter: Option<&Type>, direction: EdgeDirection) -> Result<u64, Error> {
        self.request(&json!({
            "action": "get_edge_count",
            "id": id,
            "type_filter": type_filter,
            "direction": direction
        }))
    }

    fn get_global_metadata(&self, name: &str) -> Result<Option<JsonValue>, Error> {
        self.request(&json!({
            "action": "get_global_metadata",
            "name": name
        }))
    }

    fn set_global_metadata(&self, name: &str, value: &JsonValue) -> Result<(), Error> {
        self.request(&json!({
            "action": "set_global_metadata",
            "name": name,
            "value": value
        }))
    }

    fn delete_global_metadata(&self, name: &str) -> Result<(), Error> {
        self.request(&json!({
            "action": "delete_global_metadata",
            "name": name
        }))
    }

    fn get_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<Vec<VertexMetadata>, Error> {
        self.request(&json!({
            "action": "get_vertex_metadata",
            "query": q,
            "name": name
        }))
    }

    fn set_vertex_metadata(&self, q: &VertexQuery, name: &str, value: &JsonValue) -> Result<(), Error> {
        self.request(&json!({
            "action": "set_vertex_metadata",
            "query": q,
            "name": name,
            "value": value
        }))
    }

    fn delete_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<(), Error> {
        self.request(&json!({
            "action": "delete_vertex_metadata",
            "query": q,
            "name": name
        }))
    }

    fn get_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<Vec<EdgeMetadata>, Error> {
        self.request(&json!({
            "action": "get_edge_metadata",
            "query": q,
            "name": name
        }))
    }

    fn set_edge_metadata(&self, q: &EdgeQuery, name: &str, value: &JsonValue) -> Result<(), Error> {
        self.request(&json!({
            "action": "set_edge_metadata",
            "query": q,
            "name": name,
            "value": value
        }))
    }

    fn delete_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<(), Error> {
        self.request(&json!({
            "action": "delete_edge_metadata",
            "query": q,
            "name": name
        }))
    }
}

pub fn datastore() -> BatchDatastore {
    BatchDatastore::default()
}

full_test_impl!(datastore());
