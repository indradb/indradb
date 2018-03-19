#![recursion_limit = "1024"]

extern crate chan_signal;
extern crate chrono;
extern crate common;
extern crate core;
extern crate grpcio;
extern crate indradb;
extern crate libc;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate uuid;
extern crate protobuf;
extern crate futures;

mod grpc;
mod service;

use std::sync::Arc;
use futures::future::Future;

fn main() {
    let env = Arc::new(grpcio::Environment::new(4));
    let instance = service::IndraDbService::new();
    let service = grpc::create_indra_db(instance);
    let mut server = grpcio::ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 27615)
        .build()
        .unwrap();

    server.start();
    
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    
    let _ = chan_signal::notify(&[chan_signal::Signal::INT, chan_signal::Signal::TERM]);
    let _ = server.shutdown().wait();
}
