// Ignored because of warnings in crossbeam that we cannot control.
// TODO: remove after crossbeam fixes this.
#![cfg_attr(feature = "cargo-clippy", allow(deref_addrof, never_loop))]

use super::counter::Counter;
use super::worker::{Worker, WorkerError, WorkerTask};
use crossbeam_channel::{bounded, unbounded, Sender};
use indradb::Vertex;
use script::converters;
use serde_json::value::Value as JsonValue;
use statics;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

const CHANNEL_TIMEOUT: u64 = 5;
const CHANNEL_CAPACITY: usize = 1000;

pub struct Master {
    router_thread: JoinHandle<Result<JsonValue, WorkerError>>,
    in_sender: Sender<Vertex>,
    shutdown_sender: Sender<()>,
    sent: Counter,
}

impl Master {
    pub fn start(
        contents: &str,
        path: &str,
        arg: &JsonValue,
        sent: Counter,
        processing: &Counter,
        finished: &Counter,
    ) -> Self {
        let (master_in_sender, master_in_receiver) = bounded::<Vertex>(CHANNEL_CAPACITY);
        let (worker_in_sender, worker_in_receiver) = bounded::<WorkerTask>(CHANNEL_CAPACITY);
        let (worker_out_sender, worker_out_receiver) = unbounded::<converters::JsonValue>();
        let (error_sender, error_receiver) = bounded::<WorkerError>(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);
        let (shutdown_sender, shutdown_receiver) = bounded::<()>(1);
        let mut worker_threads: Vec<Worker> = Vec::with_capacity(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);

        for _ in 0..*statics::MAP_REDUCE_WORKER_POOL_SIZE {
            worker_threads.push(Worker::start(
                contents.to_string(),
                path.to_string(),
                arg.clone(),
                worker_in_receiver.clone(),
                worker_out_sender.clone(),
                error_sender.clone(),
            ));
        }

        let router_thread = {
            let processing = processing.clone();
            let finished = finished.clone();

            spawn(move || -> Result<JsonValue, WorkerError> {
                let mut should_force_shutdown = false;
                let mut should_gracefully_shutdown = false;
                let mut last_reduced_item: Option<converters::JsonValue> =
                    Some(converters::JsonValue::new(JsonValue::Null));
                let mut last_error: Option<WorkerError> = None;

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
                            finished.increment();
                            processing.decrement();

                            if let Some(last_reduced_item_inner) = last_reduced_item {
                                // If this errors out, all of the workers are dead
                                if worker_in_sender.send(WorkerTask::Reduce((last_reduced_item_inner, value))).is_err() {
                                    should_force_shutdown = true;
                                }

                                processing.increment();
                                last_reduced_item = None;
                            } else {
                                last_reduced_item = Some(value);
                            }
                        },
                        recv(master_in_receiver, vertex) => {
                            // If this errors out, all of the workers are dead
                            if worker_in_sender.send(WorkerTask::Map(vertex)).is_err() {
                                should_force_shutdown = true;
                            }

                            processing.increment();
                        },
                        timed_out(Duration::from_secs(CHANNEL_TIMEOUT)) => {}
                    }

                    // Check to see if we should shutdown
                    if should_force_shutdown
                        || (should_gracefully_shutdown && processing.get() == 0 && master_in_receiver.is_empty())
                    {
                        // Join all threads
                        for worker_thread in worker_threads {
                            worker_thread.join();
                        }

                        return if should_force_shutdown {
                            // If it's a hard error, find an error to return
                            Err(last_error.unwrap_or_else(|| {
                                error_receiver
                                    .try_recv()
                                    .expect("Expected to be able to read the error channel")
                            }))
                        } else {
                            // Get the final value to return
                            Ok(match last_reduced_item {
                                // This should only happen if the graph is empty
                                None => JsonValue::Null,
                                // This should always happen otherwise
                                Some(value) => value.0,
                            })
                        };
                    }
                }
            })
        };

        Self {
            router_thread,
            in_sender: master_in_sender,
            shutdown_sender,
            sent,
        }
    }

    pub fn add_vertex(&self, vertex: Vertex) -> bool {
        let ok = self.in_sender.send(vertex).is_ok();
        self.sent.increment();
        ok
    }

    pub fn join(self) -> Result<JsonValue, WorkerError> {
        self.shutdown_sender.send(()).ok();
        self.router_thread.join().expect("Expected router thread to not panic")
    }
}

#[cfg(test)]
mod tests {
    use super::super::Counter;
    use super::Master;
    use indradb::{Type, Vertex};
    use serde_json::Value as JsonValue;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    fn run(insert_count: u64, expected_finish_count: u64, expected_result: JsonValue) {
        let file_path_str = "test_scripts/mapreduce/count.lua";
        let file_path = Path::new(file_path_str);
        let mut file = File::open(file_path).expect("Could not open script file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not get script file contents");

        let sent = Counter::new();
        let processing = Counter::new();
        let finished = Counter::new();

        let engine = Master::start(
            &contents,
            file_path_str,
            &json!(2),
            sent.clone(),
            &processing,
            &finished,
        );

        for _ in 0..insert_count {
            let t = Type::new("foo".to_string()).unwrap();
            engine.add_vertex(Vertex::new(t));
        }

        assert_eq!(engine.join().unwrap(), expected_result);
        assert_eq!(sent.get(), insert_count);
        assert_eq!(processing.get(), 0);
        assert_eq!(finished.get(), expected_finish_count);
    }

    #[test]
    fn should_handle_zero_items() {
        run(0, 0, JsonValue::Null);
    }

    #[test]
    fn should_handle_one_item() {
        run(1, 2, json!(3.0));
    }

    #[test]
    fn should_handle_many_even_items() {
        run(6, 12, json!(13.0));
    }

    #[test]
    fn should_handle_many_odd_items() {
        run(5, 10, json!(11.0));
    }
}
