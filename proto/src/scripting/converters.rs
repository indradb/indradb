use std::collections::BTreeMap;
use std::str::FromStr;

use super::api;

use chrono::offset::Utc;
use chrono::{DateTime, NaiveDateTime};
use rlua::prelude::*;
// use rlua::{Error as LuaError, FromLua, Lua, Result as LuaResult, Table, ToLua, UserData, UserDataMethods, Value};

macro_rules! proxy_fn {
    ($methods:expr, $name:expr, $func:expr) => {
        $methods.add_method($name, |_, this, args| match this.0.as_ref() {
            Some(trans) => $func(trans, args).map_err(|err| LuaError::RuntimeError(format!("{}", err))),
            None => Err(LuaError::RuntimeError(
                "The transaction has already finished".to_string(),
            )),
        });
    };
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum JsonMapKey {
    String(String),
    Number(u64),
}

#[derive(Debug)]
pub struct JsonValue(pub serde_json::Value);

impl JsonValue {
    pub fn new(value: serde_json::Value) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for JsonValue {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        match value {
            LuaValue::Nil => Ok(Self::new(serde_json::Value::Null)),
            LuaValue::Boolean(value) => Ok(Self::new(serde_json::Value::Bool(value))),
            LuaValue::Integer(value) => Ok(Self::new(serde_json::Value::Number(serde_json::Number::from(value)))),
            LuaValue::Number(value) => {
                let num = serde_json::Number::from_f64(value)
                    .expect("Expected to be able to create a JSON number from a float");
                Ok(Self::new(serde_json::Value::Number(num)))
            }
            LuaValue::String(_) => {
                let value_str: String = String::from_lua(value, l)?;
                Ok(Self::new(serde_json::Value::String(value_str)))
            }
            LuaValue::Table(value) => {
                let mut map = BTreeMap::new();

                for pair in value.pairs::<LuaValue, LuaValue>() {
                    let (key, value) = pair?;
                    let value_json = Self::from_lua(value, l)?;

                    match Self::from_lua(key, l)? {
                        JsonValue(serde_json::Value::String(key_string)) => {
                            map.insert(JsonMapKey::String(key_string), value_json.0);
                        }
                        JsonValue(serde_json::Value::Number(ref key_number)) if key_number.is_u64() => {
                            map.insert(JsonMapKey::Number(key_number.as_u64().unwrap()), value_json.0);
                        }
                        _ => {
                            return Err(new_from_lua_error(
                                "table key",
                                "JSON map key",
                                Some("the table contains an invalid key".to_string()),
                            ));
                        }
                    };
                }

                // Figure out if this is an array or a map
                let mut next_k = 1;
                let is_array = map.keys().all(|k| match *k {
                    JsonMapKey::String(_) => false,
                    JsonMapKey::Number(k) => {
                        if k == next_k {
                            next_k += 1;
                            true
                        } else {
                            false
                        }
                    }
                });

                // Convert the transient map to an appropriate JSON value -
                // either a JSON map, an array, or a null value, depending on
                // the shape
                if is_array {
                    let vec: Vec<serde_json::Value> = map.values().cloned().collect();
                    Ok(Self::new(serde_json::Value::Array(vec)))
                } else {
                    let mut obj = serde_json::Map::new();

                    for (k, v) in map {
                        match k {
                            JsonMapKey::String(k) => obj.insert(k, v),
                            JsonMapKey::Number(k) => obj.insert(k.to_string(), v),
                        };
                    }

                    Ok(Self::new(serde_json::Value::Object(obj)))
                }
            }
            LuaValue::LightUserData(_) => Err(new_from_lua_error("light userdata", "JSON", None)),
            LuaValue::Function(_) => Err(new_from_lua_error("function", "JSON", None)),
            LuaValue::Thread(_) => Err(new_from_lua_error("thread", "JSON", None)),
            LuaValue::UserData(_) => Err(new_from_lua_error("userdata", "JSON", None)),
            LuaValue::Error(_) => Err(new_from_lua_error("error", "JSON", None)),
        }
    }
}

impl<'lua> ToLua<'lua> for JsonValue {
    fn to_lua(self, l: LuaContext) -> LuaResult<LuaValue<'lua>> {
        match self.0 {
            serde_json::Value::Null => Ok(LuaValue::Nil),
            serde_json::Value::Bool(value) => Ok(LuaValue::Boolean(value)),
            serde_json::Value::Number(value) => {
                let value_float = value
                    .as_f64()
                    .expect("Expected to be able to create a float from a JSON number");
                Ok(LuaValue::Number(value_float))
            }
            serde_json::Value::String(value) => value.to_lua(l),
            // For the following two, we need to remap the values from
            // `serde_json::Value` to `JsonValue`, because only `JsonValue`
            // implements `ToLua`.
            serde_json::Value::Array(value) => {
                let mapped = value.into_iter().map(JsonValue::new);
                Ok(LuaValue::Table(l.create_sequence_from(mapped)?))
            }
            serde_json::Value::Object(value) => {
                let mapped = value.into_iter().map(|(x, y)| (x, JsonValue::new(y)));
                Ok(LuaValue::Table(l.create_table_from(mapped)?))
            }
        }
    }
}

