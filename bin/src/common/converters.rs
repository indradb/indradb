//! Converts between gRPC and native IndraDB models. `From` can be
//! implemented for gRPC models, since they are defined in this crate, but not
//! for IndraDB models, since they're defined in the lib. So we define a
//! separate trait for those conversions. Alternatively, we could use
//! newtypes, but that would introduce its own baggage.

use chrono::{DateTime, Utc};
use chrono::TimeZone;
use errors::Result;
use indradb;
use protobuf;
use protobuf::well_known_types;
use serde_json;
use std::str::FromStr;
use uuid::Uuid;
use autogen;

pub trait ReverseFrom<T>: Sized {
    fn reverse_from(&T) -> Result<Self>;
}

impl From<indradb::Vertex> for autogen::Vertex {
    fn from(vertex: indradb::Vertex) -> Self {
        let mut grpc_vertex: autogen::Vertex = autogen::Vertex::new();
        grpc_vertex.id = vertex.id.hyphenated().to_string();
        grpc_vertex.field_type = vertex.t.0;
        grpc_vertex
    }
}

impl ReverseFrom<autogen::Vertex> for indradb::Vertex {
    fn reverse_from(grpc_vertex: &autogen::Vertex) -> Result<Self> {
        let id = Uuid::from_str(&grpc_vertex.id)?;
        let t = indradb::Type::new(grpc_vertex.field_type.clone())?;
        Ok(indradb::Vertex::with_id(id, t))
    }
}

impl From<Vec<indradb::Vertex>> for autogen::Vertices {
    fn from(vertices: Vec<indradb::Vertex>) -> Self {
        let mapped = vertices.into_iter().map(autogen::Vertex::from).collect();
        let mut vertices = autogen::Vertices::new();
        vertices.set_vertices(protobuf::RepeatedField::from_vec(mapped));
        vertices
    }
}

impl From<indradb::Edge> for autogen::Edge {
    fn from(edge: indradb::Edge) -> Self {
        let mut grpc_edge: autogen::Edge = autogen::Edge::new();
        grpc_edge.set_key(autogen::EdgeKey::from(edge.key));
        grpc_edge.set_created_datetime(timestamp_from_datetime(&edge.created_datetime));
        grpc_edge
    }
}

impl ReverseFrom<autogen::Edge> for indradb::Edge {
    fn reverse_from(grpc_edge: &autogen::Edge) -> Result<Self> {
        let key = indradb::EdgeKey::reverse_from(grpc_edge.get_key())?;
        let created_datetime = datetime_from_timestamp(grpc_edge.get_created_datetime());
        Ok(indradb::Edge::new(key, created_datetime))
    }
}

impl From<Vec<indradb::Edge>> for autogen::Edges {
    fn from(edges: Vec<indradb::Edge>) -> Self {
        let mapped = edges.into_iter().map(autogen::Edge::from).collect();
        let mut edges = autogen::Edges::new();
        edges.set_edges(protobuf::RepeatedField::from_vec(mapped));
        edges
    }
}

impl From<indradb::EdgeKey> for autogen::EdgeKey {
    fn from(key: indradb::EdgeKey) -> Self {
        let mut grpc_key: autogen::EdgeKey = autogen::EdgeKey::new();
        grpc_key.set_outbound_id(key.outbound_id.hyphenated().to_string());
        grpc_key.set_field_type(key.t.0);
        grpc_key.set_inbound_id(key.inbound_id.hyphenated().to_string());
        grpc_key
    }
}

impl ReverseFrom<autogen::EdgeKey> for indradb::EdgeKey {
    fn reverse_from(grpc_key: &autogen::EdgeKey) -> Result<Self> {
        Ok(indradb::EdgeKey::new(
            Uuid::from_str(grpc_key.get_outbound_id())?,
            indradb::Type::new(grpc_key.get_field_type().to_string())?,
            Uuid::from_str(grpc_key.get_inbound_id())?,
        ))
    }
}

impl From<indradb::VertexMetadata> for autogen::VertexMetadata {
    fn from(metadata: indradb::VertexMetadata) -> Self {
        let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
        let mut grpc_metadata: autogen::VertexMetadata = autogen::VertexMetadata::new();
        grpc_metadata.set_id(metadata.id.hyphenated().to_string());
        grpc_metadata.set_value(value);
        grpc_metadata
    }
}

