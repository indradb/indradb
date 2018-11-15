#![recursion_limit = "1024"]

extern crate chrono;
extern crate common;
extern crate core;
extern crate futures;
extern crate indradb;
extern crate libc;
extern crate num_cpus;
extern crate regex;
extern crate serde_json;
extern crate uuid;

use std::env;

const DEFAULT_PORT: u16 = 27615;

fn main() {
    let port = match env::var("PORT") {
        Ok(value) => value
            .parse::<u16>()
            .expect("Could not parse environment variable `PORT`"),
        Err(_) => DEFAULT_PORT,
    };

    let binding = format!("127.0.0.1:{}", port);

    let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| "memory://".to_string());

    let worker_count = match env::var("WORKER_COUNT") {
        Ok(value) => value
            .parse::<usize>()
            .expect("Could not parse environment variable `WORKER_COUNT`"),
        Err(_) => num_cpus::get() * 2,
    };

    common::server::start(&binding, &connection_string, worker_count).expect("Expected to be able to start the server");
}
