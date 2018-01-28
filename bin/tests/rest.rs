extern crate chrono;
extern crate hyper;
#[macro_use]
extern crate indradb;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;
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

pub use indradb::*;
pub use common::*;

pub struct RestTransaction {
    port: usize,
    account_id: Uuid,
    secret: String,
}

impl RestTransaction {
    fn request<'a>(
        &self,
        client: &'a Client,
        method_str: &str,
        path: &str,
        query_pairs: Vec<(&str, String)>,
    ) -> RequestBuilder<'a> {
        request(
            client,
            self.port,
            self.account_id,
            self.secret.clone(),
            method_str,
            path,
            query_pairs,
        )
    }
}

impl HttpTransaction for RestTransaction {
    fn new(port: usize, account_id: Uuid, secret: String) -> Self {
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
        let req = self.request(&client, "POST", "/vertex", vec![("type", t.0)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "GET", "/vertex", vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "DELETE", "/vertex", vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn create_edge(&self, key: EdgeKey) -> Result<(), Error> {
        let client = Client::new();
        let path = format!("/edge/{}/{}/{}", key.outbound_id, key.t.0, key.inbound_id);
        let req = self.request(
            &client,
            "PUT",
            &path[..],
            vec![],
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "GET", "/edge", vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(&client, "DELETE", "/edge", vec![("q", q_json)]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            "/edge",
            vec![("action", "count".to_string()), ("q", q_json)],
        );
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

    fn get_vertex_metadata(
        &self,
        _: VertexQuery,
        _: String,
    ) -> Result<HashMap<Uuid, JsonValue>, Error> {
        unimplemented!();
    }

    fn set_vertex_metadata(&self, _: VertexQuery, _: String, _: JsonValue) -> Result<(), Error> {
        unimplemented!();
    }

    fn delete_vertex_metadata(&self, _: VertexQuery, _: String) -> Result<(), Error> {
        unimplemented!();
    }

    fn get_edge_metadata(
        &self,
        _: EdgeQuery,
        _: String,
    ) -> Result<HashMap<EdgeKey, JsonValue>, Error> {
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
        Err(Error::Unexpected(
            "Cannot rollback an HTTP-based transaction".to_string(),
        ))
    }
}

pub fn response_to_obj<T>(res: &mut Response) -> Result<T, Error>
where
    for<'a> T: Deserialize<'a>,
{
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
    HttpDatastore::<RestTransaction>::default()
}

// Vertex queries
define_test!(should_get_all_vertices, datastore());
define_test!(should_get_all_vertices_with_zero_limit, datastore());
define_test!(should_get_all_vertices_out_of_range, datastore());
define_test!(should_get_single_vertices, datastore());
define_test!(should_get_single_vertices_nonexisting, datastore());
define_test!(should_get_vertices, datastore());
define_test!(should_get_vertices_piped, datastore());

// Vertex updates
define_test!(should_delete_a_valid_vertex, datastore());
define_test!(should_not_delete_an_invalid_vertex, datastore());

// Edges
define_test!(should_get_a_valid_edge, datastore());
define_test!(should_not_get_an_invalid_edge, datastore());
define_test!(should_create_a_valid_edge, datastore());
define_test!(should_not_create_an_invalid_edge, datastore());
define_test!(should_delete_a_valid_edge, datastore());
define_test!(should_not_delete_an_invalid_edge, datastore());
define_test!(should_get_an_edge_count, datastore());
define_test!(should_get_an_edge_count_with_no_type, datastore());
define_test!(should_get_an_edge_count_for_an_invalid_edge, datastore());
define_test!(should_get_an_edge_range, datastore());
define_test!(should_get_edges_with_no_type, datastore());
define_test!(should_get_no_edges_for_an_invalid_range, datastore());
define_test!(should_get_edges_with_no_high, datastore());
define_test!(should_get_edges_with_no_low, datastore());
define_test!(should_get_edges_with_no_time, datastore());
define_test!(should_get_no_edges_for_reversed_time, datastore());
define_test!(should_get_edges, datastore());
