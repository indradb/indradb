#![cfg(test)]

#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use] extern crate nutrino;
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate regex;
extern crate hyper;
extern crate uuid;

mod common;

use std::io::prelude::*;
use std::fs::File;
use uuid::Uuid;
use hyper::client::Client;
use hyper::status::StatusCode;
pub use rand::{thread_rng, Rng};
pub use regex::Regex;
pub use serde_json::Value as JsonValue;
pub use common::{request, create_account, delete_account, response_to_error_message};

lazy_static! {
	static ref RUNTIME_ERROR_RESPONSE_PATTERN: Regex = Regex::new(r"Script failed: Runtime").unwrap();
    static ref RUNTIME_ERROR_EXPECTED_PATTERN: Regex = Regex::new(r"-- error: runtime").unwrap();
    static ref OK_EXPECTED_PATTERN: Regex = Regex::new(r"-- ok: (.+)$").unwrap();
}

describe! script_tests {
	before_each {
		let email_rand = thread_rng().gen_ascii_chars().take(10).collect::<String>();
		let email = format!("script-tests-{}@nutrino.com", email_rand);
		let (account_id, secret) = create_account(email).unwrap();
	}

	after_each {
		delete_account(account_id).unwrap();
	}

	it "should run create_vertex_bad_type" {
		run_script(account_id, secret, "create_vertex_bad_type")
	}

	it "should run create_vertex" {
		run_script(account_id, secret, "create_vertex")
	}

	it "should run delete_edge" {
		run_script(account_id, secret, "delete_edge")
	}

	it "should run delete_vertex" {
		run_script(account_id, secret, "delete_vertex")
	}

	it "should run get_edge_count" {
		run_script(account_id, secret, "get_edge_count")
	}

	it "should run get_edge_range_bad_limit" {
		run_script(account_id, secret, "get_edge_range_bad_limit")
	}

	it "should run get_edge_range_bad_offset" {
		run_script(account_id, secret, "get_edge_range_bad_offset")
	}

	it "should run get_edge_range" {
		run_script(account_id, secret, "get_edge_range")
	}

	it "should run get_edge_time_range_bad_high" {
		run_script(account_id, secret, "get_edge_time_range_bad_high")
	}

	it "should run get_edge_time_range_bad_low" {
		run_script(account_id, secret, "get_edge_time_range_bad_low")
	}

	it "should run get_edge_time_range" {
		run_script(account_id, secret, "get_edge_time_range")
	}

	it "should run get_vertex_bad_id" {
		run_script(account_id, secret, "get_vertex_bad_id")
	}

	it "should run get_vertex" {
		run_script(account_id, secret, "get_vertex")
	}

	it "should run global_metadata" {
		run_script(account_id, secret, "global_metadata")
	}

	it "should run local_metadata" {
		run_script(account_id, secret, "local_metadata")
	}

	it "should run return_array" {
		run_script(account_id, secret, "return_array")
	}

	it "should run return_boolean" {
		run_script(account_id, secret, "return_boolean")
	}

	it "should run return_function" {
		run_script(account_id, secret, "return_function")
	}

	it "should run return_int" {
		run_script(account_id, secret, "return_int")
	}

	it "should run return_nil" {
		run_script(account_id, secret, "return_nil")
	}

	it "should run return_number" {
		run_script(account_id, secret, "return_number")
	}

	it "should run return_obj_with_bad_values" {
		run_script(account_id, secret, "return_obj_with_bad_values")
	}

	it "should run return_obj" {
		run_script(account_id, secret, "return_obj")
	}

	it "should run return_string" {
		run_script(account_id, secret, "return_string")
	}

	it "should run set_and_get_edge" {
		run_script(account_id, secret, "set_and_get_edge")
	}

	it "should run set_edge_bad_weight" {
		run_script(account_id, secret, "set_edge_bad_weight")
	}

	it "should run set_vertex" {
		run_script(account_id, secret, "set_vertex")
	}
}

pub fn run_script(account_id: Uuid, secret: String, name: &str) {
	let mut file = File::open(format!("test_scripts/{}.lua", name)).unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	let client = Client::new();
	let req = request(&client, 8000, account_id, secret, "POST", format!("/script/{}.lua", name));
	let mut res = req.send().unwrap();

	if RUNTIME_ERROR_EXPECTED_PATTERN.is_match(&contents[..]) {
		let error = response_to_error_message(&mut res);

		if res.status != StatusCode::InternalServerError {
			panic!("Unexpected status code: {}", res.status);
		} else if !RUNTIME_ERROR_RESPONSE_PATTERN.is_match(&error[..]) {
			panic!("Unexpected error response: {}", error);
		}
	} else if res.status == StatusCode::Ok {
		if let Some(cap) = OK_EXPECTED_PATTERN.captures(&contents[..]) {
			let s = cap.at(1).unwrap();
			let expected_result: JsonValue = serde_json::from_str(&s[..]).unwrap();

			let mut payload = String::new();
			res.read_to_string(&mut payload).unwrap();
			let actual_result: JsonValue = serde_json::from_str(&payload[..]).unwrap();
			assert_eq!(expected_result, actual_result)
		}
	} else {
		let mut payload = String::new();
		res.read_to_string(&mut payload).unwrap();
		panic!("Unexpected status code: {}", res.status)
	}
}
