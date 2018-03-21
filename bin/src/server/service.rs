use common::{ProxyDatastore, datastore};
use std::sync::Arc;
use request;
use response;
use service_grpc;
use grpcio;
use indradb;
use converters::ReverseFrom;
use indradb::{Datastore, Transaction};
use errors::{Result, Error};
use futures::{Future, Sink, Stream};

fn build_response(trans: &Transaction, request: request::TransactionRequest) -> Result<response::TransactionResponse> {
    let response = if request.has_create_vertex() {
        let request = request.get_create_vertex();
        let vertex = indradb::Vertex::reverse_from(request.take_vertex())?;
        let result = trans.create_vertex(&vertex)?;
        let mut response = response::TransactionResponse::new();
        response.set_ok(true);
        response
    } else if request.has_get_vertices() {
        let request = request.get_get_vertices();
        unimplemented!();
    } else if request.has_delete_vertices() {
        let request = request.get_delete_vertices();
        unimplemented!();
    } else if request.has_get_vertex_count() {
        let request = request.get_get_vertex_count();
        unimplemented!();
    } else if request.has_create_edge() {
        let request = request.get_create_edge();
        unimplemented!();
    } else if request.has_get_edges() {
        let request = request.get_get_edges();
        unimplemented!();
    } else if request.has_delete_edges() {
        let request = request.get_delete_edges();
        unimplemented!();
    } else if request.has_get_edge_count() {
        let request = request.get_get_edge_count();
        unimplemented!();
    } else if request.has_get_global_metadata() {
        let request = request.get_get_global_metadata();
        unimplemented!();
    } else if request.has_set_global_metadata() {
        let request = request.get_set_global_metadata();
        unimplemented!();
    } else if request.has_delete_global_metadata() {
        let request = request.get_delete_global_metadata();
        unimplemented!();
    } else if request.has_get_vertex_metadata() {
        let request = request.get_get_vertex_metadata();
        unimplemented!();
    } else if request.has_set_vertex_metadata() {
        let request = request.get_set_vertex_metadata();
        unimplemented!();
    } else if request.has_delete_vertex_metadata() {
        let request = request.get_delete_vertex_metadata();
        unimplemented!();
    } else if request.has_get_edge_metadata() {
        let request = request.get_get_edge_metadata();
        unimplemented!();
    } else if request.has_set_edge_metadata() {
        let request = request.get_set_edge_metadata();
        unimplemented!();
    } else if request.has_delete_edge_metadata() {
        let request = request.get_delete_edge_metadata();
        unimplemented!();
    } else {
        panic!("Unexpected request: {:?}", request);
    };

    Ok(response)
}

#[derive(Clone)]
pub struct IndraDbService {
    datastore: Arc<ProxyDatastore>
}

impl IndraDbService {
    pub fn new() -> Self {
        Self { datastore: Arc::new(datastore()) }
    }
}

impl service_grpc::IndraDb for IndraDbService {
    fn transaction(&self, ctx: grpcio::RpcContext, stream: grpcio::RequestStream<request::TransactionRequest>, sink: grpcio::DuplexSink<response::TransactionResponse>) {
        let datastore = self.datastore.clone();

        let trans = match datastore.transaction() {
            Ok(trans) => trans,
            Err(err) => {
                eprintln!("Error setting up transaction: {}", err);
                return;
            }
        };

        let responses = stream
            .map(|request| {
                let response = match build_response(&trans, request) {
                    Ok(response) => response,
                    Err(err) => {
                        let mut response = response::TransactionResponse::new();
                        response.set_error(format!("{}", err));
                        response
                    }
                };

                (response, grpcio::WriteFlags::default())
            });

        let f = sink.send_all(responses)
            .map(|_| {})
            .map_err(|err| eprintln!("Error sending response: {}", err));

        ctx.spawn(f)
    }
}
