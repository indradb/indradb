#![cfg(test)]

pub use super::datastore::*;
pub use datastore::*;
pub use std::env;
pub use traits::Id;

pub fn datastore() -> PostgresDatastore {
	let connection_string = env::var("DATABASE_URL").expect("Expected a DATABASE_URL");
	let secret = "OME88YorohonzPNWEFsi0dIsouXWqeO$".to_string();
	PostgresDatastore::new(Some(1), connection_string, secret)
}

test_account_management_impl! {
	postgres_account_management datastore()
}

test_transaction_impl! {
	postgres_transaction datastore()
}

test_metadata_impl! {
	postgres_metadata datastore()
}
