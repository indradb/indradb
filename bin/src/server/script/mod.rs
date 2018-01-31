mod api;
mod context;
mod converters;
mod errors;
mod mapreduce;

#[cfg(test)]
mod tests;

use serde_json::value::Value as JsonValue;
use uuid::Uuid;

pub use self::mapreduce::MapReducer;

/// Runs a script.
///
/// # Errors
/// Returns an error if the script produced an error.
///
/// # Panics
/// We try to avoid panics, but there is a lot of unsafe code here.
pub fn execute(
    account_id: Uuid,
    contents: String,
    path: String,
    arg: JsonValue,
) -> Result<JsonValue, errors::ScriptError> {
    let l = context::create(account_id, arg)?;
    let value: converters::JsonValue = l.exec(&contents, Some(&path))?;
    Ok(value.0)
}
