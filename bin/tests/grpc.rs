extern crate chan_signal;
extern crate chrono;
extern crate common;
extern crate core;
extern crate futures;
extern crate grpcio;
#[macro_use]
extern crate indradb;
extern crate libc;
extern crate protobuf;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate lazy_static;

use common::ReverseFrom;
use futures::{Future, Sink, Stream};
use futures::stream::Wait;
use grpcio::{ChannelBuilder, ClientDuplexReceiver, ClientDuplexSender, Environment, WriteFlags};
pub use indradb::tests;
pub use regex::Regex;
use serde_json::value::Value as JsonValue;
use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

const START_PORT: usize = 27615;

lazy_static! {
    static ref PORT: AtomicUsize = AtomicUsize::new(START_PORT);
}

pub struct GrpcDatastore {
    server: Child,
    client: common::IndraDbClient,
}

impl GrpcDatastore {
    fn default() -> Self {
        let port = PORT.fetch_add(1, Ordering::SeqCst);

        let mut envs = HashMap::new();
        envs.insert("PORT", port.to_string());

        let server = Command::new("../target/debug/indradb-server")
            .envs(envs)
            .spawn()
            .expect("Server failed to start");

        let env = Arc::new(Environment::new(1));
        let channel = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", port));
        let client = common::IndraDbClient::new(channel);

        for _ in 0..5 {
            sleep(Duration::from_secs(1));

            if let Ok(response) = client.ping(&common::PingRequest::new()) {
                if response.get_ok() {
                    return Self {
                        server: server,
                        client: client,
                    };
                }
            }
        }

        panic!("Server failed to initialize after a few seconds");
    }
}

impl Drop for GrpcDatastore {
    fn drop(&mut self) {
        if let Err(err) = self.server.kill() {
            panic!(format!("Could not kill server instance: {}", err))
        }
    }
}

impl indradb::Datastore<GrpcTransaction> for GrpcDatastore {
    fn transaction(&self) -> Result<GrpcTransaction, indradb::Error> {
        let (sink, receiver) = self.client.transaction().unwrap();
        let channel = GrpcTransactionDuplex::new(sink, receiver.wait());
        Ok(GrpcTransaction::new(channel))
    }
}

struct GrpcTransactionDuplex {
    sink: Option<ClientDuplexSender<common::TransactionRequest>>,
    receiver: Wait<ClientDuplexReceiver<common::TransactionResponse>>,
}

impl GrpcTransactionDuplex {
    fn new(
        sink: ClientDuplexSender<common::TransactionRequest>,
        receiver: Wait<ClientDuplexReceiver<common::TransactionResponse>>,
    ) -> Self {
        Self {
            sink: Some(sink),
            receiver: receiver,
        }
    }

    fn request(&mut self, req: common::TransactionRequest) -> Result<common::TransactionResponse, indradb::Error> {
        let sink: ClientDuplexSender<common::TransactionRequest> = self.sink.take().unwrap();
        self.sink = Some(sink.send((req, WriteFlags::default())).wait().unwrap());
        let response = self.receiver.next().unwrap().unwrap();

        if response.has_error() {
            Err(response.get_error().into())
        } else {
            Ok(response)
        }
    }
}

pub struct GrpcTransaction {
    channel: Mutex<GrpcTransactionDuplex>,
}

impl GrpcTransaction {
    fn new(channel: GrpcTransactionDuplex) -> Self {
        GrpcTransaction {
            channel: Mutex::new(channel),
        }
    }
}

