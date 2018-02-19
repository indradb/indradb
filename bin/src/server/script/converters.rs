use rlua::{Error as LuaError, FromLua, Lua, Result as LuaResult, Table, ToLua, UserData,
           UserDataMethods, Value};
use serde_json::{Map, Number as JsonNumber, Value as ExternalJsonValue};
use common::ProxyTransaction as ExternalProxyTransaction;
use indradb::{Edge as ExternalEdge, EdgeKey as ExternalEdgeKey,
              EdgeMetadata as ExternalEdgeMetadata, EdgeQuery as ExternalEdgeQuery,
              EdgeDirection as ExternalEdgeDirection, Type as ExternalType,
              Vertex as ExternalVertex, VertexMetadata as ExternalVertexMetadata,
              VertexQuery as ExternalVertexQuery};
use uuid::Uuid as ExternalUuid;
use core::str::FromStr;
use std::collections::BTreeMap;
use chrono::{DateTime, NaiveDateTime};
use chrono::offset::Utc;
use super::api;

macro_rules! proxy_fn {
    ($methods:expr, $name:expr, $func:expr) => {
        $methods.add_method($name, |_, this, args| {
            match this.0.as_ref() {
                Some(trans) => $func(trans, args).map_err(|err| LuaError::RuntimeError(format!("{}", err))),
                None => Err(LuaError::RuntimeError("The transaction has already finished".to_string()))
            }
        });
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum JsonMapKey {
    String(String),
    Number(u64),
}

#[derive(Debug)]
pub struct JsonValue(pub ExternalJsonValue);

impl JsonValue {
    pub fn new(value: ExternalJsonValue) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for JsonValue {
    fn from_lua(value: Value<'lua>, l: &'lua Lua) -> LuaResult<Self> {
        match value {
            Value::Nil => Ok(Self::new(ExternalJsonValue::Null)),
            Value::Boolean(value) => Ok(Self::new(ExternalJsonValue::Bool(value))),
            Value::LightUserData(_) => Err(new_from_lua_error("light userdata", "JSON", None)),
            Value::Integer(value) => Ok(Self::new(ExternalJsonValue::Number(JsonNumber::from(
                value,
            )))),
            Value::Number(value) => {
                let num = JsonNumber::from_f64(value)
                    .expect("Expected to be able to create a JSON number from a float");
                Ok(Self::new(ExternalJsonValue::Number(num)))
            }
            Value::String(value) => {
                let value_str = value
                    .to_str()
                    .map_err(|err| {
                        new_from_lua_error(
                            "string",
                            "JSON",
                            Some(format!("the lua string is not valid utf-8: {}", err)),
                        )
                    })?
                    .to_string();

                Ok(Self::new(ExternalJsonValue::String(value_str)))
            }
            Value::Table(value) => {
                let mut map = BTreeMap::new();

                for pair in value.pairs::<Value, Value>() {
                    let (key, value) = pair?;
                    let value_json = Self::from_lua(value, l)?;

                    match Self::from_lua(key, l)? {
                        JsonValue(ExternalJsonValue::String(key_string)) => {
                            map.insert(JsonMapKey::String(key_string), value_json.0);
                        }
                        JsonValue(ExternalJsonValue::Number(key_number)) => {
                            if key_number.is_u64() {
                                map.insert(
                                    JsonMapKey::Number(key_number.as_u64().unwrap()),
                                    value_json.0,
                                );
                            } else {
                                return Err(new_from_lua_error(
                                    "table key",
                                    "JSON map key",
                                    Some("the table contains an invalid key".to_string()),
                                ));
                            }
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
                    JsonMapKey::Number(k) => if k == next_k {
                        next_k += 1;
                        true
                    } else {
                        false
                    },
                });

                // Convert the transient map to an appropriate JSON value -
                // either a JSON map, an array, or a null value, depending on
                // the shape
                if is_array {
                    let vec: Vec<ExternalJsonValue> = map.values().cloned().collect();
                    Ok(Self::new(ExternalJsonValue::Array(vec)))
                } else {
                    let mut obj = Map::new();

                    for (k, v) in map {
                        match k {
                            JsonMapKey::String(k) => obj.insert(k, v),
                            JsonMapKey::Number(k) => obj.insert(k.to_string(), v),
                        };
                    }

                    Ok(Self::new(ExternalJsonValue::Object(obj)))
                }
            }
            Value::Function(_) => Err(new_from_lua_error("function", "JSON", None)),
            Value::Thread(_) => Err(new_from_lua_error("thread", "JSON", None)),
            Value::UserData(_) => Err(new_from_lua_error("userdata", "JSON", None)),
            Value::Error(_) => Err(new_from_lua_error("error", "JSON", None)),
        }
    }
}

impl<'lua> ToLua<'lua> for JsonValue {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        match self.0 {
            ExternalJsonValue::Null => Ok(Value::Nil),
            ExternalJsonValue::Bool(value) => Ok(Value::Boolean(value)),
            ExternalJsonValue::Number(value) => {
                let value_float = value
                    .as_f64()
                    .expect("Expected to be able to create a float from a JSON number");
                Ok(Value::Number(value_float))
            }
            ExternalJsonValue::String(value) => value.to_lua(l),
            // For the following two, we need to remap the values from
            // `ExternalJsonValue` to `JsonValue`, because only `JsonValue`
            // implements `ToLua`.
            ExternalJsonValue::Array(value) => {
                let mapped = value.into_iter().map(JsonValue::new);
                Ok(Value::Table(l.create_sequence_from(mapped)?))
            }
            ExternalJsonValue::Object(value) => {
                let mapped = value.into_iter().map(|(x, y)| (x, JsonValue::new(y)));
                Ok(Value::Table(l.create_table_from(mapped)?))
            }
        }
    }
}

#[derive(Debug)]
pub struct ProxyTransaction(pub Option<ExternalProxyTransaction>);

impl ProxyTransaction {
    pub fn new(trans: ExternalProxyTransaction) -> Self {
        Self { 0: Some(trans) }
    }
}

impl UserData for ProxyTransaction {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        proxy_fn!(methods, "create_vertex", api::create_vertex);
        proxy_fn!(methods, "get_vertices", api::get_vertices);
        proxy_fn!(methods, "delete_vertices", api::delete_vertices);

        proxy_fn!(methods, "create_edge", api::create_edge);
        proxy_fn!(methods, "get_edges", api::get_edges);
        proxy_fn!(methods, "delete_edges", api::delete_edges);
        proxy_fn!(methods, "get_edge_count", api::get_edge_count);

        proxy_fn!(methods, "get_global_metadata", api::get_global_metadata);
        proxy_fn!(methods, "set_global_metadata", api::set_global_metadata);
        proxy_fn!(
            methods,
            "delete_global_metadata",
            api::delete_global_metadata
        );
        proxy_fn!(methods, "get_vertex_metadata", api::get_vertex_metadata);
        proxy_fn!(methods, "set_vertex_metadata", api::set_vertex_metadata);
        proxy_fn!(
            methods,
            "delete_vertex_metadata",
            api::delete_vertex_metadata
        );
        proxy_fn!(methods, "get_edge_metadata", api::get_edge_metadata);
        proxy_fn!(methods, "set_edge_metadata", api::set_edge_metadata);
        proxy_fn!(methods, "delete_edge_metadata", api::delete_edge_metadata);
    }
}

#[derive(Debug)]
pub struct Type(pub ExternalType);

impl Type {
    pub fn new(value: ExternalType) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for Type {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        let value_string = string_from_value(value)?;
        let value_type = ExternalType::new(value_string.to_string())
            .map_err(|e| new_from_lua_error("string", "type", Some(format!("{}", e))))?;
        Ok(Type::new(value_type))
    }
}

impl<'lua> ToLua<'lua> for Type {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let value = (self.0).0;
        Ok(Value::String(l.create_string(&value[..])?))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct EdgeKey(pub ExternalEdgeKey);

impl EdgeKey {
    pub fn new(value: ExternalEdgeKey) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for EdgeKey {
    fn from_lua(value: Value<'lua>, l: &'lua Lua) -> LuaResult<Self> {
        if let Value::Table(value) = value {
            let outbound_id = Uuid::from_lua(get_table_value(&value, "outbound_id")?, l)?.0;
            let t = Type::from_lua(get_table_value(&value, "type")?, l)?.0;
            let inbound_id = Uuid::from_lua(get_table_value(&value, "inbound_id")?, l)?.0;
            Ok(EdgeKey::new(ExternalEdgeKey::new(
                outbound_id,
                t,
                inbound_id,
            )))
        } else {
            Err(new_from_lua_error("non-table", "edge key", None))
        }
    }
}

impl<'lua> ToLua<'lua> for EdgeKey {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let table = l.create_table()?;
        table.set(
            "outbound_id",
            l.create_string(&self.0.outbound_id.hyphenated().to_string()[..])?,
        )?;
        table.set("type", self.0.t.0)?;
        table.set(
            "inbound_id",
            l.create_string(&self.0.inbound_id.hyphenated().to_string()[..])?,
        )?;
        Ok(Value::Table(table))
    }
}

#[derive(Debug)]
pub struct Vertex(pub ExternalVertex);

impl Vertex {
    pub fn new(value: ExternalVertex) -> Self {
        Self { 0: value }
    }
}

impl<'lua> ToLua<'lua> for Vertex {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let table = l.create_table()?;
        table.set(
            "id",
            l.create_string(&self.0.id.hyphenated().to_string()[..])?,
        )?;
        table.set("type", self.0.t.0)?;
        Ok(Value::Table(table))
    }
}

#[derive(Debug)]
pub struct Edge(pub ExternalEdge);

impl Edge {
    pub fn new(value: ExternalEdge) -> Self {
        Self { 0: value }
    }
}

impl<'lua> ToLua<'lua> for Edge {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let table = l.create_table()?;
        table.set("key", EdgeKey::new(self.0.key).to_lua(l)?)?;
        table.set(
            "created_datetime",
            l.create_string(&self.0.created_datetime.to_string()[..])?,
        )?;
        Ok(Value::Table(table))
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Uuid(pub ExternalUuid);

impl Uuid {
    pub fn new(value: ExternalUuid) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for Uuid {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        let value_string = string_from_value(value)?;
        let value_uuid = ExternalUuid::from_str(&value_string[..])
            .map_err(|e| new_from_lua_error("string", "uuid", Some(format!("{}", e))))?;
        Ok(Uuid::new(value_uuid))
    }
}

impl<'lua> ToLua<'lua> for Uuid {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let s = self.0.hyphenated().to_string();
        Ok(Value::String(l.create_string(&s[..])?))
    }
}

#[derive(Debug)]
pub struct VertexQuery(pub ExternalVertexQuery);

impl VertexQuery {
    pub fn new(value: ExternalVertexQuery) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for VertexQuery {
    fn from_lua(value: Value<'lua>, l: &'lua Lua) -> LuaResult<Self> {
        if let Value::Table(value) = value {
            let t = string_from_value(get_table_value(&value, "type")?)?;

            if t == "all" {
                let start_id =
                    match optional_string_from_value(get_table_value(&value, "start_id")?)? {
                        Some(start_id) => Some(ExternalUuid::from_str(&start_id[..]).map_err(
                            |e| new_from_lua_error("string", "uuid", Some(format!("{}", e))),
                        )?),
                        None => None,
                    };

                let limit = u32::from_lua(get_table_value(&value, "limit")?, l)?;
                Ok(VertexQuery::new(ExternalVertexQuery::All {
                    start_id,
                    limit,
                }))
            } else if t == "vertices" {
                if let Value::Table(ids_values) = get_table_value(&value, "ids")? {
                    let mut ids = vec![];

                    for pair in ids_values.pairs::<Value, Value>() {
                        let (_, id) = pair?;
                        ids.push(Uuid::from_lua(id, l)?.0);
                    }

                    Ok(VertexQuery::new(ExternalVertexQuery::Vertices { ids }))
                } else {
                    Err(new_from_lua_error(
                        "",
                        "",
                        Some("`vertices` attribute is not a table".to_string()),
                    ))
                }
            } else if t == "pipe" {
                let edge_query =
                    Box::new(EdgeQuery::from_lua(get_table_value(&value, "edge_query")?, l)?.0);
                let converter = EdgeDirection::from_lua(get_table_value(&value, "converter")?, l)?.0;
                let limit = u32::from_lua(get_table_value(&value, "limit")?, l)?;
                Ok(VertexQuery::new(ExternalVertexQuery::Pipe {
                    edge_query,
                    converter,
                    limit,
                }))
            } else {
                Err(new_from_lua_error(
                    "",
                    "",
                    Some("Unexpected vertex query type".to_string()),
                ))
            }
        } else {
            Err(new_from_lua_error("non-table", "vertex query", None))
        }
    }
}

#[derive(Debug)]
pub struct EdgeQuery(pub ExternalEdgeQuery);

impl EdgeQuery {
    pub fn new(value: ExternalEdgeQuery) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for EdgeQuery {
    fn from_lua(value: Value<'lua>, l: &'lua Lua) -> LuaResult<Self> {
        if let Value::Table(value) = value {
            let t = string_from_value(get_table_value(&value, "type")?)?;

            if t == "edges" {
                if let Value::Table(edges) = get_table_value(&value, "keys")? {
                    let mut keys = vec![];

                    for pair in edges.pairs::<Value, Value>() {
                        let (_, key) = pair?;
                        keys.push(EdgeKey::from_lua(key, l)?.0);
                    }

                    Ok(EdgeQuery::new(ExternalEdgeQuery::Edges { keys }))
                } else {
                    Err(new_from_lua_error(
                        "",
                        "",
                        Some("`edges` attribute is not a table".to_string()),
                    ))
                }
            } else if t == "pipe" {
                let vertex_query =
                    Box::new(VertexQuery::from_lua(get_table_value(&value, "vertex_query")?, l)?.0);
                let converter = EdgeDirection::from_lua(get_table_value(&value, "converter")?, l)?.0;

                let type_filter =
                    match optional_string_from_value(get_table_value(&value, "type_filter")?)? {
                        Some(type_filter_str) => {
                            let type_filter = ExternalType::new(type_filter_str.to_string())
                                .map_err(|e| {
                                    new_from_lua_error("string", "type", Some(format!("{}", e)))
                                })?;
                            Some(type_filter)
                        }
                        None => None,
                    };

                let high_filter =
                    optional_datetime_from_value(&get_table_value(&value, "high_filter")?)?;
                let low_filter =
                    optional_datetime_from_value(&get_table_value(&value, "low_filter")?)?;
                let limit = u32::from_lua(get_table_value(&value, "limit")?, l)?;
                Ok(EdgeQuery::new(ExternalEdgeQuery::Pipe {
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
        } else {
            Err(new_from_lua_error("non-table", "edge query", None))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VertexMetadata(pub ExternalVertexMetadata);

impl VertexMetadata {
    pub fn new(value: ExternalVertexMetadata) -> Self {
        Self { 0: value }
    }
}

impl<'lua> ToLua<'lua> for VertexMetadata {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let table = l.create_table()?;
        table.set("id", Uuid::new(self.0.id).to_lua(l)?)?;
        table.set("value", JsonValue::new(self.0.value).to_lua(l)?)?;
        Ok(Value::Table(table))
    }
}

#[derive(Debug, PartialEq)]
pub struct EdgeMetadata(pub ExternalEdgeMetadata);

impl EdgeMetadata {
    pub fn new(value: ExternalEdgeMetadata) -> Self {
        Self { 0: value }
    }
}

impl<'lua> ToLua<'lua> for EdgeMetadata {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let table = l.create_table()?;
        table.set("key", EdgeKey::new(self.0.key).to_lua(l)?)?;
        table.set("value", JsonValue::new(self.0.value).to_lua(l)?)?;
        Ok(Value::Table(table))
    }
}

#[derive(Debug)]
pub struct EdgeDirection(pub ExternalEdgeDirection);

impl EdgeDirection {
    pub fn new(value: ExternalEdgeDirection) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for EdgeDirection {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        match &string_from_value(value)?[..] {
            "outbound" => Ok(EdgeDirection::new(ExternalEdgeDirection::Outbound)),
            "inbound" => Ok(EdgeDirection::new(ExternalEdgeDirection::Inbound)),
            _ => Err(new_from_lua_error(
                "",
                "",
                Some("Unexpected converter type".to_string()),
            )),
        }
    }
}

fn new_from_lua_error(from: &'static str, to: &'static str, message: Option<String>) -> LuaError {
    LuaError::FromLuaConversionError {
        from: from,
        to: to,
        message: message,
    }
}

fn get_table_value<'lua>(table: &'lua Table, name: &str) -> LuaResult<Value<'lua>> {
    table
        .get(name)
        .map_err(|_| new_from_lua_error("", "", Some(format!("missing `{}` in the table", name))))
}

fn string_from_value(value: Value) -> LuaResult<String> {
    if let Value::String(value) = value {
        let value_string = value.to_str().map_err(|e| {
            new_from_lua_error("lua string", "native string", Some(format!("{}", e)))
        })?;
        Ok(value_string.to_string())
    } else {
        Err(new_from_lua_error("non-string", "string", None))
    }
}

fn optional_string_from_value(value: Value) -> LuaResult<Option<String>> {
    match value {
        Value::String(value) => {
            let value_string = value.to_str().map_err(|e| {
                new_from_lua_error("lua string", "native string", Some(format!("{}", e)))
            })?;
            Ok(Some(value_string.to_string()))
        }
        Value::Nil => Ok(None),
        _ => Err(new_from_lua_error("non-string", "string", None)),
    }
}

fn optional_datetime_from_value(value: &Value) -> LuaResult<Option<DateTime<Utc>>> {
    let timestamp = match *value {
        Value::Integer(value) => value as i64,
        Value::Number(value) => value as i64,
        Value::Nil => return Ok(None),
        _ => return Err(new_from_lua_error("non-number", "datetime", None)),
    };

    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
    Ok(Some(dt))
}
