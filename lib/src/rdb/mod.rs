//! The rocksdb datastore implementation.

mod datastore;
mod instance;
mod keys;
mod managers;

#[cfg(feature = "test-suite")]
mod tests;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!(instance::datastore());

#[cfg(feature = "test-suite")]
full_test_impl!(instance::datastore());
