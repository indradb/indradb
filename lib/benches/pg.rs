#![feature(test)]
#![cfg(feature = "postgres-datastore")]
#![cfg(feature = "bench-suite")]

#[macro_use]
extern crate indradb;

pub use indradb::PostgresDatastore;
pub use std::env;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

fn datastore() -> PostgresDatastore {
    let connection_string = env::var("TEST_POSTGRES_URL").expect("Expected a TEST_POSTGRES_URL");

    START.call_once(|| {
        PostgresDatastore::create_schema(connection_string.clone()).unwrap();
    });

    PostgresDatastore::new(Some(1), connection_string).unwrap()
}

bench_transaction_impl!(datastore());
