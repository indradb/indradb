use rlua::{Table, Function, Error as LuaError};
use serde_json::value::Value as JsonValue;
use indradb;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::thread::{spawn, JoinHandle};
use script::context;
use script::converters;

error_chain! {
    types {
        WorkerError, WorkerErrorKind, WorkerResultExt, WorkerResult;
    }

    links {
        Setup(indradb::Error, indradb::ErrorKind);
    }

    foreign_links {
        Call(LuaError);
    }
}

macro_rules! try_or_send {
    ($expr:expr, $description:expr, $error_sender:expr) => {
        match $expr.map_err(|err| WorkerError::with_chain(err, $description)) {
            Ok(value) => value,
            Err(err) => {
                $error_sender.send(err).expect("Expected error channel to be open");
                return;
            }
        }
    }
}

pub enum WorkerTask {
    Map(indradb::Vertex),
    Reduce((converters::JsonValue, converters::JsonValue))
}

pub struct Worker {
    thread: JoinHandle<()>,
    shutdown_sender: Sender<()>
}

impl Worker {
    pub fn start(contents: String, path: String, arg: JsonValue, in_receiver: Receiver<WorkerTask>, out_sender: Sender<converters::JsonValue>, error_sender: Sender<WorkerError>) -> Self {
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);

        let thread = spawn(move || {
            let l = try_or_send!(
                context::create(arg),
                "Could not setup lua context",
                error_sender
            );

            let table: Table = try_or_send!(
                l.exec(&contents, Some(&path)),
                "Script did not return a table",
                error_sender
            );

            let mapper: Function = try_or_send!(
                table.get("map"),
                "Script did not return a `map` function",
                error_sender
            );

            let reducer: Function = try_or_send!(
                table.get("reduce"),
                "Script did not return a `reduce` function",
                error_sender
            );

            loop {
                select_loop! {
                    recv(in_receiver, task) => {
                        let value = match task {
                            WorkerTask::Map(vertex) => {
                                try_or_send!(
                                    mapper.call(converters::Vertex::new(vertex)),
                                    "Map call failed",
                                    error_sender
                                )
                            },
                            WorkerTask::Reduce((first, second)) => {
                                try_or_send!(
                                    reducer.call((first, second)),
                                    "Reduce call failed",
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
            shutdown_sender: shutdown_sender
        }
    }

    pub fn join(self) {
        self.shutdown_sender.send(()).ok();
        self.thread.join().expect("Expected worker thread to not panic")
    }
}