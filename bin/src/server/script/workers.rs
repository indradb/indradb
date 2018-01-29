use rlua::{Table, Value, Function};
use rlua::prelude::*;
use serde_json::value::Value as JsonValue;
use std::path::Path;
use uuid::Uuid;
use indradb::{Transaction, Vertex, Datastore, VertexQuery};
use statics;
use std::convert::From;
use crossbeam_channel::{Receiver, Sender, bounded};
use std::time::Duration;
use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex};
use super::errors;
use super::converters;
use std::collections::VecDeque;

const MAPREDUCE_CHANNEL_CAPACITY: usize = 1000;
const WORKER_CHANNEL_RECV_TIMEOUT_SECONDS: usize = 1;

struct WorkerContext {
    l: Lua,
    main: Function
}

impl WorkerContext {
    fn new(account_id: Uuid, contents: String, path: String, arg: JsonValue) -> Result<Self, errors::ScriptError> {
        let l = Lua::new();

        {
            let globals = l.globals();

            // Update the `package.path` to include the script root, so it's easier
            // for scripts to require each other.
            {
                let package: Table = globals.get("package")?;
                let old_path: String = package.get("path")?;
                let script_path = Path::new(&statics::SCRIPT_ROOT[..])
                    .join("?.lua")
                    .to_str()
                    .unwrap()
                    .to_string();
                package.set("path", format!("{};{}", old_path, script_path))?;
            }

            // Create a new transaction for the script
            let trans = statics::DATASTORE.transaction(account_id)?;

            // Add globals
            globals.set("trans", converters::ProxyTransaction::new(trans))?;
            globals.set("account_id", account_id.to_string())?;
            globals.set("arg", converters::JsonValue::new(arg))?;
        }

        l.load(&contents, Some(&path))?;
        let fun = l.load(&contents, Some(&path))?;
        
        Ok(Self {
            l: l,
            main: fun
        })
    }
}

pub struct Worker {
    thread: JoinHandle<(), errors::ScriptError>,
    shutdown_sender: Sender<()>
}

impl Worker {
    fn start<T, U>(account_id: Uuid, contents: String, path: String, arg: JsonValue, in_receiver: Receiver<T>, out_sender: Sender<U>, worker: |WorkerContext, T| -> Result<U, errors::ScriptError>) -> Self {
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);

        let thread = spawn(move || -> Result<(), script::ScriptError> {
            let should_shutdown = false;
            let context = WorkerContext::new(account_id, contents, path, arg);

            loop {
                select_loop! {
                    recv(in_receiver, in_value) => {
                        let out_value = worker(context, in_value);
                        out_sender.send(out_value).unwrap();
                    },
                    recv(shutdown_receiver, _) => {
                        should_shutdown = true;
                    },
                    timed_out(Duration::from_secs(WORKER_CHANNEL_RECV_TIMEOUT_SECONDS)) => {
                        if should_shutdown {
                            return Ok(());
                        }
                    }
                }
            }
        };

        Self {
            thread: thread,
            shutdown_sender: shutdown_sender
        }
    }

    fn join(self) -> Result<(), errors::ScriptError> {
        self.shutdown_sender.send(()).unwrap();
        self.thread.join().map_err(|err| errors::ScriptError::ThreadPanic { err })?
    }
}

enum MapReduceWorkerTask {
    Map(Vertex),
    Reduce((converters::JsonValue, converters::JsonValue))
}

pub struct MapReduceWorkerPool {
    router_thread: JoinHandle<Result<JsonValue, errors::ScriptError>>,
    in_sender: Sender<Vertex>,
    shutdown_sender: Sender<()>
}

impl MapReduceWorkerPool {
    fn start(account_id: Uuid, contents: String, path: String, arg: JsonValue) -> Self {
        let (mapreduce_in_sender, mapreduce_in_receiver) = bounded::<Vertex>(MAPREDUCE_CHANNEL_CAPACITY);
        let (worker_in_sender, worker_in_receiver) = bounded::<MapReduceWorkerTask>(MAPREDUCE_CHANNEL_CAPACITY);
        let (worker_out_sender, worker_out_receiver) = bounded::<converters::JsonValue>(MAPREDUCE_CHANNEL_CAPACITY);
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);
        let mut worker_threads: Vec<Worker> = Vec::with_capacity(*statics::MAP_REDUCE_WORKER_POOL_SIZE);

        for _ in (0..*statics::MAP_REDUCE_WORKER_POOL_SIZE) {
            worker_threads.push(Worker::start<MapReduceWorkerTask, converters::JsonValue>(account_id, contents.clone(), path.clone(), arg.clone(), worker_in_receiver.clone(), worker_out_sender.clone(), |context, task| {
                // TODO: this could be made more efficient by not repeatedly calling main
                let (mapper, reducer): (Function, Function) = context.main.call(Value::Nil)?;

                match task {
                    Map(vertex) => {
                        mapper.call(converters::Vertex::new(vertex))
                    },
                    Reduce((first, second)) => {
                        reducer.call((first, second))
                    }
                }
            }));
        }

        let router_thread = {
            spawn(move || -> JoinHandle<Result<JsonValue, errors::ScriptError>> {
                let mut should_shutdown = false;
                let mut pending_tasks: usize = 0;
                let mut last_reduced_item: Option<converters::JsonValue> = None;

                loop {
                    select_loop! {
                        recv(mapreduce_in_receiver, vertex) => {
                            pending_tasks += 1;
                            worker_in_sender.send(MapReduceWorkerTask::Map(vertex)).unwrap();
                        },
                        recv(worker_out_receiver, value) => {
                            pending_tasks -= 1;

                            if let Some(last_reduced_item_inner) = last_reduced_item {
                                pending_tasks += 1;
                                worker_in_sender.send(MapReduceWorkerTask::Reduce((last_reduced_item_inner, value)));
                                last_reduced_item = None;
                            } else {
                                last_reduced_item = Some(value);
                            }
                        },
                        recv(shutdown_receiver, _) => {
                            should_shutdown = true;
                        },
                        timed_out(Duration::from_secs(WORKER_CHANNEL_RECV_TIMEOUT_SECONDS)) => {
                            if should_shutdown && pending_tasks == 0 {
                                // Join all threads and check for any errors
                                let result: Result<Vec<Result<(), errors::ScriptError>>, errors::ScriptError> = worker_threads.into_iter().map(|t| t.join()).collect().map_err(|err| {
                                    errors::ScriptError::ThreadPanic { err }
                                });

                                result?.collect()?;

                                // Get the final value to return
                                let final_value = match last_reduced_item_inner {
                                    // This should only happen if the graph is empty
                                    None => JsonValue::Null,
                                    // This should always ahppen otherwise
                                    Some(value) => value.0
                                };

                                return Ok(final_value)
                            }
                        }
                    }
                }
            })
        };

        Self {
            router_thread: router_thread,
            in_sender: in_sender,
            shutdown_sender: shutdown_sender
        }
    }

    fn add_vertex(&self, vertex: Vertex) {
        self.in_sender.send(vertex).unwrap();
    }

    fn join(self) -> Result<JsonValue, errors::ScriptError> {
        self.shutdown_sender.send(()).unwrap();
        self.router_thread.join().map_err(|err| errors::ScriptError::ThreadPanic { err })?
    }
}
