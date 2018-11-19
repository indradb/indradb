#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
#[cfg(feature = "sled-datastore")]
use sled::Error as SledError;
use serde_json::Error as JsonError;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Json(JsonError);
        RocksDb(RocksDbError) #[cfg(feature = "rocksdb-datastore")];
    }
}

error_chain! {
    types {
        ValidationError, ValidationErrorKind, ValidationResultExt, ValidationResult;
    }
}

// Does not use foreign links for sled errors because of `sled::Error`'s use
// of generics
// TODO: better error messaging
#[cfg(feature = "sled-datastore")]
impl<Actual> From<SledError<Actual>> for Error {
    fn from(err: SledError<Actual>) -> Self {
        match err {
            SledError::CasFailed(_) => "atomic operation failed".into(),
            SledError::Unsupported(err) => panic!(err),
            SledError::ReportableBug(err) => panic!(err),
            SledError::Io(err) => format!("i/o error: {}", err).into(),
            SledError::Corruption { at } => format!("database corrupted at {}", at).into()
        }
    }
}
