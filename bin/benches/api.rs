#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

use common::client_datastore::ClientDatastore;
use common::server;
use std::thread::spawn;

const TEST_PORT: u16 = 27616;

full_bench_impl!({
    spawn(move || server::start(&format!("127.0.0.1:{}", TEST_PORT)));
    ClientDatastore::new(TEST_PORT)
});
