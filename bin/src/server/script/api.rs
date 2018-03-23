#![allow(unreachable_code)]
// Above ignore is there because otherwise the macro is noisy

use super::converters;
use common::ProxyTransaction;
use indradb::{Error, Transaction};
use serde_json::Value as JsonValue;

pub fn create_vertex_from_type(trans: &ProxyTransaction, t: converters::Type) -> Result<converters::Uuid, Error> {
    Ok(converters::Uuid::new(trans.create_vertex_from_type(t.0)?))
}

pub fn get_vertices(trans: &ProxyTransaction, q: converters::VertexQuery) -> Result<Vec<converters::Vertex>, Error> {
    Ok(trans
        .get_vertices(&q.0)?
        .into_iter()
        .map(converters::Vertex::new)
        .collect())
}

pub fn delete_vertices(trans: &ProxyTransaction, q: converters::VertexQuery) -> Result<(), Error> {
    trans.delete_vertices(&q.0)
}

pub fn get_vertex_count(trans: &ProxyTransaction, _: ()) -> Result<u64, Error> {
    trans.get_vertex_count()
}

pub fn create_edge(trans: &ProxyTransaction, key: converters::EdgeKey) -> Result<bool, Error> {
    Ok(trans.create_edge(&key.0)?)
}

pub fn get_edges(trans: &ProxyTransaction, q: converters::EdgeQuery) -> Result<Vec<converters::Edge>, Error> {
    Ok(trans
        .get_edges(&q.0)?
        .into_iter()
        .map(converters::Edge::new)
        .collect())
}

pub fn delete_edges(trans: &ProxyTransaction, q: converters::EdgeQuery) -> Result<(), Error> {
    trans.delete_edges(&q.0)?;
    Ok(())
}

pub fn get_edge_count(
    trans: &ProxyTransaction,
    (id, type_filter, direction): (
        converters::Uuid,
        Option<converters::Type>,
        converters::EdgeDirection,
    ),
) -> Result<u64, Error> {
    Ok(trans.get_edge_count(id.0, type_filter.as_ref().map(|t| &t.0), direction.0)?)
}

pub fn get_global_metadata(trans: &ProxyTransaction, key: String) -> Result<converters::JsonValue, Error> {
    Ok(converters::JsonValue::new(
        trans
            .get_global_metadata(&key)?
            .unwrap_or_else(|| JsonValue::Null),
    ))
}

pub fn set_global_metadata(
    trans: &ProxyTransaction,
    (key, value): (String, converters::JsonValue),
) -> Result<(), Error> {
    trans.set_global_metadata(&key, &value.0)?;
    Ok(())
}

pub fn delete_global_metadata(trans: &ProxyTransaction, key: String) -> Result<(), Error> {
    trans.delete_global_metadata(&key)?;
    Ok(())
}

pub fn get_vertex_metadata(
    trans: &ProxyTransaction,
    (q, key): (converters::VertexQuery, String),
) -> Result<Vec<converters::VertexMetadata>, Error> {
    Ok(trans
        .get_vertex_metadata(&q.0, &key)?
        .into_iter()
        .map(converters::VertexMetadata::new)
        .collect())
}

pub fn set_vertex_metadata(
    trans: &ProxyTransaction,
    (q, key, value): (converters::VertexQuery, String, converters::JsonValue),
) -> Result<(), Error> {
    Ok(trans.set_vertex_metadata(&q.0, &key, &value.0)?)
}

pub fn delete_vertex_metadata(
    trans: &ProxyTransaction,
    (q, key): (converters::VertexQuery, String),
) -> Result<(), Error> {
    trans.delete_vertex_metadata(&q.0, &key)?;
    Ok(())
}

pub fn get_edge_metadata(
    trans: &ProxyTransaction,
    (q, key): (converters::EdgeQuery, String),
) -> Result<Vec<converters::EdgeMetadata>, Error> {
    Ok(trans
        .get_edge_metadata(&q.0, &key)?
        .into_iter()
        .map(converters::EdgeMetadata::new)
        .collect())
}

pub fn set_edge_metadata(
    trans: &ProxyTransaction,
    (q, key, value): (converters::EdgeQuery, String, converters::JsonValue),
) -> Result<(), Error> {
    trans.set_edge_metadata(&q.0, &key, &value.0)?;
    Ok(())
}

pub fn delete_edge_metadata(trans: &ProxyTransaction, (q, key): (converters::EdgeQuery, String)) -> Result<(), Error> {
    trans.delete_edge_metadata(&q.0, &key)?;
    Ok(())
}
