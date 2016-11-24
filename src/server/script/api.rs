use lua;
use common::ProxyTransaction;
use nutrino::{Vertex, Edge, Transaction};
use std::i32;
use super::util::*;
use super::errors::LuaError;

lua_fn! {
    pub unsafe fn get_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_uuid_param(l, 1));
        let result = try!(trans.get_vertex(id));
        l.newtable();
        add_string_field_to_table(l, "id", &result.id.to_string()[..]);
        add_string_field_to_table(l, "type", &result.t.0[..]);
        Ok(1)
    }

    pub unsafe fn create_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let t = try!(get_type_param(l, 1));
        let result = try!(trans.create_vertex(t));
        l.pushstring(&result.to_string()[..]);
        Ok(1)
    }

    pub unsafe fn set_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let v = Vertex::new(id, t);
        try!(trans.set_vertex(v));
        Ok(0)
    }

    pub unsafe fn delete_vertex(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_uuid_param(l, 1));
        try!(trans.delete_vertex(id));
        Ok(0)
    }

    pub unsafe fn get_edge(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let inbound_id = try!(get_uuid_param(l, 3));
        let result = try!(trans.get_edge(outbound_id, t, inbound_id));
        serialize_edge(l, &result);
        Ok(1)
    }

    pub unsafe fn set_edge(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let inbound_id = try!(get_uuid_param(l, 3));
        let weight = try!(get_weight_param(l, 4));
        let e = Edge::new(outbound_id, t, inbound_id, weight);
        try!(trans.set_edge(e));
        Ok(1)
    }

    pub unsafe fn delete_edge(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let inbound_id = try!(get_uuid_param(l, 3));
        try!(trans.delete_edge(outbound_id, t, inbound_id));
        Ok(0)
    }

    pub unsafe fn get_edge_count(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let result = try!(trans.get_edge_count(outbound_id, t));
        serialize_u64(l, result);
        Ok(1)
    }

    pub unsafe fn get_edge_range(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let offset = try!(get_offset_param(l, 3));
        let limit = try!(get_limit_param(l, 4));
        let result = try!(trans.get_edge_range(outbound_id, t, offset, limit));
        serialize_edges(l, result);
        Ok(1)
    }

    pub unsafe fn get_edge_time_range(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let high = try!(get_optional_datetime_param(l, 3));
        let low = try!(get_optional_datetime_param(l, 4));
        let limit = try!(get_limit_param(l, 5));
        let result = try!(trans.get_edge_time_range(outbound_id, t, high, low, limit));
        serialize_edges(l, result);
        Ok(1)
    }

    pub unsafe fn get_reversed_edge_count(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let inbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let result = try!(trans.get_reversed_edge_count(inbound_id, t));
        serialize_u64(l, result);
        Ok(1)
    }

    pub unsafe fn get_reversed_edge_range(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let inbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let offset = try!(get_offset_param(l, 3));
        let limit = try!(get_limit_param(l, 4));
        let result = try!(trans.get_reversed_edge_range(inbound_id, t, offset, limit));
        serialize_edges(l, result);
        Ok(1)
    }

    pub unsafe fn get_reversed_edge_time_range(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let inbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let high = try!(get_optional_datetime_param(l, 3));
        let low = try!(get_optional_datetime_param(l, 4));
        let limit = try!(get_limit_param(l, 5));
        let result = try!(trans.get_reversed_edge_time_range(inbound_id, t, high, low, limit));
        serialize_edges(l, result);
        Ok(1)
    }

    pub unsafe fn get_global_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = try!(get_string_param(l, 1));
        let result = try!(trans.get_global_metadata(key.clone()));
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_global_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = try!(get_string_param(l, 1));
        let value = try!(deserialize_json(l, 2));
        try!(trans.set_global_metadata(key, value));
        Ok(0)
    }

    pub unsafe fn delete_global_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = try!(get_string_param(l, 1));
        try!(trans.delete_global_metadata(key));
        Ok(0)
    }

    pub unsafe fn get_account_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_uuid_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let result = try!(trans.get_account_metadata(owner_id, key));
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_account_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_uuid_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let value = try!(deserialize_json(l, 3));
        try!(trans.set_account_metadata(owner_id, key, value));
        Ok(0)
    }

    pub unsafe fn delete_account_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_uuid_param(l, 1));
        let key = try!(get_string_param(l, 2));
        try!(trans.delete_account_metadata(owner_id, key));
        Ok(0)
    }

    pub unsafe fn get_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_uuid_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let result = try!(trans.get_vertex_metadata(owner_id, key));
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_uuid_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let value = try!(deserialize_json(l, 3));
        try!(trans.set_vertex_metadata(owner_id, key, value));
        Ok(0)
    }

    pub unsafe fn delete_vertex_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_uuid_param(l, 1));
        let key = try!(get_string_param(l, 2));
        try!(trans.delete_vertex_metadata(owner_id, key));
        Ok(0)
    }

    pub unsafe fn get_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let inbound_id = try!(get_uuid_param(l, 3));
        let key = try!(get_string_param(l, 4));
        let result = try!(trans.get_edge_metadata(outbound_id, t, inbound_id, key));
        serialize_json(l, result);
        Ok(1)
    }

    pub unsafe fn set_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let inbound_id = try!(get_uuid_param(l, 3));
        let key = try!(get_string_param(l, 4));
        let value = try!(deserialize_json(l, 5));
        try!(trans.set_edge_metadata(outbound_id, t, inbound_id, key, value));
        Ok(0)
    }

    pub unsafe fn delete_edge_metadata(trans: &mut ProxyTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_uuid_param(l, 1));
        let t = try!(get_type_param(l, 2));
        let inbound_id = try!(get_uuid_param(l, 3));
        let key = try!(get_string_param(l, 4));
        try!(trans.delete_edge_metadata(outbound_id, t, inbound_id, key));
        Ok(0)
    }
}
