//! The in-memory-only datastore implementation. This is the simplest and
//! generally fastest implementation, but it has these drawbacks:
//!
//! * Data is not persisted.
//! * Locking is coarse-grained; only one thread can write to the datastore at
//!   a time.

mod datastore;

pub use self::datastore::{MemoryDatastore, MemoryTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
full_test_impl!(MemoryDatastore::default());
