use actix::prelude::*;
use indradb::Vertex;
use rlua::{Error as LuaError, Function, Table, Lua, RegistryKey};
use script::{converters, Reader, Request, ReaderError, create as create_context};

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
    type Result = Result<converters::JsonValue, WorkerError>;
}

pub struct ReduceRequest {
    req: Request,
    accumulator: Result<converters::JsonValue, WorkerError>,
    value: Result<converters::JsonValue, WorkerError>
}

impl ReduceRequest {
    pub fn new(req: Request, accumulator: Result<converters::JsonValue, WorkerError>, value: Result<converters::JsonValue, WorkerError>) -> Self {
        Self { req, accumulator, value }
    }
}

impl Message for ReduceRequest {
    type Result = Result<converters::JsonValue, WorkerError>;
}

#[derive(Debug, Clone)]
pub enum WorkerError {
    Reader(ReaderError),
    Lua(LuaError)
}

impl From<ReaderError> for WorkerError {
    fn from(err: ReaderError) -> Self {
        WorkerError::Reader(err)
    }
}

impl From<LuaError> for WorkerError {
    fn from(err: LuaError) -> Self {
        WorkerError::Lua(err)
    }
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
    fn initialize(&mut self, req: Request) -> Result<(), WorkerError> {
        if self.context.is_some() {
            return Ok(());
        }
        
        let context = create_context(req.payload)?;

        let (mapper, reducer) = {
            let mut reader = Reader::new();
            let value = reader.get(&req.name)?;

            let table = {
                let t: Table = context.exec(&value.contents, Some(&value.path))?;
                t
            };

            let mapper = {
                let f: Function = table.get("map")?;
                context.create_registry_value(f)?
            };

            let reducer = {
                let f: Function = table.get("reduce")?;
                context.create_registry_value(f)?
            };

            (mapper, reducer)
        };

        self.context = Some(context);
        self.mapper = Some(mapper);
        self.reducer = Some(reducer);
        Ok(())
    }
}

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<MapRequest> for Worker {
    type Result = Result<converters::JsonValue, WorkerError>;

    fn handle(&mut self, req: MapRequest, _: &mut Self::Context) -> Self::Result {
        self.initialize(req.req)?;
        let context = self.context.unwrap();
        let mapper: Function = context.registry_value(&self.mapper.unwrap())?;
        let value: converters::JsonValue = mapper.call(converters::Vertex::new(req.vertex))?;
        Ok(value)
    }
}

impl Handler<ReduceRequest> for Worker {
    type Result = Result<converters::JsonValue, WorkerError>;

    fn handle(&mut self, req: ReduceRequest, _: &mut Self::Context) -> Self::Result {
        match (req.accumulator, req.value) {
            (Err(err), _) => Err(err),
            (_, Err(err)) => Err(err),
            (Ok(accumulator), Ok(value)) => {
                self.initialize(req.req)?;
                let reducer: Function = self.context.unwrap().registry_value(&self.reducer.unwrap())?;
                let value: converters::JsonValue = reducer.call((accumulator, value))?;
                Ok(value)
            }
        }
    }
}
