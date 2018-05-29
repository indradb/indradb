use actix::prelude::*;
use actix_web::{Error as ActixError, ws};
use rlua::Error as LuaError;
use uuid::Uuid;
use super::worker::{Worker, MapRequest, ReduceRequest};
use indradb::{Vertex, Error, VertexQuery};
use script::{Request, converters};
use serde_json::value::Value as JsonValue;
use statics;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;
use futures::{Stream, stream};
use indradb::{Datastore, Transaction};

pub struct GetStatus;

impl Message for GetStatus {
    type Result = Result<RouterStatus, ()>;
}

pub struct ProcessNextBatch;

impl Message for ProcessNextBatch {
    type Result = Result<bool, RouterError>;
}

pub enum RouterError {
    Query(Error),
    Worker(LuaError)
}

impl From<Error> for RouterError {
    fn from(err: Error) -> Self {
        RouterError::Query(err)
    }
}

impl From<LuaError> for RouterError {
    fn from(err: LuaError) -> Self {
        RouterError::Worker(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterStatus {
    done: bool,
    processed: u64,
    reduced_value: Option<JsonValue>
}

pub struct Router {
    req: Request,
    workers: Addr<Syn, Worker>,
    last_id: Option<Uuid>,
    status: RouterStatus
}

impl Router {
    pub fn new(req: Request) -> Self {
        Self {
            req,
            workers: SyncArbiter::start(*statics::POOL_SIZE as usize, move || {
                Worker::default()
            }),
            last_id: None,
            status: RouterStatus {
                done: false,
                processed: 0,
                reduced_value: None,
            }
        }
    }
}

impl Actor for Router {
    type Context = SyncContext<Self>;
}

impl Handler<GetStatus> for Router {
    type Result = Result<RouterStatus, ()>;

    fn handle(&mut self, _: GetStatus, _: &mut Self::Context) -> Self::Result {
        Ok(self.status.clone())
    }
}

impl Handler<ProcessNextBatch> for Router {
    type Result = Result<bool, RouterError>;

    fn handle(&mut self, _: ProcessNextBatch, context: &mut Self::Context) -> Self::Result {
        let trans = statics::DATASTORE.transaction()?;

        let q = VertexQuery::All {
            start_id: self.last_id,
            limit: *statics::MAP_REDUCE_QUERY_LIMIT,
        };

        let vertices = trans.get_vertices(&q)?;
        let done = vertices.len() < *statics::MAP_REDUCE_QUERY_LIMIT as usize;
        self.status.done = done;

        if vertices.len() > 0 {
            self.status.processed += vertices.len() as u64;
            self.last_id = Some(vertices.last().unwrap().id);

            let fs = vertices.into_iter().map(|v| {
                self.workers.send(MapRequest::new(self.req.clone(), v))
            });

            let reduced_value = self.status.reduced_value.map(|v| converters::JsonValue::new(v));

            let s = stream::futures_unordered(fs).fold(reduced_value, |first, second| {
                self.workers.send(ReduceRequest::new(
                    self.req.clone(),
                    first.or(converters::JsonValue::new(JsonValue::Null)),
                    second
                ))
            });

            match s.wait() {
                Ok(value) => self.status.reduced_value = value.0,
                Err(err) => return Err(err)
            };
        }

        Ok(done)
    }
}
