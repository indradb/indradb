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
extern crate braid;
extern crate lua;
extern crate hyper;
extern crate common;
#[macro_use]
extern crate lazy_static;

mod http;
mod script;
mod util;
mod statics;

use std::env;

/// App for exposing a `RESTful` API for a datastore
fn main() {
    let port_str = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port = port_str.parse::<u16>().expect("Could not parse environment variable `PORT`");
    http::start(port);
}
