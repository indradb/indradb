#[macro_use]
mod macros;
mod api;
mod errors;
mod util;

use lua;
use libc;
use serde_json::value::Value as JsonValue;
use common::ProxyTransaction;
use std::path::Path;
use uuid::Uuid;
pub use self::errors::ScriptError;
use statics;

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
    path: &Path,
    arg: &JsonValue,
) -> Result<JsonValue, ScriptError> {
    let mut l = lua::State::new();
    l.openlibs();

    l.register("create_vertex", api::create_vertex);
    l.register("get_vertices", api::get_vertices);
    l.register("delete_vertices", api::delete_vertices);

    l.register("create_edge", api::create_edge);
    l.register("get_edges", api::get_edges);
    l.register("delete_edges", api::delete_edges);
    l.register("get_edge_count", api::get_edge_count);

    l.register("get_global_metadata", api::get_global_metadata);
    l.register("set_global_metadata", api::set_global_metadata);
    l.register("delete_global_metadata", api::delete_global_metadata);
    l.register("get_account_metadata", api::get_account_metadata);
    l.register("set_account_metadata", api::set_account_metadata);
    l.register("delete_account_metadata", api::delete_account_metadata);
    l.register("get_vertex_metadata", api::get_vertex_metadata);
    l.register("set_vertex_metadata", api::set_vertex_metadata);
    l.register("delete_vertex_metadata", api::delete_vertex_metadata);
    l.register("get_edge_metadata", api::get_edge_metadata);
    l.register("set_edge_metadata", api::set_edge_metadata);
    l.register("delete_edge_metadata", api::delete_edge_metadata);

    if let Err(err) = l.loadfile(Some(path)) {
        return Err(ScriptError::new_from_load_file_error(&mut l, err));
    }

    // Update the `package.path` to include the script root, so it's easier
    // for scripts to require each other.
    {
        l.getglobal("package");
        l.getfield(-1, "path");
        let old_path = l.checkstring(-1).unwrap().to_string();
        let script_path = Path::new(&statics::SCRIPT_ROOT[..])
            .join("?.lua")
            .to_str()
            .unwrap()
            .to_string();
        let new_path = format!("{};{}", old_path, script_path);
        l.pop(1);
        l.pushstring(&new_path[..]);
        l.setfield(-2, "path");
        l.pop(1);
    }

    // Add the transaction as a global variable.
    {
        let trans_ptr: *mut libc::c_void = &mut trans as *mut _ as *mut libc::c_void;
        l.pushlightuserdata(trans_ptr);
        l.setglobal("trans");
    }

    // Add the account id as a global variable.
    {
        l.pushstring(&account_id.to_string()[..]);
        l.setglobal("account_id");
    }

    // Add the input arg as a global variable.
    {
        unsafe {
            util::serialize_json(l.as_extern(), &arg);
        }
        l.setglobal("arg");
    }

    if let Err(err) = l.pcall(0, lua::MULTRET, 0) {
        return Err(ScriptError::new_from_pcallerror(&mut l, err));
    }

    if l.gettop() == 0 {
        Ok(JsonValue::Null)
    } else {
        let payload = unsafe { util::deserialize_json(l.as_extern(), -1)? };
        Ok(payload)
    }
}
