#[macro_use]
extern crate braid;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate regex;
extern crate hyper;
extern crate uuid;

#[macro_use]
mod common;

use std::io::Read;

use hyper::client::{Client, RequestBuilder};
use serde_json::value::Value as JsonValue;
use serde::Deserialize;
use hyper::status::StatusCode;
use hyper::client::response::Response;
use uuid::Uuid;
use std::collections::HashMap;

pub use braid::*;
pub use common::*;

pub struct RestTransaction {
    port: i32,
    account_id: Uuid,
    secret: String,
}

impl RestTransaction {
    fn request<'a>(&self, client: &'a Client, method_str: &str, path: String, query_pairs: Vec<(&str, String)>) -> RequestBuilder<'a> {
        request(
            client,
            self.port,
            self.account_id,
            self.secret.clone(),
            method_str,
            path,
            query_pairs
        )
    }
}

impl HttpTransaction for RestTransaction {
    fn new(port: i32, account_id: Uuid, secret: String) -> Self {
        RestTransaction {
            port: port,
            account_id: account_id,
            secret: secret,
        }
    }
}

impl Transaction for RestTransaction {
    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        let client = Client::new();
        let req = self.request(&client, "POST", "/vertex".to_string(), vec![("type", t.0)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "GET", "/vertex".to_string(), vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn set_vertices(&self, q: VertexQuery, t: Type) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "PUT", "/vertex".to_string(), vec![("q", q_json), ("type", t.0)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "DELETE", "/vertex".to_string(), vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn create_edge(&self, key: EdgeKey, weight: Weight) -> Result<(), Error> {
        let client = Client::new();
        let path = format!("/edge/{}/{}/{}", key.outbound_id, key.t.0, key.inbound_id);
        let req = self.request(&client, "PUT", path, vec![("weight", weight.0.to_string())]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }
    
    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "GET", "/edge".to_string(), vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn set_edges(&self, q: EdgeQuery, weight: Weight) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "PUT", "/edge".to_string(), vec![("q", q_json), ("weight", weight.0.to_string())]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "DELETE", "/edge".to_string(), vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "GET", "/edge".to_string(), vec![("action", "count".to_string()), ("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_global_metadata(&self, _: String) -> Result<JsonValue, Error> {
        unimplemented!();
    }

    fn set_global_metadata(&self, _: String, _: JsonValue) -> Result<(), Error> {
        unimplemented!();
    }

    fn delete_global_metadata(&self, _: String) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_account_metadata(&self, _: Uuid, _: String) -> Result<JsonValue, Error> {
        unimplemented!();
    }

    fn set_account_metadata(&self, _: Uuid, _: String, _: JsonValue) -> Result<(), Error> {
        unimplemented!();
    }

    fn delete_account_metadata(&self, _: Uuid, _: String) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_vertex_metadata(&self, _: VertexQuery, _: String) -> Result<HashMap<Uuid, JsonValue>, Error> {
        unimplemented!();
    }

    fn set_vertex_metadata(&self, _: VertexQuery, _: String, _: JsonValue) -> Result<(), Error> {
        unimplemented!();
    }

    fn delete_vertex_metadata(&self, _: VertexQuery, _: String) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_edge_metadata(&self, _: EdgeQuery, _: String) -> Result<HashMap<EdgeKey, JsonValue>, Error> {
        unimplemented!();
    }

    fn set_edge_metadata(&self, _: EdgeQuery, _: String, _: JsonValue) -> Result<(), Error> {
        unimplemented!();
    }

    fn delete_edge_metadata(&self, _: EdgeQuery, _: String) -> Result<(), Error> {
        unimplemented!();
    }

    fn commit(self) -> Result<(), Error> {
        Ok(())
    }

    fn rollback(self) -> Result<(), Error> {
        Err(Error::Unexpected("Cannot rollback an HTTP-based transaction".to_string()))
    }
}

pub fn response_to_obj<T: Deserialize>(res: &mut Response) -> Result<T, Error> {
    match res.status {
        StatusCode::Ok => {
            let mut payload = String::new();
            res.read_to_string(&mut payload).unwrap();
            let v: T = serde_json::from_str(&payload[..]).unwrap();
            Ok(v)
        }
        _ => {
            let message = response_to_error_message(res);
            Err(Error::description_to_error(&message[..]))
        }
    }
}

pub fn datastore() -> HttpDatastore<RestTransaction> {
    HttpDatastore::<RestTransaction>::new(8000)
}

test_transaction_impl!(datastore());
