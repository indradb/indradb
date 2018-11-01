//! Converts between cnp and native IndraDB models

use autogen;
use capnp;
use capnp::Error as CapnpError;
use chrono::{DateTime, TimeZone, Utc};
use indradb;
use serde_json;
use std::error::Error;
use uuid::Uuid;
use std::vec::IntoIter;

const NANOS_PER_SEC: u64 = 1_000_000_000;

#[macro_export]
macro_rules! map_err {
    ($e:expr) => {
        $e.map_err(|err| capnp::Error::failed(err.description().to_string()))
    };
}

pub fn from_vertex<'a>(vertex: &indradb::Vertex, mut builder: autogen::vertex::Builder<'a>) {
    builder.set_id(vertex.id.as_bytes());
    builder.set_type(&vertex.t.0);
}

pub fn to_vertex<'a>(reader: &autogen::vertex::Reader<'a>) -> Result<indradb::Vertex, CapnpError> {
    let id = map_err!(Uuid::from_slice(reader.get_id()?))?;
    let t = map_err!(indradb::Type::new(reader.get_type()?.to_string()))?;
    Ok(indradb::Vertex::with_id(id, t))
}

pub fn from_edge<'a>(edge: &indradb::Edge, mut builder: autogen::edge::Builder<'a>) -> Result<(), CapnpError> {
    builder.set_created_datetime(edge.created_datetime.timestamp() as u64);
    from_edge_key(&edge.key, builder.init_key());
    Ok(())
}

pub fn to_edge<'a>(reader: &autogen::edge::Reader<'a>) -> Result<indradb::Edge, CapnpError> {
    let key = to_edge_key(&reader.get_key()?)?;
    let created_datetime = Utc.timestamp(reader.get_created_datetime() as i64, 0);
    Ok(indradb::Edge::new(key, created_datetime))
}

pub fn from_edge_key<'a>(key: &indradb::EdgeKey, mut builder: autogen::edge_key::Builder<'a>) {
    builder.set_outbound_id(key.outbound_id.as_bytes());
    builder.set_type(&key.t.0);
    builder.set_inbound_id(key.inbound_id.as_bytes());
}

pub fn to_edge_key<'a>(reader: &autogen::edge_key::Reader<'a>) -> Result<indradb::EdgeKey, CapnpError> {
    let outbound_id = map_err!(Uuid::from_slice(reader.get_outbound_id()?))?;
    let t = map_err!(indradb::Type::new(reader.get_type()?.to_string()))?;
    let inbound_id = map_err!(Uuid::from_slice(reader.get_inbound_id()?))?;
    Ok(indradb::EdgeKey::new(outbound_id, t, inbound_id))
}

pub fn from_vertex_property<'a>(
    property: &indradb::VertexProperty,
    mut builder: autogen::vertex_property::Builder<'a>,
) {
    builder.set_id(property.id.as_bytes());
    builder.set_value(&property.value.to_string());
}

pub fn to_vertex_property<'a>(
    reader: &autogen::vertex_property::Reader<'a>,
) -> Result<indradb::VertexProperty, CapnpError> {
    let id = map_err!(Uuid::from_slice(reader.get_id()?))?;
    let value = map_err!(serde_json::from_str(reader.get_value()?))?;
    Ok(indradb::VertexProperty::new(id, value))
}

pub fn from_edge_property<'a>(property: &indradb::EdgeProperty, mut builder: autogen::edge_property::Builder<'a>) {
    builder.set_value(&property.value.to_string());
    from_edge_key(&property.key, builder.init_key());
}

pub fn to_edge_property<'a>(reader: &autogen::edge_property::Reader<'a>) -> Result<indradb::EdgeProperty, CapnpError> {
    let key = to_edge_key(&reader.get_key()?)?;
    let value = map_err!(serde_json::from_str(reader.get_value()?))?;
    Ok(indradb::EdgeProperty::new(key, value))
}

pub fn from_vertex_query<'a>(q: &indradb::VertexQuery, builder: autogen::vertex_query::Builder<'a>) {
    match q {
        indradb::VertexQuery::All { start_id, limit } => {
            let mut builder = builder.init_all();

            if let Some(start_id) = start_id {
                builder.set_start_id(start_id.as_bytes());
            }

            builder.set_limit(*limit);
        }
        indradb::VertexQuery::Vertices { ids } => {
            let mut builder = builder.init_vertices().init_ids(ids.len() as u32);

            for (i, id) in ids.iter().enumerate() {
                builder.set(i as u32, id.as_bytes());
            }
        }
        indradb::VertexQuery::Pipe {
            edge_query,
            converter,
            limit,
        } => {
            let mut builder = builder.init_pipe();
            builder.set_converter(from_edge_direction(*converter));
            builder.set_limit(*limit);
            from_edge_query(&edge_query, builder.init_edge_query());
        }
    }
}

pub fn to_vertex_query<'a>(reader: &autogen::vertex_query::Reader<'a>) -> Result<indradb::VertexQuery, CapnpError> {
    match reader.which()? {
        autogen::vertex_query::All(params) => {
            let start_id_bytes = params.get_start_id()?;

            Ok(indradb::VertexQuery::All {
                start_id: if start_id_bytes.is_empty() {
                    None
                } else {
                    Some(map_err!(Uuid::from_slice(start_id_bytes))?)
                },
                limit: params.get_limit(),
            })
        }
        autogen::vertex_query::Vertices(params) => {
            let ids: Result<Vec<Uuid>, CapnpError> = params
                .get_ids()?
                .into_iter()
                .map(|bytes| map_err!(Uuid::from_slice(bytes?)))
                .collect();
            Ok(indradb::VertexQuery::Vertices { ids: ids? })
        }
        autogen::vertex_query::Pipe(params) => {
            let edge_query = Box::new(to_edge_query(&params.get_edge_query()?)?);
            let converter = to_edge_direction(params.get_converter()?);
            let limit = params.get_limit();
            Ok(indradb::VertexQuery::Pipe {
                edge_query,
                converter,
                limit,
            })
        }
    }
}

