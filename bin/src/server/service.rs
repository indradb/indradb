use common::{ProxyDatastore, datastore};
use std::sync::Arc;
use transaction_request;
use transaction_response;
use service_grpc;
use grpcio;
use indradb;
use indradb::Datastore;
use futures::{Future, Sink, Stream};

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
    fn transaction(&self, ctx: grpcio::RpcContext, stream: grpcio::RequestStream<transaction_request::TransactionRequest>, sink: grpcio::DuplexSink<transaction_response::TransactionResponse>) {
        let datastore = self.datastore.clone();
        let trans = datastore.transaction();

        let responses = stream
            .map(|request| {
                let response = if request.has_create_vertex() {
                    let request = request.get_create_vertex();
                    let vertex = indradb::Vertex::from(request.get_vertex());
                    let result = trans.create_vertex(&vertex);
                    let mut inner_response = transaction_response::CreateVertexResponse::new();

                    if let Err(err) = result {
                        inner_response.set_error(format!("{}", err));
                    }

                    let mut response = transaction_response::TransactionResponse::new();
                    response.set_create_vertex(inner_response);
                    response
                } else if request.has_get_vertices() {
                    let request = request.get_get_vertices();
                } else if request.has_delete_vertices() {
                    let request = request.get_delete_vertices();
                } else if request.has_get_vertex_count() {
                    let request = request.get_get_vertex_count();
                } else if request.has_create_edge() {
                    let request = request.get_create_edge();
                } else if request.has_get_edges() {
                    let request = request.get_get_edges();
                } else if request.has_delete_edges() {
                    let request = request.get_delete_edges();
                } else if request.has_get_edge_count() {
                    let request = request.get_get_edge_count();
                } else if request.has_get_global_metadata() {
                    let request = request.get_get_global_metadata();
                } else if request.has_set_global_metadata() {
                    let request = request.get_set_global_metadata();
                } else if request.has_delete_global_metadata() {
                    let request = request.get_delete_global_metadata();
                } else if request.has_get_vertex_metadata() {
                    let request = request.get_get_vertex_metadata();
                } else if request.has_set_vertex_metadata() {
                    let request = request.get_set_vertex_metadata();
                } else if request.has_delete_vertex_metadata() {
                    let request = request.get_delete_vertex_metadata();
                } else if request.has_get_edge_metadata() {
                    let request = request.get_get_edge_metadata();
                } else if request.has_set_edge_metadata() {
                    let request = request.get_set_edge_metadata();
                } else if request.has_delete_edge_metadata() {
                    let request = request.get_delete_edge_metadata();
                } else {
                    panic!("Unexpected request: {:?}", request);
                };
                
                (response, grpcio::WriteFlags::default())
            });

        let f = sink.send_all(responses)
            .map(|_| {})
            .map_err(|err| println!("Error: {}", err));

        ctx.spawn(f)
    }
}
