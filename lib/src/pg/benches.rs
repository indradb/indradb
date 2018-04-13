use super::PostgresDatastore;
use std::env;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

fn datastore() -> PostgresDatastore {
    let connection_string = env::var("TEST_POSTGRES_URL").expect("Expected a TEST_POSTGRES_URL");

    START.call_once(|| {
        PostgresDatastore::create_schema(connection_string.clone()).unwrap();
    });

    PostgresDatastore::new(Some(1), connection_string).unwrap()
}

full_bench_impl!(datastore());
