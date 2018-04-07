use autogen;
use common::{datastore, ProxyDatastore};
use converters;
use converters::ReverseFrom;
use errors::Result;
use futures::{Future, Sink, Stream};
use grpcio;
use indradb;
use indradb::{Datastore, Transaction};
use protobuf;
use serde_json;
use std::error::Error as StdError;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::spawn;
use uuid::Uuid;

macro_rules! send_sink {
    ($sink:ident, $response:expr) => (
        match $sink.send($response).wait() {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Could not send response: {}", err);
                return;
            }
        }
    )
}

fn build_response(
    trans: &Transaction,
    request: &autogen::TransactionRequest,
) -> Result<(autogen::TransactionResponse, grpcio::WriteFlags)> {
    let mut response = autogen::TransactionResponse::new();

    if request.has_create_vertex() {
        let request = request.get_create_vertex();
        let vertex = indradb::Vertex::reverse_from(request.get_vertex())?;
        trans.create_vertex(&vertex)?;
        response.set_ok(true);
    } else if request.has_create_vertex_from_type() {
        let request = request.get_create_vertex_from_type();
        let t = indradb::Type::new(request.get_field_type().to_string())?;
        let id = trans.create_vertex_from_type(t)?;
        response.set_uuid(id.hyphenated().to_string());
    } else if request.has_get_vertices() {
        let request = request.get_get_vertices();
        let query = indradb::VertexQuery::reverse_from(request.get_query())?;
        let mut vertices = autogen::Vertices::new();
        let list = trans
            .get_vertices(&query)?
            .into_iter()
            .map(autogen::Vertex::from)
            .collect();
        vertices.set_vertices(protobuf::RepeatedField::from_vec(list));
        response.set_vertices(vertices);
    } else if request.has_delete_vertices() {
        let request = request.get_delete_vertices();
        let query = indradb::VertexQuery::reverse_from(request.get_query())?;
        trans.delete_vertices(&query)?;
        response.set_ok(true);
    } else if request.has_get_vertex_count() {
        let count = trans.get_vertex_count()?;
        response.set_count(count);
    } else if request.has_create_edge() {
        let request = request.get_create_edge();
        let key = indradb::EdgeKey::reverse_from(request.get_key())?;
        let created = trans.create_edge(&key)?;
        response.set_ok(created);
    } else if request.has_get_edges() {
        let request = request.get_get_edges();
        let query = indradb::EdgeQuery::reverse_from(request.get_query())?;
        let mut edges = autogen::Edges::new();
        let list = trans
            .get_edges(&query)?
            .into_iter()
            .map(autogen::Edge::from)
            .collect();
        edges.set_edges(protobuf::RepeatedField::from_vec(list));
        response.set_edges(edges);
    } else if request.has_delete_edges() {
        let request = request.get_delete_edges();
        let query = indradb::EdgeQuery::reverse_from(request.get_query())?;
        trans.delete_edges(&query)?;
        response.set_ok(true);
    } else if request.has_get_edge_count() {
        let request = request.get_get_edge_count();
        let id = Uuid::from_str(request.get_id())?;
        let type_filter = converters::from_defaultable(&request.get_type_filter(), |t| {
            Ok(indradb::Type::new(t.to_string())?)
        })?;
        let direction = indradb::EdgeDirection::from_str(request.get_direction())?;
        let count = trans.get_edge_count(id, type_filter.as_ref(), direction)?;
        response.set_count(count);
    } else if request.has_get_global_metadata() {
        let request = request.get_get_global_metadata();
        let name = request.get_name();
        let metadata = trans.get_global_metadata(name)?;

        if metadata.is_some() {
            response.set_json(serde_json::to_string(&metadata)?);
        } else {
            response.set_json("".to_string());
        }
    } else if request.has_set_global_metadata() {
        let request = request.get_set_global_metadata();
        let name = request.get_name();
        let value = serde_json::from_str(request.get_value())?;
        trans.set_global_metadata(name, &value)?;
        response.set_ok(true);
    } else if request.has_delete_global_metadata() {
        let request = request.get_delete_global_metadata();
        let name = request.get_name();
        trans.delete_global_metadata(name)?;
        response.set_ok(true);
    } else if request.has_get_vertex_metadata() {
        let request = request.get_get_vertex_metadata();
        let query = indradb::VertexQuery::reverse_from(request.get_query())?;
        let name = request.get_name();
        let mut metadatas = autogen::VertexMetadatas::new();
        let list = trans
            .get_vertex_metadata(&query, name)?
            .into_iter()
            .map(autogen::VertexMetadata::from)
            .collect();
        metadatas.set_values(protobuf::RepeatedField::from_vec(list));
        response.set_vertex_metadatas(metadatas);
    } else if request.has_set_vertex_metadata() {
        let request = request.get_set_vertex_metadata();
        let name = request.get_name();
        let query = indradb::VertexQuery::reverse_from(request.get_query())?;
        let value = serde_json::from_str(request.get_value())?;
        trans.set_vertex_metadata(&query, name, &value)?;
        response.set_ok(true);
    } else if request.has_delete_vertex_metadata() {
        let request = request.get_delete_vertex_metadata();
        let name = request.get_name();
        let query = indradb::VertexQuery::reverse_from(request.get_query())?;
        trans.delete_vertex_metadata(&query, name)?;
        response.set_ok(true);
    } else if request.has_get_edge_metadata() {
        let request = request.get_get_edge_metadata();
        let query = indradb::EdgeQuery::reverse_from(request.get_query())?;
        let name = request.get_name();
        let mut metadatas = autogen::EdgeMetadatas::new();
        let list = trans
            .get_edge_metadata(&query, name)?
            .into_iter()
            .map(autogen::EdgeMetadata::from)
            .collect();
        metadatas.set_values(protobuf::RepeatedField::from_vec(list));
        response.set_edge_metadatas(metadatas);
    } else if request.has_set_edge_metadata() {
        let request = request.get_set_edge_metadata();
        let name = request.get_name();
        let query = indradb::EdgeQuery::reverse_from(request.get_query())?;
        let value = serde_json::from_str(request.get_value())?;
        trans.set_edge_metadata(&query, name, &value)?;
        response.set_ok(true);
    } else if request.has_delete_edge_metadata() {
        let request = request.get_delete_edge_metadata();
        let name = request.get_name();
        let query = indradb::EdgeQuery::reverse_from(request.get_query())?;
        trans.delete_edge_metadata(&query, name)?;
        response.set_ok(true);
    } else {
        panic!("Unexpected request: {:?}", request);
    };

    Ok((response, grpcio::WriteFlags::default()))
}

