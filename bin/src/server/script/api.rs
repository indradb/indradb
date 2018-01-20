#![allow(unreachable_code)]
// Above ignore is there because otherwise the macro is noisy

use super::converters::*;
use indradb::{Error, Transaction};
use std::collections::HashMap;

pub fn create_vertex((trans, t): (ProxyTransaction, Type)) -> Result<Uuid, Error> {
    Ok(Uuid::new(trans.trans.create_vertex(t.0)?))
}

pub fn get_vertices((trans, q): (ProxyTransaction, VertexQuery)) -> Result<Vec<Vertex>, Error> {
    Ok(trans
        .trans
        .get_vertices(q.0)?
        .into_iter()
        .map(Vertex::new)
        .collect())
}

pub fn delete_vertices((trans, q): (ProxyTransaction, VertexQuery)) -> Result<(), Error> {
    trans.trans.delete_vertices(q.0)
}

pub fn create_edge((trans, key, weight): (ProxyTransaction, EdgeKey, Weight)) -> Result<(), Error> {
    trans.trans.create_edge(key.0, weight.0)?;
    Ok(())
}

pub fn get_edges((trans, q): (ProxyTransaction, EdgeQuery)) -> Result<Vec<Edge>, Error> {
    Ok(trans
        .trans
        .get_edges(q.0)?
        .into_iter()
        .map(Edge::new)
        .collect())
}

pub fn delete_edges((trans, q): (ProxyTransaction, EdgeQuery)) -> Result<(), Error> {
    trans.trans.delete_edges(q.0)?;
    Ok(())
}

pub fn get_edge_count((trans, q): (ProxyTransaction, EdgeQuery)) -> Result<u64, Error> {
    Ok(trans.trans.get_edge_count(q.0)?)
}

pub fn get_global_metadata((trans, key): (ProxyTransaction, String)) -> Result<JsonValue, Error> {
    Ok(JsonValue::new(trans.trans.get_global_metadata(key)?))
}

pub fn set_global_metadata(
    (trans, key, value): (ProxyTransaction, String, JsonValue),
) -> Result<(), Error> {
    trans.trans.set_global_metadata(key, value.0)?;
    Ok(())
}

pub fn delete_global_metadata((trans, key): (ProxyTransaction, String)) -> Result<(), Error> {
    trans.trans.delete_global_metadata(key)?;
    Ok(())
}

pub fn get_account_metadata(
    (trans, owner_id, key): (ProxyTransaction, Uuid, String),
) -> Result<JsonValue, Error> {
    Ok(JsonValue::new(trans
        .trans
        .get_account_metadata(owner_id.0, key)?))
}

pub fn set_account_metadata(
    (trans, owner_id, key, value): (ProxyTransaction, Uuid, String, JsonValue),
) -> Result<(), Error> {
    Ok(trans.trans.set_account_metadata(owner_id.0, key, value.0)?)
}

pub fn delete_account_metadata(
    (trans, owner_id, key): (ProxyTransaction, Uuid, String),
) -> Result<(), Error> {
    trans.trans.delete_account_metadata(owner_id.0, key)?;
    Ok(())
}

pub fn get_vertex_metadata(
    (trans, q, key): (ProxyTransaction, VertexQuery, String),
) -> Result<HashMap<Uuid, JsonValue>, Error> {
    let old_map = trans.trans.get_vertex_metadata(q.0, key)?;
    let mut new_map = HashMap::with_capacity(old_map.len());

    for (k, v) in old_map {
        new_map.insert(Uuid::new(k), JsonValue::new(v));
    }

    Ok(new_map)
}

pub fn set_vertex_metadata(
    (trans, q, key, value): (ProxyTransaction, VertexQuery, String, JsonValue),
) -> Result<(), Error> {
    Ok(trans.trans.set_vertex_metadata(q.0, key, value.0)?)
}

pub fn delete_vertex_metadata(
    (trans, q, key): (ProxyTransaction, VertexQuery, String),
) -> Result<(), Error> {
    trans.trans.delete_vertex_metadata(q.0, key)?;
    Ok(())
}

pub fn get_edge_metadata(
    (trans, q, key): (ProxyTransaction, EdgeQuery, String),
) -> Result<HashMap<EdgeKey, JsonValue>, Error> {
    let old_map = trans.trans.get_edge_metadata(q.0, key)?;
    let mut new_map = HashMap::with_capacity(old_map.len());

    for (k, v) in old_map {
        new_map.insert(EdgeKey::new(k), JsonValue::new(v));
    }

    Ok(new_map)
}

pub fn set_edge_metadata(
    (trans, q, key, value): (ProxyTransaction, EdgeQuery, String, JsonValue),
) -> Result<(), Error> {
    trans.trans.set_edge_metadata(q.0, key, value.0)?;
    Ok(())
}

pub fn delete_edge_metadata(
    (trans, q, key): (ProxyTransaction, EdgeQuery, String),
) -> Result<(), Error> {
    trans.trans.delete_edge_metadata(q.0, key)?;
    Ok(())
}
