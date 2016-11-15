#![cfg(test)]

pub use super::datastore::*;
pub use datastore::*;
pub use super::super::util::generate_random_secret;
pub use std::env;
pub use traits::Id;
use std::path::{Path, PathBuf};

pub fn datastore() -> RocksdbDatastore {
	let test_rdb_directory = env::var("TEST_RDB_DIRECTORY").unwrap_or("/tmp/test-rdb".to_string());
	let unique = generate_random_secret();
	let path = Path::new(&test_rdb_directory[..]).join(format!("rdb-test-{}", unique)).to_str().unwrap().to_string();
	RocksdbDatastore::new(path).unwrap()
}

test_account_management_impl! {
	test_rocksdb_account_management datastore()
}

test_transaction_impl! {
	test_rocksdb_transaction datastore()
}

bench_transaction_impl! {
	bench_rocksdb_transaction datastore()
}

test_metadata_impl! {
	test_rocksdb_metadata datastore()
}