fn build_error_response<E: StdError>(err: &E) -> (autogen::TransactionResponse, grpcio::WriteFlags) {
    let mut response = autogen::TransactionResponse::new();
    response.set_error(format!("{}", err));
    (response, grpcio::WriteFlags::default())
}

#[derive(Clone)]
pub struct IndraDbService {
    datastore: Arc<ProxyDatastore>,
}

impl IndraDbService {
    pub fn new() -> Self {
        Self {
            datastore: Arc::new(datastore()),
        }
    }
}

impl autogen::IndraDb for IndraDbService {
    fn ping(&self, ctx: grpcio::RpcContext, _: autogen::PingRequest, sink: grpcio::UnarySink<autogen::PingResponse>) {
        let mut response = autogen::PingResponse::new();
        response.set_ok(true);
        let f = sink.success(response)
            .map_err(|err| eprintln!("Could not send response: {}", err));
        ctx.spawn(f);
    }

    fn transaction(
        &self,
        _: grpcio::RpcContext,
        stream: grpcio::RequestStream<autogen::TransactionRequest>,
        mut sink: grpcio::DuplexSink<autogen::TransactionResponse>,
    ) {
        let datastore = Arc::clone(&self.datastore);

        // TODO: ensure thread joins
        spawn(move || {
            let trans = match datastore.transaction() {
                Ok(trans) => trans,
                Err(err) => {
                    sink = send_sink!(sink, build_error_response(&err));
                    return;
                }
            };

            for result in stream.wait() {
                sink = send_sink!(
                    sink,
                    match result {
                        Ok(request) => {
                            build_response(&trans, &request).unwrap_or_else(|err| build_error_response(&err))
                        }
                        Err(err) => build_error_response(&err),
                    }
                );
            }
        });
    }
}
