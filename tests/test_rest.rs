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

use hyper::header::{Authorization, Basic};
use hyper::client::{Client, RequestBuilder};
use hyper::client::response::Response;
use hyper::status::StatusCode;
use hyper::method::Method;

use serde::Deserialize;
use serde_json::value::Value as JsonValue;

use std::io::Read;
use std::str::FromStr;

pub use nutrino::*;
pub use common::{HttpDatastore, HttpTransaction};

use std::collections::BTreeMap;
use chrono::NaiveDateTime;

pub struct RestTransaction {
	port: i32,
	account_id: i64,
	secret: String
}

impl RestTransaction {
	fn req<'a>(&self, client: &'a Client, method_str: &str, path: String) -> RequestBuilder<'a> {
		let method = Method::from_str(method_str).unwrap();
		let url = format!("http://localhost:{}{}", self.port, path);

		let auth = Authorization(
			Basic {
				username: self.account_id.to_string(),
				password: Some(self.secret.clone())
			}
		);

		client.request(method, &url[..]).header(auth)
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
		let req = self.req(&client, "GET", format!("/vertex/{}", id));
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn create_vertex(&self, t: String, properties: BTreeMap<String, JsonValue>) -> Result<i64, Error> {
		let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
		d.insert("type".to_string(), JsonValue::String(t));
		d.insert("properties".to_string(), JsonValue::Object(properties));
		let body = serde_json::to_string(&d).unwrap();

		let client = Client::new();
		let req = self.req(&client, "POST", "/".to_string()).body(&body[..]);
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn set_vertex(&self, v: Vertex<i64>) -> Result<(), Error> {
		let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
		d.insert("type".to_string(), JsonValue::String(v.t));
		d.insert("properties".to_string(), JsonValue::Object(v.properties));
		let body = serde_json::to_string(&d).unwrap();

		let client = Client::new();
		let req = self.req(&client, "PUT", format!("/vertex/{}", v.id)).body(&body[..]);
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn delete_vertex(&self, id: i64) -> Result<(), Error> {
		let client = Client::new();
		let req = self.req(&client, "DELETE", format!("/vertex/{}", id));
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn get_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<Edge<i64>, Error> {
		let client = Client::new();
		let req = self.req(&client, "GET", format!("/edge/{}/{}/{}", outbound_id, t, inbound_id));
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn set_edge(&self, e: Edge<i64>) -> Result<(), Error> {
		let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
		d.insert("weight".to_string(), JsonValue::F64(e.weight as f64));
		d.insert("properties".to_string(), JsonValue::Object(e.properties));
		let body = serde_json::to_string(&d).unwrap();

		let client = Client::new();
		let req = self.req(&client, "PUT", format!("/edge/{}/{}/{}", e.outbound_id, e.t, e.inbound_id)).body(&body[..]);
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn delete_edge(&self, outbound_id: i64, t: String, inbound_id: i64) -> Result<(), Error> {
		let client = Client::new();
		let req = self.req(&client, "DELETE", format!("/edge/{}/{}/{}", outbound_id, t, inbound_id));
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn get_edge_count(&self, outbound_id: i64, t: String) -> Result<i64, Error> {
		let client = Client::new();
		let req = self.req(&client, "GET", format!("/edge/{}/{}?action=count", outbound_id, t));
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn get_edge_range(&self, outbound_id: i64, t: String, offset: i64, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		let client = Client::new();
		let req = self.req(&client, "GET", format!("/edge/{}/{}?action=position&limit={}&offset={}", outbound_id, t, limit, offset));
		let mut res = req.send().unwrap();
		handle_response(&mut res)
	}

	fn get_edge_time_range(&self, outbound_id: i64, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<Edge<i64>>, Error> {
		let client = Client::new();

		let qp = match (high, low) {
			(Some(high), Some(low)) => format!("&high={}&low={}", high.timestamp(), low.timestamp()),
			(Some(high), None) => format!("&high={}", high.timestamp()),
			(None, Some(low)) => format!("&low={}", low.timestamp()),
			(None, None) => "".to_string(),
		};

		let req = self.req(&client, "GET", format!("/edge/{}/{}?action=time&limit={}{}", outbound_id, t, limit, qp));
		let mut res = req.send().unwrap();
		handle_response(&mut res)
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

fn handle_response<T: Deserialize>(res: &mut Response) -> Result<T, Error> {
	match res.status {
		StatusCode::Ok => {
			let mut payload = String::new();
			res.read_to_string(&mut payload).unwrap();
			let v: T = serde_json::from_str(&payload[..]).unwrap();
			Ok(v)
		},
		StatusCode::NotFound => {
			let msg = get_error_message(res);

			match &msg[..] {
				"Account not found" => Err(Error::AccountNotFound),
				"Vertex does not exist" => Err(Error::VertexDoesNotExist),
				"Edge does not exist" => Err(Error::EdgeDoesNotExist),
				"Metadata does not exist" => Err(Error::MetadataDoesNotExist),
				_ => Err(Error::Unexpected(format!("Unexpected error message: {}", msg)))
			}
		},
		StatusCode::BadRequest => {
			let msg = get_error_message(res);

			match &msg[..] {
				"Weight out of range" => Err(Error::WeightOutOfRange),
				"Limit out of range" => Err(Error::LimitOutOfRange),
				"Offset out of range" => Err(Error::OffsetOutOfRange),
				_ => Err(Error::Unexpected(format!("Unexpected error message: {}", msg)))
			}
		},
		_ => Err(Error::Unexpected(format!("Unexpected return status code: {}", res.status)))
	}
}

fn get_error_message(res: &mut Response) -> String {
	let mut payload = String::new();
	res.read_to_string(&mut payload).unwrap();
	let o: BTreeMap<String, JsonValue> = serde_json::from_str(&payload[..]).unwrap();

	match o.get("error") {
		Some(&JsonValue::String(ref error)) => error.clone(),
		_ => panic!("Could not unpack error message")
	}
}
