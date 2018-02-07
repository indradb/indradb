extern crate chrono;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate indradb;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate uuid;
extern crate url;

#[macro_use]
mod common;

use serde_json::value::Value as JsonValue;
use serde::Deserialize;
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
    fn request<T>(
        &self,
        method_str: &str,
        path: &str,
        query_pairs: Vec<(&str, String)>
    ) -> Result<T, Error> where for<'a> T: Deserialize<'a> {
        let response = CLIENT.call(
            self.port,
            self.account_id,
            self.secret.clone(),
            method_str,
            path,
            query_pairs,
            None,
        );

        from_response::<T>(response).map_err(|err| {
            Error::description_to_error(&err)
        })
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
        self.request("POST", "/vertex", vec![("type", t.0)])
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request("GET", "/vertex", vec![("q", q_json)])
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request("DELETE", "/vertex", vec![("q", q_json)])
    }

    fn create_edge(&self, key: EdgeKey, weight: Weight) -> Result<(), Error> {
        let path = format!("/edge/{}/{}/{}", key.outbound_id, key.t.0, key.inbound_id);
        self.request(
            "PUT",
            &path[..],
            vec![("weight", weight.0.to_string())],
        )
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request("GET", "/edge", vec![("q", q_json)])
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request("DELETE", "/edge", vec![("q", q_json)])
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request(
            "GET",
            "/edge",
            vec![("action", "count".to_string()), ("q", q_json)],
        )
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
