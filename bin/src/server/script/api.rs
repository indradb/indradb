#![allow(unreachable_code)]
// Above ignore is there because otherwise the macro is noisy

use super::converters;
use indradb::{Error, Transaction};
use common::ProxyTransaction;
use std::collections::HashMap;

pub fn create_vertex(trans: &ProxyTransaction, t: converters::Type) -> Result<converters::Uuid, Error> {
    Ok(converters::Uuid::new(trans.create_vertex(t.0)?))
}

pub fn get_vertices(trans: &ProxyTransaction, q: converters::VertexQuery) -> Result<Vec<converters::Vertex>, Error> {
    Ok(trans
        .get_vertices(q.0)?
        .into_iter()
        .map(converters::Vertex::new)
        .collect())
}

pub fn delete_vertices(trans: &ProxyTransaction, q: converters::VertexQuery) -> Result<(), Error> {
    trans.delete_vertices(q.0)
}

pub fn create_edge(trans: &ProxyTransaction, key: converters::EdgeKey) -> Result<(), Error> {
    trans.create_edge(key.0)?;
    Ok(())
}

pub fn get_edges(trans: &ProxyTransaction, q: converters::EdgeQuery) -> Result<Vec<converters::Edge>, Error> {
    Ok(trans
        .get_edges(q.0)?
        .into_iter()
        .map(converters::Edge::new)
        .collect())
}

pub fn delete_edges(trans: &ProxyTransaction, q: converters::EdgeQuery) -> Result<(), Error> {
    trans.delete_edges(q.0)?;
    Ok(())
}

pub fn get_edge_count(trans: &ProxyTransaction, q: converters::EdgeQuery) -> Result<u64, Error> {
    Ok(trans.get_edge_count(q.0)?)
}

pub fn get_global_metadata(trans: &ProxyTransaction, key: String) -> Result<converters::JsonValue, Error> {
    Ok(converters::JsonValue::new(trans.get_global_metadata(key)?))
}

pub fn set_global_metadata(
    trans: &ProxyTransaction, (key, value): (String, converters::JsonValue),
) -> Result<(), Error> {
    trans.set_global_metadata(key, value.0)?;
    Ok(())
}

pub fn delete_global_metadata(trans: &ProxyTransaction, key: String) -> Result<(), Error> {
    trans.delete_global_metadata(key)?;
    Ok(())
}

pub fn get_vertex_metadata(
    trans: &ProxyTransaction, (q, key): (converters::VertexQuery, String),
) -> Result<HashMap<converters::Uuid, converters::JsonValue>, Error> {
    let old_map = trans.get_vertex_metadata(q.0, key)?;
    let mut new_map = HashMap::with_capacity(old_map.len());

    for (k, v) in old_map {
        new_map.insert(converters::Uuid::new(k), converters::JsonValue::new(v));
    }

    Ok(new_map)
}

pub fn set_vertex_metadata(
    trans: &ProxyTransaction, (q, key, value): (converters::VertexQuery, String, converters::JsonValue),
) -> Result<(), Error> {
    Ok(trans.set_vertex_metadata(q.0, key, value.0)?)
}

pub fn delete_vertex_metadata(
    trans: &ProxyTransaction, (q, key): (converters::VertexQuery, String),
) -> Result<(), Error> {
    trans.delete_vertex_metadata(q.0, key)?;
    Ok(())
}

pub fn get_edge_metadata(
    trans: &ProxyTransaction, (q, key): (converters::EdgeQuery, String),
) -> Result<HashMap<converters::EdgeKey, converters::JsonValue>, Error> {
    let old_map = trans.get_edge_metadata(q.0, key)?;
    let mut new_map = HashMap::with_capacity(old_map.len());

    for (k, v) in old_map {
        new_map.insert(converters::EdgeKey::new(k), converters::JsonValue::new(v));
    }

    Ok(new_map)
}

pub fn set_edge_metadata(
    trans: &ProxyTransaction, (q, key, value): (converters::EdgeQuery, String, converters::JsonValue),
) -> Result<(), Error> {
    trans.set_edge_metadata(q.0, key, value.0)?;
    Ok(())
}

pub fn delete_edge_metadata(
    trans: &ProxyTransaction, (q, key): (converters::EdgeQuery, String),
) -> Result<(), Error> {
    trans.delete_edge_metadata(q.0, key)?;
    Ok(())
}
