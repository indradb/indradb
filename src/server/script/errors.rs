use lua;
use nutrino::{Error, ValidationError};
use std::i32;

/// Error that is returnable from lua-exposed functions.
///
/// The lua_fn! macro takes these errors and serializes them appropriately to
/// lua.
#[derive(Debug)]
pub enum LuaError {
    Arg(i32, String),
    Generic(String)
}

impl LuaError {
    pub unsafe fn serialize(&self, l: &mut lua::ExternState) {
        match *self {
            LuaError::Arg(idx, ref msg) => l.argerror(idx, &msg[..]),
            LuaError::Generic(ref msg) => l.errorstr(&msg[..])
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
    Panicked(String)
}

impl ScriptError {
    pub fn new_from_loaderror(state: &mut lua::State, err: lua::LoadError) -> ScriptError {
        match err {
            lua::LoadError::ErrSyntax => ScriptError::Syntax(String::from(state.checkstring(-1).unwrap())),
            lua::LoadError::ErrMem => ScriptError::Memory
        }
    }

    pub fn new_from_pcallerror(state: &mut lua::State, err: lua::PCallError) -> ScriptError {
        match err {
            lua::PCallError::ErrRun => ScriptError::Runtime(String::from(state.checkstring(-1).unwrap())),
            lua::PCallError::ErrMem => ScriptError::Memory,
            lua::PCallError::ErrErr => ScriptError::Panicked("Unknown pcall error".to_string())
        }
    }
}

impl From<LuaError> for ScriptError {
    fn from(err: LuaError) -> ScriptError {
		ScriptError::Runtime(format!("{:?}", err))
	}
}
