extern crate chrono;
extern crate hyper;
#[macro_use]
extern crate indradb;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;
extern crate url;
extern crate futures;

#[macro_use]
mod common;

use tokio_core::reactor::Core;
use serde::Deserialize;
use hyper::client::{Client, HttpConnector};
use serde_json::value::Value as JsonValue;
pub use regex::Regex;
use uuid::Uuid;
pub use indradb::*;
pub use common::*;
use std::collections::HashMap;

lazy_static! {
    static ref ITEM_ERROR_MESSAGE_PATTERN: Regex = Regex::new(r"Item #0: (.+)").unwrap();
}

pub struct BatchTransaction {
    port: usize,
    account_id: Uuid,
    secret: String,
}

impl HttpTransaction for BatchTransaction {
    fn new(port: usize, account_id: Uuid, secret: String) -> Self {
        BatchTransaction {
            port: port,
            account_id: account_id,
            secret: secret,
        }
    }
}

impl BatchTransaction {
    fn request<T>(&self, body: JsonValue) -> Result<T, Error>
    where
        for<'a> T: Deserialize<'a>,
    {
        let mut event_loop = Core::new().unwrap();
        let handle = event_loop.handle();
        let client = Client::new(&handle);

        let req = request(
            self.port,
            self.account_id,
            self.secret.clone(),
            "POST",
            "/transaction",
            vec![],
            Some(body)
        );

        let res = client.request(req);

        handle_response::<T>(res, event_loop).map_err(|err| {
            if let Some(cap) = ITEM_ERROR_MESSAGE_PATTERN.captures(&err) {
                let message = cap.get(1).unwrap().as_str();
                Error::description_to_error(message)
            } else {
                panic!(format!("Unexpected error received: {}", err))
            }
        })
    }
}

impl Transaction for BatchTransaction {
    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        self.request(json!({
            "action": "create_vertex",
            "type": t.0
        }))
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>, Error> {
        self.request(json!({
            "action": "get_vertices",
            "query": q
        }))
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        self.request(json!({
            "action": "delete_vertices",
            "query": q
        }))
    }

    fn create_edge(&self, e: EdgeKey, weight: Weight) -> Result<(), Error> {
        self.request(json!({
            "action": "create_edge",
            "key": e,
            "weight": weight.0
        }))
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>, Error> {
        self.request(json!({
            "action": "get_edges",
            "query": q
        }))
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        self.request(json!({
            "action": "delete_edges",
            "query": q
        }))
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        self.request(json!({
            "action": "get_edge_count",
            "query": q
        }))
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

pub fn datastore() -> HttpDatastore<BatchTransaction> {
    HttpDatastore::<BatchTransaction>::default()
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
