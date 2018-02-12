use iron::prelude::*;
use iron::status;
use indradb::{EdgeKey, EdgeQuery, Error, Transaction, Type, VertexQuery, Datastore};
use common::ProxyTransaction;
use serde_json::value::Value as JsonValue;
use serde_json;
use serde::ser::Serialize;
use std::u16;
use statics;
use script;
use super::util::*;
use std::thread::{spawn, sleep};
use iron::typemap::TypeMap;
use iron::headers::{ContentType, Headers, Encoding, TransferEncoding};
use super::response_chan::{bounded, Update};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::time::Duration;

lazy_static! {
    static ref REPORT_TIME: Duration = Duration::from_secs(10);
}

pub fn script(req: &mut Request) -> IronResult<Response> {
    // Get the inputs
    let name: String = get_url_param(req, "name")?;
    let payload = read_json(&mut req.body)?.unwrap_or_else(|| JsonValue::Null);
    let (path, contents) = get_script_file(name)?;

    match script::execute(contents, path, payload) {
        Ok(value) => Ok(to_response(status::Ok, &value)),
        Err(err) => {
            let error_message = format!("Script failed: {:?}", err);
            Err(create_iron_error(
                status::InternalServerError,
                error_message,
            ))
        }
    }
}

pub fn mapreduce(req: &mut Request) -> IronResult<Response> {
    // Get the inputs
    let name: String = get_url_param(req, "name")?;
    let payload = read_json(&mut req.body)?.unwrap_or_else(|| JsonValue::Null);
    let (path, contents) = get_script_file(name)?;

    // Construct a response
    let mut hs = Headers::new();
    hs.set(ContentType(get_json_mime()));
    hs.set(TransferEncoding(vec![Encoding::Chunked]));

    let (sender, receiver) = bounded(1);

    let response = Response {
        status: Some(status::Ok),
        headers: hs,
        extensions: TypeMap::new(),
        body: Some(Box::new(receiver))
    };

    let queued: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    // Spawn a thread to feed updates
    {
        let queued = queued.clone();
        let sender = sender.clone();

        spawn(move || {
            loop {
                sleep(*REPORT_TIME);
                let cur_queued = { *queued.lock().unwrap() };
                if sender.0.send(Update::Ping(json!(cur_queued))).is_err() {
                    return;
                }
            }
        });
    }

    // Spawn a thread to stream to the response
    spawn(move || {
        let trans = match statics::DATASTORE.transaction() {
            Ok(trans) => trans,
            Err(err) => {
                let message = format!("Query setup failed: {:?}", err);
                sender.0.send(Update::Err(json!({"error": message}))).expect("Expected send channel to be open");
                return;
            }
        };

        let mapreducer = script::MapReducer::start(contents, path, payload);
        let mut last_id: Option<Uuid> = None;

        loop {
            let q = VertexQuery::All { start_id: last_id, limit: *statics::MAP_REDUCE_QUERY_LIMIT };

            let vertices = match trans.get_vertices(q) {
                Ok(vertices) => vertices,
                Err(err) => {
                    let message = format!("Query failed: {:?}", err);
                    sender.0.send(Update::Err(json!({"error": message}))).ok();
                    break;
                }
            };

            // Returned less than the expected number of results, implying that
            // the next query will not have any results
            let mut done = vertices.len() < *statics::MAP_REDUCE_QUERY_LIMIT as usize;

            if let Some(last_vertex) = vertices.last() {
                last_id = Some(last_vertex.id);
            }

            for vertex in vertices.into_iter() {
                // Add the vertex to the queue
                if !mapreducer.add_vertex(vertex) {
                    // The vertex couldn't be added, which means the channel is
                    // disconnected. This can only be caused if all of the workers
                    // failed, at which point we need to bail.
                    done = true;
                    break;
                }

                let mut queued = queued.lock().unwrap();
                *queued += 1;
            }

            if done {
                break;
            }
        }

        match mapreducer.join() {
            Ok(value) => {
                sender.0.send(Update::Ok(value)).ok();
            },
            Err(err) => {
                let message = format!("Mapreduce failed: {:?}", err);
                sender.0.send(Update::Err(json!({"error": message}))).ok();
            }
        }
    });

    Ok(response)
}

