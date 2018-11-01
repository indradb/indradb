use serde_json::Value as JsonValue;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulkInsertItem<T> {
    pub value: T,
    pub properties: Vec<BulkInsertProperty>
}

impl<T> BulkInsertItem<T> {
    pub fn new(value: T, properties: Vec<BulkInsertProperty>) -> Self {
        Self { value, properties }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulkInsertProperty {
    pub name: String,
    pub value: JsonValue
}

impl BulkInsertProperty {
    pub fn new(name: String, value: JsonValue) -> Self {
        Self { name, value }
    }
}
