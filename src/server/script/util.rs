use lua;
use serde_json::value::Value as JsonValue;
use serde_json::{Map, Number};
use braid::{Vertex, Edge, Type, Weight, VertexQuery};
use chrono::{DateTime, UTC, NaiveDateTime};
use std::{isize, i32, u16};
use uuid::Uuid;
use core::str::FromStr;
use super::errors::LuaError;
use serde_json;

unsafe fn debug_stack(l: &mut lua::ExternState) {
    let top = l.gettop();

    for i in 1..top+1 {
        match l.type_(i) {
            Some(lua::Type::Nil) => println!("{}: nil", i),
            Some(lua::Type::Boolean) => println!("{}: boolean: {:?}", i, l.toboolean(i)),
            Some(lua::Type::Number) => println!("{}: number: {:?}", i, l.tonumber(i)),
            Some(lua::Type::String) => println!("{}: string: {:?}", i, l.tostring(i)),
            _ => println!("{}: {:?}", i, l.type_(i))
        }
    }
}

/// Deserializes a lua value into a JSON value.
/// NOTE: `l.checkstring` doesn't seem to properly handle `nil` values, so in
/// functions that accept optional lua strings, we take empty strings instead
/// of nil values.
pub unsafe fn deserialize_json(l: &mut lua::ExternState, offset: i32) -> Result<JsonValue, LuaError> {
    Ok(match l.type_(offset) {
        Some(lua::Type::Nil) |
        None => JsonValue::Null,
        Some(lua::Type::Boolean) => JsonValue::Bool(l.toboolean(offset)),
        Some(lua::Type::Number) => JsonValue::Number(Number::from_f64(l.tonumber(offset)).unwrap()),
        Some(lua::Type::String) => {
            JsonValue::String(l.checkstring(offset).unwrap().to_string().clone())
        }
        Some(lua::Type::Table) => {
            let stack_size_before = l.gettop();
            l.pushvalue(offset);
            l.pushnil();
            let mut is_array = false;

            while l.next(-2) {
                if l.type_(-2) == Some(lua::Type::Number) {
                    is_array = true;
                }

                l.pop(1);
                break;
            }

            // Stack size could be 1 or 2 larger than before, depending on how
            // many keys are in the table. This handles either.
            let stack_size_after = l.gettop();
            l.pop(stack_size_after - stack_size_before);
            l.pushvalue(offset);
            l.pushnil();

            let shit = if is_array {
                let mut v: Vec<JsonValue> = Vec::new();
                let mut next_index = 1;

                while l.next(-2) {
                    assert_eq!(l.checkinteger(-2), next_index);
                    next_index += 1;
                    v.push(deserialize_json(l, -1)?);
                    l.pop(1);
                }

                l.pop(1);
                JsonValue::Array(v)
            } else {
                let mut o: Map<String, JsonValue> = Map::new();

                while l.next(-2) {
                    let k = l.checkstring(-2).unwrap_or("").to_string().clone();
                    let v: JsonValue = deserialize_json(l, -1)?;
                    o.insert(k, v);
                    l.pop(1);
                }

                l.pop(1);
                JsonValue::Object(o)
            };

            shit
        }
        _ => return Err(LuaError::Generic("Could not deserialize return value".to_string())),
    })
}

/// Serializes a JSON value into a lua value.
pub unsafe fn serialize_json(l: &mut lua::ExternState, json: JsonValue) {
    match json {
        JsonValue::Null => l.pushnil(),
        JsonValue::Bool(v) => l.pushboolean(v),
        JsonValue::Number(v) => {
            if v.is_f64() {
                l.pushnumber(v.as_f64().unwrap());
            } else {
                l.pushstring(&v.to_string()[..]);
            }
        },
        JsonValue::String(v) => l.pushstring(&v[..]),
        JsonValue::Array(v) => {
            l.newtable();

            for (i, iv) in v.iter().enumerate() {
                l.pushinteger((i + 1) as isize);
                serialize_json(l, iv.clone());
                l.settable(-3);
            }
        }
        JsonValue::Object(v) => {
            l.newtable();

            for (k, iv) in &v {
                l.pushstring(k);
                serialize_json(l, iv.clone());
                l.settable(-3);
            }
        }
    }
}

/// Serializes a vertex range into a lua table.
pub unsafe fn serialize_vertices(l: &mut lua::ExternState, vertices: Vec<Vertex>) {
    l.newtable();

    for (i, vertex) in vertices.iter().enumerate() {
        l.pushinteger((i + 1) as isize);
        serialize_vertex(l, vertex);
        l.settable(-3);
    }
}

/// Serializes an edge range into a lua table.
pub unsafe fn serialize_edges(l: &mut lua::ExternState, edges: Vec<Edge>) {
    l.newtable();

    for (i, edge) in edges.iter().enumerate() {
        l.pushinteger((i + 1) as isize);
        serialize_edge(l, edge);
        l.settable(-3);
    }
}

/// Serializes a avertex into a lua table.
pub unsafe fn serialize_vertex(l: &mut lua::ExternState, vertex: &Vertex) {
    l.newtable();
    add_string_field_to_table(l, "id", &vertex.id.to_string()[..]);
    add_string_field_to_table(l, "type", &vertex.t.0[..]);
}

