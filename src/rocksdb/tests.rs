#![cfg(test)]

pub use super::datastore::*;
pub use datastore::*;
pub use super::super::util::generate_random_secret;
pub use std::env;
pub use traits::Id;

pub fn datastore() -> RocksdbDatastore {
	let unique = generate_random_secret();
	RocksdbDatastore::new(format!("./rdb-test-{}", unique)).unwrap()
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
