//! The postgres datastore implementation.
//!
//! This should generally be considered by far the slowest implementation,
//! however it provides a few major benefits:
//!
//! * Transaction changes can be rolled back on error.
//! * Multiple `IndradB` server processes can run on the same datastore at the
//!   same time.
//! * You can use all of the postgres tooling to poke around at the results.
//! * Thanks to foreign keys et al., this is probably less buggy than other
//!   implementations.

mod datastore;
mod schema;
mod util;

#[cfg(all(feature = "bench-suite", feature = "nightly"))]
mod benches;

#[cfg(feature = "test-suite")]
mod tests;

pub use self::datastore::{PostgresDatastore, PostgresTransaction};
