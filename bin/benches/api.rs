#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

use common::client_datastore::ClientDatastore;

const TEST_PORT: u16 = 27616;

full_bench_impl!({
    ClientDatastore::new(TEST_PORT, "memory://".to_string())
});
