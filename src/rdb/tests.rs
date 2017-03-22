#![cfg(test)]

pub use super::datastore::*;
pub use datastore::*;
pub use datastore::tests::*;
pub use super::super::util::generate_random_secret;
pub use std::env;
use std::path::Path;

fn datastore() -> RocksdbDatastore {
	let test_rdb_directory = env::var("TEST_RDB_DIRECTORY").unwrap_or("/tmp/test-rdb".to_string());
    let unique = generate_random_secret();
    let path = Path::new(&test_rdb_directory[..]).join(unique);

    let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or("512".to_string());
    let max_open_files = max_open_files_str.parse::<i32>().unwrap();

    RocksdbDatastore::new(path.to_str().unwrap(), Some(max_open_files)).unwrap()
}

test_account_impl!(datastore());
test_transaction_impl!(datastore());
test_metadata_impl!(datastore());
