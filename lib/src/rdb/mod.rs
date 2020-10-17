//! The rocksdb datastore implementation.

mod datastore;
mod managers;

#[cfg(feature = "test-suite")]
mod tests;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};

mod normal_config {
    #[cfg(feature = "bench-suite")]
    full_bench_impl!({
        use super::RocksdbDatastore;
        use crate::util::generate_temporary_path;
        RocksdbDatastore::new(&generate_temporary_path(), Some(1), false).unwrap()
    });

    #[cfg(feature = "test-suite")]
    full_test_impl!({
        use super::RocksdbDatastore;
        use crate::util::generate_temporary_path;
        RocksdbDatastore::new(&generate_temporary_path(), Some(1), false).unwrap()
    });
}

mod bulk_load_optimized_config {
    #[cfg(feature = "bench-suite")]
    full_bench_impl!({
        use super::RocksdbDatastore;
        use crate::util::generate_temporary_path;
        RocksdbDatastore::new(&generate_temporary_path(), Some(1), true).unwrap()
    });

    #[cfg(feature = "test-suite")]
    full_test_impl!({
        use super::RocksdbDatastore;
        use crate::util::generate_temporary_path;
        RocksdbDatastore::new(&generate_temporary_path(), Some(1), true).unwrap()
    });
}
