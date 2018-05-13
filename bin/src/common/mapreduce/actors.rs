use actix::prelude::*;
use actix_web::{error, Error};
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use serde_json::Value as JsonValue;
use statics;
use std::collections::BTreeMap;
use indradb::Datastore;
use regex;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::btree_map::Entry;
use super::context;

lazy_static! {
    static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+(\.lua)?$").unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    name: String,
    payload: JsonValue
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
    type Result = Result<String, Error>;

    fn handle(&mut self, req: Request, _: &mut Self::Context) -> Self::Result {
        if !SCRIPT_NAME_VALIDATOR.is_match(&req.name) {
            return Err(error::ErrorBadRequest("Invalid script name"));
        }

        let path = Path::new(&statics::SCRIPT_ROOT).join(name);
        let path_str = path.to_str().map_err(|_| error::ErrorInternalServerError("Could not stringify script path"))?;

        let contents = match self.cache.get(&name) {
            Some(value) => value,
            None => match File::open(&path) {
                Ok(mut file) => {
                    let mut contents = String::new();

                    match file.read_to_string(&mut contents) {
                        Ok(_) => contents,
                        Err(_) => return Err(error::ErrorNotFound("Could not read script"))
                    }
                }
                Err(_) => {
                    return Err(error::ErrorNotFound("Could not load script"))
                }
            }
        };

        Ok(context::execute(&contents, &path_str, req.payload).map_err(|err| {
            error::ErrorInternalServerError(err)
        })?)
    }
}
