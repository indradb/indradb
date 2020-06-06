#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;
use std::result::Result as StdResult;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "json error: {}", inner)]
    Json { inner: JsonError },
    #[cfg(feature = "rocksdb-datastore")]
    #[fail(display = "rocksdb error: {}", inner)]
    Rocksdb { inner: RocksDbError },
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json { inner: err }
    }
}

#[cfg(feature = "rocksdb-datastore")]
impl From<RocksDbError> for Error {
    fn from(err: RocksDbError) -> Self {
        Error::Rocksdb { inner: err }
    }
}

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Fail)]
pub enum ValidationError {
    #[fail(display = "invalid value")]
    InvalidValue,
    #[fail(display = "value too long")]
    ValueTooLong,
}

pub type ValidationResult<T> = StdResult<T, ValidationError>;
