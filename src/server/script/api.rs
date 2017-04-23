#![allow(unreachable_code)]
// Above ignore is there because otherwise the macro is noisy

use lua;
use common::ProxyTransaction;
use braid::{Transaction, EdgeKey};
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
        let key = EdgeKey::new(outbound_id, t, inbound_id);
        trans.create_edge(key, weight)?;
        Ok(0)
    }

    pub unsafe fn get_edges(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        let result = trans.get_edges(q)?;
        serialize_edges(l, result);
        Ok(1)
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
        let result = trans.get_global_metadata(key)?;
        serialize_json(l, &result);
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
        serialize_json(l, &result);
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
        let q = get_vertex_query_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let result = trans.get_vertex_metadata(q, key)?;
        
        // Return the results
        l.newtable();
        for (id, v) in result {
            // Push the key
            l.pushstring(&id.to_string()[..]);
            // Push the value
            serialize_json(l, &v);
            // Add the key/value to the table
            l.settable(-3);
        }
        
        Ok(1)
    }

    pub unsafe fn set_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_vertex_query_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let value = deserialize_json(l, 3)?;
        trans.set_vertex_metadata(q, key, value)?;
        Ok(0)
    }

    pub unsafe fn delete_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_vertex_query_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        trans.delete_vertex_metadata(q, key)?;
        Ok(0)
    }

    pub unsafe fn get_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let result = trans.get_edge_metadata(q, key)?;
        
        // Return the results
        l.newtable();
        for (k, v) in result {
            // Push the key
            {
                l.newtable();
                l.pushinteger(1);
                l.pushstring(&k.outbound_id.to_string()[..]);
                l.settable(-3);
                l.pushinteger(2);
                l.pushstring(&k.t.0[..]);
                l.settable(-3);
                l.pushinteger(3);
                l.pushstring(&k.inbound_id.to_string()[..]);
                l.settable(-3);
            }
            // Push the value
            serialize_json(l, &v);
            // Add the key/value to the table
            l.settable(-3);
        }

        Ok(1)
    }

    pub unsafe fn set_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        let value = deserialize_json(l, 3)?;
        trans.set_edge_metadata(q, key, value)?;
        Ok(0)
    }

    pub unsafe fn delete_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let q = get_edge_query_param(l, 1)?;
        let key = get_string_param(l, 2)?;
        trans.delete_edge_metadata(q, key)?;
        Ok(0)
    }
}
