//! The rocksdb datastore implementation.

mod keys;
mod instance;
mod managers;
mod datastore;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};

#[cfg(feature = "bench-suite")]
indradb_full_bench_impl!(instance::datastore());

#[cfg(feature = "test-suite")]
indradb_full_test_impl!(instance::datastore());
