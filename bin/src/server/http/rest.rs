use iron::prelude::*;
use iron::status;
use indradb::{EdgeKey, EdgeQuery, Transaction, Type, VertexQuery};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use script;
use std::time;
use indradb::Datastore;
use std::thread::{spawn, sleep};
use iron::typemap::TypeMap;
use iron::headers::{ContentType, Headers, Encoding, TransferEncoding};
use super::response_chan::{bounded, Update};
use super::util::*;
use std::sync::{Arc, Mutex};
use statics;

lazy_static! {
    static ref REPORT_TIME: time::Duration = time::Duration::from_secs(10);
}

pub fn create_vertex(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let t = get_query_param::<Type>(query_params, "type", true)?.unwrap();
    let response = datastore_request(trans.create_vertex(t))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn get_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<VertexQuery>(query_params)?;
    let response = datastore_request(trans.get_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn delete_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<VertexQuery>(query_params)?;
    datastore_request(trans.delete_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn create_edge(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "t")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let query_params = get_query_params(req)?;
    let weight = get_weight_query_param(query_params)?;
    let key = EdgeKey::new(outbound_id, t, inbound_id);
    datastore_request(trans.create_edge(key, weight))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn get_edges(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<EdgeQuery>(query_params)?;
    let action = get_query_param::<String>(query_params, "action", false)?;

    if action == Some("count".to_string()) {
        let response = datastore_request(trans.get_edge_count(q))?;
        Ok(to_response(status::Ok, &response))
    } else {
        let response = datastore_request(trans.get_edges(q))?;
        Ok(to_response(status::Ok, &response))
    }
}

pub fn delete_edges(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<EdgeQuery>(query_params)?;
    datastore_request(trans.delete_edges(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn script(req: &mut Request) -> IronResult<Response> {
    let name: String = get_url_param(req, "name")?;

    let payload = match read_optional_json(&mut req.body)? {
        Some(val) => val,
        None => JsonValue::Null,
    };

    let account_id = get_account_id(req);
    let (path, contents) = get_script_file(name)?;

    match script::execute(account_id, contents, path, payload) {
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

    let payload = match read_optional_json(&mut req.body)? {
        Some(val) => val,
        None => JsonValue::Null,
    };

    let account_id = get_account_id(req);
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
        let trans = match statics::DATASTORE.transaction(account_id) {
            Ok(trans) => trans,
            Err(err) => {
                let message = format!("Query setup failed: {:?}", err);
                sender.0.send(Update::Err(json!({"error": message}))).expect("Expected send channel to be open");
                return;
            }
        };

        let mapreducer = script::MapReducer::start(account_id, contents, path, payload);
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
