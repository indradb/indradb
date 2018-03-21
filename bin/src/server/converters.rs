//! Converts between gRPC and native IndraDB models. `From` can be
//! implemented for gRPC models, since they are defined in this crate, but not
//! for IndraDB models, since they're defined in the lib. So we define a
//! separate trait for those conversions. Alternatively, we could use
//! newtypes, but that would introduce its own baggage.

use indradb;
use edges as grpc_edges;
use vertices as grpc_vertices;
use queries as grpc_queries;
use metadata as grpc_metadata;
use protobuf;
use serde_json;
use uuid;
use uuid::Uuid;
use errors::{Result, Error};
use std::str::FromStr;

pub trait ReverseFrom<T>: Sized {
    fn reverse_from(T) -> Result<Self>;
}

impl From<indradb::Vertex> for grpc_vertices::Vertex {
    fn from(vertex: indradb::Vertex) -> Self {
        let mut grpc_vertex: grpc_vertices::Vertex = grpc_vertices::Vertex::new();
        grpc_vertex.id = vertex.id.hyphenated().to_string();
        grpc_vertex.field_type = vertex.t.0;
        grpc_vertex
    }
}

impl ReverseFrom<grpc_vertices::Vertex> for indradb::Vertex {
    fn reverse_from(grpc_vertex: grpc_vertices::Vertex) -> Result<Self> {
        let id = Uuid::from_str(&grpc_vertex.id)?;
        let t = indradb::Type::new(grpc_vertex.field_type)?;
        Ok(indradb::Vertex::with_id(id, t))
    }
}

impl From<Vec<indradb::Vertex>> for grpc_vertices::Vertices {
    fn from(vertices: Vec<indradb::Vertex>) -> Self {
        let mapped = vertices.into_iter().map(grpc_vertices::Vertex::from).collect();
        let mut vertices = grpc_vertices::Vertices::new();
        vertices.set_vertices(protobuf::RepeatedField::from_vec(mapped));
        vertices
    }
}

impl From<indradb::Edge> for grpc_edges::Edge {
    fn from(edge: indradb::Edge) -> Self {
        let mut grpc_edge: grpc_edges::Edge = grpc_edges::Edge::new();
        grpc_edge.set_key(grpc_edges::EdgeKey::from(edge.key));
        grpc_edge.set_created_datetime(edge.created_datetime.timestamp() as u32);
        grpc_edge
    }
}

impl From<Vec<indradb::Edge>> for grpc_edges::Edges {
    fn from(edges: Vec<indradb::Edge>) -> Self {
        let mapped = edges.into_iter().map(|edge| grpc_edges::Edge::from(edge)).collect();
        let mut edges = grpc_edges::Edges::new();
        edges.set_edges(protobuf::RepeatedField::from_vec(mapped));
        edges
    }
}

impl From<indradb::EdgeKey> for grpc_edges::EdgeKey {
    fn from(key: indradb::EdgeKey) -> Self {
        let mut grpc_key: grpc_edges::EdgeKey = grpc_edges::EdgeKey::new();
        grpc_key.set_outbound_id(key.outbound_id.hyphenated().to_string());
        grpc_key.set_field_type(key.t.0);
        grpc_key.set_inbound_id(key.inbound_id.hyphenated().to_string());
        grpc_key
    }
}

impl From<indradb::VertexMetadata> for grpc_metadata::VertexMetadata {
    fn from(metadata: indradb::VertexMetadata) -> Self {
        let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
        let mut grpc_metadata: grpc_metadata::VertexMetadata = grpc_metadata::VertexMetadata::new();
        grpc_metadata.set_id(metadata.id.hyphenated().to_string());
        grpc_metadata.set_value(value);
        grpc_metadata
    }
}

impl From<Vec<indradb::VertexMetadata>> for grpc_metadata::VertexMetadatas {
    fn from(metadata: Vec<indradb::VertexMetadata>) -> Self {
        let mapped = metadata.into_iter().map(|metadata| grpc_metadata::VertexMetadata::from(metadata)).collect();
        let mut metadata = grpc_metadata::VertexMetadatas::new();
        metadata.set_metadata(protobuf::RepeatedField::from_vec(mapped));
        metadata
    }
}

impl From<indradb::EdgeMetadata> for grpc_metadata::EdgeMetadata {
    fn from(metadata: indradb::EdgeMetadata) -> Self {
        let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
        let mut grpc_metadata: grpc_metadata::EdgeMetadata = grpc_metadata::EdgeMetadata::new();
        grpc_metadata.set_key(grpc_edges::EdgeKey::from(metadata.key));
        grpc_metadata.set_value(value);
        grpc_metadata
    }
}

impl From<Vec<indradb::EdgeMetadata>> for grpc_metadata::EdgeMetadatas {
    fn from(metadata: Vec<indradb::EdgeMetadata>) -> Self {
        let mapped = metadata.into_iter().map(|metadata| grpc_metadata::EdgeMetadata::from(metadata)).collect();
        let mut metadata = grpc_metadata::EdgeMetadatas::new();
        metadata.set_metadata(protobuf::RepeatedField::from_vec(mapped));
        metadata
    }
}
