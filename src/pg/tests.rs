#![cfg(test)]

pub use super::datastore::PostgresDatastore;
pub use super::super::tests;
pub use std::env;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;

fn datastore() -> PostgresDatastore {
    let connection_string = env::var("TEST_POSTGRES_URL").expect("Expected a TEST_POSTGRES_URL");

    START.call_once(|| {
        PostgresDatastore::create_schema(connection_string.clone()).unwrap();
    });

    let secret = "OME88YorohonzPNWEFsi0dIsouXWqeO$".to_string();
    PostgresDatastore::new(Some(1), connection_string, secret, false)
}

test_account_impl!(datastore());
test_transaction_impl!(datastore());
test_metadata_impl!(datastore());
