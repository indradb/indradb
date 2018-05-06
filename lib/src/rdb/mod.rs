//! The rocksdb datastore implementation.
//!
//! This should be substantially faster than the postgres implementation -
//! especially on SSDs - however it comes at a cost:
//!
//! * Transactions cannot be rolled back on error.
//! * Only a single server process can run on the same datastore at the same
//!   time.

mod keys;
mod instance;
mod managers;
mod datastore;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};

#[cfg(feature = "bench-suite")]
indradb_full_bench_impl!(instance::datastore());

#[cfg(feature = "test-suite")]
indradb_full_test_impl!(instance::datastore());
