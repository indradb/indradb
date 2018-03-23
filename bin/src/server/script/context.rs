use super::{converters, globals};
use indradb::Datastore;
use rlua::Table;
use rlua::prelude::*;
use serde_json::value::Value as JsonValue;
use statics;
use std::path::Path;

pub fn create(arg: JsonValue) -> Result<Lua, LuaError> {
    let l = Lua::new();

    {
        let g = l.globals();

        // Update the `package.path` to include the script root, so it's easier
        // for scripts to require each other.
        {
            let package: Table = g.get("package")?;
            let old_path: String = package.get("path")?;
            let script_path = Path::new(&statics::SCRIPT_ROOT[..])
                .join("?.lua")
                .to_str()
                .unwrap()
                .to_string();
            package.set("path", format!("{};{}", old_path, script_path))?;
        }

        // Add globals
        g.set("arg", converters::JsonValue::new(arg))?;
        g.set(
            "transaction",
            l.create_function(|_, ()| {
                let trans = statics::DATASTORE
                    .transaction()
                    .map_err(|err| LuaError::RuntimeError(format!("{}", err)))?;
                Ok(converters::ProxyTransaction::new(trans))
            })?,
        )?;
    }

    let _: () = l.eval(globals::GLOBALS, Some("globals.lua"))?;
    Ok(l)
}
