#![allow(unreachable_code, unused_variables)]

// Above warnings are ignored because the lua_fn! macro generates too much noise

use lua;
use std::panic;
use serde_json;
use serde_json::value::Value as JsonValue;
use std::collections::BTreeMap;
use nutrino::{Vertex, Edge, Transaction, PostgresTransaction};
use nutrino::Request as DatastoreRequest;
use nutrino::Response as DatastoreResponse;
use chrono::naive::datetime::NaiveDateTime;
use nutrino::ErrorResponse as DatastoreErrorResponse;
use std::cell::RefCell;

thread_local!(static TRANSACTION: RefCell<Option<PostgresTransaction>> = RefCell::new(None));

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
    ($(unsafe fn $name:ident($arg:ident: &mut $typ:ty) -> Result<i32, LuaError> $code:block)+) => (
        $(
            unsafe extern "C" fn $name($arg: *mut ::lua::raw::lua_State) -> ::libc::c_int {
                let mut $arg = &mut ::lua::ExternState::from_lua_State($arg);

                return match inner(&mut $arg) {
                    Ok(i) => i,
                    Err(err) => {
                        err.serialize($arg);
                        1
                    }
                } as ::libc::c_int;

                unsafe fn inner($arg: &mut $typ) -> Result<i32, LuaError> $code
            }
        )+
    )
}

pub fn run(transaction: PostgresTransaction, user_id: i64, source: &str, arg: JsonValue) -> Result<JsonValue, ScriptError> {
    TRANSACTION.with(|t| {
        *t.borrow_mut() = Some(transaction);
    });

    let result = panic::catch_unwind(|| {
		run_in_thread(user_id, source, arg)
	});

	match result {
		Ok(inner) => inner,
		Err(err) => Err(ScriptError::Panicked(format!("{:?}", err)))
	}
}

fn run_in_thread(user_id: i64, source: &str, arg: JsonValue) -> Result<JsonValue, ScriptError> {
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
    l.register("transaction", transaction);
    l.register("get_metadata", get_metadata);
    l.register("set_metadata", set_metadata);
    l.register("delete_metadata", delete_metadata);

    match l.loadstring(source) {
        Err(err) => return Err(ScriptError::new_from_loaderror(&mut l, err)),
        _ => ()
    };

    unsafe {
        serialize_json(&mut l, arg);
    }

    l.setglobal("arg");

    match l.pcall(0, lua::MULTRET, 0) {
        Err(err) => return Err(ScriptError::new_from_pcallerror(&mut l, err)),
        _ => ()
    };

    if l.gettop() == 0 {
        Ok(JsonValue::Null)
    } else {
        deserialize_json(&mut l, -1)
    }
}

