use lua;
use braid::{Error, ValidationError};
use std::i32;

/// Error that is returnable from lua-exposed functions.
///
/// The `lua_fn!` macro takes these errors and serializes them appropriately
/// to lua.
#[derive(Debug)]
pub enum LuaError {
    Arg(i32, String),
    Generic(String),
}

impl LuaError {
    pub unsafe fn serialize(&self, l: &mut lua::ExternState) {
        match *self {
            LuaError::Arg(idx, ref msg) => l.argerror(idx, &msg[..]),
            LuaError::Generic(ref msg) => l.errorstr(&msg[..]),
        }
    }
}

impl From<Error> for LuaError {
    fn from(err: Error) -> LuaError {
        LuaError::Generic(format!("{:?}", err))
    }
}

impl From<ValidationError> for LuaError {
    fn from(err: ValidationError) -> LuaError {
        LuaError::Generic(format!("{:?}", err))
    }
}

/// Error that may be returned when calling a script.
#[derive(Debug)]
pub enum ScriptError {
    Syntax(String),
    Memory,
    Runtime(String),
    Panicked(String),
    File,
}

impl ScriptError {
    pub fn new_from_load_file_error(
        state: &mut lua::State,
        err: lua::LoadFileError,
    ) -> ScriptError {
        match err {
            lua::LoadFileError::ErrSyntax => {
                ScriptError::Syntax(String::from(state.checkstring(-1).unwrap()))
            }
            lua::LoadFileError::ErrMem => ScriptError::Memory,
            lua::LoadFileError::ErrFile => ScriptError::File,
        }
    }

    pub fn new_from_pcallerror(state: &mut lua::State, err: lua::PCallError) -> ScriptError {
        match err {
            lua::PCallError::ErrRun => {
                ScriptError::Runtime(String::from(state.checkstring(-1).unwrap()))
            }
            lua::PCallError::ErrMem => ScriptError::Memory,
            lua::PCallError::ErrErr => ScriptError::Panicked("Unknown pcall error".to_string()),
        }
    }
}

impl From<LuaError> for ScriptError {
    fn from(err: LuaError) -> ScriptError {
        ScriptError::Runtime(format!("{:?}", err))
    }
}
