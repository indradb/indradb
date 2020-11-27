#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;
#[cfg(feature = "sled-datastore")]
use sled::{Error as SledError, transaction::TransactionError as SledTransactionError};
use std::result::Result as StdResult;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "json error: {}", inner)]
    Json { inner: JsonError },
    
    #[cfg(feature = "rocksdb-datastore")]
    #[fail(display = "rocksdb error: {}", inner)]
    Rocksdb { inner: RocksDbError },
    
    #[cfg(feature = "sled-datastore")]
    #[fail(display = "sled error: {}", inner)]
    Sled { inner: SledError },
    
    #[cfg(feature = "sled-datastore")]
    #[fail(display = "sled transaction aborted")]
    SledTransactionAborted,
    
    #[fail(display = "UUID already taken")]
    UuidTaken,
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

#[cfg(feature = "sled-datastore")]
impl From<SledError> for Error {
    fn from(err: SledError) -> Self {
        Error::Sled { inner: err }
    }
}

#[cfg(feature = "sled-datastore")]
impl From<SledTransactionError<()>> for Error {
    fn from(err: SledTransactionError<()>) -> Self {
        match err {
            SledTransactionError::Abort(_) => Error::SledTransactionAborted,
            SledTransactionError::Storage(err) => Error::Sled { inner: err }
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Fail)]
pub enum ValidationError {
    #[fail(display = "invalid value")]
    InvalidValue,
    #[fail(display = "value too long")]
    ValueTooLong,
    #[fail(display = "could not increment the UUID")]
    CannotIncrementUuid,
}

pub type ValidationResult<T> = StdResult<T, ValidationError>;
