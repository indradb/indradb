use capnp_rpc::{RpcSystem, twoparty};
use capnp_rpc::rpc_twoparty_capnp::Side;
use errors;
use autogen;
use futures::{Future, Sink, Stream};
use futures::stream::Wait;
use indradb;
use serde_json;
use serde_json::value::Value as JsonValue;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;
use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_io::AsyncRead;

pub struct ClientDatastore {
    client: autogen::service::Client,
}

impl ClientDatastore {
    pub fn new(port: u16) -> Self {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let addr = format!("127.0.0.1:{}", port).to_socket_addrs().unwrap().next().unwrap();
        let stream = core.run(TcpStream::connect(&addr, &handle)).unwrap();
        stream.set_nodelay(true).unwrap();
        let (reader, writer) = stream.split();
        let rpc_network = Box::new(twoparty::VatNetwork::new(reader, writer, Side::Client, Default::default()));
        let mut rpc_system = RpcSystem::new(rpc_network, None);
        let client: autogen::service::Client = rpc_system.bootstrap(Side::Server);
        handle.spawn(rpc_system.map_err(|_e| ()));

        for _ in 0..5 {
            sleep(Duration::from_secs(1));

            let req = client.ping_request();
            let res = core.run(req.send().promise).unwrap();
            let ready = res.get().unwrap().get_ready();

            if ready {
                return Self {
                    client: client,
                }
            }
        }

        panic!("Server failed to initialize after a few seconds");
    }
}

impl indradb::Datastore<ClientTransaction> for ClientDatastore {
    fn transaction(&self) -> Result<ClientTransaction, indradb::Error> {
        let trans = self.client.transaction_request().send().pipeline.get_transaction();
        Ok(ClientTransaction::new(trans))
    }
}

pub struct ClientTransaction {
    trans: autogen::transaction::Client
}

impl ClientTransaction {
    fn new(trans: autogen::transaction::Client) -> Self {
        ClientTransaction {
            trans: trans,
        }
    }
}

impl indradb::Transaction for ClientTransaction {
    fn create_vertex(&self, v: &indradb::Vertex) -> Result<bool, indradb::Error> {
        // let mut inner = autogen::CreateVertexRequest::new();
        // inner.set_vertex(autogen::Vertex::from(v.clone()));
        // let mut request = autogen::TransactionRequest::new();
        // request.set_create_vertex(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // Ok(response.get_ok())
        unimplemented!();
    }

    fn create_vertex_from_type(&self, t: indradb::Type) -> Result<Uuid, indradb::Error> {
        // let mut inner = autogen::CreateVertexFromTypeRequest::new();
        // inner.set_field_type(t.0);
        // let mut request = autogen::TransactionRequest::new();
        // request.set_create_vertex_from_type(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // Ok(Uuid::parse_str(response.get_uuid()).unwrap())
        unimplemented!();
    }

    fn get_vertices(&self, q: &indradb::VertexQuery) -> Result<Vec<indradb::Vertex>, indradb::Error> {
        // let mut inner = autogen::GetVerticesRequest::new();
        // inner.set_query(autogen::VertexQuery::from(q.clone()));
        // let mut request = autogen::TransactionRequest::new();
        // request.set_get_vertices(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // let vertices: Result<Vec<indradb::Vertex>, errors::Error> = response
        //     .get_vertices()
        //     .get_vertices()
        //     .into_iter()
        //     .map(indradb::Vertex::reverse_from)
        //     .collect();
        // Ok(vertices.unwrap())
        unimplemented!();
    }

