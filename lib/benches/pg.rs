#![feature(test)]
#![cfg(feature = "postgres-datastore")]

extern crate indradb;
extern crate test;

#[macro_use]
mod common;

pub use indradb::PostgresDatastore;
pub use indradb::tests;
pub use std::env;
use std::sync::{Once, ONCE_INIT};
pub use test::Bencher;

static START: Once = ONCE_INIT;

fn datastore() -> PostgresDatastore {
    let connection_string = env::var("TEST_POSTGRES_URL").expect("Expected a TEST_POSTGRES_URL");

    START.call_once(|| {
        PostgresDatastore::create_schema(connection_string.clone()).unwrap();
    });

    PostgresDatastore::new(Some(1), connection_string, false).unwrap()
}

bench_transaction_impl!(datastore());
