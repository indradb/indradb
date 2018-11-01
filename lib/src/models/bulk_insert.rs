use serde_json::Value as JsonValue;

/// Represents an item and its properties to be bulk inserted.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BulkInsertItem<T> {
    pub value: T,
    pub properties: Vec<BulkInsertProperty>
}

impl<T> BulkInsertItem<T> {
    /// Creates a new bulk insert item/
    ///
    /// # Arguments
    /// * `value`: The underlying value.
    /// * `properties`: The properties to set on the item.
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
