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
#[macro_use] extern crate nutrino;
#[macro_use] extern crate lua;
#[macro_use] extern crate hyper;
#[macro_use] extern crate common;
#[macro_use] extern crate lazy_static;

mod http;
mod scripts;
mod util;
mod datastore;

fn main() {
	http::start(8000);
}
