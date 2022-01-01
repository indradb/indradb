use indradb_plugin_host as plugin;
use std::sync::Arc;

pub struct HelloWorldPlugin {}

impl plugin::Plugin for HelloWorldPlugin {
    fn call(
        &self,
        _datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, plugin::Error> {
        let greeting = format!("hello, {}", arg);
        Ok(serde_json::Value::String(greeting))
    }
}

plugin::register_plugins!(0, "hello_world", Box::new(crate::HelloWorldPlugin {}));
