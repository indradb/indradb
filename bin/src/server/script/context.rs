use rlua::Table;
use rlua::prelude::*;
use serde_json::value::Value as JsonValue;
use std::path::Path;
use indradb::Datastore;
use statics;
use super::converters;

pub fn create(arg: JsonValue) -> Result<Lua, LuaError> {
    let l = Lua::new();

    {
        let globals = l.globals();

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

        // Add globals
        globals.set("arg", converters::JsonValue::new(arg))?;
        globals.set("transaction", l.create_function(|_, ()| {
            let trans = statics::DATASTORE.transaction().map_err(|err| LuaError::RuntimeError(format!("{}", err)))?;
            Ok(converters::ProxyTransaction::new(trans))
        })?)?;
    }

    Ok(l)
}
