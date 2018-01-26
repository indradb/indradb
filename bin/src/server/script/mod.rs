mod api;
mod converters;

#[cfg(test)]
mod tests;

use rlua::{Table, Value};
use rlua::prelude::*;
use serde_json::value::Value as JsonValue;
use std::path::Path;
use uuid::Uuid;
use indradb::Datastore;
use statics;
use std::sync::Arc;

/// Runs a script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn run(
    account_id: Uuid,
    contents: &str,
    path: &Path,
    arg: JsonValue,
) -> Result<JsonValue, LuaError> {
    let l = Lua::new();
    let globals = l.globals();

    let fun = l.load(contents, path.to_str())?;

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

    // Add the transaction as a global variable.
    let trans = statics::DATASTORE.transaction(account_id).map_err(|err| {
        LuaError::ExternalError(Arc::new(err))
    })?;

    globals.set("trans", converters::ProxyTransaction::new(trans))?;

    // Add the account id as a global variable.
    globals.set("account_id", account_id.to_string())?;

    globals.set("arg", converters::JsonValue::new(arg))?;

    let value: converters::JsonValue = fun.call(Value::Nil)?;
    Ok(value.0)
}
