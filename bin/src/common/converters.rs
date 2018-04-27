//! Converts between cnp and native IndraDB models. `From` can be
//! implemented for cnp models, since they are defined in this crate, but not
//! for IndraDB models, since they're defined in the lib. So we define a
//! separate trait for those conversions. Alternatively, we could use
//! newtypes, but that would introduce its own baggage.

use chrono::{DateTime, Utc};
use chrono::TimeZone;
use errors::Result;
use indradb;
use serde_json;
use std::str::FromStr;
use uuid::Uuid;
use autogen;
use capnp;
use errors::Error;

pub trait ErrorableFrom<T>: Sized {
    fn errorable_from(&T) -> Result<Self>;
}

// impl From<indradb::Vertex> for autogen::vertex::Vertex {
//     fn from(vertex: indradb::Vertex) -> Self {
//         let mut cnp_vertex: autogen::Vertex = autogen::Vertex::new();
//         cnp_vertex.id = vertex.id.hyphenated().to_string();
//         cnp_vertex.field_type = vertex.t.0;
//         cnp_vertex
//     }
// }

impl<'a> ErrorableFrom<autogen::vertex::Reader<'a>> for indradb::Vertex {
    fn errorable_from(reader: &autogen::vertex::Reader) -> Result<Self> {
        let id = Uuid::from_bytes(reader.get_id()?)?;
        let t = indradb::Type::new(reader.get_type()?.to_string())?;
        Ok(indradb::Vertex::with_id(id, t))
    }
}

// impl From<indradb::Edge> for autogen::edge::Edge {
//     fn from(edge: indradb::Edge) -> Self {
//         let mut cnp_edge: autogen::Edge = autogen::Edge::new();
//         cnp_edge.set_key(autogen::EdgeKey::from(edge.key));
//         cnp_edge.set_created_datetime(timestamp_from_datetime(&edge.created_datetime));
//         cnp_edge
//     }
// }

// impl ErrorableFrom<autogen::edge::Edge> for indradb::Edge {
//     fn errorable_from(cnp_edge: &autogen::Edge) -> Result<Self> {
//         let key = indradb::EdgeKey::errorable_from(cnp_edge.get_key())?;
//         let created_datetime = datetime_from_timestamp(cnp_edge.get_created_datetime());
//         Ok(indradb::Edge::new(key, created_datetime))
//     }
// }

// impl From<indradb::EdgeKey> for autogen::edge_key::EdgeKey {
//     fn from(key: indradb::EdgeKey) -> Self {
//         let mut cnp_key: autogen::EdgeKey = autogen::EdgeKey::new();
//         cnp_key.set_outbound_id(key.outbound_id.hyphenated().to_string());
//         cnp_key.set_field_type(key.t.0);
//         cnp_key.set_inbound_id(key.inbound_id.hyphenated().to_string());
//         cnp_key
//     }
// }

// impl ErrorableFrom<autogen::edge_key::EdgeKey> for indradb::EdgeKey {
//     fn errorable_from(cnp_key: &autogen::EdgeKey) -> Result<Self> {
//         Ok(indradb::EdgeKey::new(
//             Uuid::from_str(cnp_key.get_outbound_id())?,
//             indradb::Type::new(cnp_key.get_field_type().to_string())?,
//             Uuid::from_str(cnp_key.get_inbound_id())?,
//         ))
//     }
// }

// impl From<indradb::VertexMetadata> for autogen::vertex_metadata::VertexMetadata {
//     fn from(metadata: indradb::VertexMetadata) -> Self {
//         let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
//         let mut cnp_metadata: autogen::VertexMetadata = autogen::VertexMetadata::new();
//         cnp_metadata.set_id(metadata.id.hyphenated().to_string());
//         cnp_metadata.set_value(value);
//         cnp_metadata
//     }
// }

// impl ErrorableFrom<autogen::vertex_metadata::VertexMetadata> for indradb::VertexMetadata {
//     fn errorable_from(cnp_metadata: &autogen::VertexMetadata) -> Result<Self> {
//         Ok(indradb::VertexMetadata::new(
//             Uuid::from_str(cnp_metadata.get_id())?,
//             serde_json::from_str(cnp_metadata.get_value())?,
//         ))
//     }
// }

