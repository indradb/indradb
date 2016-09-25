#![cfg(test)]

use super::datastore::PostgresDatastore;
use std::env;

test_datastore_impl! {
	fn datastore() -> PostgresDatastore {
		let connection_string = env::var("DATABASE_URL").expect("Expected a DATABASE_URL");
		let secret = "OME88YorohonzPNWEFsi0dIsouXWqeO$".to_string();
		PostgresDatastore::new(Some(1), connection_string, secret)
	}
}
