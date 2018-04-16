// Dead code detection is inaccurate because this module is only used in
// conditionally compiled macros
#![allow(dead_code)]

use super::PostgresDatastore;
use std::env;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

/// Creates an instance of a pg datastore. Used for testing/benchmarking.
pub fn datastore() -> PostgresDatastore {
    let connection_string = env::var("TEST_POSTGRES_URL").expect("Expected a TEST_POSTGRES_URL");

    START.call_once(|| {
        PostgresDatastore::create_schema(connection_string.clone()).unwrap();
    });

    PostgresDatastore::new(Some(1), connection_string).unwrap()
}
