use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

use bincode::Error as BincodeError;
#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;

/// An error triggered by the datastore
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    IdTaken,

    /// An error occurred in the underlying datastore
    Datastore(Box<dyn StdError + Send + Sync>),

    /// A query occurred on a property that isn't indexed
    NotIndexed,

    /// For functionality that isn't supported
    Unsupported,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Datastore(ref err) => Some(&**err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IdTaken => write!(f, "ID already taken"),
            Error::Datastore(ref err) => write!(f, "error in the underlying datastore: {}", err),
            Error::NotIndexed => write!(f, "query attempted on a property that isn't indexed"),
            Error::Unsupported => write!(f, "functionality not supported"),
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Datastore(Box::new(err))
    }
}

impl From<BincodeError> for Error {
    fn from(err: BincodeError) -> Self {
        Error::Datastore(Box::new(err))
    }
}

#[cfg(feature = "rocksdb-datastore")]
impl From<RocksDbError> for Error {
    fn from(err: RocksDbError) -> Self {
        Error::Datastore(Box::new(err))
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
    /// The input id is the maximum value, and cannot be incremented
    CannotIncrementId,
}

impl StdError for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidationError::InvalidValue => write!(f, "invalid value"),
            ValidationError::ValueTooLong => write!(f, "value too long"),
            ValidationError::CannotIncrementId => write!(f, "could not increment the ID"),
        }
    }
}

pub type ValidationResult<T> = StdResult<T, ValidationError>;
