#![recursion_limit = "1024"]

extern crate common;
extern crate chrono;
extern crate core;
extern crate futures;
extern crate indradb;
extern crate libc;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use std::env;

fn main() {
    let port_str = env::var("PORT").unwrap_or_else(|_| "27615".to_string());
    let port = port_str
        .parse::<u16>()
        .expect("Could not parse environment variable `PORT`");
    let binding = format!("127.0.0.1:{}", port);

    let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| "memory://".to_string());

    common::server::start(&binding, &connection_string).expect("Expected to be able to start the server");
}
