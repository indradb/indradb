use errors::Error;
use bincode::serde::{SerializeError, DeserializeError};
use std::str::Utf8Error;
use serde_json;
use rocksdb::Error as RocksdbError;

impl From<RocksdbError> for Error {
    fn from(err: RocksdbError) -> Error {
        Error::Unexpected(err.to_string())
    }
}

impl From<SerializeError> for Error {
    fn from(err: SerializeError) -> Error {
        Error::Unexpected(format!("Could not serialize contents: {:?}", err))
    }
}

impl From<DeserializeError> for Error {
    fn from(err: DeserializeError) -> Error {
        Error::Unexpected(format!("Could not deserialize contents: {:?}", err))
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Error {
        Error::Unexpected(format!("Could not parse utf-8 contents"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Unexpected(format!("Could not (de-)serialize json: {:?}", err))
    }
}
