use std::error::Error as StdError;
use std::fmt;

/// The error returned by datastore and transaction implementation methods.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Error {
    AccountNotFound,
    VertexNotFound,
    EdgeNotFound,
    MetadataNotFound,
    Unauthorized,
    OutOfRange(String),
    Unexpected(String),
}

impl Error {
    /// A utility method that converts an error message back to an error
    /// object.
    pub fn description_to_error(message: &str) -> Self {
        match &message[..] {
            "Account not found" => Error::AccountNotFound,
            "Vertex does not exist" => Error::VertexNotFound,
            "Edge does not exist" => Error::EdgeNotFound,
            "Metadata does not exist" => Error::MetadataNotFound,
            "Unauthorized" => Error::Unauthorized,
            _ => {
                if message.starts_with("Value out of range: ") {
                    Error::OutOfRange(message[20..message.len()].to_string())
                } else {
                    Error::Unexpected(message.to_string())
                }
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AccountNotFound => "Account not found",
            Error::VertexNotFound => "Vertex does not exist",
            Error::EdgeNotFound => "Edge does not exist",
            Error::MetadataNotFound => "Metadata does not exist",
            Error::Unauthorized => "Unauthorized",
            Error::OutOfRange(_) => "Value out of range",
            Error::Unexpected(_) => "Unexpected error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Unexpected(ref msg) => write!(f, "{}", msg),
            Error::OutOfRange(ref name) => write!(f, "Value out of range: {}", name), 
            _ => write!(f, "{}", self.description()),
        }
    }
}

/// The error returned when there is an attempt to instantiate a model with an
/// invalid value.
#[derive(Debug)]
pub struct ValidationError {
    description: String,
}

impl ValidationError {
    pub fn new(description: String) -> ValidationError {
        ValidationError { description: description }
    }
}

impl StdError for ValidationError {
    fn description(&self) -> &str {
        &self.description[..]
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
