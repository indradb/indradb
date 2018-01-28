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

use std::collections::BTreeMap;

use serde::Deserialize;
use hyper::client::Client;
use hyper::status::StatusCode;
use serde_json::value::Value as JsonValue;
pub use regex::Regex;
use uuid::Uuid;
pub use indradb::*;
pub use common::*;
use std::io::Read;
use std::collections::HashMap;
use serde_json::Number as JsonNumber;

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
    fn request<T>(&self, d: BTreeMap<String, JsonValue>) -> Result<T, Error>
    where
        for<'a> T: Deserialize<'a>,
    {
        let body = serde_json::to_string(&vec![d]).unwrap();
        let client = Client::new();
        let req = request(
            &client,
            self.port,
            self.account_id,
            self.secret.clone(),
            "POST",
            "/transaction",
            vec![],
        ).body(&body[..]);
        let mut res = req.send().unwrap();

        let mut payload = String::new();
        res.read_to_string(&mut payload).unwrap();

        match res.status {
            StatusCode::Ok => {
                let mut v: Vec<T> = serde_json::from_str(&payload[..]).unwrap();
                let o = v.pop().unwrap();
                Ok(o)
            }
            _ => {
                let o: BTreeMap<String, JsonValue> = serde_json::from_str(&payload[..]).unwrap();

                match o.get("error") {
                    Some(&JsonValue::String(ref error)) => {
                        if let Some(cap) = ITEM_ERROR_MESSAGE_PATTERN.captures(error) {
                            let message = cap.get(1).unwrap().as_str();
                            Err(Error::description_to_error(message))
                        } else {
                            panic!(format!("Unexpected error received: {}", error))
                        }
                    }
                    _ => panic!("Could not unpack error message"),
                }
            }
        }
    }
}

impl Transaction for BatchTransaction {
    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        self.request(btreemap!{
            "action".to_string() => JsonValue::String("create_vertex".to_string()),
            "type".to_string() => JsonValue::String(t.0)
        })
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>, Error> {
        self.request(btreemap!{
            "action".to_string() => JsonValue::String("get_vertices".to_string()),
            "query".to_string() => serde_json::to_value::<VertexQuery>(q).unwrap(),
        })
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        self.request(btreemap!{
            "action".to_string() => JsonValue::String("delete_vertices".to_string()),
            "query".to_string() => serde_json::to_value::<VertexQuery>(q).unwrap(),
        })
    }

    fn create_edge(&self, e: EdgeKey) -> Result<(), Error> {
        self.request(btreemap!{
            "action".to_string() => JsonValue::String("create_edge".to_string()),
            "key".to_string() => serde_json::to_value::<EdgeKey>(e).unwrap(),
        })
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>, Error> {
        self.request(btreemap!{
            "action".to_string() => JsonValue::String("get_edges".to_string()),
            "query".to_string() => serde_json::to_value::<EdgeQuery>(q).unwrap(),
        })
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        self.request(btreemap!{
            "action".to_string() => JsonValue::String("delete_edges".to_string()),
            "query".to_string() => serde_json::to_value::<EdgeQuery>(q).unwrap(),
        })
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        self.request(btreemap!{
            "action".to_string() => JsonValue::String("get_edge_count".to_string()),
            "query".to_string() => serde_json::to_value::<EdgeQuery>(q).unwrap(),
        })
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
