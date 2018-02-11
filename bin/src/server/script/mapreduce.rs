use rlua::{Table, Function};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use indradb::Vertex;
use statics;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::time::Duration;
use std::thread::{spawn, JoinHandle, sleep};
use super::errors;
use super::context;
use super::converters;

const CHANNEL_TIMEOUT: u64 = 5;
const CHANNEL_CAPACITY: usize = 1000;

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

enum WorkerTask {
    Map(Vertex),
    Reduce((converters::JsonValue, converters::JsonValue))
}

struct Worker {
    thread: JoinHandle<()>,
    shutdown_sender: Sender<()>
}

impl Worker {
    fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue, in_receiver: Receiver<WorkerTask>, out_sender: Sender<converters::JsonValue>, error_sender: Sender<errors::MapReduceError>) -> Self {
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);

        let thread = spawn(move || {
            let l = try_or_send!(
                context::create(account_id, arg),
                |err| errors::MapReduceError::WorkerSetup {
                    description: "Error occurred trying to to create a lua context".to_string(),
                    cause: err
                },
                error_sender
            );

            let table: Table = try_or_send!(
                l.exec(&contents, Some(&path)),
                |err| errors::MapReduceError::WorkerSetup {
                    description: "Error occurred trying to get a table from the mapreduce script".to_string(),
                    cause: errors::ScriptError::Lua(err)
                },
                error_sender
            );

            let mapper: Function = try_or_send!(
                table.get("map"),
                |err| errors::MapReduceError::WorkerSetup {
                    description: "Error occurred trying to get the `map` function from the returned table".to_string(),
                    cause: errors::ScriptError::Lua(err)
                },
                error_sender
            );

            let reducer: Function = try_or_send!(
                table.get("reduce"),
                |err| errors::MapReduceError::WorkerSetup {
                    description: "Error occurred trying to get the `reduce` function from the returned table".to_string(),
                    cause: errors::ScriptError::Lua(err)
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
                                    |err| errors::MapReduceError::MapCall(err),
                                    error_sender
                                )
                            },
                            WorkerTask::Reduce((first, second)) => {
                                try_or_send!(
                                    reducer.call((first, second)),
                                    |err| errors::MapReduceError::ReduceCall(err),
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

    fn join(self) {
        self.shutdown_sender.send(()).ok();
        self.thread.join().expect("Expected worker thread to not panic")
    }
}

pub struct MapReducer {
    router_thread: JoinHandle<Result<JsonValue, errors::MapReduceError>>,
    in_sender: Sender<Vertex>,
    shutdown_sender: Sender<()>
}

impl MapReducer {
    pub fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue) -> Self {
        let (mapreduce_in_sender, mapreduce_in_receiver) = bounded::<Vertex>(CHANNEL_CAPACITY);
        let (worker_in_sender, worker_in_receiver) = bounded::<WorkerTask>(CHANNEL_CAPACITY);
        let (worker_out_sender, worker_out_receiver) = bounded::<converters::JsonValue>(CHANNEL_CAPACITY);
        let (error_sender, error_receiver) = bounded::<errors::MapReduceError>(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);
        let mut worker_threads: Vec<Worker> = Vec::with_capacity(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);
        let worker_in_capacity = worker_in_receiver.capacity().unwrap();

        for _ in 0..*statics::MAP_REDUCE_WORKER_POOL_SIZE {
            worker_threads.push(Worker::start(
                account_id,
                contents.clone(),
                path.clone(),
                arg.clone(),
                worker_in_receiver.clone(),
                worker_out_sender.clone(),
                error_sender.clone(),
            ));
        }

        let router_thread = spawn(move || -> Result<JsonValue, errors::MapReduceError> {
            let mut should_force_shutdown = false; 
            let mut should_gracefully_shutdown = false;
            let mut pending_tasks: usize = 0;
            let mut last_reduced_item: Option<converters::JsonValue> = None;
            let mut last_error: Option<errors::MapReduceError> = None;

            loop {
                select_loop! {
                    recv(error_receiver, err) => {
                        last_error = Some(err);
                        should_force_shutdown = true;
                    },
                    recv(shutdown_receiver, _) => {
                        should_gracefully_shutdown = true;
                    },
                    recv(worker_out_receiver, value) => {
                        pending_tasks -= 1;

                        if let Some(last_reduced_item_inner) = last_reduced_item {
                            // If this errors out, all of the workers are dead
                            if worker_in_sender.send(WorkerTask::Reduce((last_reduced_item_inner, value))).is_err() {
                                should_force_shutdown = true;
                            }
                            
                            pending_tasks += 1;
                            last_reduced_item = None;
                        } else {
                            last_reduced_item = Some(value);
                        }
                    },
                    recv(mapreduce_in_receiver, vertex) => {
                        if worker_in_receiver.len() < worker_in_capacity {
                            // If this errors out, all of the workers are dead
                            if worker_in_sender.send(WorkerTask::Map(vertex)).is_err() {
                                should_force_shutdown = true;
                            }

                            pending_tasks += 1;
                        }
                    },
                    timed_out(Duration::from_secs(CHANNEL_TIMEOUT)) => {}
                }

                // Check to see if we should shutdown
                if should_force_shutdown || (should_gracefully_shutdown && pending_tasks == 0 && mapreduce_in_receiver.len() == 0) {
                    // Join all threads
                    for worker_thread in worker_threads.into_iter() {
                        worker_thread.join();
                    }

                    return if should_force_shutdown {
                        // If it's a hard error, find an error to return
                        Err(last_error.unwrap_or_else(|| error_receiver.try_recv().expect("Expected to be able to read the error channel")))
                    } else {
                        // Get the final value to return
                        Ok(match last_reduced_item {
                            // This should only happen if the graph is empty
                            None => JsonValue::Null,
                            // This should always happen otherwise
                            Some(value) => value.0
                        })
                    }
                }
            }
        });

        Self {
            router_thread: router_thread,
            in_sender: mapreduce_in_sender,
            shutdown_sender: shutdown_sender
        }
    }

    pub fn add_vertex(&self, vertex: Vertex) -> bool {
        self.in_sender.send(vertex).is_ok()
    }

    pub fn join(self) -> Result<JsonValue, errors::MapReduceError> {
        self.shutdown_sender.send(()).ok();
        self.router_thread.join().expect("Expected router thread to not panic")
    }
}