#[derive(Debug)]
pub struct Transaction<T: indradb::Transaction + Send + Sync + 'static>(pub T);

impl<T: indradb::Transaction + Send + Sync + 'static> Transaction<T> {
    pub fn new(trans: T) -> Self {
        Self { 0: trans }
    }
}

impl<T: indradb::Transaction + Send + Sync + 'static> LuaUserData for Transaction<T> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        proxy_fn!(methods, "create_vertex_from_type", api::create_vertex_from_type);
        proxy_fn!(methods, "get_vertices", api::get_vertices);
        proxy_fn!(methods, "delete_vertices", api::delete_vertices);
        proxy_fn!(methods, "get_vertex_count", api::get_vertex_count);

        proxy_fn!(methods, "create_edge", api::create_edge);
        proxy_fn!(methods, "get_edges", api::get_edges);
        proxy_fn!(methods, "delete_edges", api::delete_edges);
        proxy_fn!(methods, "get_edge_count", api::get_edge_count);
    }
}

#[derive(Debug)]
pub struct Identifier(pub indradb::Identifier);

impl Identifier {
    pub fn new(value: indradb::Identifier) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for Identifier {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        let value_string = String::from_lua(value, l)?;
        let value_type = indradb::Identifier::new(value_string.to_string())
            .map_err(|e| new_from_lua_error("string", "type", Some(format!("{}", e))))?;
        Ok(Identifier::new(value_type))
    }
}

impl<'lua> ToLua<'lua> for Identifier {
    fn to_lua(self, l: LuaContext) -> LuaResult<LuaValue<'lua>> {
        let value = (self.0).0;
        Ok(LuaValue::String(l.create_string(&value[..])?))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct EdgeKey(pub indradb::EdgeKey);

impl EdgeKey {
    pub fn new(value: indradb::EdgeKey) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for EdgeKey {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        let table = LuaTable::from_lua(value, l)?;
        let outbound_id = Uuid::from_lua(table.get("outbound_id")?, l)?.0;
        let t = Identifier::from_lua(table.get("type")?, l)?.0;
        let inbound_id = Uuid::from_lua(table.get("inbound_id")?, l)?.0;
        Ok(EdgeKey::new(indradb::EdgeKey::new(outbound_id, t, inbound_id)))
    }
}

impl<'lua> ToLua<'lua> for EdgeKey {
    fn to_lua(self, l: LuaContext) -> LuaResult<LuaValue<'lua>> {
        let table = l.create_table()?;
        table.set(
            "outbound_id",
            l.create_string(&self.0.outbound_id.to_hyphenated().to_string()[..])?,
        )?;
        table.set("type", self.0.t.0)?;
        table.set(
            "inbound_id",
            l.create_string(&self.0.inbound_id.to_hyphenated().to_string()[..])?,
        )?;
        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug)]
pub struct Vertex(pub indradb::Vertex);

impl Vertex {
    pub fn new(value: indradb::Vertex) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for Vertex {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        let table = LuaTable::from_lua(value, l)?;
        let id = Uuid::from_lua(table.get("id")?, l)?;
        let t = Identifier::from_lua(table.get("type")?, l)?;
        Ok(Vertex::new(indradb::Vertex::with_id(id.0, t.0)))
    }
}

impl<'lua> ToLua<'lua> for Vertex {
    fn to_lua(self, l: LuaContext) -> LuaResult<LuaValue<'lua>> {
        let table = l.create_table()?;
        table.set("id", l.create_string(&self.0.id.to_hyphenated().to_string()[..])?)?;
        table.set("type", self.0.t.0)?;
        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug)]
pub struct Edge(pub indradb::Edge);

impl Edge {
    pub fn new(value: indradb::Edge) -> Self {
        Self { 0: value }
    }
}

impl<'lua> ToLua<'lua> for Edge {
    fn to_lua(self, l: LuaContext) -> LuaResult<LuaValue<'lua>> {
        let table = l.create_table()?;
        table.set("key", EdgeKey::new(self.0.key).to_lua(l)?)?;
        table.set(
            "created_datetime",
            l.create_string(&self.0.created_datetime.to_string()[..])?,
        )?;
        Ok(LuaValue::Table(table))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(pub uuid::Uuid);

impl Uuid {
    pub fn new(value: uuid::Uuid) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for Uuid {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        let value_string = String::from_lua(value, l)?;
        let value_uuid = uuid::Uuid::from_str(&value_string[..])
            .map_err(|e| new_from_lua_error("string", "uuid", Some(format!("{}", e))))?;
        Ok(Uuid::new(value_uuid))
    }
}

impl<'lua> ToLua<'lua> for Uuid {
    fn to_lua(self, l: LuaContext) -> LuaResult<LuaValue<'lua>> {
        let s = self.0.to_hyphenated().to_string();
        Ok(LuaValue::String(l.create_string(&s[..])?))
    }
}

