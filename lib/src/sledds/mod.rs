//! The Sled datastore implementation.

mod datastore;
mod managers;

#[cfg(feature = "test-suite")]
mod tests;

pub use self::datastore::{SledDatastore, SledTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!({
    use super::SledDatastore;
    use crate::util::generate_temporary_path;
    SledDatastore::new(&generate_temporary_path()).unwrap()
});

#[cfg(feature = "test-suite")]
full_test_impl!({
    use super::SledDatastore;
    use crate::util::generate_temporary_path;
    SledDatastore::new(&generate_temporary_path()).unwrap()
});
