#![allow(unreachable_code, unused_variables)]

// Above warnings are ignored because the lua_fn! macro generates too much noise

use lua;
use libc;
use serde_json;
use serde_json::value::Value as JsonValue;
use std::collections::BTreeMap;
use nutrino::{Vertex, Edge, Transaction, PostgresTransaction, Error};
use chrono::naive::datetime::NaiveDateTime;
use std::{isize, i32};

#[derive(Debug)]
pub enum LuaError {
    Arg(i32, String),
    Generic(String)
}

impl LuaError {
    unsafe fn serialize(&self, l: &mut lua::ExternState) {
        match *self {
            LuaError::Arg(idx, ref msg) => l.argerror(idx, &msg[..]),
            LuaError::Generic(ref msg) => l.errorstr(&msg[..])
        }
    }
}

impl From<Error> for LuaError {
    fn from(err: Error) -> LuaError {
		LuaError::Generic(format!("{:?}", err))
	}
}

#[derive(Debug)]
pub enum ScriptError {
    Syntax(String),
    Memory,
    Runtime(String),
    Panicked(String)
}

impl ScriptError {
    fn new_from_loaderror(state: &mut lua::State, err: lua::LoadError) -> ScriptError {
        match err {
            lua::LoadError::ErrSyntax => ScriptError::Syntax(String::from(state.checkstring(-1).unwrap())),
            lua::LoadError::ErrMem => ScriptError::Memory
        }
    }

    fn new_from_pcallerror(state: &mut lua::State, err: lua::PCallError) -> ScriptError {
        match err {
            lua::PCallError::ErrRun => ScriptError::Runtime(String::from(state.checkstring(-1).unwrap())),
            lua::PCallError::ErrMem => ScriptError::Memory,
            lua::PCallError::ErrErr => ScriptError::Panicked("Unknown pcall error".to_string())
        }
    }
}

macro_rules! lua_fn {
    ($(unsafe fn $name:ident($targ:ident: &mut PostgresTransaction, $larg:ident: &mut $typ:ty) -> Result<i32, LuaError> $code:block)+) => (
        $(
            unsafe extern "C" fn $name($larg: *mut ::lua::raw::lua_State) -> ::libc::c_int {
                let mut $larg = &mut ::lua::ExternState::from_lua_State($larg);

                $larg.getglobal("trans");

                if !$larg.islightuserdata(-1) {
                    $larg.errorstr("Corrupted transaction");
                    return 1;
                }

                let trans_ptr = $larg.touserdata(-1);
                let $targ = &mut *(trans_ptr as *mut PostgresTransaction);

                return match inner($targ, &mut $larg) {
                    Ok(i) => i,
                    Err(err) => {
                        err.serialize($larg);
                        1
                    }
                } as ::libc::c_int;

                unsafe fn inner($targ: &mut PostgresTransaction, $larg: &mut $typ) -> Result<i32, LuaError> $code
            }
        )+
    )
}

