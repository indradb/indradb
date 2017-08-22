use errors::Error;
use bincode::Error as BincodeError;
use std::str::Utf8Error;
use serde_json;
use rocksdb::Error as RocksdbError;

impl From<RocksdbError> for Error {
    fn from(err: RocksdbError) -> Error {
        Error::Unexpected(err.to_string())
    }
}

impl From<BincodeError> for Error {
    fn from(err: BincodeError) -> Error {
        Error::Unexpected(format!("Could not (de-)serialize contents: {:?}", err))
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Error {
        Error::Unexpected("Could not parse utf-8 contents".to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Unexpected(format!("Could not (de-)serialize json: {:?}", err))
    }
}
