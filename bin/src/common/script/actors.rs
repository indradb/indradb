use actix::prelude::*;
use actix_web::{error, Error};
use serde_json::Value as JsonValue;
use super::context;
use super::reader::{Reader, ReaderError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub payload: JsonValue
}

impl Request {
    pub fn new(name: String, payload: JsonValue) -> Self {
        Self { name, payload }
    }
}

impl Message for Request {
    type Result = Result<JsonValue, Error>;
}

pub struct Executor {
    reader: Reader
}

impl Executor {
    pub fn new() -> Self {
        Self {
            reader: Reader::new()
        }
    }
}

impl Actor for Executor {
    type Context = SyncContext<Self>;
}

impl Handler<Request> for Executor {
    type Result = Result<JsonValue, Error>;

    fn handle(&mut self, req: Request, _: &mut Self::Context) -> Self::Result {
        let value = self.reader.get(&req.name).map_err(|err| {
            match err {
                ReaderError::InvalidName => error::ErrorBadRequest("Invalid script name"),
                ReaderError::InvalidPath => error::ErrorInternalServerError("Could not stringify path. The script root config might be invalid."),
                ReaderError::Read => error::ErrorNotFound("Script not found")
            }
        })?;

        Ok(context::execute(&value.contents, &value.path, req.payload).map_err(|err| {
            error::ErrorInternalServerError(err)
        })?)
    }
}
