use std::error::Error;
use std::fmt;

/// `SimpleError`s are just errors with string contents.
#[derive(Debug)]
pub struct SimpleError {
    description: String,
}

impl SimpleError {
    pub fn new(description: String) -> SimpleError {
        SimpleError { description }
    }
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        &self.description[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}