pub fn run(mut trans: PostgresTransaction, account_id: i64, source: &str, arg: JsonValue) -> Result<JsonValue, ScriptError> {
    let mut l = lua::State::new();
    l.openlibs();

    l.register("get_vertex", get_vertex);
    l.register("create_vertex", create_vertex);
    l.register("set_vertex", set_vertex);
    l.register("delete_vertex", delete_vertex);
    l.register("get_edge", get_edge);
    l.register("set_edge", set_edge);
    l.register("delete_edge", delete_edge);
    l.register("get_edge_count", get_edge_count);
    l.register("get_edge_range", get_edge_range);
    l.register("get_edge_time_range", get_edge_time_range);

    l.register("get_global_metadata", get_global_metadata);
    l.register("set_global_metadata", set_global_metadata);
    l.register("delete_global_metadata", delete_global_metadata);
    l.register("get_account_metadata", get_account_metadata);
    l.register("set_account_metadata", set_account_metadata);
    l.register("delete_account_metadata", delete_account_metadata);
    l.register("get_vertex_metadata", get_vertex_metadata);
    l.register("set_vertex_metadata", set_vertex_metadata);
    l.register("delete_vertex_metadata", delete_vertex_metadata);
    l.register("get_edge_metadata", get_edge_metadata);
    l.register("set_edge_metadata", set_edge_metadata);
    l.register("delete_edge_metadata", delete_edge_metadata);

    if let Err(err) = l.loadstring(source) {
        return Err(ScriptError::new_from_loaderror(&mut l, err));
    }

    let trans_ptr: *mut libc::c_void = &mut trans as *mut _ as *mut libc::c_void;

    l.pushlightuserdata(trans_ptr);
    l.setglobal("trans");

    l.pushstring(&account_id.to_string()[..]);
    l.setglobal("account_id");

    unsafe {
        serialize_json(l.as_extern(), arg);
    }

    l.setglobal("arg");

    if let Err(err) = l.pcall(0, lua::MULTRET, 0) {
        return Err(ScriptError::new_from_pcallerror(&mut l, err));
    }

    if let Err(err) = trans.commit() {
        return Err(ScriptError::Runtime(format!("Could not commit script transaction: {}", err)))
    }

    if l.gettop() == 0 {
        Ok(JsonValue::Null)
    } else {
        unsafe {
            deserialize_json(l.as_extern(), -1)
        }
    }
}

unsafe fn deserialize_json(l: &mut lua::ExternState, offset: i32) -> Result<JsonValue, ScriptError> {
    Ok(match l.type_(-1) {
        Some(lua::Type::Nil) | None => JsonValue::Null,
        Some(lua::Type::Boolean) => JsonValue::Bool(l.toboolean(-1)),
        Some(lua::Type::Number) => JsonValue::F64(l.tonumber(-1)),
        Some(lua::Type::String) => JsonValue::String(l.checkstring(-1).unwrap().to_string().clone()),
        Some(lua::Type::Table) => {
            l.pushnil();
            let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();

            while l.next(offset - 1) {
                // Keys could be strings or numbers, depending on whether it's a map-shaped table
                // or an array-shaped table. We can't rely on `l.tostring` because we're in the
                // middle of a next() loop.
                let k = match l.type_(-2) {
                    Some(lua::Type::String) => {
                        l.checkstring(-2).unwrap().to_string().clone()
                    },
                    Some(lua::Type::Number) => {
                        l.checknumber(-2).to_string()
                    },
                    k_type => {
                        panic!("Unknown key type: {:?}", k_type);
                    }
                };

                let v: JsonValue = try!(deserialize_json(l, -1));
                d.insert(k, v);
                l.pop(1);
            }

            JsonValue::Object(d)
        },
        _ => {
            return Err(ScriptError::Runtime("Could not deserialize return value".to_string()))
        }
    })
}

unsafe fn serialize_json(l: &mut lua::ExternState, json: JsonValue) {
    match json {
        JsonValue::Null => l.pushnil(),
        JsonValue::Bool(v) => l.pushboolean(v),
        JsonValue::I64(v) => l.pushstring(&v.to_string()[..]),
        JsonValue::U64(v) => l.pushstring(&v.to_string()[..]),
        JsonValue::F64(v) => l.pushnumber(v),
        JsonValue::String(v) => l.pushstring(&v[..]),
        JsonValue::Array(v) => {
            l.newtable();

            for (i, iv) in v.iter().enumerate() {
                l.pushinteger((i + 1) as isize);
                serialize_json(l, iv.clone());
                l.settable(-3);
            };

            l.settable(-3);
        },
        JsonValue::Object(v) => {
            l.newtable();

            for (k, iv) in &v {
                serialize_json(l, iv.clone());
                l.setfield(-2, k);
            }

            l.settable(-3);
        }
    }
}

