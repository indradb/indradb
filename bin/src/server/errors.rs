use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "i/o error: {}", inner)]
    Io { inner: io::Error },
    #[fail(display = "could not parse address binding")]
    CouldNotParseBinding,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io { inner: err }
    }
}
