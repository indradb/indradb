#![cfg(test)]

pub use super::datastore::PostgresDatastore;
pub use std::env;

pub use datastore::{Datastore, Transaction, DatastoreTestSandbox};
pub use traits::Id;

test_datastore_impl! {
	postgres {
		let connection_string = env::var("DATABASE_URL").expect("Expected a DATABASE_URL");
		let secret = "OME88YorohonzPNWEFsi0dIsouXWqeO$".to_string();
		PostgresDatastore::new(Some(1), connection_string, secret)
	}
}
