use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::result::Result as StdResult;

#[cfg(feature = "rocksdb-datastore")]
use bincode::Error as BincodeError;
use rmp_serde::encode::Error as RmpEncodeError;
#[cfg(feature = "rocksdb-datastore")]
use rocksdb::Error as RocksDbError;
use serde_json::Error as JsonError;

/// An error triggered by the datastore.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// The requested UUID is already taken.
    UuidTaken,

    /// An error occurred in the underlying datastore.
    Datastore(Box<dyn StdError + Send + Sync>),

    /// A generic I/O error occurred.
    Io(IoError),

    /// A query occurred on a property that isn't indexed.
    NotIndexed,

    /// For functionality that isn't supported.
    Unsupported,

    /// A validation error occurred.
    Invalid(ValidationError),

    /// The operation cannot work with the given query, based off it's output
    /// type (e.g. attempting to delete using a query that outputs a count.)
    OperationOnQuery,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Datastore(ref err) => Some(&**err),
            Error::Io(ref err) => Some(err),
            Error::Invalid(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UuidTaken => write!(f, "UUID already taken"),
            Error::Datastore(ref err) => write!(f, "error in the underlying datastore: {err}"),
            Error::Io(ref err) => write!(f, "I/O error: {err}"),
            Error::NotIndexed => write!(f, "query attempted on a property that isn't indexed"),
            Error::Unsupported => write!(f, "functionality not supported"),
            Error::Invalid(ref err) => write!(f, "{err}"),
            Error::OperationOnQuery => write!(f, "the operation cannot work with the given query"),
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Datastore(Box::new(err))
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

#[cfg(feature = "rocksdb-datastore")]
impl From<BincodeError> for Error {
    fn from(err: BincodeError) -> Self {
        Error::Datastore(Box::new(err))
    }
}

impl From<RmpEncodeError> for Error {
    fn from(err: RmpEncodeError) -> Self {
        Error::Datastore(Box::new(err))
    }
}

#[cfg(feature = "rocksdb-datastore")]
impl From<RocksDbError> for Error {
    fn from(err: RocksDbError) -> Self {
        Error::Datastore(Box::new(err))
    }
}

impl From<ValidationError> for Error {
    fn from(err: ValidationError) -> Self {
        Error::Invalid(err)
    }
}

/// A result that might be an `Error`.
pub type Result<T> = StdResult<T, Error>;

/// A validation error
#[derive(Debug)]
pub enum ValidationError {
    /// The value is invalid.
    InvalidValue,
    /// The value is too long.
    ValueTooLong,
    /// The input UUID is the maximum value, and cannot be incremented.
    CannotIncrementUuid,
    /// The given query combination cannot be nested (e.g. attempting to build
    /// a query that gets vertex properties from a query that outputs a
    /// count.)
    InnerQuery,
}

impl StdError for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidationError::InvalidValue => write!(f, "invalid value"),
            ValidationError::ValueTooLong => write!(f, "value too long"),
            ValidationError::CannotIncrementUuid => write!(f, "could not increment the UUID"),
            ValidationError::InnerQuery => write!(f, "the given query combination cannot be nested"),
        }
    }
}

/// A result that might be a `ValidationError`.
pub type ValidationResult<T> = StdResult<T, ValidationError>;
