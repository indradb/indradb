use actix::prelude::*;
use uuid::Uuid;
use super::worker::{Worker, WorkerError, MapRequest, ReduceRequest};
use indradb::{Error, VertexQuery};
use script::{Request, converters};
use serde_json::value::Value as JsonValue;
use statics;
use futures::{Stream, stream, future, Future};
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
    Worker(WorkerError),
    Mailbox(MailboxError)
}

impl From<Error> for RouterError {
    fn from(err: Error) -> Self {
        RouterError::Query(err)
    }
}

impl From<WorkerError> for RouterError {
    fn from(err: WorkerError) -> Self {
        RouterError::Worker(err)
    }
}

impl From<MailboxError> for RouterError {
    fn from(err: MailboxError) -> Self {
        RouterError::Mailbox(err)
    }
}

#[derive(Debug, Clone)]
pub struct RouterStatus {
    done: bool,
    processed: u64,
    reduced_value: Result<converters::JsonValue, WorkerError>
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
                reduced_value: Ok(converters::JsonValue::new(JsonValue::Null)),
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

            let s = stream::futures_unordered(fs)
                .map_err(|err| RouterError::Mailbox(err))
                .fold(self.status.reduced_value, |accumulator, value| {
                    self.workers.send(ReduceRequest::new(self.req.clone(), accumulator, value))
                });

            self.status.reduced_value = s.wait()?;
        }

        Ok(done)
    }
}
