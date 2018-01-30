mod api;
mod converters;
mod errors;
mod workers;

#[cfg(test)]
mod tests;

use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use indradb::{Transaction, Datastore, VertexQuery};
use statics;

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
    let l = workers::create_lua_context(account_id, arg)?;
    let value: converters::JsonValue = l.exec(&contents, Some(&path))?;
    Ok(value.0)
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
) -> Result<JsonValue, errors::MapReduceError> {
    let pool = workers::MapReduceWorkerPool::start(account_id, contents, path, arg);
    let trans = statics::DATASTORE.transaction(account_id).map_err(|err| errors::MapReduceError::Query(err))?;
    let mut last_id: Option<Uuid> = None;

    loop {
        let q = VertexQuery::All { start_id: last_id, limit: *statics::MAP_REDUCE_QUERY_LIMIT };
        let vertices = trans.get_vertices(q).map_err(|err| errors::MapReduceError::Query(err))?;
        let num_vertices = vertices.len() as u32;

        if let Some(last_vertex) = vertices.last() {
            last_id = Some(last_vertex.id);
        }

        for vertex in vertices.into_iter() {
            if !pool.add_vertex(vertex) {
                // The vertex couldn't be added, which means the channel is
                // disconnected. This can only be caused if all of the workers
                // failed, at which point we need to bail.
                break;
            }
        }

        // Returned less than the expected number of results, implying that
        // the next query will not have any results
        if num_vertices < *statics::MAP_REDUCE_QUERY_LIMIT {
            break;
        }
    }

    pool.join()
}
