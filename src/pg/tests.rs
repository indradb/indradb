#![cfg(test)]

pub use super::datastore::*;
pub use super::super::tests;
pub use std::env;

fn datastore() -> PostgresDatastore {
    let connection_string = env::var("DATABASE_URL").expect("Expected a DATABASE_URL");
    let secret = "OME88YorohonzPNWEFsi0dIsouXWqeO$".to_string();
    PostgresDatastore::new(Some(1), connection_string, secret)
}

test_account_impl!(datastore());
test_transaction_impl!(datastore());
test_metadata_impl!(datastore());
