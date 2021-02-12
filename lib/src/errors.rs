use std::error::Error as StdError;
use std::io::Error as IoError;
use std::result::Result as StdResult;

use bincode::Error as BincodeError;
use failure::Fail;
#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;
#[cfg(feature = "sled-datastore")]
use sled::Error as SledError;
use tempfile::PersistError as TempFilePersistError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "json error: {}", inner)]
    Json { inner: JsonError },

    #[cfg(feature = "rocksdb-datastore")]
    #[deprecated(since = "2.1.0", note = "use the Datastore variant instead")]
    #[fail(display = "rocksdb error: {}", inner)]
    Rocksdb { inner: RocksDbError },

    #[cfg(feature = "sled-datastore")]
    #[deprecated(since = "2.1.0", note = "use the Datastore variant instead")]
    #[fail(display = "sled error: {}", inner)]
    Sled { inner: SledError },

    #[fail(display = "UUID already taken")]
    UuidTaken,
    #[fail(display = "underlying datastore failure: {}", inner)]
    Datastore { inner: Box<dyn StdError + Send + Sync> },
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json { inner: err }
    }
}

#[cfg(feature = "rocksdb-datastore")]
impl From<RocksDbError> for Error {
    fn from(err: RocksDbError) -> Self {
        Error::Datastore { inner: Box::new(err) }
    }
}

#[cfg(feature = "sled-datastore")]
impl From<SledError> for Error {
    fn from(err: SledError) -> Self {
        Error::Datastore { inner: Box::new(err) }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Datastore { inner: Box::new(err) }
    }
}

impl From<BincodeError> for Error {
    fn from(err: BincodeError) -> Self {
        Error::Datastore { inner: Box::new(err) }
    }
}

impl From<TempFilePersistError> for Error {
    fn from(err: TempFilePersistError) -> Self {
        Error::Datastore { inner: Box::new(err) }
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