fn deserialize_json(l: &mut lua::State, offset: i32) -> Result<JsonValue, ScriptError> {
    Ok(match l.type_(-1) {
        Some(lua::Type::Nil) => JsonValue::Null,
        None => JsonValue::Null,
        Some(lua::Type::Boolean) => JsonValue::Bool(l.toboolean(-1)),
        Some(lua::Type::Number) => JsonValue::F64(l.tonumber(-1)),
        Some(lua::Type::String) => JsonValue::String(l.tostring(-1).unwrap().to_string().clone()),
        Some(lua::Type::Table) => {
            l.pushnil();
            let mut d: BTreeMap<String, JsonValue> = BTreeMap::new();

            while l.next(offset - 1) {
                let k = l.checkstring(-2).unwrap().to_string().clone();
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

unsafe fn serialize_json(l: &mut lua::State, json: JsonValue) {
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

            for (k, iv) in v.iter() {
                serialize_json(l, iv.clone());
                l.setfield(-2, k);
            }

            l.settable(-3);
        }
    }
}

unsafe fn init_request_table(l: &mut lua::ExternState, t: &str) {
    l.newtable();
    add_string_field_to_table(l, "_type", t);
}

unsafe fn add_string_field_to_table(l: &mut lua::ExternState, k: &str, v: &str) {
    l.pushstring(v);
    l.setfield(-2, k);
}

unsafe fn add_json_field_to_table(l: &mut lua::ExternState, k: &str, v: JsonValue) {
    let s = serde_json::to_string(&v).unwrap();
    l.pushstring(&s[..]);
    l.setfield(-2, k);
}

unsafe fn add_int_field_to_table(l: &mut lua::ExternState, k: &str, v: isize) {
    l.pushinteger(v);
    l.setfield(-2, k);
}

unsafe fn add_number_field_to_table(l: &mut lua::ExternState, k: &str, v: f64) {
    l.pushnumber(v);
    l.setfield(-2, k);
}

unsafe fn get_string_field_from_table(l: &mut lua::ExternState, narg: i32, name: &str) -> String {
    l.getfield(narg, name);
    let t = l.checkstring(-1);
    l.pop(1);

    match t {
        Some(s) => String::from(s),
        None => String::from("")
    }
}

unsafe fn get_number_field_from_table(l: &mut lua::ExternState, narg: i32, name: &str) -> f64 {
    l.getfield(narg, name);
    let f = l.checknumber(-1);
    l.pop(1);
    f
}

unsafe fn get_i64_field_from_table(l: &mut lua::ExternState, narg: i32, name: &str) -> Result<i64, LuaError> {
    match i64::from_str_radix(&get_string_field_from_table(l, narg, name)[..], 10) {
        Ok(i) => Ok(i),
        Err(_) => Err(LuaError::Generic("Expected i64 as string".to_string()))
    }
}

unsafe fn get_optional_i64_field_from_table(l: &mut lua::ExternState, narg: i32, name: &str) -> Result<Option<i64>, LuaError> {
    let s = get_string_field_from_table(l, narg, name);

    if s == "" {
        Ok(None)
    } else {
        match i64::from_str_radix(&s[..], 10) {
            Ok(i) => Ok(Some(i)),
            Err(_) => Err(LuaError::Generic("Expected i64 as string".to_string()))
        }
    }
}

unsafe fn get_datetime_field_from_table(l: &mut lua::ExternState, narg: i32, name: &str) -> Result<Option<NaiveDateTime>, LuaError> {
    let s = get_string_field_from_table(l, narg, name);

    if s == "" {
        Ok(None)
    } else {
        match i64::from_str_radix(&s[..], 10) {
            Ok(i) => Ok(Some(NaiveDateTime::from_timestamp(i, 0))),
            Err(_) => Err(LuaError::Generic("Expected timestamp as string".to_string()))
        }
    }
}

unsafe fn get_json_field_from_table(l: &mut lua::ExternState, narg: i32, name: &str) -> Result<JsonValue, LuaError> {
    match serde_json::from_str(&get_string_field_from_table(l, narg, name)[..]) {
        Ok(val) => Ok(val),
        _ => Err(LuaError::Generic("Expected JSON value as string".to_string()))
    }
}

unsafe fn get_obj_field_from_table(l: &mut lua::ExternState, narg: i32, name: &str) -> Result<BTreeMap<String, JsonValue>, LuaError> {
    match serde_json::from_str(&get_string_field_from_table(l, narg, name)[..]) {
        Ok(JsonValue::Object(obj)) => Ok(obj),
        _ => Err(LuaError::Generic("Expected JSON object as string".to_string()))
    }
}

unsafe fn get_obj_param(l: &mut lua::ExternState, narg: i32) -> Result<JsonValue, LuaError> {
    let s = match l.checkstring(narg) {
        Some(s) => &s[..],
        None => {
            return Err(LuaError::Arg(narg, "Expected JSON object as string".to_string()))
        }
    };

    let json = serde_json::from_str(s);

    match json {
        Ok(JsonValue::Object(_)) => Ok(json.unwrap()),
        _ => Err(LuaError::Arg(narg, "Expected JSON object as string".to_string()))
    }
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

lua_fn! {
    unsafe fn get_vertex(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_string_param(l, 1));
        init_request_table(l, "get_vertex");
        add_string_field_to_table(l, "id", &id[..]);
        Ok(1)
    }

    unsafe fn create_vertex(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let t = try!(get_string_param(l, 1));
        let properties = try!(get_obj_param(l, 2));
        init_request_table(l, "create_vertex");
        add_string_field_to_table(l, "type", &t[..]);
        add_json_field_to_table(l, "properties", properties);
        Ok(1)
    }

    unsafe fn set_vertex(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_string_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let properties = try!(get_obj_param(l, 3));
        init_request_table(l, "set_vertex");
        add_string_field_to_table(l, "id", &id[..]);
        add_string_field_to_table(l, "type", &t[..]);
        add_json_field_to_table(l, "properties", properties);
        Ok(1)
    }

    unsafe fn delete_vertex(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let id = try!(get_string_param(l, 1));
        init_request_table(l, "delete_vertex");
        add_string_field_to_table(l, "id", &id[..]);
        Ok(1)
    }

    unsafe fn get_edge(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_string_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_string_param(l, 3));
        init_request_table(l, "get_edge");
        add_string_field_to_table(l, "outbound_id", &outbound_id[..]);
        add_string_field_to_table(l, "type", &t[..]);
        add_string_field_to_table(l, "inbound_id", &inbound_id[..]);
        Ok(1)
    }

    unsafe fn set_edge(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_string_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_string_param(l, 3));
        let weight = l.checknumber(4);
        let properties = try!(get_obj_param(l, 5));
        init_request_table(l, "set_edge");
        add_string_field_to_table(l, "outbound_id", &outbound_id[..]);
        add_string_field_to_table(l, "type", &t[..]);
        add_string_field_to_table(l, "inbound_id", &inbound_id[..]);
        add_number_field_to_table(l, "weight", weight);
        add_json_field_to_table(l, "properties", properties);
        Ok(1)
    }

    unsafe fn delete_edge(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_string_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let inbound_id = try!(get_string_param(l, 3));
        init_request_table(l, "delete_edge");
        add_string_field_to_table(l, "outbound_id", &outbound_id[..]);
        add_string_field_to_table(l, "type", &t[..]);
        add_string_field_to_table(l, "inbound_id", &inbound_id[..]);
        Ok(1)
    }

    unsafe fn get_edge_count(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_string_param(l, 1));
        let t = try!(get_string_param(l, 2));
        init_request_table(l, "get_edge_count");
        add_string_field_to_table(l, "outbound_id", &outbound_id[..]);
        add_string_field_to_table(l, "type", &t[..]);
        Ok(1)
    }

    unsafe fn get_edge_range(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_string_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let offset = try!(get_string_param(l, 3));
        let limit = try!(get_string_param(l, 4));
        init_request_table(l, "get_edge_range");
        add_string_field_to_table(l, "outbound_id", &outbound_id[..]);
        add_string_field_to_table(l, "type", &t[..]);
        add_string_field_to_table(l, "offset", &offset[..]);
        add_string_field_to_table(l, "limit", &limit[..]);
        Ok(1)
    }

    unsafe fn get_edge_time_range(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let outbound_id = try!(get_string_param(l, 1));
        let t = try!(get_string_param(l, 2));
        let high = try!(get_string_param(l, 3));
        let low = try!(get_string_param(l, 4));
        let limit = l.checkinteger(5);
        init_request_table(l, "get_edge");
        add_string_field_to_table(l, "outbound_id", &outbound_id[..]);
        add_string_field_to_table(l, "type", &t[..]);
        add_string_field_to_table(l, "high", &high[..]);
        add_string_field_to_table(l, "low", &low[..]);
        add_int_field_to_table(l, "limit", limit);
        Ok(1)
    }

    unsafe fn get_metadata(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_string_param(l, 1));
        let key = try!(get_string_param(l, 2));
        init_request_table(l, "get_metadata");
        add_string_field_to_table(l, "owner_id", &owner_id[..]);
        add_string_field_to_table(l, "key", &key[..]);
        Ok(1)
    }

    unsafe fn set_metadata(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_string_param(l, 1));
        let key = try!(get_string_param(l, 2));
        let value = try!(get_json_param(l, 3));
        init_request_table(l, "set_metadata");
        add_string_field_to_table(l, "owner_id", &owner_id[..]);
        add_string_field_to_table(l, "key", &key[..]);
        add_json_field_to_table(l, "value", value);
        Ok(1)
    }

    unsafe fn delete_metadata(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        let owner_id = try!(get_string_param(l, 1));
        let key = try!(get_string_param(l, 2));
        init_request_table(l, "delete_metadata");
        add_string_field_to_table(l, "owner_id", &owner_id[..]);
        add_string_field_to_table(l, "key", &key[..]);
        Ok(1)
    }

    unsafe fn transaction(l: &mut lua::ExternState) -> Result<i32, LuaError> {
        TRANSACTION.with(|trans_ref| {
            let mut trans = (*trans_ref.borrow()).clone().unwrap();

            l.checktype(1, lua::Type::Table);

            // Iterate through each item in the table
            for i in 1..1002 {
                if i == 1001 {
                    // Only allow a maximum of 1000 items
                    l.pop(1);
                    return Err(LuaError::Generic("Too many items in this transaction".to_string()));
                }

                l.rawgeti(1, i);

                if l.isnil(-1) {
                    // Hit the end of the table - break out
                    l.pop(1);
                    break;
                }

                // Check that the current value in the table is also a table
                l.checktype(-1, lua::Type::Table);

                trans.request(try!(deserialize_request(l, i)));

                // TODO: Currently we won't pop if deserialize_request returns an Err. Is this OK?
                l.pop(1);
            }

            match trans.commit() {
                Ok(res) => {
                    l.newtable();

                    for (i, item) in res.iter().enumerate() {
                        l.pushinteger((i + 1) as isize);

                        match *item {
                            Ok(ref i) => serialize_ok_res(l, i.clone()),
                            Err(ref e) => serialize_err_res(l, e.clone())
                        };

                        l.rawset(-3);
                    };

                    Ok(1)
                },
                Err(err) => Err(LuaError::Generic(format!("Transaction failed: {:?}", err)))
            }
        })
    }
}

