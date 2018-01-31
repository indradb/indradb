use rlua::prelude::*;
use indradb::Error;

#[derive(Debug)]
pub enum ScriptError {
    Lua(LuaError),
    Transaction(Error)
}

impl From<Error> for ScriptError {
    fn from(err: Error) -> ScriptError {
        ScriptError::Transaction(err)
    }
}

impl From<LuaError> for ScriptError {
    fn from(err: LuaError) -> ScriptError {
        ScriptError::Lua(err)
    }
}

#[derive(Debug)]
pub enum MapReduceError {
    WorkerSetup { description: String, cause: ScriptError },
    MapCall(LuaError),
    ReduceCall(LuaError)
}