#[derive(Debug)]
pub struct VertexQuery(pub indradb::VertexQuery);

impl VertexQuery {
    pub fn new(value: indradb::VertexQuery) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for VertexQuery {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        let table = LuaTable::from_lua(value, l)?;
        let t = String::from_lua(table.get("type")?, l)?;

        if t == "range" {
            let start_id = match Option::<String>::from_lua(table.get("start_id")?, l)? {
                Some(s) => Some(
                    uuid::Uuid::from_str(&s)
                        .map_err(|e| new_from_lua_error("string", "uuid", Some(format!("{}", e))))?,
                ),
                None => None,
            };

            let limit = u32::from_lua(table.get("limit")?, l)?;

            Ok(VertexQuery::new(indradb::VertexQuery::All { start_id, limit }))
        } else if t == "specific" {
            let ids: Vec<Uuid> = Vec::<Uuid>::from_lua(table.get("ids")?, l)?;
            let ids: Vec<uuid::Uuid> = ids.into_iter().map(|id| id.0).collect();
            Ok(VertexQuery::new(indradb::VertexQuery::Vertices { ids }))
        } else if t == "pipe" {
            let edge_query = Box::new(EdgeQuery::from_lua(table.get("edge_query")?, l)?.0);
            let converter = EdgeDirection::from_lua(table.get("converter")?, l)?.0;
            let limit = u32::from_lua(table.get("limit")?, l)?;
            Ok(VertexQuery::new(VertexQuery::Pipe {
                edge_query,
                converter,
                limit,
            }))
        } else if t == "property_presence" {
            // TODO
        } else if t == "property_value" {
            // TODO
        } else if t == "pipe_property_presence" {
            // TODO
        } else if t == "pipe_property_value" {
            // TODO
        } else {
            Err(new_from_lua_error(
                "",
                "",
                Some("Unexpected vertex query type".to_string()),
            ))
        }
    }
}

#[derive(Debug)]
pub struct EdgeQuery(pub indradb::EdgeQuery);

impl EdgeQuery {
    pub fn new(value: indradb::EdgeQuery) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for EdgeQuery {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        let table = LuaTable::from_lua(value, l)?;
        let t = String::from_lua(table.get("type")?, l)?;

        if t == "edges" {
            let keys: Vec<EdgeKey> = Vec::<EdgeKey>::from_lua(table.get("keys")?, l)?;
            let keys: Vec<indradb::EdgeKey> = keys.into_iter().map(|edge_key| edge_key.0).collect();
            Ok(EdgeQuery::new(indradb::EdgeQuery::Edges { keys }))
        } else if t == "pipe" {
            let vertex_query = Box::new(VertexQuery::from_lua(table.get("vertex_query")?, l)?.0);
            let converter = EdgeDirection::from_lua(table.get("converter")?, l)?.0;

            let type_filter = match Option::<String>::from_lua(table.get("type_filter")?, l)? {
                Some(s) => Some(
                    indradb::Identifier::new(s)
                        .map_err(|e| new_from_lua_error("string", "type", Some(format!("{}", e))))?,
                ),
                None => None,
            };

            let high_filter = optional_datetime_from_value(&table.get("high_filter")?)?;
            let low_filter = optional_datetime_from_value(&table.get("low_filter")?)?;
            let limit = u32::from_lua(table.get("limit")?, l)?;
            Ok(EdgeQuery::new(indradb::EdgeQuery::Pipe {
                vertex_query,
                converter,
                type_filter,
                high_filter,
                low_filter,
                limit,
            }))
        } else {
            Err(new_from_lua_error(
                "",
                "",
                Some("Unexpected edge query type".to_string()),
            ))
        }
    }
}

#[derive(Debug)]
pub struct EdgeDirection(pub indradb::EdgeDirection);

impl EdgeDirection {
    pub fn new(value: indradb::EdgeDirection) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for EdgeDirection {
    fn from_lua(value: LuaValue<'lua>, l: LuaContext) -> LuaResult<Self> {
        match &String::from_lua(value, l)?[..] {
            "outbound" => Ok(EdgeDirection::new(indradb::EdgeDirection::Outbound)),
            "inbound" => Ok(EdgeDirection::new(indradb::EdgeDirection::Inbound)),
            _ => Err(new_from_lua_error(
                "",
                "",
                Some("Unexpected converter type".to_string()),
            )),
        }
    }
}

fn new_from_lua_error(from: &'static str, to: &'static str, message: Option<String>) -> LuaError {
    LuaError::FromLuaConversionError { from, to, message }
}

fn optional_datetime_from_value(value: &LuaValue) -> LuaResult<Option<DateTime<Utc>>> {
    let timestamp = match *value {
        LuaValue::Integer(value) => value as i64,
        LuaValue::Number(value) => value as i64,
        LuaValue::Nil => return Ok(None),
        _ => return Err(new_from_lua_error("non-number", "datetime", None)),
    };

    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    Ok(Some(dt))
}
