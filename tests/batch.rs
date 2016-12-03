#![cfg(test)]

#![feature(plugin, test, proc_macro)]
#![plugin(stainless)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nutrino;
#[macro_use]
extern crate lazy_static;
extern crate test;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate regex;
extern crate hyper;
extern crate uuid;

mod common;

use std::collections::BTreeMap;

use serde::Deserialize;
use hyper::client::Client;
use hyper::status::StatusCode;
use serde_json::value::Value as JsonValue;
use chrono::NaiveDateTime;
pub use regex::Regex;
use uuid::Uuid;
pub use nutrino::*;
pub use common::{HttpDatastore, HttpTransaction, request, response_to_error_message};
use std::io::Read;

lazy_static! {
	static ref ITEM_ERROR_MESSAGE_PATTERN: Regex = Regex::new(r"Item #0: (.+)").unwrap();
}

pub struct BatchTransaction {
    port: i32,
    account_id: Uuid,
    secret: String,
}

impl HttpTransaction<BatchTransaction> for BatchTransaction {
    fn new(port: i32, account_id: Uuid, secret: String) -> Self {
        BatchTransaction {
            port: port,
            account_id: account_id,
            secret: secret,
        }
    }
}

impl BatchTransaction {
    fn request<T: Deserialize>(&self, d: BTreeMap<String, JsonValue>) -> Result<T, Error> {
        let body = serde_json::to_string(&vec![d]).unwrap();
        let client = Client::new();
        let req = request(
            &client,
            self.port,
            self.account_id,
            self.secret.clone(),
            "POST",
            "/transaction".to_string()
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
                        let cap = ITEM_ERROR_MESSAGE_PATTERN.captures(error).unwrap();
                        let message = cap.at(1).unwrap();
                        Err(Error::description_to_error(message))
                    }
                    _ => panic!("Could not unpack error message"),
                }
            }
        }
    }

    fn datetime_to_json(&self, datetime: Option<NaiveDateTime>) -> JsonValue {
        match datetime {
            Some(val) => JsonValue::I64(val.timestamp()),
            None => JsonValue::Null,
        }
    }
}

impl Transaction<Uuid> for BatchTransaction {
    fn get_vertex(&self, id: Uuid) -> Result<Vertex<Uuid>, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_vertex".to_string()),
			"id".to_string() => JsonValue::String(id.hyphenated().to_string())
		})
    }

    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("create_vertex".to_string()),
			"type".to_string() => JsonValue::String(t.0)
		})
    }

    fn set_vertex(&self, v: Vertex<Uuid>) -> Result<(), Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("set_vertex".to_string()),
			"id".to_string() => JsonValue::String(v.id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(v.t.0)
		})
    }

    fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("delete_vertex".to_string()),
			"id".to_string() => JsonValue::String(id.hyphenated().to_string())
		})
    }

    fn get_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<Edge<Uuid>, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge".to_string()),
			"outbound_id".to_string() => JsonValue::String(outbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0),
			"inbound_id".to_string() => JsonValue::String(inbound_id.hyphenated().to_string())
		})
    }

    fn set_edge(&self, e: Edge<Uuid>) -> Result<(), Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("set_edge".to_string()),
			"outbound_id".to_string() => JsonValue::String(e.outbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(e.t.0),
			"inbound_id".to_string() => JsonValue::String(e.inbound_id.hyphenated().to_string()),
			"weight".to_string() => JsonValue::F64(e.weight.0 as f64)
		})
    }

    fn delete_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<(), Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("delete_edge".to_string()),
			"outbound_id".to_string() => JsonValue::String(outbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0),
			"inbound_id".to_string() => JsonValue::String(inbound_id.hyphenated().to_string())
		})
    }

    fn get_edge_count(&self, outbound_id: Uuid, t: Type) -> Result<u64, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge_count".to_string()),
			"outbound_id".to_string() => JsonValue::String(outbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0)
		})
    }

    fn get_edge_range(&self, outbound_id: Uuid, t: Type, offset: u64, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge_range".to_string()),
			"outbound_id".to_string() => JsonValue::String(outbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0),
			"offset".to_string() => JsonValue::U64(offset),
			"limit".to_string() => JsonValue::U64(limit as u64)
		})
    }

    fn get_edge_time_range(&self, outbound_id: Uuid, t: Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_edge_time_range".to_string()),
			"outbound_id".to_string() => JsonValue::String(outbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0),
			"high".to_string() => self.datetime_to_json(high),
			"low".to_string() => self.datetime_to_json(low),
			"limit".to_string() => JsonValue::I64(limit as i64)
		})
    }

    fn get_reversed_edge_count(&self, inbound_id: Uuid, t: Type) -> Result<u64, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_reversed_edge_count".to_string()),
			"inbound_id".to_string() => JsonValue::String(inbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0)
		})
    }

    fn get_reversed_edge_range(&self, inbound_id: Uuid, t: Type, offset: u64, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_reversed_edge_range".to_string()),
			"inbound_id".to_string() => JsonValue::String(inbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0),
			"offset".to_string() => JsonValue::U64(offset),
			"limit".to_string() => JsonValue::U64(limit as u64)
		})
    }

    fn get_reversed_edge_time_range(&self, inbound_id: Uuid, t: Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        self.request(btreemap!{
			"action".to_string() => JsonValue::String("get_reversed_edge_time_range".to_string()),
			"inbound_id".to_string() => JsonValue::String(inbound_id.hyphenated().to_string()),
			"type".to_string() => JsonValue::String(t.0),
			"high".to_string() => self.datetime_to_json(high),
			"low".to_string() => self.datetime_to_json(low),
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

    fn get_account_metadata(&self, _: Uuid, _: String) -> Result<JsonValue, Error> {
        panic!("Unimplemented")
    }

    fn set_account_metadata(&self, _: Uuid, _: String, _: JsonValue) -> Result<(), Error> {
        panic!("Unimplemented")
    }

    fn delete_account_metadata(&self, _: Uuid, _: String) -> Result<(), Error> {
        panic!("Unimplemented")
    }

    fn get_vertex_metadata(&self, _: Uuid, _: String) -> Result<JsonValue, Error> {
        panic!("Unimplemented")
    }

    fn set_vertex_metadata(&self, _: Uuid, _: String, _: JsonValue) -> Result<(), Error> {
        panic!("Unimplemented")
    }

    fn delete_vertex_metadata(&self, _: Uuid, _: String) -> Result<(), Error> {
        panic!("Unimplemented")
    }

    fn get_edge_metadata(&self, _: Uuid, _: Type, _: Uuid, _: String) -> Result<JsonValue, Error> {
        panic!("Unimplemented")
    }

    fn set_edge_metadata(&self, _: Uuid, _: Type, _: Uuid, _: String, _: JsonValue) -> Result<(), Error> {
        panic!("Unimplemented")
    }

    fn delete_edge_metadata(&self, _: Uuid, _: Type, _: Uuid, _: String) -> Result<(), Error> {
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
