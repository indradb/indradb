#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate nutrino;
#[macro_use]
extern crate lazy_static;
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
    static ref OK_EXPECTED_PATTERN: Regex = Regex::new(r"-- ok: (.+)$").unwrap();
}

macro_rules! test_script {
	($name:ident) => (
		#[test]
		fn $name() {
			let email_rand = thread_rng().gen_ascii_chars().take(10).collect::<String>();
			let email = format!("script-tests-{}@nutrino.com", email_rand);
			let (account_id, secret) = create_account(email).unwrap();
			run_script(account_id, secret, stringify!($name));
		}
	)
}

fn run_script(account_id: Uuid, secret: String, name: &str) {
    let mut file = File::open(format!("test_scripts/{}.lua", name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let client = Client::new();
    let req = request(
		&client,
		8000,
		account_id,
		secret,
		"POST",
		format!("/script/{}.lua", name),
		vec![]
	);
	let mut res = req.send().unwrap();

    let mut payload = String::new();
    res.read_to_string(&mut payload).unwrap();

    if res.status == StatusCode::Ok {
        if let Some(cap) = OK_EXPECTED_PATTERN.captures(&contents[..]) {
            let s = cap.get(1).unwrap().as_str();
            let expected_result: JsonValue = serde_json::from_str(s).unwrap();
            let actual_result: JsonValue = serde_json::from_str(&payload[..]).unwrap();
            assert_eq!(expected_result, actual_result)
        }
    } else {
        panic!("Unexpected status code: {} - payload: {}", res.status, payload)
    }
}


test_script!(create_vertex_bad_type);
test_script!(create_vertex);
test_script!(delete_edge);
test_script!(delete_vertex);
test_script!(get_edge_count);
test_script!(get_edge_range_bad_limit);
test_script!(get_edge_range_bad_offset);
test_script!(get_edge_range);
test_script!(get_edge_time_range_bad_high);
test_script!(get_edge_time_range_bad_low);
test_script!(get_edge_time_range);
test_script!(get_vertex_bad_id);
test_script!(get_vertex_range);
test_script!(get_vertex);
test_script!(global_metadata);
test_script!(account_metadata);
test_script!(vertex_metadata);
test_script!(edge_metadata);
test_script!(return_array);
test_script!(return_boolean);
test_script!(return_int);
test_script!(return_nil);
test_script!(return_number);
test_script!(return_obj);
test_script!(return_string);
test_script!(set_and_get_edge);
test_script!(set_vertex);
