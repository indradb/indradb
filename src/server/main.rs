#[macro_use]
extern crate serde_derive;
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
#[macro_use]
extern crate nutrino;
#[macro_use]
extern crate lua;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate common;
#[macro_use]
extern crate lazy_static;

mod http;
mod script;
mod util;
mod statics;

use std::env;

/// App for exposing a RESTful API for a datastore
fn main() {
    let port_str = env::var("PORT").unwrap_or("8000".to_string());
    let port = port_str.parse::<u16>().expect("Could not parse environment variable `PORT`");
    http::start(port);
}
