#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate chrono;
extern crate core;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate urlencoded;
extern crate libc;
extern crate regex;
extern crate uuid;
#[macro_use] extern crate nutrino;
#[macro_use] extern crate lua;
#[macro_use] extern crate hyper;
#[macro_use] extern crate common;
#[macro_use] extern crate lazy_static;

mod http;
mod script;
mod util;
mod datastore;

use std::env;

fn main() {
	let port: u16 = match env::var("PORT") {
		Ok(s) => {
			match s.parse::<u16>() {
				Ok(val) => val,
				Err(_) => panic!("Could not parse environment variable `PORT`") 
			}
		},
		Err(_) => 8000
	};

	http::start(port);
}