unsafe fn serialize_edges(l: &mut lua::ExternState, edges: Vec<Edge<i64>>) {
    l.newtable();

    for (i, edge) in edges.iter().enumerate() {
        l.pushinteger((i + 1) as isize);
        serialize_edge(l, &edge);
        l.settable(-3);
    }
}

unsafe fn serialize_edge(l: &mut lua::ExternState, edge: &Edge<i64>) {
    l.newtable();
    add_string_field_to_table(l, "outbound_id", &edge.outbound_id.to_string()[..]);
    add_string_field_to_table(l, "type", &edge.t[..]);
    add_string_field_to_table(l, "inbound_id", &edge.inbound_id.to_string()[..]);
    add_number_field_to_table(l, "weight", edge.weight as f64);
}

unsafe fn add_string_field_to_table(l: &mut lua::ExternState, k: &str, v: &str) {
    l.pushstring(v);
    l.setfield(-2, k);
}

unsafe fn add_number_field_to_table(l: &mut lua::ExternState, k: &str, v: f64) {
    l.pushnumber(v);
    l.setfield(-2, k);
}

unsafe fn get_json_param(l: &mut lua::ExternState, narg: i32) -> Result<JsonValue, LuaError> {
    let s = match l.checkstring(narg) {
        Some(s) => &s[..],
        None => {
            return Err(LuaError::Arg(narg, "Expected JSON value as string".to_string()))
        }
    };

    match serde_json::from_str(s) {
        Ok(val) => Ok(val),
        _ => Err(LuaError::Arg(narg, "Expected JSON value as string".to_string()))
    }
}

unsafe fn get_string_param(l: &mut lua::ExternState, narg: i32) -> Result<String, LuaError> {
    match l.checkstring(narg) {
        Some(s) => Ok(s.to_string()),
        None => Err(LuaError::Arg(narg, "Expected string".to_string()))
    }
}

unsafe fn get_i64_param(l: &mut lua::ExternState, narg: i32) -> Result<i64, LuaError> {
    let s = try!(get_string_param(l, narg));

    match i64::from_str_radix(&s[..], 10) {
        Ok(i) => Ok(i),
        Err(_) => Err(LuaError::Generic("Expected i64 as string".to_string()))
    }
}

unsafe fn get_optional_i64_param(l: &mut lua::ExternState, narg: i32) -> Result<Option<i64>, LuaError> {
    let s = try!(get_string_param(l, narg));

    if s == "" {
        return Ok(None);
    }

    match i64::from_str_radix(&s[..], 10) {
        Ok(i) => Ok(Some(i)),
        Err(_) => Err(LuaError::Generic("Expected i64 as string".to_string()))
    }
}

unsafe fn get_optional_datetime_param(l: &mut lua::ExternState, narg: i32) -> Result<Option<NaiveDateTime>, LuaError> {
    match try!(get_optional_i64_param(l, narg)) {
        Some(i) => Ok(Some(NaiveDateTime::from_timestamp(i, 0))),
        None => Ok(None)
    }
}

