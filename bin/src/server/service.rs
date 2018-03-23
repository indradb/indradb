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
use std::error::Error as StdError;

fn build_response(trans: &Transaction, request: &request::TransactionRequest) -> Result<(response::TransactionResponse, grpcio::WriteFlags)> {
    let response = if request.has_create_vertex() {
        let request = request.get_create_vertex();
        let vertex = indradb::Vertex::reverse_from(request.get_vertex().clone())?;
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

    Ok((response, grpcio::WriteFlags::default()))
}

fn build_error_response<E: StdError>(err: &E) -> (response::TransactionResponse, grpcio::WriteFlags) {
    let mut response = response::TransactionResponse::new();
    response.set_error(format!("{}", err));
    (response, grpcio::WriteFlags::default())
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
    fn transaction(&self, ctx: grpcio::RpcContext, stream: grpcio::RequestStream<request::TransactionRequest>, mut sink: grpcio::DuplexSink<response::TransactionResponse>) {
        let datastore = self.datastore.clone();
        let trans = datastore.transaction().unwrap();

        for result in stream.wait() {
            let response = match result {
                Ok(request) => build_response(&trans, &request).unwrap_or_else(|err| build_error_response(&err)),
                Err(err) => build_error_response(&err)
            };

            sink = sink.send(response).wait().unwrap();
        }
    }
}
