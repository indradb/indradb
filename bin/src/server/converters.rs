//! Converts between gRPC and native IndraDB models. `From` can be
//! implemented for gRPC models, since they are defined in this crate, but not
//! for IndraDB models, since they're defined in the lib. So we define a
//! separate trait for those conversions. Alternatively, we could use
//! newtypes, but that would introduce its own baggage.

use indradb;
use models as grpc_models;
use queries as grpc_queries;
use protobuf;
use serde_json;
use uuid::Uuid;
use std::str::FromStr;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
    }
}

trait ReverseFrom<T>: Sized {
    fn from(T) -> Result<Self>;
}

impl From<indradb::Vertex> for grpc_models::Vertex {
    fn from(vertex: indradb::Vertex) -> Self {
        let mut grpc_vertex: grpc_models::Vertex = grpc_models::Vertex::new();
        grpc_vertex.id = vertex.id.hyphenated().to_string();
        grpc_vertex.field_type = vertex.t.0;
        grpc_vertex
    }
}

impl ReverseFrom<grpc_models::Vertex> for indradb::Vertex {
    fn from(grpc_vertex: grpc_models::Vertex) -> Result<Self> {
        let id = Uuid::from_str(&grpc_vertex.id)?;
        let t = indradb::Type::new(grpc_vertex.field_type)?;
        indradb::Vertex::with_id(id, t)
    }
}

impl From<Vec<indradb::Vertex>> for grpc_models::Vertices {
    fn from(vertices: Vec<indradb::Vertex>) -> Self {
        let mapped = vertices.into_iter().map(grpc_models::Vertex::from).collect();
        let mut vertices = grpc_models::Vertices::new();
        vertices.set_vertices(protobuf::RepeatedField::from_vec(mapped));
        vertices
    }
}

impl From<indradb::Edge> for grpc_models::Edge {
    fn from(edge: indradb::Edge) -> Self {
        let mut grpc_edge: grpc_models::Edge = grpc_models::Edge::new();
        grpc_edge.set_key(grpc_models::EdgeKey::from(edge.key));
        grpc_edge.set_created_datetime(edge.created_datetime.timestamp() as u32);
        grpc_edge
    }
}

impl From<Vec<indradb::Edge>> for grpc_models::Edges {
    fn from(edges: Vec<indradb::Edge>) -> Self {
        let mapped = edges.into_iter().map(|edge| grpc_models::Edge::from(edge)).collect();
        let mut edges = grpc_models::Edges::new();
        edges.set_edges(protobuf::RepeatedField::from_vec(mapped));
        edges
    }
}

impl From<indradb::EdgeKey> for grpc_models::EdgeKey {
    fn from(key: indradb::EdgeKey) -> Self {
        let mut grpc_key: grpc_models::EdgeKey = grpc_models::EdgeKey::new();
        grpc_key.set_outbound_id(key.outbound_id.hyphenated().to_string());
        grpc_key.set_field_type(key.t.0);
        grpc_key.set_inbound_id(key.inbound_id.hyphenated().to_string());
        grpc_key
    }
}

impl From<indradb::VertexMetadata> for grpc_models::VertexMetadata {
    fn from(metadata: indradb::VertexMetadata) -> Self {
        let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
        let mut grpc_metadata: grpc_models::VertexMetadata = grpc_models::VertexMetadata::new();
        grpc_metadata.set_id(metadata.id.hyphenated().to_string());
        grpc_metadata.set_value(value);
        grpc_metadata
    }
}

impl From<Vec<indradb::VertexMetadata>> for grpc_models::VertexMetadatas {
    fn from(metadata: Vec<indradb::VertexMetadata>) -> Self {
        let mapped = metadata.into_iter().map(|metadata| grpc_models::VertexMetadata::from(metadata)).collect();
        let mut metadata = grpc_models::VertexMetadatas::new();
        metadata.set_metadata(protobuf::RepeatedField::from_vec(mapped));
        metadata
    }
}

impl From<indradb::EdgeMetadata> for grpc_models::EdgeMetadata {
    fn from(metadata: indradb::EdgeMetadata) -> Self {
        let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
        let mut grpc_metadata: grpc_models::EdgeMetadata = grpc_models::EdgeMetadata::new();
        grpc_metadata.set_key(grpc_models::EdgeKey::from(metadata.key));
        grpc_metadata.set_value(value);
        grpc_metadata
    }
}

impl From<Vec<indradb::EdgeMetadata>> for grpc_models::EdgeMetadatas {
    fn from(metadata: Vec<indradb::EdgeMetadata>) -> Self {
        let mapped = metadata.into_iter().map(|metadata| grpc_models::EdgeMetadata::from(metadata)).collect();
        let mut metadata = grpc_models::EdgeMetadatas::new();
        metadata.set_metadata(protobuf::RepeatedField::from_vec(mapped));
        metadata
    }
}
