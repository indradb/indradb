#![feature(test)]

extern crate test;
extern crate braid;

#[macro_use]
mod common;

pub use braid::RocksdbDatastore;
pub use braid::util::generate_random_secret;
pub use std::env;
use std::path::Path;
pub use test::Bencher;
pub use braid::tests;

fn datastore() -> RocksdbDatastore {
    // RocksDB can only have one connection open to a database at a time.
    // Because Rust may run the tests in parallel, we need to add a random
    // secret to the test database directory to ensure we can have multiple
    // different database connections open simultaneously.
	let test_rdb_directory = env::var("TEST_RDB_DIRECTORY").unwrap_or("/tmp/test-rdb".to_string());
    let unique = generate_random_secret();
    let path = Path::new(&test_rdb_directory[..]).join(unique);

    let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or("512".to_string());
    let max_open_files = max_open_files_str.parse::<i32>().unwrap();

    RocksdbDatastore::new(path.to_str().unwrap(), Some(max_open_files), false).unwrap()
}

bench_transaction_impl!(datastore());
