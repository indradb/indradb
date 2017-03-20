#![cfg(test)]

pub use super::datastore::*;
pub use datastore::*;
pub use std::env;

test_datastore_impl!({
	let connection_string = env::var("DATABASE_URL").expect("Expected a DATABASE_URL");
    let secret = "OME88YorohonzPNWEFsi0dIsouXWqeO$".to_string();
    PostgresDatastore::new(Some(1), connection_string, secret)
});