// impl From<indradb::EdgeMetadata> for autogen::edge_metadata::EdgeMetadata {
//     fn from(metadata: indradb::EdgeMetadata) -> Self {
//         let value = serde_json::to_string(&metadata.value).expect("Expected to be able to serialize JSON");
//         let mut cnp_metadata: autogen::EdgeMetadata = autogen::EdgeMetadata::new();
//         cnp_metadata.set_key(autogen::EdgeKey::from(metadata.key));
//         cnp_metadata.set_value(value);
//         cnp_metadata
//     }
// }

// impl ErrorableFrom<autogen::edge_metadata::EdgeMetadata> for indradb::EdgeMetadata {
//     fn errorable_from(cnp_metadata: &autogen::EdgeMetadata) -> Result<Self> {
//         Ok(indradb::EdgeMetadata::new(
//             indradb::EdgeKey::errorable_from(cnp_metadata.get_key())?,
//             serde_json::from_str(cnp_metadata.get_value())?,
//         ))
//     }
// }

// impl From<indradb::VertexQuery> for autogen::vertex_query::VertexQuery {
//     fn from(query: indradb::VertexQuery) -> Self {
//         let mut cnp_query = autogen::VertexQuery::new();

//         match query {
//             indradb::VertexQuery::All { start_id, limit } => {
//                 let mut cnp_inner_query = autogen::AllVertexQuery::new();

//                 if let Some(start_id) = start_id {
//                     cnp_inner_query.set_start_id(start_id.hyphenated().to_string());
//                 }

//                 cnp_inner_query.set_limit(limit);
//                 cnp_query.set_all(cnp_inner_query);
//             }
//             indradb::VertexQuery::Vertices { ids } => {
//                 let mut cnp_inner_query = autogen::VerticesVertexQuery::new();
//                 cnp_inner_query.set_ids(ids.iter().map(|id| id.hyphenated().to_string()).collect());
//                 cnp_query.set_vertices(cnp_inner_query);
//             }
//             indradb::VertexQuery::Pipe {
//                 edge_query,
//                 converter,
//                 limit,
//             } => {
//                 let mut cnp_inner_query = autogen::PipeVertexQuery::new();
//                 cnp_inner_query.set_edge_query(autogen::EdgeQuery::from(*edge_query));
//                 cnp_inner_query.set_converter(String::from(converter));
//                 cnp_inner_query.set_limit(limit);
//                 cnp_query.set_pipe(cnp_inner_query);
//             }
//         };

//         cnp_query
//     }
// }

// impl ErrorableFrom<autogen::vertex_query::VertexQuery> for indradb::VertexQuery {
//     fn errorable_from(cnp_query: &autogen::VertexQuery) -> Result<Self> {
//         if cnp_query.has_all() {
//             let query = cnp_query.get_all();
//             Ok(indradb::VertexQuery::All {
//                 start_id: from_defaultable(&query.get_start_id(), |s| Ok(Uuid::from_str(s)?))?,
//                 limit: query.get_limit(),
//             })
//         } else if cnp_query.has_vertices() {
//             let query = cnp_query.get_vertices();
//             let ids: Result<Vec<Uuid>> = query
//                 .get_ids()
//                 .iter()
//                 .map(|s| Ok(Uuid::from_str(s)?))
//                 .collect();
//             Ok(indradb::VertexQuery::Vertices { ids: ids? })
//         } else if cnp_query.has_pipe() {
//             let query = cnp_query.get_pipe();
//             Ok(indradb::VertexQuery::Pipe {
//                 edge_query: Box::new(indradb::EdgeQuery::errorable_from(query.get_edge_query())?),
//                 converter: indradb::EdgeDirection::from_str(query.get_converter())?,
//                 limit: query.get_limit(),
//             })
//         } else {
//             unreachable!();
//         }
//     }
// }

