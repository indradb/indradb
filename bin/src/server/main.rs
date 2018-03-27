#![recursion_limit = "1024"]

extern crate chan_signal;
extern crate chrono;
extern crate common;
extern crate core;
#[macro_use]
extern crate error_chain;
extern crate grpcio;
extern crate indradb;
extern crate libc;
extern crate protobuf;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate uuid;
extern crate futures;

mod converters;
mod edges;
mod errors;
mod metadata;
mod queries;
mod request;
mod response;
mod service;
mod service_grpc;
mod vertices;

use std::sync::Arc;
use futures::future::Future;

fn main() {
    let env = Arc::new(grpcio::Environment::new(4));
    let instance = service::IndraDbService::new();
    let service = service_grpc::create_indra_db(instance);
    let mut server = grpcio::ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 27615)
        .build()
        .unwrap();

    server.start();
    
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    
    let signal = chan_signal::notify(&[chan_signal::Signal::INT, chan_signal::Signal::TERM]);
    signal.recv().unwrap();
    let _ = server.shutdown().wait();
}