lua_fn! {
    unsafe fn get_vertex(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_i64_param(l, 1));
        let result = try!(trans.get_vertex(id));
        l.newtable();
        add_string_field_to_table(l, "id", &result.id.to_string()[..]);
        add_string_field_to_table(l, "type", &result.t[..]);
        Ok(1)
    }

    unsafe fn create_vertex(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let t = try!(get_string_param(l, 1));
        let result = try!(trans.create_vertex(t));
        l.pushstring(&result.to_string()[..]);
        Ok(1)
    }

    unsafe fn set_vertex(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let v = Vertex::new(id, t);
        try!(trans.set_vertex(v));
        Ok(0)
    }

    unsafe fn delete_vertex(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_i64_param(l, 1));
        try!(trans.delete_vertex(id));
        Ok(0)
    }

    unsafe fn get_edge(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_i64_param(l, 3));
        let result = try!(trans.get_edge(outbound_id, t, inbound_id));
        serialize_edge(l, &result);
        Ok(1)
    }

    unsafe fn set_edge(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_i64_param(l, 3));
        let weight = l.checknumber(4);
        let e = Edge::new(outbound_id, t, inbound_id, weight as f32);
        try!(trans.set_edge(e));
        Ok(1)
    }

    unsafe fn delete_edge(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_i64_param(l, 3));
        try!(trans.delete_edge(outbound_id, t, inbound_id));
        Ok(0)
    }

    unsafe fn get_edge_count(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let result = try!(trans.get_edge_count(outbound_id, t));

        l.pushinteger(match result {
            i if i > isize::MAX as i64 => isize::MAX,
            i => i as isize
        });

        Ok(1)
    }

    unsafe fn get_edge_range(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let offset = l.checkinteger(3);

        let limit = match l.checkinteger(4) {
            i if i > i32::MAX as isize => i32::MAX,
            i => i as i32
        };
        
        let result = try!(trans.get_edge_range(outbound_id, t, offset as i64, limit));
        serialize_edges(l, result);
        Ok(1)
    }

    unsafe fn get_edge_time_range(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let high = try!(get_optional_datetime_param(l, 3));
        let low = try!(get_optional_datetime_param(l, 4));
        
        let limit = match l.checkinteger(5) {
            i if i > i32::MAX as isize => i32::MAX,
            i => i as i32
        };

        let result = try!(trans.get_edge_time_range(outbound_id, t, high, low, limit));
        serialize_edges(l, result);
        Ok(1)
    }

    unsafe fn get_global_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = try!(get_string_param(l, 2));
        let result = try!(trans.get_global_metadata(key));
        serialize_json(l, result);
        Ok(1)
    }

    unsafe fn set_global_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = try!(get_string_param(l, 2));
        let value = try!(get_json_param(l, 3));
        try!(trans.set_global_metadata(key, value));
        Ok(0)
    }

    unsafe fn delete_global_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let key = try!(get_string_param(l, 2));
        try!(trans.delete_global_metadata(key));
        Ok(0)
    }

    unsafe fn get_account_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_i64_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let result = try!(trans.get_account_metadata(owner_id, key));
        serialize_json(l, result);
        Ok(1)
    }

    unsafe fn set_account_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_i64_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let value = try!(get_json_param(l, 3));
        try!(trans.set_account_metadata(owner_id, key, value));
        Ok(0)
    }

    unsafe fn delete_account_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_i64_param(l, 1));
        let key = try!(get_string_param(l, 2));
        try!(trans.delete_account_metadata(owner_id, key));
        Ok(0)
    }

    unsafe fn get_vertex_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_i64_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let result = try!(trans.get_vertex_metadata(owner_id, key));
        serialize_json(l, result);
        Ok(1)
    }

    unsafe fn set_vertex_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_i64_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let value = try!(get_json_param(l, 3));
        try!(trans.set_vertex_metadata(owner_id, key, value));
        Ok(0)
    }

    unsafe fn delete_vertex_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_i64_param(l, 1));
        let key = try!(get_string_param(l, 2));
        try!(trans.delete_vertex_metadata(owner_id, key));
        Ok(0)
    }

    unsafe fn get_edge_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_i64_param(l, 3));
        let key = try!(get_string_param(l, 4));
        let result = try!(trans.get_edge_metadata(outbound_id, t, inbound_id, key));
        serialize_json(l, result);
        Ok(1)
    }

    unsafe fn set_edge_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_i64_param(l, 3));
        let key = try!(get_string_param(l, 4));
        let value = try!(get_json_param(l, 5));
        try!(trans.set_edge_metadata(outbound_id, t, inbound_id, key, value));
        Ok(0)
    }

    unsafe fn delete_edge_metadata(trans: &mut PostgresTransaction, l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_i64_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_i64_param(l, 3));
        let key = try!(get_string_param(l, 4));
        try!(trans.delete_edge_metadata(outbound_id, t, inbound_id, key));
        Ok(0)
    }
}
