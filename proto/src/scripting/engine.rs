use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;

use super::{converters, globals};

use indradb::Datastore;
use rlua::prelude::*;
use rlua::Table;
use serde_json::value::Value as JsonValue;
use tokio::sync::mpsc;
use tonic::Status;

#[derive(Clone)]
pub struct Engine<
    D: indradb::Datastore<Trans = T> + Send + Sync + 'static,
    T: indradb::Transaction + Send + Sync + 'static,
> {
    datastore: Arc<D>,
    lua: Arc<Lua>,
}

impl<D: indradb::Datastore<Trans = T> + Send + Sync + 'static, T: indradb::Transaction + Send + Sync + 'static> Engine<D, T> {
    pub fn new(datastore: Arc<D>, script_root: Option<&OsStr>) -> Result<Self, LuaError> {
        let lua = Lua::new();

        if let Some(script_root) = script_root {
            // Update the `package.path` to include the script root, so it's
            // easier for scripts to require each other.
            lua.context(|ctx| -> Result<(), LuaError> {
                let package: Table = ctx.globals().get("package")?;
                let old_path: String = package.get("path")?;
                let script_path = Path::new(script_root)
                    .join("?.lua")
                    .to_str()
                    .unwrap()
                    .to_string();
                package.set("path", format!("{};{}", old_path, script_path));
                Ok(())
            })?;
        }

        lua.context(|ctx| -> Result<(), LuaError> {
            ctx.load(globals::GLOBALS).set_name("globals.lua")?.exec()
        })?;

        Ok(Self {
            datastore,
            lua: Arc::new(lua),
        })
    }

    // TODO: add support for simple run/return
    pub fn exec(&self, source: &str, arg: JsonValue, tx: mpsc::Sender<Result<crate::EvalScriptResponse, Status>>) {
        let res = self.lua.context(|ctx| -> Result<(), LuaError> {
            ctx.scope(|scope| {
                let globals = ctx.globals();

                globals.set("arg", converters::JsonValue::new(arg))?;

                globals.set(
                    "transaction",
                    scope.create_function_mut(|_, ()| {
                        let trans = self.datastore.transaction().map_err(|err| LuaError::RuntimeError(format!("{}", err)))?;
                        Ok(converters::Transaction::new(trans))
                    })?,
                )?;

                globals.set("respond", scope.create_function_mut(|_, ()| {
                    let res = crate::EvalScriptResponse {
                        // TODO: set output
                        output: Some(crate::Json { value: "null".to_string() }),
                    };
                    if let Err(err) = tx.try_send(Ok(res)) {
                        eprintln!("could not send message to client: {}", err);
                    }
                    Ok(())
                })?)?;

                let value: converters::JsonValue = ctx.load(source).eval()?;
                // TODO: send value over tx

                ctx.load(source).exec()
            })?;

            Ok(())
        });

        if let Err(err) = res {
            if let Err(err) = tx.try_send(Err(Status::unknown(format!("scripting error: {}", err)))) {
                eprintln!("could not send message to client: {}", err);
            }
        }
    }
}

