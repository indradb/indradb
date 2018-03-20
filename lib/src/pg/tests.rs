#![cfg(test)]

pub use super::datastore::PostgresDatastore;
pub use super::super::tests;
pub use std::env;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

full_test_impl!({
    let connection_string = env::var("TEST_POSTGRES_URL").expect("Expected a TEST_POSTGRES_URL");

    START.call_once(|| {
        PostgresDatastore::create_schema(connection_string.clone()).unwrap();
    });

    PostgresDatastore::new(Some(1), connection_string).unwrap()
});