// impl From<indradb::EdgeQuery> for autogen::edge_query::EdgeQuery {
//     fn from(query: indradb::EdgeQuery) -> Self {
//         let mut cnp_query = autogen::EdgeQuery::new();

//         match query {
//             indradb::EdgeQuery::Edges { keys } => {
//                 let mut cnp_inner_query = autogen::EdgesEdgeQuery::new();
//                 cnp_inner_query.set_keys(keys.into_iter().map(autogen::EdgeKey::from).collect());
//                 cnp_query.set_edges(cnp_inner_query);
//             }
//             indradb::EdgeQuery::Pipe {
//                 vertex_query,
//                 converter,
//                 type_filter,
//                 high_filter,
//                 low_filter,
//                 limit,
//             } => {
//                 let mut cnp_inner_query = autogen::PipeEdgeQuery::new();
//                 cnp_inner_query.set_vertex_query(autogen::VertexQuery::from(*vertex_query));
//                 cnp_inner_query.set_converter(String::from(converter));

//                 if let Some(type_filter) = type_filter {
//                     cnp_inner_query.set_type_filter(type_filter.0);
//                 }

//                 if let Some(high_filter) = high_filter {
//                     cnp_inner_query.set_high_filter(timestamp_from_datetime(&high_filter));
//                 }

//                 if let Some(low_filter) = low_filter {
//                     cnp_inner_query.set_low_filter(timestamp_from_datetime(&low_filter));
//                 }

//                 cnp_inner_query.set_limit(limit);
//                 cnp_query.set_pipe(cnp_inner_query);
//             }
//         };

//         cnp_query
//     }
// }

// impl ErrorableFrom<autogen::edge_query::EdgeQuery> for indradb::EdgeQuery {
//     fn errorable_from(cnp_query: &autogen::EdgeQuery) -> Result<Self> {
//         if cnp_query.has_edges() {
//             let query = cnp_query.get_edges();
//             let ids: Result<Vec<indradb::EdgeKey>> = query
//                 .get_keys()
//                 .iter()
//                 .map(indradb::EdgeKey::errorable_from)
//                 .collect();
//             Ok(indradb::EdgeQuery::Edges { keys: ids? })
//         } else if cnp_query.has_pipe() {
//             let query = cnp_query.get_pipe();
//             Ok(indradb::EdgeQuery::Pipe {
//                 vertex_query: Box::new(indradb::VertexQuery::errorable_from(
//                     query.get_vertex_query(),
//                 )?),
//                 converter: indradb::EdgeDirection::from_str(query.get_converter())?,
//                 type_filter: from_defaultable(&query.get_type_filter(), |t| {
//                     Ok(indradb::Type::new(t.to_string())?)
//                 })?,
//                 high_filter: if query.has_high_filter() {
//                     Some(datetime_from_timestamp(query.get_high_filter()))
//                 } else {
//                     None
//                 },
//                 low_filter: if query.has_low_filter() {
//                     Some(datetime_from_timestamp(query.get_low_filter()))
//                 } else {
//                     None
//                 },
//                 limit: query.get_limit(),
//             })
//         } else {
//             unreachable!();
//         }
//     }
// }

// fn datetime_from_timestamp(ts: &well_known_types::Timestamp) -> DateTime<Utc> {
//     Utc.timestamp(ts.get_seconds(), ts.get_nanos() as u32)
// }

// fn timestamp_from_datetime(dt: &DateTime<Utc>) -> well_known_types::Timestamp {
//     let mut timestamp = well_known_types::Timestamp::new();
//     timestamp.set_seconds(dt.timestamp());
//     timestamp.set_nanos(dt.timestamp_subsec_nanos() as i32);
//     timestamp
// }

// pub fn from_defaultable<T, U, F>(t: &T, mapper: F) -> Result<Option<U>>
// where
//     T: Default + PartialEq,
//     F: Fn(&T) -> Result<U>,
// {
//     if t == &T::default() {
//         Ok(None)
//     } else {
//         let val = mapper(t)?;
//         Ok(Some(val))
//     }
// }
