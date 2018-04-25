#![feature(test)]

extern crate common;
extern crate grpcio;
#[macro_use]
extern crate indradb;
#[macro_use]
extern crate lazy_static;
extern crate test;

use std::sync::Arc;

const TEST_PORT: u16 = 27616;

lazy_static! {
    static ref ENVIRONMENT: Arc<grpcio::Environment> = Arc::new(grpcio::Environment::new(1));
    static ref SERVER: grpcio::Server = common::start_server((*ENVIRONMENT).clone(), "127.0.0.1", TEST_PORT);
}

full_bench_impl!({
    println!("Server: {:?}", *SERVER);
    common::GrpcClientDatastore::new((*ENVIRONMENT).clone(), TEST_PORT)
});
