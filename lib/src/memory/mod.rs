//! The in-memory-only datastore implementation. This is the simplest and
//! generally fastest implementation, but it has these drawbacks:
//!
//! * Data is not persisted.
//! * Transaction changes cannot be rolled back on error.
//! * Locking is coarse-grained; only one thread can write to the datastore at
//!   a time. Consequently, this may actually perform worse on highly
//!   concurrent write-heavy workloads.

mod datastore;

#[cfg(all(feature = "bench-suite", feature = "nightly"))]
mod benches;

#[cfg(feature = "test-suite")]
mod tests;

pub use self::datastore::{MemoryDatastore, MemoryTransaction};
