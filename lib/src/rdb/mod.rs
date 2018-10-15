//! The rocksdb datastore implementation.

mod datastore;
mod keys;
mod managers;

#[cfg(feature = "test-suite")]
mod tests;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!({
    use std::path::Path;
    use util::generate_random_secret;

    // TODO: do not hardcode the temp directory to support non-POSIX
    let path = Path::new("/tmp/test-rdb").join(generate_random_secret(8));
    RocksdbDatastore::new(path.to_str().unwrap(), Some(1)).unwrap()
});

#[cfg(feature = "test-suite")]
full_test_impl!({
    use std::path::Path;
    use util::generate_random_secret;

    // TODO: do not hardcode the temp directory to support non-POSIX
    let path = Path::new("/tmp/test-rdb").join(generate_random_secret(8));
    RocksdbDatastore::new(path.to_str().unwrap(), Some(1)).unwrap()
});
