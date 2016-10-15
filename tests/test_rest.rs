#![cfg(test)]

#![feature(custom_derive, plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use] extern crate nutrino;
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate regex;

mod common;

pub use nutrino::*;
use std::collections::BTreeMap;
pub use std::env;
pub use common::{HttpDatastore, HttpTransaction};
use serde_json::value::Value as JsonValue;
use chrono::NaiveDateTime;

pub struct RestTransaction {
	port: i32,
	account_id: i64,
	secret: String
}

impl HttpTransaction<RestTransaction> for RestTransaction {
	fn new(port: i32, account_id: i64, secret: String) -> Self {
		RestTransaction {
			port: port,
			account_id: account_id,
			secret: secret
		}
	}
}

impl Transaction<i64> for RestTransaction {
	fn get_vertex(&self, id: i64) -> Result<Vertex<i64>, Error> {
		Err(Error::VertexDoesNotExist)
	}

	fn create_vertex(&self, t: String, properties: BTreeMap<String, JsonValue>) -> Result<i64, Error> {
		panic!("Unreachable point hit")
	}

	fn set_vertex(&self, v: Vertex<i64>) -> Result<(), Error> {
		Err(Error::VertexDoesNotExist)
	}

	fn delete_vertex(&self, id: i64) -> Result<(), Error> {
		Err(Error::VertexDoesNotExist)
	}

	fn get_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<Edge<i64>, Error> {
		Err(Error::EdgeDoesNotExist)
	}

	fn set_edge(&self, e: Edge<i64>) -> Result<(), Error> {
		Err(Error::VertexDoesNotExist)
	}

	fn delete_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<(), Error> {
		Err(Error::EdgeDoesNotExist)
	}

	fn get_edge_count(&self, outbound_id: i64, t: String) -> Result<i64, Error> {
		panic!("Unreachable point hit")
	}

	fn get_edge_range(&self, outbound_id: i64, t: String, offset: i64, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		panic!("Unreachable point hit")
	}

	fn get_edge_time_range(&self, outbound_id: i64, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		panic!("Unreachable point hit")
	}

	fn get_metadata(&self, owner_id: Option<i64>, key: String) -> Result<JsonValue, Error> {
		panic!("Unimplemented")
	}

	fn set_metadata(&self, owner_id: Option<i64>, key: String, value: JsonValue) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn delete_metadata(&self, owner_id: Option<i64>, key: String) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn commit(self) -> Result<(), Error> {
		panic!("Unreachable point hit")
	}

	fn rollback(self) -> Result<(), Error> {
		panic!("Unreachable point hit")
	}
}

test_transaction_impl! {
	rest_transaction {
	    HttpDatastore::<RestTransaction, RestTransaction>::new(8000)
	}
}
