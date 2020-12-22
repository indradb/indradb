//! The rocksdb datastore implementation.

mod datastore;
mod managers;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!({
    use super::RocksdbDatastore;
    use crate::util::generate_temporary_path;
    RocksdbDatastore::new(&generate_temporary_path(), Some(1)).unwrap()
});

#[cfg(feature = "test-suite")]
full_test_impl!({
    use super::RocksdbDatastore;
    use crate::util::generate_temporary_path;
    RocksdbDatastore::new(&generate_temporary_path(), Some(1)).unwrap()
});

#[cfg(feature = "test-suite")]
#[test]
fn should_repair() {
    use super::RocksdbDatastore;
    use crate::util::generate_temporary_path;

    let path = generate_temporary_path();

    // // Make sure we just initialize the database
    RocksdbDatastore::new(&path, Some(1)).unwrap();

    // Now try to repair
    RocksdbDatastore::repair(&path, Some(1)).unwrap();
}
