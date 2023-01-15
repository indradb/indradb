//! Demonstrates a basic plugin.

use std::sync::Arc;

use indradb_plugin_host as plugin;

pub struct HelloWorldPlugin {}

impl plugin::Plugin for HelloWorldPlugin {
    fn call(
        &self,
        _txn: plugin::DynTransaction<'_>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, plugin::Error> {
        let greeting = format!("hello, {}", arg);
        Ok(serde_json::Value::String(greeting))
    }
}

plugin::register_plugins!(1, "hello_world", || Box::new(crate::HelloWorldPlugin {}));
