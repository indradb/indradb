//! The rocksdb datastore implementation.

mod datastore;
mod keys;
mod managers;

#[cfg(feature = "test-suite")]
mod tests;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!({
    use util::generate_temporary_path;
    RocksdbDatastore::new(&generate_temporary_path(), Some(1)).unwrap()
});

#[cfg(feature = "test-suite")]
full_test_impl!({
    use util::generate_temporary_path;
    RocksdbDatastore::new(&generate_temporary_path(), Some(1)).unwrap()
});
