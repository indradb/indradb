use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::result::Result as StdResult;

use bincode::Error as BincodeError;
#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;
#[cfg(feature = "sled-datastore")]
use sled::Error as SledError;
use tempfile::PersistError as TempFilePersistError;

/// An error triggered by the datastore
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Json (de-)serialization failed
    Json {
        inner: JsonError,
    },

    #[cfg(feature = "rocksdb-datastore")]
    #[deprecated(since = "2.1.0", note = "use the Datastore variant instead")]
    Rocksdb {
        inner: RocksDbError,
    },

    #[cfg(feature = "sled-datastore")]
    #[deprecated(since = "2.1.0", note = "use the Datastore variant instead")]
    Sled {
        inner: SledError,
    },

    UuidTaken,

    /// An error occurred in the underlying datastore
    Datastore {
        inner: Box<dyn StdError + Send + Sync>,
    },
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Json { ref inner } => Some(inner),
            Error::Datastore { ref inner } => Some(&**inner),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Json { ref inner } => write!(f, "json error: {}", inner),
            Error::UuidTaken => write!(f, "UUID already taken"),
            Error::Datastore { ref inner } => write!(f, "error in the underlying datastore: {}", inner),
            #[cfg(feature = "rocksdb-datastore")]
            #[allow(deprecated)]
            Error::Rocksdb { ref inner } => write!(f, "rocksdb error: {}", inner),
            #[cfg(feature = "sled-datastore")]
            #[allow(deprecated)]
            Error::Sled { ref inner } => write!(f, "sled error: {}", inner),
        }
    }
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

/// A validation error
#[derive(Debug)]
pub enum ValidationError {
    /// The value is invalid
    InvalidValue,
    /// The value is too long
    ValueTooLong,
    /// The input UUID is the maximum value, and cannot be incremented
    CannotIncrementUuid,
}

impl StdError for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidationError::InvalidValue => write!(f, "invalid value"),
            ValidationError::ValueTooLong => write!(f, "value too long"),
            ValidationError::CannotIncrementUuid => write!(f, "could not increment the UUID"),
        }
    }
}

pub type ValidationResult<T> = StdResult<T, ValidationError>;
