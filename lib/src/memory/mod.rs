//! The in-memory datastore implementation. This is the simplest and generally
//! fastest implementation, but it has some drawbacks:
//!
//! * No support for graphs larger than what can be stored in-memory.
//! * No transactional guarantees.
//! * Data is only persisted to disk when explicitly requested.

mod datastore;

pub use self::datastore::{MemoryDatastore, MemoryTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
full_test_impl!(MemoryDatastore::default());
