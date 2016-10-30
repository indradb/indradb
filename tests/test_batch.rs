#![cfg(test)]

#![feature(custom_derive, plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use] extern crate maplit;
#[macro_use] extern crate nutrino;
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate regex;
extern crate hyper;

mod common;

use std::collections::BTreeMap;

use serde::Deserialize;
use hyper::client::Client;
use hyper::status::StatusCode;
use serde_json::value::Value as JsonValue;
use chrono::NaiveDateTime;
pub use regex::Regex;

pub use nutrino::*;
pub use common::{HttpDatastore, HttpTransaction, request, response_to_error_message};

use std::io::Read;

lazy_static! {
	static ref ITEM_ERROR_MESSAGE_PATTERN: Regex = Regex::new(r"Item #0: (.+)").unwrap();
}

pub struct BatchTransaction {
	port: i32,
	account_id: i64,
	secret: String
}

impl HttpTransaction<BatchTransaction> for BatchTransaction {
	fn new(port: i32, account_id: i64, secret: String) -> Self {
		BatchTransaction {
			port: port,
			account_id: account_id,
			secret: secret
		}
	}
}

impl BatchTransaction {
	fn request<T: Deserialize>(&self, d: BTreeMap<String, JsonValue>) -> Result<T, Error> {
		let body = serde_json::to_string(&vec!(d)).unwrap();
		let client = Client::new();
		let req = request(&client, self.port, self.account_id, self.secret.clone(), "POST", "/transaction".to_string()).body(&body[..]);
		let mut res = req.send().unwrap();

		let mut payload = String::new();
		res.read_to_string(&mut payload).unwrap();

		match res.status {
			StatusCode::Ok => {
				let mut v: Vec<T> = serde_json::from_str(&payload[..]).unwrap();
				let o = v.pop().unwrap();
				Ok(o)
			},
	        _ => {
				let mut o: BTreeMap<String, JsonValue> = serde_json::from_str(&payload[..]).unwrap();

				match o.get("error") {
					Some(&JsonValue::String(ref error)) => {
						let cap = ITEM_ERROR_MESSAGE_PATTERN.captures(error).unwrap();
						let message = cap.at(1).unwrap();
						Err(Error::description_to_error(message))
					},
					_ => panic!("Could not unpack error message")
				}
	        }
		}
	}
}

impl Transaction<i64> for BatchTransaction {
	fn get_vertex(&self, id: i64) -> Result<Vertex<i64>, Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_vertex".to_string()),
			"id".to_string() => JsonValue::I64(id)
		})
	}

	fn create_vertex(&self, t: String) -> Result<i64, Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("create_vertex".to_string()),
			"type".to_string() => JsonValue::String(t)
		})
	}

	fn set_vertex(&self, v: Vertex<i64>) -> Result<(), Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("set_vertex".to_string()),
			"id".to_string() => JsonValue::I64(v.id),
			"type".to_string() => JsonValue::String(v.t)
		})
	}

	fn delete_vertex(&self, id: i64) -> Result<(), Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("delete_vertex".to_string()),
			"id".to_string() => JsonValue::I64(id)
		})
	}

	fn get_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<Edge<i64>, Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge".to_string()),
			"outbound_id".to_string() => JsonValue::I64(outbound_id),
			"type".to_string() => JsonValue::String(t),
			"inbound_id".to_string() => JsonValue::I64(inbound_id)
		})
	}

	fn set_edge(&self, e: Edge<i64>) -> Result<(), Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("set_edge".to_string()),
			"outbound_id".to_string() => JsonValue::I64(e.outbound_id),
			"type".to_string() => JsonValue::String(e.t),
			"inbound_id".to_string() => JsonValue::I64(e.inbound_id),
			"weight".to_string() => JsonValue::F64(e.weight as f64)
		})
	}

	fn delete_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<(), Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("delete_edge".to_string()),
			"outbound_id".to_string() => JsonValue::I64(outbound_id),
			"type".to_string() => JsonValue::String(t),
			"inbound_id".to_string() => JsonValue::I64(inbound_id)
		})
	}

	fn get_edge_count(&self, outbound_id: i64, t: String) -> Result<i64, Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge_count".to_string()),
			"outbound_id".to_string() => JsonValue::I64(outbound_id),
			"type".to_string() => JsonValue::String(t)
		})
	}

	fn get_edge_range(&self, outbound_id: i64, t: String, offset: i64, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge_range".to_string()),
			"outbound_id".to_string() => JsonValue::I64(outbound_id),
			"type".to_string() => JsonValue::String(t),
			"offset".to_string() => JsonValue::I64(offset),
			"limit".to_string() => JsonValue::I64(limit as i64)
		})
	}

	fn get_edge_time_range(&self, outbound_id: i64, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		let datetime_converter = |val: Option<NaiveDateTime>| {
			match val {
				Some(val) => JsonValue::I64(val.timestamp()),
				None => JsonValue::Null
			}
		};

		self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge_time_range".to_string()),
			"outbound_id".to_string() => JsonValue::I64(outbound_id),
			"type".to_string() => JsonValue::String(t),
			"high".to_string() => datetime_converter(high),
			"low".to_string() => datetime_converter(low),
			"limit".to_string() => JsonValue::I64(limit as i64)
		})
	}

	fn get_global_metadata(&self, _: String) -> Result<JsonValue, Error> {
		panic!("Unimplemented")
	}

	fn set_global_metadata(&self, _: String, _: JsonValue) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn delete_global_metadata(&self, _: String) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn get_account_metadata(&self, _: i64, _: String) -> Result<JsonValue, Error> {
		panic!("Unimplemented")
	}

	fn set_account_metadata(&self, _: i64, _: String, _: JsonValue) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn delete_account_metadata(&self, _: i64, _: String) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn get_vertex_metadata(&self, _: i64, _: String) -> Result<JsonValue, Error> {
		panic!("Unimplemented")
	}

	fn set_vertex_metadata(&self, _: i64, _: String, _: JsonValue) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn delete_vertex_metadata(&self, _: i64, _: String) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn get_edge_metadata(&self, _: i64, _: String, _: i64, _: String) -> Result<JsonValue, Error> {
		panic!("Unimplemented")
	}

	fn set_edge_metadata(&self, _: i64, _: String, _: i64, _: String, _: JsonValue) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn delete_edge_metadata(&self, _: i64, _: String, _: i64, _: String) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn commit(self) -> Result<(), Error> {
		Ok(())
	}

	fn rollback(self) -> Result<(), Error> {
		Err(Error::Unexpected("Cannot rollback an HTTP-based transaction".to_string()))
	}
}

test_transaction_impl! {
	test_batch_transaction {
	    HttpDatastore::<BatchTransaction, BatchTransaction>::new(8000)
	}
}

bench_transaction_impl! {
	bench_batch_transaction {
	    HttpDatastore::<BatchTransaction, BatchTransaction>::new(8000)
	}
}