impl indradb::Transaction for GrpcTransaction {
    fn create_vertex(&self, v: &indradb::Vertex) -> Result<bool, indradb::Error> {
        let mut inner = common::CreateVertexRequest::new();
        inner.set_vertex(common::Vertex::from(v.clone()));
        let mut request = common::TransactionRequest::new();
        request.set_create_vertex(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        Ok(response.get_ok())
    }

    fn create_vertex_from_type(&self, t: indradb::Type) -> Result<Uuid, indradb::Error> {
        let mut inner = common::CreateVertexFromTypeRequest::new();
        inner.set_field_type(t.0);
        let mut request = common::TransactionRequest::new();
        request.set_create_vertex_from_type(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        Ok(Uuid::parse_str(response.get_uuid()).unwrap())
    }

    fn get_vertices(&self, q: &indradb::VertexQuery) -> Result<Vec<indradb::Vertex>, indradb::Error> {
        let mut inner = common::GetVerticesRequest::new();
        inner.set_query(common::VertexQuery::from(q.clone()));
        let mut request = common::TransactionRequest::new();
        request.set_get_vertices(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        let vertices: Result<Vec<indradb::Vertex>, common::Error> = response
            .get_vertices()
            .get_vertices()
            .into_iter()
            .map(indradb::Vertex::reverse_from)
            .collect();
        Ok(vertices.unwrap())
    }

    fn delete_vertices(&self, q: &indradb::VertexQuery) -> Result<(), indradb::Error> {
        let mut inner = common::DeleteVerticesRequest::new();
        inner.set_query(common::VertexQuery::from(q.clone()));
        let mut request = common::TransactionRequest::new();
        request.set_delete_vertices(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64, indradb::Error> {
        let mut request = common::TransactionRequest::new();
        request.set_get_vertex_count(common::GetVertexCountRequest::new());
        let response = self.channel.lock().unwrap().request(request)?;
        Ok(response.get_count())
    }

    fn create_edge(&self, e: &indradb::EdgeKey) -> Result<bool, indradb::Error> {
        let mut inner = common::CreateEdgeRequest::new();
        inner.set_key(common::EdgeKey::from(e.clone()));
        let mut request = common::TransactionRequest::new();
        request.set_create_edge(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        Ok(response.get_ok())
    }

    fn get_edges(&self, q: &indradb::EdgeQuery) -> Result<Vec<indradb::Edge>, indradb::Error> {
        let mut inner = common::GetEdgesRequest::new();
        inner.set_query(common::EdgeQuery::from(q.clone()));
        let mut request = common::TransactionRequest::new();
        request.set_get_edges(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        let vertices: Result<Vec<indradb::Edge>, common::Error> = response
            .get_edges()
            .get_edges()
            .into_iter()
            .map(indradb::Edge::reverse_from)
            .collect();
        Ok(vertices.unwrap())
    }

    fn delete_edges(&self, q: &indradb::EdgeQuery) -> Result<(), indradb::Error> {
        let mut inner = common::DeleteEdgesRequest::new();
        inner.set_query(common::EdgeQuery::from(q.clone()));
        let mut request = common::TransactionRequest::new();
        request.set_delete_edges(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }

    fn get_edge_count(
        &self,
        id: Uuid,
        type_filter: Option<&indradb::Type>,
        direction: indradb::EdgeDirection,
    ) -> Result<u64, indradb::Error> {
        let mut inner = common::GetEdgeCountRequest::new();
        inner.set_id(id.hyphenated().to_string());

        if let Some(type_filter) = type_filter {
            inner.set_type_filter(type_filter.0.clone());
        }

        inner.set_direction(String::from(direction));
        let mut request = common::TransactionRequest::new();
        request.set_get_edge_count(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        Ok(response.get_count())
    }

    fn get_global_metadata(&self, name: &str) -> Result<Option<JsonValue>, indradb::Error> {
        let mut inner = common::GetGlobalMetadataRequest::new();
        inner.set_name(name.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_get_global_metadata(inner);
        let response = self.channel.lock().unwrap().request(request)?;

        if response.get_json() == "" {
            Ok(None)
        } else {
            Ok(Some(serde_json::from_str(response.get_json()).unwrap()))
        }
    }

    fn set_global_metadata(&self, name: &str, value: &JsonValue) -> Result<(), indradb::Error> {
        let mut inner = common::SetGlobalMetadataRequest::new();
        inner.set_name(name.to_string());
        inner.set_value(value.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_set_global_metadata(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }

    fn delete_global_metadata(&self, name: &str) -> Result<(), indradb::Error> {
        let mut inner = common::DeleteGlobalMetadataRequest::new();
        inner.set_name(name.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_delete_global_metadata(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }

    fn get_vertex_metadata(
        &self,
        q: &indradb::VertexQuery,
        name: &str,
    ) -> Result<Vec<indradb::VertexMetadata>, indradb::Error> {
        let mut inner = common::GetVertexMetadataRequest::new();
        inner.set_query(common::VertexQuery::from(q.clone()));
        inner.set_name(name.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_get_vertex_metadata(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        let metadata: Result<Vec<indradb::VertexMetadata>, common::Error> = response
            .get_vertex_metadatas()
            .get_values()
            .into_iter()
            .map(indradb::VertexMetadata::reverse_from)
            .collect();
        Ok(metadata.unwrap())
    }

    fn set_vertex_metadata(
        &self,
        q: &indradb::VertexQuery,
        name: &str,
        value: &JsonValue,
    ) -> Result<(), indradb::Error> {
        let mut inner = common::SetVertexMetadataRequest::new();
        inner.set_query(common::VertexQuery::from(q.clone()));
        inner.set_name(name.to_string());
        inner.set_value(value.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_set_vertex_metadata(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }

    fn delete_vertex_metadata(&self, q: &indradb::VertexQuery, name: &str) -> Result<(), indradb::Error> {
        let mut inner = common::DeleteVertexMetadataRequest::new();
        inner.set_query(common::VertexQuery::from(q.clone()));
        inner.set_name(name.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_delete_vertex_metadata(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }

    fn get_edge_metadata(
        &self,
        q: &indradb::EdgeQuery,
        name: &str,
    ) -> Result<Vec<indradb::EdgeMetadata>, indradb::Error> {
        let mut inner = common::GetEdgeMetadataRequest::new();
        inner.set_query(common::EdgeQuery::from(q.clone()));
        inner.set_name(name.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_get_edge_metadata(inner);
        let response = self.channel.lock().unwrap().request(request)?;
        let metadata: Result<Vec<indradb::EdgeMetadata>, common::Error> = response
            .get_edge_metadatas()
            .get_values()
            .into_iter()
            .map(indradb::EdgeMetadata::reverse_from)
            .collect();
        Ok(metadata.unwrap())
    }

    fn set_edge_metadata(&self, q: &indradb::EdgeQuery, name: &str, value: &JsonValue) -> Result<(), indradb::Error> {
        let mut inner = common::SetEdgeMetadataRequest::new();
        inner.set_query(common::EdgeQuery::from(q.clone()));
        inner.set_name(name.to_string());
        inner.set_value(value.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_set_edge_metadata(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }

    fn delete_edge_metadata(&self, q: &indradb::EdgeQuery, name: &str) -> Result<(), indradb::Error> {
        let mut inner = common::DeleteEdgeMetadataRequest::new();
        inner.set_query(common::EdgeQuery::from(q.clone()));
        inner.set_name(name.to_string());
        let mut request = common::TransactionRequest::new();
        request.set_delete_edge_metadata(inner);
        self.channel.lock().unwrap().request(request)?;
        Ok(())
    }
}

full_test_impl!({ GrpcDatastore::default() });
