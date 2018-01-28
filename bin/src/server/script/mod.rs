mod api;
mod converters;
mod errors;

#[cfg(test)]
mod tests;

use rlua::{Table, Value, Function};
use rlua::prelude::*;
use serde_json::value::Value as JsonValue;
use std::path::Path;
use uuid::Uuid;
use indradb::{Transaction, Vertex, Datastore, VertexQuery};
use statics;
use std::convert::From;
use std::sync::atomic::{AtomicBool, Ordering};
use crossbeam_channel::{Receiver, Sender, bounded};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

fn create_lua_context(account_id: Uuid, arg: JsonValue) -> Result<Lua, errors::ScriptError> {
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

    Ok(l)
}

/// Runs a script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn execute(
    account_id: Uuid,
    contents: String,
    path: String,
    arg: JsonValue,
) -> Result<JsonValue, errors::ScriptError> {
    let l = create_lua_context(account_id, arg)?;
    let fun = l.load(&contents, Some(&path))?;

    // Run the script
    let value: Result<converters::JsonValue, LuaError> = fun.call(Value::Nil);

    match value {
        Ok(value) => Ok(value.0),
        Err(err) => Err(errors::ScriptError::from(err))
    }
}

fn mapreduce_worker(account_id: Uuid, reductions_left: Arc<Mutex<u64>>, contents: String, path: String, arg: JsonValue, mapper_receiver: Receiver<Vertex>, reducer_sender: Sender<converters::JsonValue>, reducer_receiver: Receiver<converters::JsonValue>, shutdown_receiver: Receiver<()>) -> Result<Option<JsonValue>, errors::ScriptError> {
    let l = create_lua_context(account_id, arg)?;
    let fun = l.load(&contents, Some(&path))?;
    let (mapper, reducer): (Function, Function) = fun.call(Value::Nil)?;
    let mut last_reducer_value: Option<converters::JsonValue> = None;

    loop {
        select_loop! {
            recv(mapper_receiver, vertex) => {
                let value = mapper.call(converters::Vertex::new(vertex))?;
                reducer_sender.send(value).unwrap();
            }
            recv(reducer_receiver, value) => {
                let cur_reductions_left = {
                    let mut reductions_left = reductions_left.lock().unwrap();
                    *reductions_left -= 1;
                    *reductions_left
                };

                match (last_reducer_value, cur_reductions_left) {
                    (Some(last_reducer_value_inner), 0) => {
                        let reduced_value: converters::JsonValue = reducer.call((last_reducer_value_inner, value))?;
                        return Ok(Some(reduced_value.0));
                    },
                    (Some(last_reducer_value_inner), _) => {
                        let reduced_value: converters::JsonValue = reducer.call((last_reducer_value_inner, value))?;
                        reducer_sender.send(reduced_value).unwrap();
                        last_reducer_value = None;
                    },
                    (None, 0) => {
                        return Ok(Some(value.0));
                    },
                    (None, _) => {
                        last_reducer_value = Some(value);
                    }
                }
            }
            recv(shutdown_receiver, _) => {
                return Ok(None);
            }
        }
    }
}

fn mapreduce_query(account_id: Uuid, reductions_left: Arc<Mutex<u64>>, finished: Arc<AtomicBool>, mapper_sender: Sender<Vertex>) -> Result<bool, errors::ScriptError> {
    let trans = statics::DATASTORE.transaction(account_id)?;
    let mut last_id: Option<Uuid> = None;
    let mut first_query = true;

    loop {
        let q = VertexQuery::All { start_id: last_id, limit: *statics::MAP_REDUCE_QUERY_LIMIT };
        let vertices = trans.get_vertices(q)?;
        let num_vertices = vertices.len() as u32;

        if num_vertices > 0 {
            last_id = Some(vertices.last().unwrap().id);

            {
                let mut reductions_left = reductions_left.lock().unwrap();
                let old_reductions_left = *reductions_left;
                *reductions_left += num_vertices as u64;
                
                // Check for overflow
                assert!(*reductions_left > old_reductions_left);

                // The number of reductions is supposed to be the number of
                // vertices minus 1, so subtract the 1 if this is the first query
                if first_query {
                    *reductions_left -= 1;
                }
            }

            for (i, vertex) in vertices.into_iter().enumerate() {
                // Keep checking that none of the threads bailed, because
                // otherwise we could get blocked queuing up items into the
                // channel w/o any workers to handle them
                if finished.load(Ordering::SeqCst) {
                    return Ok(!first_query || i > 0);
                }

                mapper_sender.send(vertex).unwrap();
            }
        }

        // Returned less than the expected number of results, implying that
        // the next query will not have any results
        if num_vertices < *statics::MAP_REDUCE_QUERY_LIMIT {
            return Ok(!first_query || num_vertices > 0);
        }

        first_query = false;
    }
}