unsafe fn serialize_ok_res(l: &mut lua::ExternState, res: DatastoreResponse) {
    match res {
        DatastoreResponse::VertexId(id) => {
            init_request_table(l, "vertex_id");
            add_string_field_to_table(l, "id", &id.to_string()[..]);
        },
        DatastoreResponse::Vertex(vertex) => {
            init_request_table(l, "vertex");
            add_string_field_to_table(l, "id", &vertex.id.to_string()[..]);
            add_string_field_to_table(l, "type", &vertex.t[..]);
            add_string_field_to_table(l, "properties", &serde_json::to_string(&vertex.properties).unwrap()[..]);
        },
        DatastoreResponse::Edge(edge) => {
            init_request_table(l, "edge");
            add_string_field_to_table(l, "outbound_id", &edge.outbound_id.to_string()[..]);
            add_string_field_to_table(l, "type", &edge.t[..]);
            add_string_field_to_table(l, "inbound_id", &edge.inbound_id.to_string()[..]);
            add_number_field_to_table(l, "weight", edge.weight as f64);
            add_string_field_to_table(l, "properties", &serde_json::to_string(&edge.properties).unwrap()[..]);
        },
        DatastoreResponse::Count(i) => {
            init_request_table(l, "count");
            add_int_field_to_table(l, "count", i as isize);
        },
        DatastoreResponse::Edges(edges) => {
            init_request_table(l, "edges");
            l.pushstring("edges");
            l.newtable();

            for (i, edge) in edges.iter().enumerate() {
                l.pushinteger((i + 1) as isize);
                l.newtable();
                add_string_field_to_table(l, "outbound_id", &edge.outbound_id.to_string()[..]);
                add_string_field_to_table(l, "type", &edge.t[..]);
                add_string_field_to_table(l, "inbound_id", &edge.inbound_id.to_string()[..]);
                add_number_field_to_table(l, "weight", edge.weight as f64);
                add_string_field_to_table(l, "properties", &serde_json::to_string(&edge.properties).unwrap()[..]);
                l.settable(-3);
            };

            l.settable(-3);
        },
        DatastoreResponse::Ok => {
            init_request_table(l, "ok");
        },
        DatastoreResponse::Metadata(value) => {
            init_request_table(l, "metadata");
            add_string_field_to_table(l, "value", &serde_json::to_string(&value).unwrap()[..]);
        }
    }
}

