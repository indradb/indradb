use rlua::{Table, Function};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use indradb::Vertex;
use statics;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::time::Duration;
use std::thread::{spawn, JoinHandle};
use super::errors;
use super::context;
use super::converters;

const CHANNEL_CAPACITY: usize = 1000;
const CHANNEL_RECV_TIMEOUT_SECONDS: u64 = 1;
const REPORT_SECONDS: u64 = 30;

enum WorkerTask {
    Map(Vertex),
    Reduce((converters::JsonValue, converters::JsonValue))
}

struct Worker {
    thread: JoinHandle<Result<(), errors::MapReduceError>>,
    shutdown_sender: Sender<()>
}

impl Worker {
    fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue, in_receiver: Receiver<WorkerTask>, out_sender: Sender<converters::JsonValue>) -> Self {
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);

        let thread = spawn(move || -> Result<(), errors::MapReduceError> {
            let mut should_shutdown = false;
            let l = context::create(account_id, arg).map_err(|err| errors::MapReduceError::WorkerSetup(err))?;
            let table: Table = l.exec(&contents, Some(&path)).map_err(|err| errors::MapReduceError::WorkerSetup(errors::ScriptError::Lua(err)))?;
            let mapper: Function = table.get("map").map_err(|err| errors::MapReduceError::WorkerSetup(errors::ScriptError::Lua(err)))?;
            let reducer: Function = table.get("reduce").map_err(|err| errors::MapReduceError::WorkerSetup(errors::ScriptError::Lua(err)))?;

            loop {
                select_loop! {
                    recv(in_receiver, task) => {
                        let value = match task {
                            WorkerTask::Map(vertex) => {
                                mapper.call(converters::Vertex::new(vertex)).map_err(|err| errors::MapReduceError::MapCall(err))
                            },
                            WorkerTask::Reduce((first, second)) => {
                                reducer.call((first, second)).map_err(|err| errors::MapReduceError::ReduceCall(err))
                            }
                        }?;

                        out_sender.send(value).expect("Expected worker output channel to be open");
                    },
                    recv(shutdown_receiver, _) => {
                        should_shutdown = true;
                    },
                    timed_out(Duration::from_secs(CHANNEL_RECV_TIMEOUT_SECONDS)) => {}
                }

                if should_shutdown {
                    return Ok(());
                }
            }
        });

        Self {
            thread: thread,
            shutdown_sender: shutdown_sender
        }
    }

    fn join(self) -> Result<(), errors::MapReduceError> {
        // This ignores the error. An error should only occur if the remote
        // end of the channel disconnected, implying that the thread crashed
        // anyways.
        self.shutdown_sender.send(()).ok();
        self.thread.join().expect("Expected worker thread to not panic")
    }
}

pub struct WorkerPool {
    reporter_thread: JoinHandle<()>,
    router_thread: JoinHandle<Result<JsonValue, errors::MapReduceError>>,
    in_sender: Sender<Vertex>,
    shutdown_sender: Sender<()>
}

impl WorkerPool {
    pub fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue) -> Self {
        let (mapreduce_in_sender, mapreduce_in_receiver) = bounded::<Vertex>(CHANNEL_CAPACITY);
        let (worker_in_sender, worker_in_receiver) = bounded::<WorkerTask>(CHANNEL_CAPACITY);
        let (worker_out_sender, worker_out_receiver) = bounded::<converters::JsonValue>(CHANNEL_CAPACITY);
        let (reporter_sender, reporter_receiver) = bounded::<()>(0);
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(2);
        let mut worker_threads: Vec<Worker> = Vec::with_capacity(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);

        for _ in 0..*statics::MAP_REDUCE_WORKER_POOL_SIZE {
            worker_threads.push(Worker::start(
                account_id,
                contents.clone(),
                path.clone(),
                arg.clone(),
                worker_in_receiver.clone(),
                worker_out_sender.clone()
            ));
        }

        let reporter_thread = {
            let shutdown_receiver = shutdown_receiver.clone();

            spawn(move || {
                while let Err(_) = shutdown_receiver.recv_timeout(Duration::from_secs(REPORT_SECONDS)) {
                    reporter_sender.send(()).unwrap();
                }
            })
        };

        let router_thread = spawn(move || -> Result<JsonValue, errors::MapReduceError> {
            let mut should_force_shutdown = false; 
            let mut should_gracefully_shutdown = false;
            let mut pending_tasks: usize = 0;
            let mut report_num: usize = 0;
            let mut last_reduced_item: Option<converters::JsonValue> = None;

            loop {
                select_loop! {
                    recv(mapreduce_in_receiver, vertex) => {
                        // If this errors out, all of the workers are dead
                        if worker_in_sender.send(WorkerTask::Map(vertex)).is_err() {
                            should_force_shutdown = true;
                        } else {
                            pending_tasks += 1;
                        }
                    },
                    recv(worker_out_receiver, value) => {
                        pending_tasks -= 1;

                        if let Some(last_reduced_item_inner) = last_reduced_item {
                            // If this errors out, all of the workers are dead
                            if worker_in_sender.send(WorkerTask::Reduce((last_reduced_item_inner, value))).is_err() {
                                should_force_shutdown = true;
                            } else {
                                pending_tasks += 1;
                            }
                            
                            last_reduced_item = None;
                        } else {
                            last_reduced_item = Some(value);
                        }
                    },
                    recv(reporter_receiver, _) => {
                        println!("Mapreduce: report={}, pending tasks={}, winding down={}", report_num, pending_tasks, should_gracefully_shutdown);
                        report_num += 1;
                    }
                    recv(shutdown_receiver, _) => {
                        should_gracefully_shutdown = true;
                    },
                    timed_out(Duration::from_secs(CHANNEL_RECV_TIMEOUT_SECONDS)) => {}
                }

                // Check to see if we should shutdown
                if should_force_shutdown || (should_gracefully_shutdown && pending_tasks == 0) {
                    // Join all threads and check for any errors
                    let results: Result<Vec<()>, errors::MapReduceError> = worker_threads.into_iter().map(|t| t.join()).collect();
                    results?;

                    // Get the final value to return
                    return Ok(match last_reduced_item {
                        // This should only happen if the graph is empty
                        None => JsonValue::Null,
                        // This should always ahppen otherwise
                        Some(value) => value.0
                    });
                }
            }
        });

        Self {
            reporter_thread: reporter_thread,
            router_thread: router_thread,
            in_sender: mapreduce_in_sender,
            shutdown_sender: shutdown_sender
        }
    }

    pub fn add_vertex(&self, vertex: Vertex) -> bool {
        self.in_sender.send(vertex).is_ok()
    }

    pub fn join(self) -> Result<JsonValue, errors::MapReduceError> {
        for _ in 0..2 {
            // Send a shutdown notification to both the reporter and router.
            // This ignores the error. An error should only occur if the remote
            // end of the channel disconnected, implying that the thread crashed
            // anyways.
            self.shutdown_sender.send(()).ok();
        }

        self.reporter_thread.join().expect("Expected reporter thread to not panic");
        self.router_thread.join().expect("Expected router thread to not panic")
    }
}
