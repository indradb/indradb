//! The Sled datastore implementation.

mod datastore;
mod managers;

pub use self::datastore::{SledConfig, SledDatastore, SledTransaction};

mod normal_config {
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
}

mod compression_config {
    #[cfg(feature = "bench-suite")]
    full_bench_impl!({
        use super::SledConfig;
        use crate::util::generate_temporary_path;
        SledConfig::with_compression(None)
            .open(&generate_temporary_path())
            .unwrap()
    });

    #[cfg(feature = "test-suite")]
    full_test_impl!({
        use super::SledConfig;
        use crate::util::generate_temporary_path;
        SledConfig::with_compression(None)
            .open(&generate_temporary_path())
            .unwrap()
    });
}