pub fn from_edge_query<'a>(q: &indradb::EdgeQuery, builder: autogen::edge_query::Builder<'a>) {
    match q {
        indradb::EdgeQuery::Edges { keys } => {
            let mut builder = builder.init_edges().init_keys(keys.len() as u32);

            for (i, key) in keys.iter().enumerate() {
                from_edge_key(key, builder.reborrow().get(i as u32));
            }
        }
        indradb::EdgeQuery::Pipe {
            vertex_query,
            converter,
            type_filter,
            high_filter,
            low_filter,
            limit,
        } => {
            let mut builder = builder.init_pipe();
            builder.set_converter(from_edge_direction(*converter));

            if let Some(type_filter) = type_filter {
                builder.set_type_filter(&type_filter.0);
            }

            if let Some(high_filter) = high_filter {
                builder.set_high_filter(high_filter.timestamp_nanos() as u64);
            }

            if let Some(low_filter) = low_filter {
                builder.set_low_filter(low_filter.timestamp_nanos() as u64);
            }

            builder.set_limit(*limit);
            from_vertex_query(&vertex_query, builder.init_vertex_query());
        }
    }
}

pub fn to_edge_query<'a>(reader: &autogen::edge_query::Reader<'a>) -> Result<indradb::EdgeQuery, CapnpError> {
    match reader.which()? {
        autogen::edge_query::Edges(params) => {
            let keys: Result<Vec<indradb::EdgeKey>, CapnpError> = params
                .get_keys()?
                .into_iter()
                .map(|reader| to_edge_key(&reader))
                .collect();
            Ok(indradb::EdgeQuery::Edges { keys: keys? })
        }
        autogen::edge_query::Pipe(params) => {
            let vertex_query = Box::new(to_vertex_query(&params.get_vertex_query()?)?);
            let converter = to_edge_direction(params.get_converter()?);
            let type_filter = match params.get_type_filter()? {
                "" => None,
                value => Some(map_err!(indradb::Type::new(value.to_string()))?),
            };
            let high_filter = to_optional_datetime(params.get_high_filter());
            let low_filter = to_optional_datetime(params.get_low_filter());
            let limit = params.get_limit();

            Ok(indradb::EdgeQuery::Pipe {
                vertex_query,
                converter,
                type_filter,
                high_filter,
                low_filter,
                limit,
            })
        }
    }
}

pub fn to_bulk_insert_vertex_items<'a>(reader: &capnp::struct_list::Reader<'a, autogen::bulk_insert_item::Owned<autogen::vertex::Owned>>) -> Result<IntoIter<indradb::BulkInsertItem<indradb::Vertex>>, CapnpError> {
    let items: Result<Vec<indradb::BulkInsertItem<indradb::Vertex>>, CapnpError> = reader
        .into_iter()
        .map(|item| {
            let vertex = to_vertex(&item.get_value()?)?;
            let properties = to_bulk_insert_properties(&item.get_properties()?)?;
            Ok(indradb::BulkInsertItem::new(vertex, properties))
        })
        .collect();
    Ok(items?.into_iter())
}

pub fn to_bulk_insert_edge_items<'a>(reader: &capnp::struct_list::Reader<'a, autogen::bulk_insert_item::Owned<autogen::edge_key::Owned>>) -> Result<IntoIter<indradb::BulkInsertItem<indradb::EdgeKey>>, CapnpError> {
    let items: Result<Vec<indradb::BulkInsertItem<indradb::EdgeKey>>, CapnpError> = reader
        .into_iter()
        .map(|item| {
            let edge_key = to_edge_key(&item.get_value()?)?;
            let properties = to_bulk_insert_properties(&item.get_properties()?)?;
            Ok(indradb::BulkInsertItem::new(edge_key, properties))
        })
        .collect();
    Ok(items?.into_iter())
}

pub fn to_bulk_insert_properties<'a>(reader: &capnp::struct_list::Reader<'a, autogen::bulk_insert_property::Owned>) -> Result<Vec<indradb::BulkInsertProperty>, CapnpError> {
    let properties: Result<Vec<indradb::BulkInsertProperty>, CapnpError> = reader
        .into_iter()
        .map(|item| {
            let name = item.get_name()?;
            let value = map_err!(serde_json::from_str(item.get_value()?))?;
            Ok(indradb::BulkInsertProperty::new(name.to_string(), value))
        })
        .collect();
    properties
}

pub fn from_edge_direction(direction: indradb::EdgeDirection) -> autogen::EdgeDirection {
    match direction {
        indradb::EdgeDirection::Outbound => autogen::EdgeDirection::Outbound,
        indradb::EdgeDirection::Inbound => autogen::EdgeDirection::Inbound,
    }
}

pub fn to_edge_direction(direction: autogen::EdgeDirection) -> indradb::EdgeDirection {
    match direction {
        autogen::EdgeDirection::Outbound => indradb::EdgeDirection::Outbound,
        autogen::EdgeDirection::Inbound => indradb::EdgeDirection::Inbound,
    }
}

pub fn to_optional_datetime(timestamp: u64) -> Option<DateTime<Utc>> {
    if timestamp == 0 {
        None
    } else {
        let secs = timestamp / NANOS_PER_SEC;
        let nanos = timestamp % NANOS_PER_SEC;
        Some(Utc.timestamp(secs as i64, nanos as u32))
    }
}