impl ReverseFrom<autogen::VertexMetadata> for indradb::VertexMetadata {
    fn reverse_from(grpc_metadata: &autogen::VertexMetadata) -> Result<Self> {
        Ok(indradb::VertexMetadata::new(
            Uuid::from_str(grpc_metadata.get_id())?,
            serde_json::from_str(grpc_metadata.get_value())?,
        ))
    }
}

impl From<Vec<indradb::VertexMetadata>> for autogen::VertexMetadatas {
    fn from(metadata: Vec<indradb::VertexMetadata>) -> Self {
        let mapped = metadata
            .into_iter()
            .map(autogen::VertexMetadata::from)
            .collect();
        let mut grpc_metadata = autogen::VertexMetadatas::new();
        grpc_metadata.set_values(protobuf::RepeatedField::from_vec(mapped));
        grpc_metadata
    }
}

impl From<indradb::EdgeMetadata> for autogen::EdgeMetadata {
    fn from(metadata: indradb::EdgeMetadata) -> Self {
        let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
        let mut grpc_metadata: autogen::EdgeMetadata = autogen::EdgeMetadata::new();
        grpc_metadata.set_key(autogen::EdgeKey::from(metadata.key));
        grpc_metadata.set_value(value);
        grpc_metadata
    }
}

impl ReverseFrom<autogen::EdgeMetadata> for indradb::EdgeMetadata {
    fn reverse_from(grpc_metadata: &autogen::EdgeMetadata) -> Result<Self> {
        Ok(indradb::EdgeMetadata::new(
            indradb::EdgeKey::reverse_from(grpc_metadata.get_key())?,
            serde_json::from_str(grpc_metadata.get_value())?,
        ))
    }
}

impl From<Vec<indradb::EdgeMetadata>> for autogen::EdgeMetadatas {
    fn from(metadata: Vec<indradb::EdgeMetadata>) -> Self {
        let mapped = metadata
            .into_iter()
            .map(autogen::EdgeMetadata::from)
            .collect();
        let mut metadata = autogen::EdgeMetadatas::new();
        metadata.set_values(protobuf::RepeatedField::from_vec(mapped));
        metadata
    }
}

impl From<indradb::VertexQuery> for autogen::VertexQuery {
    fn from(query: indradb::VertexQuery) -> Self {
        let mut grpc_query = autogen::VertexQuery::new();

        match query {
            indradb::VertexQuery::All { start_id, limit } => {
                let mut grpc_inner_query = autogen::AllVertexQuery::new();

                if let Some(start_id) = start_id {
                    grpc_inner_query.set_start_id(start_id.hyphenated().to_string());
                }

                grpc_inner_query.set_limit(limit);
                grpc_query.set_all(grpc_inner_query);
            }
            indradb::VertexQuery::Vertices { ids } => {
                let mut grpc_inner_query = autogen::VerticesVertexQuery::new();
                grpc_inner_query.set_ids(ids.iter().map(|id| id.hyphenated().to_string()).collect());
                grpc_query.set_vertices(grpc_inner_query);
            }
            indradb::VertexQuery::Pipe {
                edge_query,
                converter,
                limit,
            } => {
                let mut grpc_inner_query = autogen::PipeVertexQuery::new();
                grpc_inner_query.set_edge_query(autogen::EdgeQuery::from(*edge_query));
                grpc_inner_query.set_converter(String::from(converter));
                grpc_inner_query.set_limit(limit);
                grpc_query.set_pipe(grpc_inner_query);
            }
        };

        grpc_query
    }
}

impl ReverseFrom<autogen::VertexQuery> for indradb::VertexQuery {
    fn reverse_from(grpc_query: &autogen::VertexQuery) -> Result<Self> {
        if grpc_query.has_all() {
            let query = grpc_query.get_all();
            Ok(indradb::VertexQuery::All {
                start_id: from_defaultable(&query.get_start_id(), |s| Ok(Uuid::from_str(s)?))?,
                limit: query.get_limit(),
            })
        } else if grpc_query.has_vertices() {
            let query = grpc_query.get_vertices();
            let ids: Result<Vec<Uuid>> = query
                .get_ids()
                .iter()
                .map(|s| Ok(Uuid::from_str(s)?))
                .collect();
            Ok(indradb::VertexQuery::Vertices { ids: ids? })
        } else if grpc_query.has_pipe() {
            let query = grpc_query.get_pipe();
            Ok(indradb::VertexQuery::Pipe {
                edge_query: Box::new(indradb::EdgeQuery::reverse_from(query.get_edge_query())?),
                converter: indradb::EdgeDirection::from_str(query.get_converter())?,
                limit: query.get_limit(),
            })
        } else {
            unreachable!();
        }
    }
}

