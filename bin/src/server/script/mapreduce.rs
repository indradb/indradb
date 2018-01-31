use rlua::{Table, Function};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use indradb::{Vertex, VertexQuery};
use statics;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::time::Duration;
use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex};
use super::errors;
use super::context;
use super::converters;

const CHANNEL_TIMEOUT: u64 = 5;
const CHANNEL_CAPACITY: usize = 1000;
const REPORT_SECONDS: u64 = 10;

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

enum WorkerMessage {
    Map(Vertex),
    Reduce((converters::JsonValue, converters::JsonValue))
}

struct Worker {
    thread: JoinHandle<()>,
    shutdown_sender: Sender<()>
}

impl Worker {
    fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue, in_receiver: Receiver<WorkerMessage>, out_sender: Sender<converters::JsonValue>, error_sender: Sender<errors::MapReduceError>) -> Self {
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
                            WorkerMessage::Map(vertex) => {
                                try_or_send!(
                                    mapper.call(converters::Vertex::new(vertex)),
                                    |err| errors::MapReduceError::MapCall(err),
                                    error_sender
                                )
                            },
                            WorkerMessage::Reduce((first, second)) => {
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
        // This ignores the error. An error should only occur if the remote
        // end of the channel disconnected, implying that the thread crashed
        // anyways.
        self.shutdown_sender.send(()).ok();
        self.thread.join().expect("Expected worker thread to not panic")
    }
}

struct WorkerPool {
    router_thread: JoinHandle<Result<JsonValue, errors::MapReduceError>>,
    shutdown_sender: Sender<()>
}

impl WorkerPool {
    fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue, in_receiver: Receiver<Vertex>) -> Self {
        let (worker_in_sender, worker_in_receiver) = bounded::<WorkerMessage>(CHANNEL_CAPACITY);
        let (worker_out_sender, worker_out_receiver) = bounded::<converters::JsonValue>(CHANNEL_CAPACITY);
        let (error_sender, error_receiver) = bounded::<errors::MapReduceError>(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);
        let mut worker_threads: Vec<Worker> = Vec::with_capacity(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);

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
                            if worker_in_sender.send(WorkerMessage::Reduce((last_reduced_item_inner, value))).is_err() {
                                should_force_shutdown = true;
                            }
                            
                            pending_tasks += 1;
                            last_reduced_item = None;
                        } else {
                            last_reduced_item = Some(value);
                        }
                    },
                    recv(in_receiver, vertex) => {
                        // Only append to the queue if it won't block
                        if worker_in_receiver.len() < CHANNEL_CAPACITY {
                            // If this errors out, all of the workers are dead
                            if worker_in_sender.send(WorkerMessage::Map(vertex)).is_err() {
                                should_force_shutdown = true;
                            }

                            pending_tasks += 1;
                        }
                    }
                }

                // Check to see if we should shutdown
                if should_force_shutdown || (should_gracefully_shutdown && pending_tasks == 0) {
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
                            // This should always ahppen otherwise
                            Some(value) => value.0
                        })
                    }
                }
            }
        });

        Self {
            router_thread: router_thread,
            shutdown_sender: shutdown_sender
        }
    }

    fn join(self) -> Result<JsonValue, errors::MapReduceError> {
        // This ignores the error. An error should only occur if the remote
        // end of the channel disconnected, implying that the thread crashed
        // anyways.
        self.shutdown_sender.send(()).ok();
        self.router_thread.join().expect("Expected router thread to not panic")
    }
}

pub enum Message {
    Update(u64),
    Ok(JsonValue),
    Err(errors::MapReduceError)
}

pub struct Engine {
    pool: WorkerPool,
    reporter_thread: JoinHandle<()>,
    query_thread: JoinHandle<()>,
    update_receiver: Receiver<()>,
    shutdown_sender: Sender<()>
}

impl Engine {
    pub fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue) -> Self {
        let (in_sender, in_receiver) = bounded::<Vertex>(CHANNEL_CAPACITY);
        let (update_sender, update_receiver) = bounded::<Message>(10);
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);
        let pool = WorkerPool::start(account_id, contents, path, arg, in_receiver);

        // Using a mutex instead of an atomic because atomic u64s are currently
        // unstable
        let sent = Arc::new(Mutex::new(0u64));

        let reporter_thread = {
            let sent = sent.clone();
            let update_sender = update_sender.clone();

            spawn(move || {
                loop {
                    select_loop! {
                        recv(shutdown_sender, _) => {
                            return;
                        },
                        timed_out(Duration::from_secs(REPORT_SECONDS)) => {
                            let sent = sent.lock().unwrap();
                            update_sender.send(*sent);
                        }
                    }
                }
            });
        };

        let query_thread = spawn(move || {
            let trans = try_or_send!(
                statics::DATASTORE.transaction(account_id),
                |err| Message::Err(errors::MapReduceError::Query(err)),
                update_sender
            );

            let mut last_id: Option<Uuid> = None;

            loop {
                let q = VertexQuery::All { start_id: last_id, limit: *statics::MAP_REDUCE_QUERY_LIMIT };

                let vertices = try_or_send!(
                    trans.get_vertices(q),
                    |err| errors::MapReduceError::Query(err),
                    update_sender
                );

                let num_vertices = vertices.len() as u32;

                if let Some(last_vertex) = vertices.last() {
                    last_id = Some(last_vertex.id);
                }

                for vertex in vertices.into_iter() {
                    if in_sender.send(vertex).is_err() {
                        // The vertex couldn't be added, which means the channel is
                        // disconnected. This can only be caused if all of the workers
                        // failed, at which point we need to bail.
                        break;
                    }
                }

                {
                    let sent = sent.lock().unwrap();
                    *sent += num_vertices;
                }

                // Returned less than the expected number of results, implying that
                // the next query will not have any results
                if num_vertices < *statics::MAP_REDUCE_QUERY_LIMIT {
                    break;
                }
            }
        });

        Self {
            pool: pool,
            reporter_thread: reporter_thread,
            query_thread: query_thread,
            update_receiver: update_receiver,
            shutdown_sender: shutdown_sender
        }
    }

    pub fn get_update(&self) -> Message {
        self.update_receiver.recv().expect("Expected to be able to receive an update")
    }

    pub fn join(self) -> Result<JsonValue, errors::MapReduceError> {
        self.shutdown_sender.send(());
        self.reporter_thread.join().expect("Expected reporter thread to not panic");
        self.query_thread.join().expect("Expected query thread to not panic");
        self.pool.join()
    }
}