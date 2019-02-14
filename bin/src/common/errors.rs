use std::io;
use std::result::Result as StdResult;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "i/o error: {}", inner)]
    Io { inner: io::Error },
    #[fail(display = "could not parse")]
    CouldNotParse,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io { inner: err }
    }
}

pub type Result<T> = StdResult<T, Error>;