unsafe fn serialize_err_res(l: &mut lua::ExternState, res: DatastoreErrorResponse) {
	match res {
        DatastoreErrorResponse::Unexpected(err) => {
            init_request_table(l, "unexpected");
            add_string_field_to_table(l, "message", &err[..]);
        },
    	DatastoreErrorResponse::VertexDoesNotExist(id) => {
            init_request_table(l, "vertex_does_not_exist");
            add_string_field_to_table(l, "id", &id.to_string()[..]);
        },
    	DatastoreErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id) => {
            init_request_table(l, "edge_does_not_exist");
            add_string_field_to_table(l, "outbound_id", &outbound_id.to_string()[..]);
            add_string_field_to_table(l, "type", &t[..]);
            add_string_field_to_table(l, "inbound_id", &inbound_id.to_string()[..]);
        },
    	DatastoreErrorResponse::WeightOutOfRange => {
            init_request_table(l, "weight_out_of_range");
        },
    	DatastoreErrorResponse::OffsetOutOfRange => {
            init_request_table(l, "offset_out_of_range");
        },
    	DatastoreErrorResponse::LimitOutOfRange => {
            init_request_table(l, "limit_out_of_range");
        },
        DatastoreErrorResponse::MetadataDoesNotExist(owner_id, key) => {
            init_request_table(l, "metadata_does_not_exist");

            let owner_id_str = match owner_id {
                Some(owner_id) => owner_id.to_string(),
                None => "".to_string()
            };

            add_string_field_to_table(l, "owner_id", &owner_id_str[..]);
        }
    };
}

