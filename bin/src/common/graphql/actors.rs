use actix::prelude::*;
use actix_web::{error, Error};
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use serde_json;
use statics;
use indradb::Datastore;
use super::Schema;
use super::context::Context;

#[derive(Serialize, Deserialize)]
pub struct Request(GraphQLRequest);

impl Message for Request {
    type Result = Result<String, Error>;
}

pub struct Executor {
    schema: Arc<Schema>
}

impl Executor {
    pub fn new(schema: Arc<Schema>) -> Self {
        Self {
            schema: schema,
        }
    }
}

impl Actor for Executor {
    type Context = SyncContext<Self>;
}

impl Handler<Request> for Executor {
    type Result = Result<String, Error>;

    fn handle(&mut self, req: Request, _: &mut Self::Context) -> Self::Result {
        // TODO: currently we construct a transaction for each request. See if
        // there's a way across multiple requests from a given user
        let trans = statics::DATASTORE.transaction().map_err(|err| -> Error {
            error::ErrorInternalServerError(format!("{}", err))
        })?;
        let context = Context::new(trans);
        let res = req.0.execute(&self.schema, &context);
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}
