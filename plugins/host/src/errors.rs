use std::error::Error as StdError;
use std::fmt;

use indradb::Error as IndraDBError;
use serde_json::Error as JsonError;

/// A plugin error.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Json (de-)serialization error.
    Json(JsonError),
    /// IndraDB error.
    IndraDB(IndraDBError),
    // When the input argument is valid JSON, but invalid for plugin-specific
    // reasons.
    InvalidArgument(String),
    /// Any other kind of error.
    Other(Box<dyn StdError + Send + Sync>),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Json(ref err) => Some(err),
            Error::IndraDB(ref err) => Some(err),
            Error::Other(ref err) => Some(&**err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Json(ref err) => write!(f, "json error: {}", err),
            Error::IndraDB(ref err) => write!(f, "IndraDB error: {}", err),
            Error::InvalidArgument(ref msg) => write!(f, "{}", msg),
            Error::Other(ref err) => write!(f, "{}", err),
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl From<IndraDBError> for Error {
    fn from(err: IndraDBError) -> Self {
        Error::IndraDB(err)
    }
}

impl From<Box<dyn StdError + Send + Sync>> for Error {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Self {
        Error::Other(err)
    }
}
