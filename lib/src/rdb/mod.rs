//! The rocksdb datastore implementation.
//!
//! This should be substantially faster than the postgres implementation -
//! especially on SSDs - however it comes at a cost:
//!
//! * Transactions cannot be rolled back on error.
//! * Only a single server process can run on the same datastore at the same
//!   time.

mod keys;
mod managers;
mod datastore;
mod tests;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};