/// Runs a mapreduce script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn mapreduce(
    account_id: Uuid,
    contents: String,
    path: String,
    arg: JsonValue,
) -> Result<JsonValue, errors::ScriptError> {
    // Defines channels used in the various phases of map/reduce:
    // 1) The channel for the mapping phase.
    // 2) The channel for the reducing phase.
    // 3) The channel for sending shutdown orders to workers.
    //
    // For the mapper channel, the capacity is set so that there's enough
    // room to fill the entire channel with the full results of a single
    // query, and then make the next query to prepare for sending. The idea
    // behind this heuristic is to try to prevent IndraDB queries from being
    // the bottleneck, because the results of a query should more or less
    // always be ready to be queued up into the channel.
    //
    // The reducer channel's capacity is the same as the mapper channel's.
    // This should be more than enough, and will prevent deadlock states, i.e.
    // because all of the workers get stuck in a state of trying to add to the
    // reducer channel at the same time even though it's full.
    //
    // The shutdown channel's capacity is the worker pool size, since a
    // message will be sent to each of the workers.
    let (mapper_sender, mapper_receiver) = bounded::<Vertex>(*statics::MAP_REDUCE_QUERY_LIMIT as usize);
    let (reducer_sender, reducer_receiver) = bounded::<converters::JsonValue>(*statics::MAP_REDUCE_QUERY_LIMIT as usize);
    let (shutdown_sender, shutdown_receiver) = bounded::<()>(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);

    // A list of all the threads
    let mut worker_threads: Vec<thread::JoinHandle<Result<Option<JsonValue>, errors::ScriptError>>> = Vec::with_capacity(*statics::MAP_REDUCE_WORKER_POOL_SIZE as usize);

    // The number of reductions left
    let reductions_left = Arc::new(Mutex::<u64>::new(0));

    // Notification for when the last reduction is done
    let finished = Arc::new(AtomicBool::new(false));

    for _ in 0..*statics::MAP_REDUCE_WORKER_POOL_SIZE {
        let finished = finished.clone();
        let reductions_left = reductions_left.clone();
        let contents = contents.clone();
        let path = path.clone();
        let arg = arg.clone();
        let mapper_receiver = mapper_receiver.clone();
        let reducer_sender = reducer_sender.clone();
        let reducer_receiver = reducer_receiver.clone();
        let shutdown_receiver = shutdown_receiver.clone();

        worker_threads.push(thread::spawn(move || -> Result<Option<JsonValue>, errors::ScriptError> {
            let result = mapreduce_worker(account_id, reductions_left, contents, path, arg, mapper_receiver, reducer_sender, reducer_receiver, shutdown_receiver);
            finished.store(true, Ordering::SeqCst);
            result
        }));
    }

    // Run the query
    let enqueued_any = mapreduce_query(account_id, reductions_left, finished.clone(), mapper_sender);

    // Wait until one of the processes has exited
    // TODO: this is a busy-waiting loop and could be optimized
    if let Ok(true) = enqueued_any {
        loop {
            if finished.load(Ordering::Relaxed) {
                break;
            } else {
                thread::sleep(Duration::from_millis(1000));
            }
        }
    }

    // Send the shutdown notifications
    for _ in 0..*statics::MAP_REDUCE_WORKER_POOL_SIZE+1 {
        shutdown_sender.send(()).unwrap();
    }

    // Get the results. The function will panic if any of the workers panicked
    // or none of the workers returned a result, since neither should happen.
    let results: Result<Vec<Option<JsonValue>>, errors::ScriptError> = worker_threads.into_iter().map(|t| t.join().expect("Expected threads not to panic. This is a bug.")).collect();
    let result = results?.into_iter().filter_map(|r| r).next();

    // Return the final result. Note that we're checking for errors from
    // `mapreduce_query` here, because if it is an error, we still sent the
    // shutdown signal to workers.
    match (enqueued_any?, result) {
        (true, Some(value)) => Ok(value),
        (true, None) => panic!("None of the workers returned results. This is a bug."),
        (false, Some(_)) => panic!("A worker returned a result even though map/reduce was called on an empty graph. This is a bug."),
        (false, None) => Ok(JsonValue::Null)
    }
}
