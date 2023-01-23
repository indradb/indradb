//! Demonstrates a basic plugin.

use indradb_plugin_host as plugin;

pub struct HelloWorldPlugin {}

impl plugin::Plugin for HelloWorldPlugin {
    fn call<'a>(
        &self,
        _txn: &mut (dyn indradb::Transaction<'a> + 'a),
        arg: indradb::Json,
    ) -> Result<indradb::Json, plugin::Error> {
        let greeting = format!("hello, {}", *arg);
        Ok(indradb::Json::new(serde_json::Value::String(greeting)))
    }
}

plugin::register_plugins!(1, "hello_world", || Box::new(crate::HelloWorldPlugin {}));