unsafe fn deserialize_request(l: &mut lua::ExternState, i: i32) -> Result<DatastoreRequest, LuaError> {
    match &get_string_field_from_table(l, -1, "_type")[..] {
        "get_vertex" => {
            let id = try!(get_i64_field_from_table(l, -1, "id"));
            Ok(DatastoreRequest::GetVertex(id))
        },
        "create_vertex" => {
            let t = get_string_field_from_table(l, -1, "type");
            let properties = try!(get_obj_field_from_table(l, -1, "properties"));
            Ok(DatastoreRequest::CreateVertex(t, properties))
        },
        "set_vertex" => {
            let id = try!(get_i64_field_from_table(l, -1, "id"));
            let t = get_string_field_from_table(l, -1, "type");
            let properties = try!(get_obj_field_from_table(l, -1, "properties"));
            Ok(DatastoreRequest::SetVertex(Vertex::new_with_properties(id, t, properties)))
        },
        "delete_vertex" => {
            let id = try!(get_i64_field_from_table(l, -1, "id"));
            Ok(DatastoreRequest::DeleteVertex(id))
        },
        "get_edge" => {
            let outbound_id = try!(get_i64_field_from_table(l, -1, "outbound_id"));
            let t = get_string_field_from_table(l, -1, "type");
            let inbound_id = try!(get_i64_field_from_table(l, -1, "inbound_id"));
            Ok(DatastoreRequest::GetEdge(outbound_id, t, inbound_id))
        },
        "set_edge" => {
            let outbound_id = try!(get_i64_field_from_table(l, -1, "outbound_id"));
            let t = get_string_field_from_table(l, -1, "type");
            let inbound_id = try!(get_i64_field_from_table(l, -1, "inbound_id"));
            let weight = get_number_field_from_table(l, -1, "weight");
            let properties = try!(get_obj_field_from_table(l, -1, "properties"));
            Ok(DatastoreRequest::SetEdge(Edge::new_with_properties(outbound_id, t, inbound_id, weight as f32, properties)))
        },
        "delete_edge" => {
            let outbound_id = try!(get_i64_field_from_table(l, -1, "outbound_id"));
            let t = get_string_field_from_table(l, -1, "type");
            let inbound_id = try!(get_i64_field_from_table(l, -1, "inbound_id"));
            Ok(DatastoreRequest::DeleteEdge(outbound_id, t, inbound_id))
        },
        "get_edge_count" => {
            let outbound_id = try!(get_i64_field_from_table(l, -1, "outbound_id"));
            let t = get_string_field_from_table(l, -1, "type");
            Ok(DatastoreRequest::GetEdgeCount(outbound_id, t))
        },
        "get_edge_range" => {
            let outbound_id = try!(get_i64_field_from_table(l, -1, "outbound_id"));
            let t = get_string_field_from_table(l, -1, "type");
            let offset = try!(get_i64_field_from_table(l, -1, "offset"));
            let limit = try!(get_i64_field_from_table(l, -1, "limit"));
            Ok(DatastoreRequest::GetEdgeRange(outbound_id, t, offset, limit))
        },
        "get_edge_time_range" => {
            let outbound_id = try!(get_i64_field_from_table(l, -1, "outbound_id"));
            let t = get_string_field_from_table(l, -1, "type");
            let high = try!(get_datetime_field_from_table(l, -1, "high"));
            let low = try!(get_datetime_field_from_table(l, -1, "low"));
            let limit = try!(get_i64_field_from_table(l, -1, "limit"));
            Ok(DatastoreRequest::GetEdgeTimeRange(outbound_id, t, high, low, limit))
        },
        "get_metadata" => {
            let owner_id = try!(get_optional_i64_field_from_table(l, -1, "owner_id"));
            let key = get_string_field_from_table(l, -1, "key");
            Ok(DatastoreRequest::GetMetadata(owner_id, key))
        },
        "set_metadata" => {
            let owner_id = try!(get_optional_i64_field_from_table(l, -1, "owner_id"));
            let key = get_string_field_from_table(l, -1, "key");
            let value = try!(get_json_field_from_table(l, -1, "value"));
            Ok(DatastoreRequest::SetMetadata(owner_id, key, value))
        },
        "delete_metadata" => {
            let owner_id = try!(get_optional_i64_field_from_table(l, -1, "owner_id"));
            let key = get_string_field_from_table(l, -1, "key");
            Ok(DatastoreRequest::DeleteMetadata(owner_id, key))
        },
        _ => {
            Err(LuaError::Generic(format!("Unknown transaction type at index #{}", i)))
        }
    }
}
