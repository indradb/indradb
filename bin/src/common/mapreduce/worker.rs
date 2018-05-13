use crossbeam_channel::{bounded, Receiver, Sender};
use indradb::Vertex;
use rlua::{Error as LuaError, Function, Table};
use script::context;
use script::converters;
use serde_json::value::Value as JsonValue;
use std::thread::{spawn, JoinHandle};

#[derive(Debug)]
pub enum WorkerError {
    Setup {
        description: String,
        cause: LuaError,
    },
    MapCall(LuaError),
    ReduceCall(LuaError),
}

macro_rules! try_or_send {
    ($expr:expr, $error_mapper:expr, $error_sender:expr) => {
        match $expr {
            Ok(value) => value,
            Err(err) => {
                $error_sender.send($error_mapper(err)).expect("Expected error channel to be open");
                return;
            }
        }
    }
}

pub enum WorkerTask {
    Map(Vertex),
    Reduce((converters::JsonValue, converters::JsonValue)),
}

pub struct Worker {
    thread: JoinHandle<()>,
    shutdown_sender: Sender<()>,
}

impl Worker {
    pub fn start(
        contents: String,
        path: String,
        arg: JsonValue,
        in_receiver: Receiver<WorkerTask>,
        out_sender: Sender<converters::JsonValue>,
        error_sender: Sender<WorkerError>,
    ) -> Self {
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);

        let thread = spawn(move || {
            let l = try_or_send!(
                context::create(arg),
                |err| WorkerError::Setup {
                    description: "Error occurred trying to to create a lua context".to_string(),
                    cause: err,
                },
                error_sender
            );

            let table: Table = try_or_send!(
                l.exec(&contents, Some(&path)),
                |err| WorkerError::Setup {
                    description: "Error occurred trying to get a table from the mapreduce script".to_string(),
                    cause: err,
                },
                error_sender
            );

            let mapper: Function = try_or_send!(
                table.get("map"),
                |err| WorkerError::Setup {
                    description: "Error occurred trying to get the `map` function from the returned table".to_string(),
                    cause: err,
                },
                error_sender
            );

            let reducer: Function = try_or_send!(
                table.get("reduce"),
                |err| WorkerError::Setup {
                    description: "Error occurred trying to get the `reduce` function from the returned table"
                        .to_string(),
                    cause: err,
                },
                error_sender
            );

            loop {
                select_loop! {
                    recv(in_receiver, task) => {
                        let value = match task {
                            WorkerTask::Map(vertex) => {
                                try_or_send!(
                                    mapper.call(converters::Vertex::new(vertex)),
                                    |err| WorkerError::MapCall(err),
                                    error_sender
                                )
                            },
                            WorkerTask::Reduce((first, second)) => {
                                try_or_send!(
                                    reducer.call((first, second)),
                                    |err| WorkerError::ReduceCall(err),
                                    error_sender
                                )
                            }
                        };

                        out_sender.send(value).expect("Expected worker output channel to be open");
                    },
                    recv(shutdown_receiver, _) => {
                        return;
                    }
                }
            }
        });

        Self {
            thread: thread,
            shutdown_sender: shutdown_sender,
        }
    }

    pub fn join(self) {
        self.shutdown_sender.send(()).ok();
        self.thread
            .join()
            .expect("Expected worker thread to not panic")
    }
}
