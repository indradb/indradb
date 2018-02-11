extern crate chrono;
extern crate common;
extern crate core;
#[macro_use]
extern crate crossbeam_channel;
extern crate hyper;
extern crate indradb;
extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate num_cpus;
extern crate regex;
extern crate rlua;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate urlencoded;
extern crate uuid;

mod http;
mod script;
mod util;
mod statics;

use std::env;

/// App for exposing a `RESTful` API for a datastore
fn main() {
    let port_str = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port = port_str
        .parse::<u16>()
        .expect("Could not parse environment variable `PORT`");
    http::start(port);
}