    fn delete_vertices(&self, q: &indradb::VertexQuery) -> Result<(), indradb::Error> {
        // let mut inner = autogen::DeleteVerticesRequest::new();
        // inner.set_query(autogen::VertexQuery::from(q.clone()));
        // let mut request = autogen::TransactionRequest::new();
        // request.set_delete_vertices(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }

    fn get_vertex_count(&self) -> Result<u64, indradb::Error> {
        // let mut request = autogen::TransactionRequest::new();
        // request.set_get_vertex_count(autogen::GetVertexCountRequest::new());
        // let response = self.channel.lock().unwrap().request(request)?;
        // Ok(response.get_count())
        unimplemented!();
    }

    fn create_edge(&self, e: &indradb::EdgeKey) -> Result<bool, indradb::Error> {
        // let mut inner = autogen::CreateEdgeRequest::new();
        // inner.set_key(autogen::EdgeKey::from(e.clone()));
        // let mut request = autogen::TransactionRequest::new();
        // request.set_create_edge(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // Ok(response.get_ok())
        unimplemented!();
    }

    fn get_edges(&self, q: &indradb::EdgeQuery) -> Result<Vec<indradb::Edge>, indradb::Error> {
        // let mut inner = autogen::GetEdgesRequest::new();
        // inner.set_query(autogen::EdgeQuery::from(q.clone()));
        // let mut request = autogen::TransactionRequest::new();
        // request.set_get_edges(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // let vertices: Result<Vec<indradb::Edge>, errors::Error> = response
        //     .get_edges()
        //     .get_edges()
        //     .into_iter()
        //     .map(indradb::Edge::reverse_from)
        //     .collect();
        // Ok(vertices.unwrap())
        unimplemented!();
    }

    fn delete_edges(&self, q: &indradb::EdgeQuery) -> Result<(), indradb::Error> {
        // let mut inner = autogen::DeleteEdgesRequest::new();
        // inner.set_query(autogen::EdgeQuery::from(q.clone()));
        // let mut request = autogen::TransactionRequest::new();
        // request.set_delete_edges(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }

    fn get_edge_count(
        &self,
        id: Uuid,
        type_filter: Option<&indradb::Type>,
        direction: indradb::EdgeDirection,
    ) -> Result<u64, indradb::Error> {
        // let mut inner = autogen::GetEdgeCountRequest::new();
        // inner.set_id(id.hyphenated().to_string());

        // if let Some(type_filter) = type_filter {
        //     inner.set_type_filter(type_filter.0.clone());
        // }

        // inner.set_direction(String::from(direction));
        // let mut request = autogen::TransactionRequest::new();
        // request.set_get_edge_count(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // Ok(response.get_count())
        unimplemented!();
    }

    fn get_global_metadata(&self, name: &str) -> Result<Option<JsonValue>, indradb::Error> {
        // let mut inner = autogen::GetGlobalMetadataRequest::new();
        // inner.set_name(name.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_get_global_metadata(inner);
        // let response = self.channel.lock().unwrap().request(request)?;

        // if response.get_json() == "" {
        //     Ok(None)
        // } else {
        //     Ok(Some(serde_json::from_str(response.get_json()).unwrap()))
        // }
        unimplemented!();
    }

    fn set_global_metadata(&self, name: &str, value: &JsonValue) -> Result<(), indradb::Error> {
        // let mut inner = autogen::SetGlobalMetadataRequest::new();
        // inner.set_name(name.to_string());
        // inner.set_value(value.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_set_global_metadata(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }

    fn delete_global_metadata(&self, name: &str) -> Result<(), indradb::Error> {
        // let mut inner = autogen::DeleteGlobalMetadataRequest::new();
        // inner.set_name(name.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_delete_global_metadata(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }

    fn get_vertex_metadata(
        &self,
        q: &indradb::VertexQuery,
        name: &str,
    ) -> Result<Vec<indradb::VertexMetadata>, indradb::Error> {
        // let mut inner = autogen::GetVertexMetadataRequest::new();
        // inner.set_query(autogen::VertexQuery::from(q.clone()));
        // inner.set_name(name.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_get_vertex_metadata(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // let metadata: Result<Vec<indradb::VertexMetadata>, errors::Error> = response
        //     .get_vertex_metadatas()
        //     .get_values()
        //     .into_iter()
        //     .map(indradb::VertexMetadata::reverse_from)
        //     .collect();
        // Ok(metadata.unwrap())
        unimplemented!();
    }

    fn set_vertex_metadata(
        &self,
        q: &indradb::VertexQuery,
        name: &str,
        value: &JsonValue,
    ) -> Result<(), indradb::Error> {
        // let mut inner = autogen::SetVertexMetadataRequest::new();
        // inner.set_query(autogen::VertexQuery::from(q.clone()));
        // inner.set_name(name.to_string());
        // inner.set_value(value.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_set_vertex_metadata(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }

    fn delete_vertex_metadata(&self, q: &indradb::VertexQuery, name: &str) -> Result<(), indradb::Error> {
        // let mut inner = autogen::DeleteVertexMetadataRequest::new();
        // inner.set_query(autogen::VertexQuery::from(q.clone()));
        // inner.set_name(name.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_delete_vertex_metadata(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }

    fn get_edge_metadata(
        &self,
        q: &indradb::EdgeQuery,
        name: &str,
    ) -> Result<Vec<indradb::EdgeMetadata>, indradb::Error> {
        // let mut inner = autogen::GetEdgeMetadataRequest::new();
        // inner.set_query(autogen::EdgeQuery::from(q.clone()));
        // inner.set_name(name.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_get_edge_metadata(inner);
        // let response = self.channel.lock().unwrap().request(request)?;
        // let metadata: Result<Vec<indradb::EdgeMetadata>, errors::Error> = response
        //     .get_edge_metadatas()
        //     .get_values()
        //     .into_iter()
        //     .map(indradb::EdgeMetadata::reverse_from)
        //     .collect();
        // Ok(metadata.unwrap())
        unimplemented!();
    }

    fn set_edge_metadata(&self, q: &indradb::EdgeQuery, name: &str, value: &JsonValue) -> Result<(), indradb::Error> {
        // let mut inner = autogen::SetEdgeMetadataRequest::new();
        // inner.set_query(autogen::EdgeQuery::from(q.clone()));
        // inner.set_name(name.to_string());
        // inner.set_value(value.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_set_edge_metadata(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }

    fn delete_edge_metadata(&self, q: &indradb::EdgeQuery, name: &str) -> Result<(), indradb::Error> {
        // let mut inner = autogen::DeleteEdgeMetadataRequest::new();
        // inner.set_query(autogen::EdgeQuery::from(q.clone()));
        // inner.set_name(name.to_string());
        // let mut request = autogen::TransactionRequest::new();
        // request.set_delete_edge_metadata(inner);
        // self.channel.lock().unwrap().request(request)?;
        // Ok(())
        unimplemented!();
    }
}
