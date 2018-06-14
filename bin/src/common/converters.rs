//! Converts between cnp and native IndraDB models

use chrono::{DateTime, Utc, NaiveDateTime};
use chrono::TimeZone;
use indradb;
use serde_json;
use std::str::FromStr;
use uuid::Uuid;
use autogen;
use capnp;
use std::error::Error;
use capnp::Error as CapnpError;

#[macro_export]
macro_rules! map_err {
    ($e:expr) => ($e.map_err(|err| capnp::Error::failed(err.description().to_string())));
}

pub fn from_vertex<'a>(vertex: indradb::Vertex, builder: &mut autogen::vertex::Builder<'a>) {
    builder.set_id(vertex.id.as_bytes());
    builder.set_type(&vertex.t.0);
}

pub fn to_vertex<'a>(reader: &autogen::vertex::Reader<'a>) -> Result<indradb::Vertex, CapnpError> {
    let id = map_err!(Uuid::from_bytes(reader.get_id()?))?;
    let t = map_err!(indradb::Type::new(reader.get_type()?.to_string()))?;
    Ok(indradb::Vertex::with_id(id, t))
}

pub fn from_edge<'a>(edge: indradb::Edge, builder: &mut autogen::edge::Builder<'a>) -> Result<(), CapnpError> {
    from_edge_key(edge.key, &mut builder.reborrow().init_key());
    builder.set_created_datetime(edge.created_datetime.timestamp() as u64);
    Ok(())
}

pub fn from_edge_key<'a>(key: indradb::EdgeKey, builder: &mut autogen::edge_key::Builder<'a>) {
    builder.set_outbound_id(key.outbound_id.as_bytes());
    builder.set_type(&key.t.0);
    builder.set_inbound_id(key.inbound_id.as_bytes());
}

pub fn to_edge_key<'a>(reader: &autogen::edge_key::Reader<'a>) -> Result<indradb::EdgeKey, CapnpError> {
    let outbound_id = map_err!(Uuid::from_bytes(reader.get_outbound_id()?))?;
    let t = map_err!(indradb::Type::new(reader.get_type()?.to_string()))?;
    let inbound_id = map_err!(Uuid::from_bytes(reader.get_outbound_id()?))?;
    Ok(indradb::EdgeKey::new(outbound_id, t, inbound_id))
}

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

pub fn to_vertex_query<'a>(reader: &autogen::vertex_query::Reader<'a>) -> Result<indradb::VertexQuery, CapnpError> {
    match reader.which()? {
        autogen::vertex_query::All(params) => {
            let start_id_bytes = params.get_start_id()?;

            Ok(indradb::VertexQuery::All {
                start_id: if start_id_bytes.len() == 0 {
                    None
                } else {
                    Some(map_err!(Uuid::from_bytes(start_id_bytes))?)
                },
                limit: params.get_limit(),
            })
        },
        autogen::vertex_query::Vertices(params) => {
            let ids: Result<Vec<Uuid>, CapnpError> = params.get_ids()?.into_iter().map(|bytes| {
                map_err!(Uuid::from_bytes(bytes?))
            }).collect();
            Ok(indradb::VertexQuery::Vertices { ids: ids? })
        },
        autogen::vertex_query::Pipe(params) => {
            let edge_query = Box::new(to_edge_query(&params.get_edge_query()?)?);
            let converter = to_edge_direction(params.get_converter()?);
            let limit = params.get_limit();
            Ok(indradb::VertexQuery::Pipe { edge_query, converter, limit })
        }
    }
}

pub fn to_edge_query<'a>(reader: &autogen::edge_query::Reader<'a>) -> Result<indradb::EdgeQuery, CapnpError> {
    match reader.which()? {
        autogen::edge_query::Edges(params) => {
            let keys: Result<Vec<indradb::EdgeKey>, CapnpError> = params.get_keys()?.into_iter().map(|reader| to_edge_key(&reader)).collect();
            Ok(indradb::EdgeQuery::Edges { keys: keys? })
        },
        autogen::edge_query::Pipe(params) => {
            let vertex_query = Box::new(to_vertex_query(&params.get_vertex_query()?)?);
            let converter = to_edge_direction(params.get_converter()?);
            let type_filter = match params.get_type_filter()? {
                "" => None,
                value => Some(map_err!(indradb::Type::new(value.to_string()))?)
            };
            let high_filter = match params.get_high_filter() {
                0 => None,
                value => Some(Utc.timestamp(value as i64, 0))
            };
            let low_filter = match params.get_low_filter() {
                0 => None,
                value => Some(Utc.timestamp(value as i64, 0))
            };
            let limit = params.get_limit();
            
            Ok(indradb::EdgeQuery::Pipe {
                vertex_query,
                converter,
                type_filter,
                high_filter,
                low_filter,
                limit
            })
        }
    }
}

pub fn to_edge_direction(direction: autogen::EdgeDirection) -> indradb::EdgeDirection {
    match direction {
        autogen::EdgeDirection::Outbound => indradb::EdgeDirection::Outbound,
        autogen::EdgeDirection::Inbound => indradb::EdgeDirection::Inbound
    }
}
