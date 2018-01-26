mod api;
mod converters;
mod errors;

#[cfg(test)]
mod tests;

use rlua::{Table, Value};
use rlua::prelude::*;
use serde_json::value::Value as JsonValue;
use std::path::Path;
use uuid::Uuid;
use indradb::Datastore;
use statics;
use common::ProxyTransaction;
use std::convert::From;

/// Runs a script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn execute(
    account_id: Uuid,
    contents: &str,
    path: &str,
    arg: JsonValue,
) -> Result<JsonValue, errors::ScriptError> {
    let l = Lua::new();
    let globals = l.globals();

    let fun = l.load(contents, Some(path))?;

    // Update the `package.path` to include the script root, so it's easier
    // for scripts to require each other.
    {
        let package: Table = globals.get("package")?;
        let old_path: String = package.get("path")?;
        let script_path = Path::new(&statics::SCRIPT_ROOT[..])
            .join("?.lua")
            .to_str()
            .unwrap()
            .to_string();
        package.set("path", format!("{};{}", old_path, script_path))?;
    }

    // Create a new transaction for the script
    let trans = statics::DATASTORE.transaction(account_id)?;

    // Add globals
    globals.set("trans", converters::ProxyTransaction::new(trans))?;
    globals.set("account_id", account_id.to_string())?;
    globals.set("arg", converters::JsonValue::new(arg))?;

    // Run the script
    let value: Result<converters::JsonValue, LuaError> = fun.call(Value::Nil);

    match value {
        Ok(value) => Ok(value.0),
        Err(err) => Err(errors::ScriptError::from(err))
    }
}

/// Runs a mapreduce script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn mapreduce(
    account_id: Uuid,
    contents: &str,
    path: &str,
    arg: JsonValue,
) -> Result<JsonValue, errors::ScriptError> {
    unimplemented!();
}
