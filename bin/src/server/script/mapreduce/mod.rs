mod counter;
mod master;
mod response_chan;
mod worker;

pub use self::response_chan::{Update, ResponseSender, ResponseReceiver, bounded};

use std::thread::{spawn, sleep};
use statics;
use uuid::Uuid;
use self::counter::Counter;
use indradb::VertexQuery;
use serde_json::Value as JsonValue;
use std::time::Duration;
use indradb::{Datastore, Transaction};

lazy_static! {
    static ref REPORT_TIME: Duration = Duration::from_secs(10);
}

pub fn execute_mapreduce(contents: String, path: String, arg: JsonValue, sender: ResponseSender) {
    let sent = Counter::new();
    let processing = Counter::new();
    let finished = Counter::new();

    // Spawn a thread to feed updates
    {
        let sender = sender.clone();
        let sent = sent.clone();
        let processing = processing.clone();
        let finished = finished.clone();

        spawn(move || {
            loop {
                sleep(*REPORT_TIME);

                let message = json!({
                    "sent": sent.get(),
                    "processing": processing.get(),
                    "finished": finished.get()
                });

                if sender.0.send(Update::Ping(message)).is_err() {
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
                sender.0.send(Update::Err(json!({"error": message}))).ok();
                return;
            }
        };

        let mapreducer = master::Master::start(contents, path, arg, sent, processing, finished);
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
}