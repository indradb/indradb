use rlua::{Error as LuaError, FromLua, Lua, Result as LuaResult, Table, ToLua, Value};
use serde_json::{Map, Number as JsonNumber, Value as ExternalJsonValue};
use common::ProxyTransaction as ExternalProxyTransaction;
use indradb::{Edge as ExternalEdge, EdgeKey as ExternalEdgeKey, EdgeQuery as ExternalEdgeQuery,
            QueryTypeConverter, Type as ExternalType, Vertex as ExternalVertex,
            VertexQuery as ExternalVertexQuery, Weight as ExternalWeight};
use uuid::Uuid as ExternalUuid;
use core::str::FromStr;
use std::collections::BTreeMap;
use chrono::{DateTime, NaiveDateTime};
use chrono::offset::Utc;

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
            Value::Integer(value) => Ok(Self::new(
                ExternalJsonValue::Number(JsonNumber::from(value)),
            )),
            Value::Number(value) => {
                let num = JsonNumber::from_f64(value)
                    .expect("Expected to be able to create a JSON number from a float");
                Ok(Self::new(ExternalJsonValue::Number(num)))
            }
            Value::String(value) => {
                let value_str = match value.to_str() {
                    Ok(s) => s.to_string(),
                    Err(err) => {
                        return Err(new_from_lua_error(
                            "string",
                            "JSON",
                            Some(format!("the lua string is not valid utf-8: {}", err)),
                        ));
                    }
                };

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
pub struct ProxyTransaction<'a> {
    pub trans: &'a mut ExternalProxyTransaction,
}

impl<'lua> ProxyTransaction<'lua> {
    pub fn new(value: &'lua mut ExternalProxyTransaction) -> Self {
        Self { trans: value }
    }
}

impl<'lua> FromLua<'lua> for ProxyTransaction<'lua> {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        if let Value::LightUserData(value) = value {
            let trans_ptr = unsafe { &mut **(value.0 as *mut &mut ExternalProxyTransaction) };
            Ok(Self::new(trans_ptr))
        } else {
            Err(new_from_lua_error("non-lightuserdata", "transaction", None))
        }
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
        Ok(Self::new(type_from_value(value)?))
    }
}

impl<'lua> ToLua<'lua> for Type {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let value = (self.0).0;
        Ok(Value::String(l.create_string(&value[..])))
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
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
        if let Value::Table(value) = value {
            let outbound_id = uuid_from_value(get_table_value(&value, "outbound_id")?)?;
            let t = type_from_value(get_table_value(&value, "type")?)?;
            let inbound_id = uuid_from_value(get_table_value(&value, "inbound_id")?)?;
            Ok(EdgeKey::new(
                ExternalEdgeKey::new(outbound_id, t, inbound_id),
            ))
        } else {
            Err(new_from_lua_error("non-table", "edge key", None))
        }
    }
}

