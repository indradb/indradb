use actix::prelude::*;
use actix_web::{error, Error};
use serde_json::Value as JsonValue;
use statics;
use std::collections::BTreeMap;
use regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use super::context;

lazy_static! {
    static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+(\.lua)?$").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    name: String,
    payload: JsonValue
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
    cache: BTreeMap<String, String>
}

impl Executor {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new()
        }
    }
}

impl Actor for Executor {
    type Context = SyncContext<Self>;
}

impl Handler<Request> for Executor {
    type Result = Result<JsonValue, Error>;

    fn handle(&mut self, req: Request, _: &mut Self::Context) -> Self::Result {
        if !SCRIPT_NAME_VALIDATOR.is_match(&req.name) {
            return Err(error::ErrorBadRequest("Invalid script name"));
        }

        let path = Path::new(&*statics::SCRIPT_ROOT).join(&req.name);
        let path_str = path.to_str().ok_or_else(|| error::ErrorInternalServerError("Could not stringify script path"))?;

        // TODO: this could probably be optimized a bit with the entry API,
        // though it's complicated by the fact that reading the file contents
        // may yield an error
        if !self.cache.contains_key(&req.name) {
            match File::open(&req.name) {
                Ok(mut file) => {
                    let mut contents = String::new();

                    match file.read_to_string(&mut contents) {
                        Ok(_) => self.cache.insert(req.name.clone(), contents.clone()),
                        Err(_) => return Err(error::ErrorNotFound("Could not read script"))
                    }
                }
                Err(_) => {
                    return Err(error::ErrorNotFound("Could not load script"))
                }
            };
        }

        Ok(context::execute(&self.cache[&req.name], &path_str, req.payload).map_err(|err| {
            error::ErrorInternalServerError(err)
        })?)
    }
}
