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
extern crate hyper;

mod common;

use std::collections::BTreeMap;
use std::io::Read;

use hyper::client::{Client, RequestBuilder};
use serde_json::value::Value as JsonValue;
use chrono::NaiveDateTime;
use serde::Deserialize;
use hyper::status::StatusCode;
use hyper::client::response::Response;

pub use nutrino::*;
pub use common::{HttpDatastore, HttpTransaction, request, response_to_error_message};

pub struct RestTransaction {
	port: i32,
	account_id: i64,
	secret: String
}

impl RestTransaction {
	fn request<'a>(&self, client: &'a Client, method_str: &str, path: String) -> RequestBuilder<'a> {
		return request(client, self.port, self.account_id, self.secret.clone(), method_str, path)
	}
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
		let client = Client::new();
		let req = self.request(&client, "GET", format!("/vertex/{}", id));
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn create_vertex(&self, t: String, properties: BTreeMap<String, JsonValue>) -> Result<i64, Error> {
		let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
		d.insert("type".to_string(), JsonValue::String(t));
		d.insert("properties".to_string(), JsonValue::Object(properties));
		let body = serde_json::to_string(&d).unwrap();

		let client = Client::new();
		let req = self.request(&client, "POST", "/vertex".to_string()).body(&body[..]);
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn set_vertex(&self, v: Vertex<i64>) -> Result<(), Error> {
		let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
		d.insert("type".to_string(), JsonValue::String(v.t));
		d.insert("properties".to_string(), JsonValue::Object(v.properties));
		let body = serde_json::to_string(&d).unwrap();

		let client = Client::new();
		let req = self.request(&client, "PUT", format!("/vertex/{}", v.id)).body(&body[..]);
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn delete_vertex(&self, id: i64) -> Result<(), Error> {
		let client = Client::new();
		let req = self.request(&client, "DELETE", format!("/vertex/{}", id));
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn get_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<Edge<i64>, Error> {
		let client = Client::new();
		let req = self.request(&client, "GET", format!("/edge/{}/{}/{}", outbound_id, t, inbound_id));
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn set_edge(&self, e: Edge<i64>) -> Result<(), Error> {
		let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
		d.insert("weight".to_string(), JsonValue::F64(e.weight as f64));
		d.insert("properties".to_string(), JsonValue::Object(e.properties));
		let body = serde_json::to_string(&d).unwrap();

		let client = Client::new();
		let req = self.request(&client, "PUT", format!("/edge/{}/{}/{}", e.outbound_id, e.t, e.inbound_id)).body(&body[..]);
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn delete_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<(), Error> {
		let client = Client::new();
		let req = self.request(&client, "DELETE", format!("/edge/{}/{}/{}", outbound_id, t, inbound_id));
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn get_edge_count(&self, outbound_id: i64, t: String) -> Result<i64, Error> {
		let client = Client::new();
		let req = self.request(&client, "GET", format!("/edge/{}/{}?action=count", outbound_id, t));
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn get_edge_range(&self, outbound_id: i64, t: String, offset: i64, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		let client = Client::new();
		let req = self.request(&client, "GET", format!("/edge/{}/{}?action=position&limit={}&offset={}", outbound_id, t, limit, offset));
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn get_edge_time_range(&self, outbound_id: i64, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		let client = Client::new();

		let qp = match (high, low) {
			(Some(high), Some(low)) => format!("&high={}&low={}", high.timestamp(), low.timestamp()),
			(Some(high), None) => format!("&high={}", high.timestamp()),
			(None, Some(low)) => format!("&low={}", low.timestamp()),
			(None, None) => "".to_string(),
		};

		let req = self.request(&client, "GET", format!("/edge/{}/{}?action=time&limit={}{}", outbound_id, t, limit, qp));
		let mut res = req.send().unwrap();
		response_to_obj(&mut res)
	}

	fn get_metadata(&self, _: Option<i64>, _: String) -> Result<JsonValue, Error> {
		panic!("Unimplemented")
	}

	fn set_metadata(&self, _: Option<i64>, _: String, _: JsonValue) -> Result<(), Error> {
		panic!("Unimplemented")
	}

	fn delete_metadata(&self, _: Option<i64>, _: String) -> Result<(), Error> {
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
	rest_transaction {
	    HttpDatastore::<RestTransaction, RestTransaction>::new(8000)
	}
}

pub fn response_to_obj<T: Deserialize>(res: &mut Response) -> Result<T, Error> {
	match res.status {
		StatusCode::Ok => {
			let mut payload = String::new();
			res.read_to_string(&mut payload).unwrap();
			let v: T = serde_json::from_str(&payload[..]).unwrap();
			Ok(v)
		},
        _ => {
            let message = response_to_error_message(res);
            Err(Error::description_to_error(&message[..]))
        }
	}
}