/// Serializes an edge into a lua table.
pub unsafe fn serialize_edge(l: &mut lua::ExternState, edge: &Edge) {
    l.newtable();
    add_string_field_to_table(l, "outbound_id", &edge.outbound_id.to_string()[..]);
    add_string_field_to_table(l, "type", &edge.t.0[..]);
    add_string_field_to_table(l, "inbound_id", &edge.inbound_id.to_string()[..]);
    add_number_field_to_table(l, "weight", edge.weight.0 as f64);
}

/// Adds a field to a table with a string value
pub unsafe fn add_string_field_to_table(l: &mut lua::ExternState, k: &str, v: &str) {
    l.pushstring(v);
    l.setfield(-2, k);
}

/// Adds a field to a table with a numeric value
pub unsafe fn add_number_field_to_table(l: &mut lua::ExternState, k: &str, v: f64) {
    l.pushnumber(v);
    l.setfield(-2, k);
}

/// Gets a string value from lua by its offset
pub unsafe fn get_vertex_query_param(l: &mut lua::ExternState, narg: i32) -> Result<VertexQuery, LuaError> {
    let q_json = deserialize_json(l, 1)?;

    match serde_json::from_value::<VertexQuery>(q_json) {
        Ok(val) => Ok(val),
        Err(err) => {
            panic!(err);
            //Err(LuaError::Arg(narg, "Expected vertex query table".to_string()))
        }
    }
}

/// Gets a string value from lua by its offset
pub unsafe fn get_string_param(l: &mut lua::ExternState, narg: i32) -> Result<String, LuaError> {
    match l.checkstring(narg) {
        Some(s) => Ok(s.to_string()),
        None => Err(LuaError::Arg(narg, "Expected string".to_string())),
    }
}

/// Gets a type value from lua by its offset
pub unsafe fn get_type_param(l: &mut lua::ExternState, narg: i32) -> Result<Type, LuaError> {
    let s = get_string_param(l, narg)?;
    Ok(Type::new(s)?)
}

/// Gets an optional type value from lua by its offset
pub unsafe fn get_optional_type_param(l: &mut lua::ExternState, narg: i32) -> Result<Option<Type>, LuaError> {
    let s = get_string_param(l, narg)?;

    if s == "" {
        Ok(None)
    } else {
        Ok(Some(Type::new(s)?))
    }
}

/// Gets either a string value that represents an i64 or a nil from lua by its offset
pub unsafe fn get_optional_i64_param(l: &mut lua::ExternState, narg: i32) -> Result<Option<i64>, LuaError> {
    let s = get_string_param(l, narg)?;

    if s == "" {
        Ok(None)
    } else {
        match i64::from_str_radix(&s[..], 10) {
            Ok(i) => Ok(Some(i)),
            Err(_) => Err(LuaError::Arg(narg, "Expected i64 as string".to_string())),
        }
    }
}

/// Gets a string value that represents a uuid from lua by its offset
pub unsafe fn get_uuid_param(l: &mut lua::ExternState, narg: i32) -> Result<Uuid, LuaError> {
    match get_optional_uuid_param(l, narg)? {
        Some(val) => Ok(val),
        None => Err(LuaError::Arg(narg, "Expected uuid as string".to_string()))
    }
}

/// Gets a string value that represents a uuid from lua by its offset
pub unsafe fn get_optional_uuid_param(l: &mut lua::ExternState, narg: i32) -> Result<Option<Uuid>, LuaError> {
    let s = get_string_param(l, narg)?;

    if s == "" {
        Ok(None)
    } else {
        match Uuid::from_str(&s[..]) {
            Ok(u) => Ok(Some(u)),
            Err(_) => Err(LuaError::Arg(narg, "Expected uuid as string".to_string())),
        }
    }
}

/// Gets either a string value that represents a timestamp or a nil from lua
pub unsafe fn get_optional_datetime_param(l: &mut lua::ExternState, narg: i32) -> Result<Option<DateTime<UTC>>, LuaError> {
    match get_optional_i64_param(l, narg)? {
        Some(i) => Ok(Some(DateTime::from_utc(NaiveDateTime::from_timestamp(i, 0), UTC))),
        None => Ok(None),
    }
}

/// Gets a limit value from lua by its offset
pub unsafe fn get_limit_param(l: &mut lua::ExternState, narg: i32) -> Result<u16, LuaError> {
    match l.checkinteger(narg) {
        i if i > u16::MAX as isize => Ok(u16::MAX),
        i if i < 0 => Err(LuaError::Arg(narg, "Limit cannot be negative".to_string())),
        i => Ok(i as u16),
    }
}

/// Gets a weight value from lua by its offset
pub unsafe fn get_weight_param(l: &mut lua::ExternState, narg: i32) -> Result<Weight, LuaError> {
    let w = l.checknumber(narg);
    Ok(Weight::new(w as f32)?)
}

/// Serializes a u64 to lua
pub unsafe fn serialize_u64(l: &mut lua::ExternState, val: u64) {
    l.pushinteger(match val {
        i if i > isize::MAX as u64 => isize::MAX,
        i => i as isize,
    })
}