impl<'lua> ToLua<'lua> for EdgeKey {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let table = l.create_table();
        table.set(
            "outbound_id",
            l.create_string(&self.0.outbound_id.hyphenated().to_string()[..]),
        )?;
        table.set("type", self.0.t.0)?;
        table.set(
            "inbound_id",
            l.create_string(&self.0.inbound_id.hyphenated().to_string()[..]),
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
        let table = l.create_table();
        table.set(
            "id",
            l.create_string(&self.0.id.hyphenated().to_string()[..]),
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
        let table = l.create_table();
        table.set("key", EdgeKey::new(self.0.key).to_lua(l)?)?;
        table.set("weight", Value::Number(f64::from(self.0.weight.0)))?;
        table.set(
            "created_datetime",
            l.create_string(&self.0.created_datetime.to_string()[..]),
        )?;
        Ok(Value::Table(table))
    }
}

#[derive(Debug)]
pub struct Weight(pub ExternalWeight);

impl Weight {
    pub fn new(value: ExternalWeight) -> Self {
        Self { 0: value }
    }
}

impl<'lua> FromLua<'lua> for Weight {
    fn from_lua(value: Value<'lua>, l: &'lua Lua) -> LuaResult<Self> {
        let value_f32 = l.coerce_number(value)? as f32;

        match ExternalWeight::new(value_f32) {
            Ok(value_weight) => Ok(Weight::new(value_weight)),
            Err(err) => Err(new_from_lua_error(
                "number",
                "weight",
                Some(format!("{}", err)),
            )),
        }
    }
}

impl<'lua> ToLua<'lua> for Weight {
    fn to_lua(self, _: &'lua Lua) -> LuaResult<Value<'lua>> {
        Ok(Value::Number(f64::from((self.0).0)))
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
        Ok(Self::new(uuid_from_value(value)?))
    }
}

impl<'lua> ToLua<'lua> for Uuid {
    fn to_lua(self, l: &'lua Lua) -> LuaResult<Value<'lua>> {
        let s = self.0.hyphenated().to_string();
        Ok(Value::String(l.create_string(&s[..])))
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
                        Some(start_id) => Some(ExternalUuid::from_str(&start_id[..]).map_err(|e| {
                            new_from_lua_error("string", "uuid", Some(format!("{}", e)))
                        })?),
                        None => None,
                    };

                let limit = limit_from_table(&value, l)?;
                Ok(VertexQuery::new(
                    ExternalVertexQuery::All { start_id, limit },
                ))
            } else if t == "vertices" {
                if let Value::Table(ids_values) = get_table_value(&value, "ids")? {
                    let mut ids = vec![];

                    for pair in ids_values.pairs::<Value, Value>() {
                        let (_, id) = pair?;
                        ids.push(uuid_from_value(id)?);
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
                let converter = converter_from_table(&value)?;
                let limit = limit_from_table(&value, l)?;
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
                let converter = converter_from_table(&value)?;

                let type_filter = match optional_string_from_value(
                    get_table_value(&value, "type_filter")?,
                )? {
                    Some(type_filter_str) => {
                        let type_filter = ExternalType::new(type_filter_str.to_string()).map_err(
                            |e| new_from_lua_error("string", "type", Some(format!("{}", e))),
                        )?;
                        Some(type_filter)
                    }
                    None => None,
                };

                let high_filter =
                    optional_datetime_from_value(&get_table_value(&value, "high_filter")?)?;
                let low_filter =
                    optional_datetime_from_value(&get_table_value(&value, "low_filter")?)?;
                let limit = limit_from_table(&value, l)?;
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

fn new_from_lua_error(from: &'static str, to: &'static str, message: Option<String>) -> LuaError {
    LuaError::FromLuaConversionError {
        from: from,
        to: to,
        message: message,
    }
}

fn get_table_value<'lua>(table: &'lua Table, name: &str) -> LuaResult<Value<'lua>> {
    table.get(name).map_err(|_| {
        new_from_lua_error("", "", Some(format!("missing `{}` in the table", name)))
    })
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

fn uuid_from_value(value: Value) -> LuaResult<ExternalUuid> {
    let value_string = string_from_value(value)?;
    let value_uuid = ExternalUuid::from_str(&value_string[..]).map_err(|e| {
        new_from_lua_error("string", "uuid", Some(format!("{}", e)))
    })?;
    Ok(value_uuid)
}

fn type_from_value(value: Value) -> LuaResult<ExternalType> {
    let value_string = string_from_value(value)?;
    let value_type = ExternalType::new(value_string.to_string()).map_err(|e| {
        new_from_lua_error("string", "type", Some(format!("{}", e)))
    })?;
    Ok(value_type)
}

fn converter_from_table(table: &Table) -> LuaResult<QueryTypeConverter> {
    match &string_from_value(get_table_value(table, "converter")?)?[..] {
        "outbound" => Ok(QueryTypeConverter::Outbound),
        "inbound" => Ok(QueryTypeConverter::Inbound),
        _ => Err(new_from_lua_error(
            "",
            "",
            Some("Unexpected converter type".to_string()),
        )),
    }
}

fn limit_from_table<'lua>(table: &'lua Table, l: &'lua Lua) -> LuaResult<u32> {
    let limit = l.coerce_integer(get_table_value(table, "limit")?)?;

    if limit < 0 {
        Err(new_from_lua_error(
            "integer",
            "limit",
            Some("value is below 0".to_string()),
        ))
    } else {
        Ok(limit as u32)
    }
}
