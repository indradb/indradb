//! The Sled datastore implementation.

mod datastore;
mod managers;

pub use self::datastore::{SledDatastore, SledTransaction, SledOptions};

mod normal_config {
    #[cfg(feature = "bench-suite")]
    full_bench_impl!({
        use super::{SledDatastore, SledOptions};
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

mod no_compression_config {
    #[cfg(feature = "bench-suite")]
    full_bench_impl!({
        use super::{SledDatastore, SledOptions};
        use crate::util::generate_temporary_path;

        let mut opts = SledOptions::default();
        opts.disable_compression = true;

        SledDatastore::new_with_options(&generate_temporary_path(), opts).unwrap()
    });

    #[cfg(feature = "test-suite")]
    full_test_impl!({
        use super::{SledDatastore, SledOptions};
        use crate::util::generate_temporary_path;

        let mut opts = SledOptions::default();
        opts.disable_compression = true;

        SledDatastore::new_with_options(&generate_temporary_path(), opts).unwrap()
    });
}

