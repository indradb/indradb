pub struct HelloWorldPlugin {}

impl indradb::plugin::Plugin for HelloWorldPlugin {
    fn call(
        &self,
        _datastore: Box<dyn indradb::Transaction>,
        _arg: serde_json::Value,
    ) -> Result<serde_json::Value, indradb::Error> {
        println!("hello world!");
        Ok(serde_json::Value::Null)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

indradb::register_plugins!("hello_world", Box::new(crate::HelloWorldPlugin {}));
