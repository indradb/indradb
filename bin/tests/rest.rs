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

#[macro_use]
mod common;

use serde_json::value::Value as JsonValue;
use serde::Deserialize;
use uuid::Uuid;
use std::collections::HashMap;
use reqwest::Method;

pub use indradb::*;
pub use common::*;

pub struct RestTransaction {
    port: usize,
}

impl RestTransaction {
    fn request<T>(
        &self,
        method: Method,
        path: &str,
        query_pairs: &Vec<(&str, &str)>
    ) -> Result<T, Error> where for<'a> T: Deserialize<'a> {
        let result = request(
            self.port,
            method,
            path,
            query_pairs,
            None,
        );

        from_result::<T>(result).map_err(|err| {
            Error::description_to_error(&err)
        })
    }
}

impl HttpTransaction for RestTransaction {
    fn new(port: usize) -> Self {
        RestTransaction {
            port: port,
        }
    }
}

impl Transaction for RestTransaction {
    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        self.request(Method::Post, "/vertex", &vec![("type", &t.0)])
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request(Method::Get, "/vertex", &vec![("q", &q_json)])
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request(Method::Delete, "/vertex", &vec![("q", &q_json)])
    }

    fn create_edge(&self, key: EdgeKey) -> Result<(), Error> {
        let path = format!("/edge/{}/{}/{}", key.outbound_id, key.t.0, key.inbound_id);
        self.request(
            Method::Put,
            &path[..],
            &vec![],
        )
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request(Method::Get, "/edge", &vec![("q", &q_json)])
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request(Method::Delete, "/edge", &vec![("q", &q_json)])
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let q_json = serde_json::to_string(&q).unwrap();
        self.request(
            Method::Get,
            "/edge",
            &vec![("action", "count"), ("q", &q_json)],
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
