#![allow(unreachable_code)]
// Above ignore is there because otherwise the macro is noisy

use lua;
use common::ProxyTransaction;
use braid::{Vertex, Edge, Transaction};
use std::i32;
use super::util::*;
use super::errors::LuaError;
use uuid::Uuid;

lua_fn! {
    pub unsafe fn get_vertex_range(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let start_id = get_optional_uuid_param(l, 1)?;
        let limit = get_limit_param(l, 2)?;
        let result = trans.get_vertex_range(start_id.unwrap_or_else(Uuid::default), limit)?;
        serialize_vertices(l, result);
        Ok(1)
    }

    pub unsafe fn get_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = get_uuid_param(l, 1)?;
        let result = trans.get_vertex(id)?;
        serialize_vertex(l, &result);
        Ok(1)
    }

    pub unsafe fn create_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let t = get_type_param(l, 1)?;
        let result = trans.create_vertex(t)?;
        l.pushstring(&result.to_string()[..]);
        Ok(1)
    }

    pub unsafe fn set_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let v = Vertex::new(id, t);
        trans.set_vertex(v)?;
        Ok(0)
    }

    pub unsafe fn delete_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = get_uuid_param(l, 1)?;
        trans.delete_vertex(id)?;
        Ok(0)
    }

    pub unsafe fn get_edge(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let inbound_id = get_uuid_param(l, 3)?;
        let result = trans.get_edge(outbound_id, t, inbound_id)?;
        serialize_edge(l, &result);
        Ok(1)
    }

    pub unsafe fn set_edge(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let inbound_id = get_uuid_param(l, 3)?;
        let weight = get_weight_param(l, 4)?;
        let e = Edge::new_with_current_datetime(outbound_id, t, inbound_id, weight);
        trans.set_edge(e)?;
        Ok(1)
    }

    pub unsafe fn delete_edge(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_type_param(l, 2)?;
        let inbound_id = get_uuid_param(l, 3)?;
        trans.delete_edge(outbound_id, t, inbound_id)?;
        Ok(0)
    }

    pub unsafe fn get_edge_count(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_optional_type_param(l, 2)?;
        let result = trans.get_edge_count(outbound_id, t)?;
        serialize_u64(l, result);
        Ok(1)
    }

    pub unsafe fn get_edge_range(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = get_uuid_param(l, 1)?;
        let t = get_optional_type_param(l, 2)?;
        let high = get_optional_datetime_param(l, 3)?;
        let low = get_optional_datetime_param(l, 4)?;
        let limit = get_limit_param(l, 5)?;
        let result = trans.get_edge_range(outbound_id, t, high, low, limit)?;
        serialize_edges(l, result);
        Ok(1)
    }

    pub unsafe fn get_reversed_edge_count(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let inbound_id = get_uuid_param(l, 1)?;
        let t = get_optional_type_param(l, 2)?;
        let result = trans.get_reversed_edge_count(inbound_id, t)?;
        serialize_u64(l, result);
        Ok(1)
    }

    pub unsafe fn get_reversed_edge_range(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let inbound_id = get_uuid_param(l, 1)?;
        let t = get_optional_type_param(l, 2)?;
        let high = get_optional_datetime_param(l, 3)?;
        let low = get_optional_datetime_param(l, 4)?;
        let limit = get_limit_param(l, 5)?;
        let result = trans.get_reversed_edge_range(inbound_id, t, high, low, limit)?;
        serialize_edges(l, result);
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
