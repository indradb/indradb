use rlua::prelude::*;
use indradb::Error;
use std::any::Any;
use std::boxed::Box;

#[derive(Debug)]
pub enum ScriptError {
    Lua(LuaError),
    Transaction(Error),
    WorkerError(Box<Any + Send + 'static>)
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

impl From<Box<Any + Send>> for ScriptError {
    fn from(err: Box<Any + Send>) -> ScriptError {
        ScriptError::WorkerError(err)
    }
}
