use actix::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use indradb::Vertex;
use rlua::{Error as LuaError, Function, Table, Lua, RegistryKey};
use script::{converters, Reader, Request, create as create_context};
use serde_json::value::Value as JsonValue;
use std::thread::{spawn, JoinHandle};

// TODO: `payload` currently copied for each request when it doesn't need to
// be. switch this to references, or find a better way to initialize the lua
// context.
pub struct MapRequest {
    req: Request,
    vertex: Vertex
}

impl MapRequest {
    pub fn new(req: Request, vertex: Vertex) -> Self {
        Self { req, vertex }
    }
}

impl Message for MapRequest {
    type Result = Result<converters::JsonValue, LuaError>;
}

pub struct ReduceRequest {
    req: Request,
    first: converters::JsonValue,
    second: converters::JsonValue
}

impl ReduceRequest {
    pub fn new(req: Request, first: converters::JsonValue, second: converters::JsonValue) -> Self {
        Self { req, first, second }
    }
}

impl Message for ReduceRequest {
    type Result = Result<converters::JsonValue, LuaError>;
}

pub struct Worker {
    context: Option<Lua>,
    mapper: Option<RegistryKey>,
    reducer: Option<RegistryKey>,
}

impl Default for Worker {
    fn default() -> Self {
        Worker {
            context: None,
            mapper: None,
            reducer: None
        }
    }
}

impl Worker {
    fn initialize(&mut self, req: Request) -> Result<(), LuaError> {
        if self.context.is_some() {
            return Ok(());
        }

        let value = Reader::new().get(&req.name)?;
        let context = create_context(req.payload)?;
        let table: Table = context.exec(&value.contents, Some(value.path))?;
        self.context = Some(context);
        self.mapper = Some(context.create_registry_value(table.get("map")?)?);
        self.reducer = Some(context.create_registry_value(table.get("reduce")?)?);
        Ok(())
    }
}

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<MapRequest> for Worker {
    type Result = Result<converters::JsonValue, LuaError>;

    fn handle(&mut self, req: MapRequest, _: &mut Self::Context) -> Self::Result {
        self.initialize(req.req)?;
        let mapper: Function = self.context.unwrap().registry_value(&self.mapper.unwrap())?;
        let value: converters::JsonValue = mapper.call(converters::Vertex::new(req.vertex))?;
        Ok(value)
    }
}

impl Handler<ReduceRequest> for Worker {
    type Result = Result<converters::JsonValue, LuaError>;

    fn handle(&mut self, req: ReduceRequest, _: &mut Self::Context) -> Self::Result {
        self.initialize(req.req)?;
        let reducer: Function = self.context.unwrap().registry_value(&self.reducer.unwrap())?;
        let value: converters::JsonValue = reducer.call((req.first, req.second))?;
        Ok(value)
    }
}
