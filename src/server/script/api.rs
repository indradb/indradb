#![allow(unreachable_code)]
// Above ignore is there because otherwise the macro is noisy

use lua;
use common::ProxyTransaction;
use braid::{Edge, Transaction, VertexQuery};
use std::i32;
use super::util::*;
use super::errors::LuaError;

lua_fn! {
    pub unsafe fn create_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let t = get_type_param(l, 1)?;
        let result = trans.create_vertex(t)?;
        l.pushstring(&result.to_string()[..]);
        Ok(1)
    }

    pub unsafe fn get_vertices(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_vertex_query_param(l, 1)?;
        let result = trans.get_vertices(q)?;
        serialize_vertices(l, result);
        Ok(1)
    }

    pub unsafe fn set_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        trans.set_vertices(VertexQuery::Vertex(id), t)?;
        Ok(0)
    }

    pub unsafe fn delete_vertices(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_vertex_query_param(l, 1)?;
        trans.delete_vertices(q)?;
        Ok(0)
    }

    pub unsafe fn create_edge(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let inbound_id = get_uuid_param(l, 3)?;
        let weight = get_weight_param(l, 4)?;
        trans.create_edge(Edge::new_with_current_datetime(outbound_id, t, inbound_id, weight))?;
        Ok(0)
    }

    pub unsafe fn get_edges(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        let result = trans.get_edges(q)?;
        serialize_edges(l, result);
        Ok(1)
    }

    pub unsafe fn set_edges(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        let weight = get_weight_param(l, 2)?;
        trans.set_edges(q, weight)?;
        Ok(0)
    }

    pub unsafe fn delete_edges(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        trans.delete_edges(q)?;
        Ok(0)
    }

    pub unsafe fn get_edge_count(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        let result = trans.get_edge_count(q)?;
        serialize_u64(l, result);
        Ok(1)
    }

    pub unsafe fn get_global_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = get_string_param(l, 1)?;
        let result = trans.get_global_metadata(key.clone())?;
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_global_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = get_string_param(l, 1)?;
        let value = deserialize_json(l, 2)?;
        trans.set_global_metadata(key, value)?;
        Ok(0)
    }

    pub unsafe fn delete_global_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = get_string_param(l, 1)?;
        trans.delete_global_metadata(key)?;
        Ok(0)
    }

    pub unsafe fn get_account_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = get_uuid_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let result = trans.get_account_metadata(owner_id, key)?;
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_account_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = get_uuid_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let value = deserialize_json(l, 3)?;
        trans.set_account_metadata(owner_id, key, value)?;
        Ok(0)
    }

    pub unsafe fn delete_account_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = get_uuid_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        trans.delete_account_metadata(owner_id, key)?;
        Ok(0)
    }

    pub unsafe fn get_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = get_uuid_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let result = trans.get_vertex_metadata(owner_id, key)?;
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = get_uuid_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let value = deserialize_json(l, 3)?;
        trans.set_vertex_metadata(owner_id, key, value)?;
        Ok(0)
    }

    pub unsafe fn delete_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = get_uuid_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        trans.delete_vertex_metadata(owner_id, key)?;
        Ok(0)
    }

    pub unsafe fn get_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let inbound_id = get_uuid_param(l, 3)?;
        let key = get_string_param(l, 4)?;
        let result = trans.get_edge_metadata(outbound_id, t, inbound_id, key)?;
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let inbound_id = get_uuid_param(l, 3)?;
        let key = get_string_param(l, 4)?;
        let value = deserialize_json(l, 5)?;
        trans.set_edge_metadata(outbound_id, t, inbound_id, key, value)?;
        Ok(0)
    }

    pub unsafe fn delete_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let inbound_id = get_uuid_param(l, 3)?;
        let key = get_string_param(l, 4)?;
        trans.delete_edge_metadata(outbound_id, t, inbound_id, key)?;
        Ok(0)
    }
}