pub fn transaction(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let mut idx: u16 = 0;
    let mut jsonable_res: Vec<JsonValue> = Vec::new();
    let body: Vec<JsonValue> = read_json(&mut req.body)?.unwrap_or_else(|| Vec::new());

    for item in body {
        if let JsonValue::Object(obj) = item {
            let action = get_json_obj_value::<String>(&obj, "action").map_err(|err| {
                let message = format!("Item #{}: {}", idx, err);
                create_iron_error(status::BadRequest, message)
            })?;

            let result: Result<JsonValue, IronError> = match &action[..] {
                "create_vertex" => create_vertex(&trans, &obj),
                "get_vertices" => get_vertices(&trans, &obj),
                "delete_vertices" => delete_vertices(&trans, &obj),

                "create_edge" => create_edge(&trans, &obj),
                "get_edges" => get_edges(&trans, &obj),
                "delete_edges" => delete_edges(&trans, &obj),
                "get_edge_count" => get_edge_count(&trans, &obj),

                "get_global_metadata" => get_global_metadata(&trans, &obj),
                "set_global_metadata" => set_global_metadata(&trans, &obj),
                "delete_global_metadata" => delete_global_metadata(&trans, &obj),

                "get_vertex_metadata" => get_vertex_metadata(&trans, &obj),
                "set_vertex_metadata" => set_vertex_metadata(&trans, &obj),
                "delete_vertex_metadata" => delete_vertex_metadata(&trans, &obj),

                "get_edge_metadata" => get_edge_metadata(&trans, &obj),
                "set_edge_metadata" => set_edge_metadata(&trans, &obj),
                "delete_edge_metadata" => delete_edge_metadata(&trans, &obj),

                _ => Err(create_iron_error(
                    status::BadRequest,
                    "Unknown action".to_string(),
                )),
            };

            match result {
                Err(err) => {
                    let message = format!("Item #{}: {}", idx, err);
                    return Err(create_iron_error(status::BadRequest, message));
                }
                Ok(value) => {
                    jsonable_res.push(value);
                }
            }
        } else {
            return Err(create_iron_error(
                status::BadRequest,
                format!("Item #{}: Invalid type", idx),
            ));
        }

        idx += 1;
    }

    trans.commit().map_err(|err| convert_to_iron_error(&err))?;
    Ok(to_response(status::Ok, &jsonable_res))
}

fn create_vertex(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let t = get_json_obj_value::<Type>(item, "type")?;
    execute_item(trans.create_vertex(t))
}

fn get_vertices(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    execute_item(trans.get_vertices(q))
}

fn delete_vertices(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    execute_item(trans.delete_vertices(q))
}

fn create_edge(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let key = get_json_obj_value::<EdgeKey>(item, "key")?;
    execute_item(trans.create_edge(key))
}

fn get_edges(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    execute_item(trans.get_edges(q))
}

fn delete_edges(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    execute_item(trans.delete_edges(q))
}

fn get_edge_count(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    execute_item(trans.get_edge_count(q))
}

fn get_global_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.get_global_metadata(name))
}

fn set_global_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let name = get_json_obj_value::<String>(item, "name")?;
    let value = get_json_obj_value::<JsonValue>(item, "value")?;
    execute_item(trans.set_global_metadata(name, value))
}

fn delete_global_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.delete_global_metadata(name))
}

fn get_vertex_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.get_vertex_metadata(q, name))
}

fn set_vertex_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    let value = get_json_obj_value::<JsonValue>(item, "value")?;
    execute_item(trans.set_vertex_metadata(q, name, value))
}

fn delete_vertex_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.delete_vertex_metadata(q, name))
}

fn get_edge_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.get_edge_metadata(q, name))
}

fn set_edge_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    let value = get_json_obj_value::<JsonValue>(item, "value")?;
    execute_item(trans.set_edge_metadata(q, name, value))
}

fn delete_edge_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.delete_edge_metadata(q, name))
}

fn execute_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
    let value = result.map_err(|err| convert_to_iron_error(&err))?;

    Ok(
        serde_json::to_value(value).map_err(|err| {
            create_iron_error(
                status::InternalServerError,
                format!("Could not serialize results: {}", err),
            )
        })?,
    )
}
