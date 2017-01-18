#![cfg(test)]

#![feature(plugin, test, proc_macro)]
#![plugin(stainless)]

#[macro_use]
extern crate serde_derive;
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
use std::io::Read;

use hyper::client::{Client, RequestBuilder};
use serde_json::value::Value as JsonValue;
use chrono::{DateTime, UTC};
use serde::Deserialize;
use hyper::status::StatusCode;
use hyper::client::response::Response;
use uuid::Uuid;

pub use nutrino::*;
pub use common::{HttpDatastore, HttpTransaction, request, response_to_error_message};

pub struct RestTransaction {
    port: i32,
    account_id: Uuid,
    secret: String,
}

impl RestTransaction {
    fn request<'a>(&self, client: &'a Client, method_str: &str, path: String, query_pairs: Vec<(&str, String)>) -> RequestBuilder<'a> {
        return request(
            client,
            self.port,
            self.account_id,
            self.secret.clone(),
            method_str,
            path,
            query_pairs
        );
    }

    fn build_time_range_query_pairs(&self, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>) -> Vec<(&str, String)> {
        let mut query_pairs = vec![];

        if let Some(high) = high {
            query_pairs.push(("high", high.to_rfc3339()));
        }

        if let Some(low) = low {
            query_pairs.push(("low", low.to_rfc3339()));
        }

        query_pairs
    }
}

impl HttpTransaction<RestTransaction> for RestTransaction {
    fn new(port: i32, account_id: Uuid, secret: String) -> Self {
        RestTransaction {
            port: port,
            account_id: account_id,
            secret: secret,
        }
    }
}

impl Transaction<Uuid> for RestTransaction {
    fn get_vertex(&self, id: Uuid) -> Result<Vertex<Uuid>, Error> {
        let client = Client::new();
        let req = self.request(&client, "GET", format!("/vertex/{}", id), vec![]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
        d.insert("type".to_string(), JsonValue::String(t.0));
        let body = serde_json::to_string(&d).unwrap();

        let client = Client::new();
        let req = self.request(&client, "POST", "/vertex".to_string(), vec![]).body(&body[..]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn set_vertex(&self, v: Vertex<Uuid>) -> Result<(), Error> {
        let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
        d.insert("type".to_string(), JsonValue::String(v.t.0));
        let body = serde_json::to_string(&d).unwrap();

        let client = Client::new();
        let req = self.request(&client, "PUT", format!("/vertex/{}", v.id), vec![]).body(&body[..]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
        let client = Client::new();
        let req = self.request(&client, "DELETE", format!("/vertex/{}", id), vec![]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<Edge<Uuid>, Error> {
        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            format!("/edge/{}/{}/{}", outbound_id, t.0, inbound_id),
            vec![]
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn set_edge(&self, e: Edge<Uuid>) -> Result<(), Error> {
        let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();
        d.insert("weight".to_string(), JsonValue::F64(e.weight.0 as f64));
        let body = serde_json::to_string(&d).unwrap();

        let client = Client::new();
        let req = self.request(
            &client,
            "PUT",
            format!("/edge/{}/{}/{}", e.outbound_id, e.t.0, e.inbound_id),
            vec![]
        ).body(&body[..]);
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn delete_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<(), Error> {
        let client = Client::new();
        let req = self.request(
            &client,
            "DELETE",
            format!("/edge/{}/{}/{}", outbound_id, t.0, inbound_id),
            vec![]
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_edge_count(&self, outbound_id: Uuid, t: Type) -> Result<u64, Error> {
        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            format!("/edge/{}/{}/_", outbound_id, t.0),
            vec![("action", "count".to_string())]
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_edge_range(&self, outbound_id: Uuid, t: Type, offset: u64, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            format!("/edge/{}/{}/_", outbound_id, t.0),
            vec![
                ("action", "position".to_string()),
                ("limit", format!("{}", limit)),
                ("offset", format!("{}", offset)),
            ]
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_edge_time_range(&self, outbound_id: Uuid, t: Type, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        let mut query_pairs = vec![
            ("action", "time".to_string()),
            ("limit", format!("{}", limit))
        ];

        query_pairs.extend(self.build_time_range_query_pairs(high, low));

        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            format!("/edge/{}/{}/_", outbound_id, t.0),
            query_pairs
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_reversed_edge_count(&self, inbound_id: Uuid, t: Type) -> Result<u64, Error> {
        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            format!("/edge/_/{}/{}", inbound_id, t.0),
            vec![("action", "count".to_string())]
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_reversed_edge_range(&self, inbound_id: Uuid, t: Type, offset: u64, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            format!("/edge/_/{}/{}", inbound_id, t.0),
            vec![
                ("action", "position".to_string()),
                ("limit", format!("{}", limit)),
                ("offset", format!("{}", offset))
            ]
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
    }

    fn get_reversed_edge_time_range(&self, inbound_id: Uuid, t: Type, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
        let mut query_pairs = vec![
            ("action", "time".to_string()),
            ("limit", format!("{}", limit))
        ];

        query_pairs.extend(self.build_time_range_query_pairs(high, low));

        let client = Client::new();
        let req = self.request(
            &client,
            "GET",
            format!("/edge/_/{}/{}", inbound_id, t.0),
            query_pairs
        );
        let mut res = req.send().unwrap();
        response_to_obj(&mut res)
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
	test_rest_transaction {
	    HttpDatastore::<RestTransaction, RestTransaction>::new(8000)
	}
}

bench_transaction_impl! {
	bench_rest_transaction {
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
        }
        _ => {
            let message = response_to_error_message(res);
            Err(Error::description_to_error(&message[..]))
        }
    }
}
