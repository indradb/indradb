// Dead code detection is inaccurate because this module is only used in
// conditionally compiled macros
#![allow(dead_code)]

use super::RocksdbDatastore;
use std::env;
use std::path::Path;
use util::generate_random_secret;

/// Gets RocksDB options from env vars. Used for testing/benchmarking.
pub fn get_options() -> (String, i32) {
    // RocksDB can only have one connection open to a database at a time.
    // Because Rust may run the tests in parallel, we need to add a random
    // secret to the test database directory to ensure we can have multiple
    // different database connections open simultaneously.
    // TODO: do not hardcode the temp directory, to support non-POSIX
    // environments
    let unique = generate_random_secret(8);
    let path = Path::new("/tmp/test-rdb").join(unique);

    let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or_else(|_| "512".to_string());
    let max_open_files = max_open_files_str.parse::<i32>().unwrap();

    (path.to_str().unwrap().to_string(), max_open_files)
}

/// Creates an instance of a rdb datastore. Used for testing/benchmarking.
pub fn datastore() -> RocksdbDatastore {
    let (path, max_open_files) = get_options();
    RocksdbDatastore::new(&path[..], Some(max_open_files)).unwrap()
}
