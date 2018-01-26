mod api;
mod converters;

#[cfg(test)]
mod tests;

use rlua::{Error, LightUserData, Table, Value};
use rlua::prelude::*;
use serde_json::value::Value as JsonValue;
use std::path::Path;
use uuid::Uuid;
use statics;
use std::os::raw::c_void;
use std::sync::Arc;
use common::ProxyTransaction;

macro_rules! proxy_fn {
    ($globals:expr, $name:expr, $l:expr, $func:expr) => {
        let f = $l.create_function(|_, args| {
            match $func(args) {
                Ok(val) => Ok(val),
                Err(err) => Err(Error::ExternalError(Arc::new(err)))
            }
        });

        $globals.set($name, f).expect("Expected to be able to set a global function");
    }
}

/// Runs a script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn run(
    mut trans: &ProxyTransaction,
    account_id: Uuid,
    contents: &str,
    path: &Path,
    arg: JsonValue,
) -> Result<JsonValue, LuaError> {
    let l = Lua::new();
    let globals = l.globals();

    proxy_fn!(globals, "create_vertex", &l, api::create_vertex);
    proxy_fn!(globals, "get_vertices", &l, api::get_vertices);
    proxy_fn!(globals, "delete_vertices", &l, api::delete_vertices);

    proxy_fn!(globals, "create_edge", &l, api::create_edge);
    proxy_fn!(globals, "get_edges", &l, api::get_edges);
    proxy_fn!(globals, "delete_edges", &l, api::delete_edges);
    proxy_fn!(globals, "get_edge_count", &l, api::get_edge_count);

    proxy_fn!(globals, "get_global_metadata", &l, api::get_global_metadata);
    proxy_fn!(globals, "set_global_metadata", &l, api::set_global_metadata);
    proxy_fn!(
        globals,
        "delete_global_metadata",
        &l,
        api::delete_global_metadata
    );
    proxy_fn!(
        globals,
        "get_account_metadata",
        &l,
        api::get_account_metadata
    );
    proxy_fn!(
        globals,
        "set_account_metadata",
        &l,
        api::set_account_metadata
    );
    proxy_fn!(
        globals,
        "delete_account_metadata",
        &l,
        api::delete_account_metadata
    );
    proxy_fn!(globals, "get_vertex_metadata", &l, api::get_vertex_metadata);
    proxy_fn!(globals, "set_vertex_metadata", &l, api::set_vertex_metadata);
    proxy_fn!(
        globals,
        "delete_vertex_metadata",
        &l,
        api::delete_vertex_metadata
    );
    proxy_fn!(globals, "get_edge_metadata", &l, api::get_edge_metadata);
    proxy_fn!(globals, "set_edge_metadata", &l, api::set_edge_metadata);
    proxy_fn!(
        globals,
        "delete_edge_metadata",
        &l,
        api::delete_edge_metadata
    );

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
    globals.set("trans", converters::ProxyTransaction::new(trans))?;

    // Add the account id as a global variable.
    globals.set("account_id", account_id.to_string())?;

    globals.set("arg", converters::JsonValue::new(arg))?;

    let value: converters::JsonValue = fun.call(Value::Nil)?;
    Ok(value.0)
}