impl From<indradb::EdgeQuery> for autogen::EdgeQuery {
    fn from(query: indradb::EdgeQuery) -> Self {
        let mut grpc_query = autogen::EdgeQuery::new();

        match query {
            indradb::EdgeQuery::Edges { keys } => {
                let mut grpc_inner_query = autogen::EdgesEdgeQuery::new();
                grpc_inner_query.set_keys(keys.into_iter().map(autogen::EdgeKey::from).collect());
                grpc_query.set_edges(grpc_inner_query);
            }
            indradb::EdgeQuery::Pipe {
                vertex_query,
                converter,
                type_filter,
                high_filter,
                low_filter,
                limit,
            } => {
                let mut grpc_inner_query = autogen::PipeEdgeQuery::new();
                grpc_inner_query.set_vertex_query(autogen::VertexQuery::from(*vertex_query));
                grpc_inner_query.set_converter(String::from(converter));

                if let Some(type_filter) = type_filter {
                    grpc_inner_query.set_type_filter(type_filter.0);
                }

                if let Some(high_filter) = high_filter {
                    grpc_inner_query.set_high_filter(timestamp_from_datetime(&high_filter));
                }

                if let Some(low_filter) = low_filter {
                    grpc_inner_query.set_low_filter(timestamp_from_datetime(&low_filter));
                }

                grpc_inner_query.set_limit(limit);
                grpc_query.set_pipe(grpc_inner_query);
            }
        };

        grpc_query
    }
}

impl ReverseFrom<autogen::EdgeQuery> for indradb::EdgeQuery {
    fn reverse_from(grpc_query: &autogen::EdgeQuery) -> Result<Self> {
        if grpc_query.has_edges() {
            let query = grpc_query.get_edges();
            let ids: Result<Vec<indradb::EdgeKey>> = query
                .get_keys()
                .iter()
                .map(indradb::EdgeKey::reverse_from)
                .collect();
            Ok(indradb::EdgeQuery::Edges { keys: ids? })
        } else if grpc_query.has_pipe() {
            let query = grpc_query.get_pipe();
            Ok(indradb::EdgeQuery::Pipe {
                vertex_query: Box::new(indradb::VertexQuery::reverse_from(
                    query.get_vertex_query(),
                )?),
                converter: indradb::EdgeDirection::from_str(query.get_converter())?,
                type_filter: from_defaultable(&query.get_type_filter(), |t| {
                    Ok(indradb::Type::new(t.to_string())?)
                })?,
                high_filter: if query.has_high_filter() {
                    Some(datetime_from_timestamp(query.get_high_filter()))
                } else {
                    None
                },
                low_filter: if query.has_low_filter() {
                    Some(datetime_from_timestamp(query.get_low_filter()))
                } else {
                    None
                },
                limit: query.get_limit(),
            })
        } else {
            unreachable!();
        }
    }
}

fn datetime_from_timestamp(ts: &well_known_types::Timestamp) -> DateTime<Utc> {
    Utc.timestamp(ts.get_seconds(), ts.get_nanos() as u32)
}

fn timestamp_from_datetime(dt: &DateTime<Utc>) -> well_known_types::Timestamp {
    let mut timestamp = well_known_types::Timestamp::new();
    timestamp.set_seconds(dt.timestamp());
    timestamp.set_nanos(dt.timestamp_subsec_nanos() as i32);
    timestamp
}

pub fn from_defaultable<T, U, F>(t: &T, mapper: F) -> Result<Option<U>>
where
    T: Default + PartialEq,
    F: Fn(&T) -> Result<U>,
{
    if t == &T::default() {
        Ok(None)
    } else {
        let val = mapper(t)?;
        Ok(Some(val))
    }
}
