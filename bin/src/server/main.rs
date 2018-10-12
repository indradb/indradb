#![recursion_limit = "1024"]

extern crate common;
extern crate chan_signal;
extern crate chrono;
extern crate core;
extern crate futures;
extern crate grpcio;
extern crate indradb;
extern crate libc;
extern crate protobuf;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use futures::future::Future;

use std::env;
use std::sync::Arc;

fn main() {
    let env = Arc::new(grpcio::Environment::new(1));

    let port_str = env::var("PORT").unwrap_or_else(|_| "27615".to_string());
    let port = port_str
        .parse::<u16>()
        .expect("Could not parse environment variable `PORT`");

    let mut server = common::start_server(env, "127.0.0.1", port);

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    let signal = chan_signal::notify(&[chan_signal::Signal::INT, chan_signal::Signal::TERM]);
    signal.recv().unwrap();
    let _ = server.shutdown().wait();
}
