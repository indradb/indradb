//! Utility functions.

use chrono::offset::Utc;
use chrono::DateTime;
use errors::ValidationResult;
use rand::{OsRng, Rng};
use std::env;
use uuid::v1::Context;
use uuid::Uuid;
use serde_json::Value as JsonValue;
use std::hash::{Hash, Hasher};

const TEMP_PATH_RANDOM_PART_LENGTH: usize = 8;
const NODE_ID: [u8; 6] = [0, 0, 0, 0, 0, 0];

lazy_static! {
    static ref CONTEXT: Context = Context::new(0);
}

/// Gets the path to a file or directory within the temporary directory, in a
/// platform-independent manner. Note that this will panic if the path cannot
/// be formatted into UTF-8.
pub fn generate_temporary_path() -> String {
    let mut path = env::temp_dir();
    path.push(generate_random_secret(TEMP_PATH_RANDOM_PART_LENGTH));
    path.to_str()
        .expect("Expected to be able to parse the temp path into UTF-8")
        .to_string()
}

/// Generates a UUID v1. this utility method uses a shared context and node ID
/// to help ensure generated UUIDs are unique.
pub fn generate_uuid_v1() -> Uuid {
    let now = Utc::now();

    Uuid::new_v1(
        &*CONTEXT,
        now.timestamp() as u64,
        now.timestamp_subsec_nanos(),
        &NODE_ID,
    )
    .expect("Expected to be able to generate a UUID")
}

/// Generates a securely random string consisting of letters (uppercase and
/// lowercase) and digits.
pub fn generate_random_secret(count: usize) -> String {
    let mut chars = vec![];
    let options = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = OsRng::new().unwrap();

    for _ in 0..count {
        let c: u8 = *rng.choose(options).unwrap();
        chars.push(c);
    }

    String::from_utf8(chars).unwrap()
}

/// Gets the next UUID that would occur after the given one.
///
/// # Arguments
///
/// * `uuid` - The input UUID.
///
/// # Errors
/// Returns a `ValidationError` if the input UUID is the great possible value
/// (i.e., FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF)
pub fn next_uuid(uuid: Uuid) -> ValidationResult<Uuid> {
    let mut bytes = *uuid.as_bytes();

    for i in (0..16).rev() {
        if bytes[i] < 255 {
            bytes[i] += 1;
            return Ok(Uuid::from_slice(&bytes[..]).unwrap());
        } else {
            bytes[i] = 0;
        }
    }

    Err("Could not increment the UUID".into())
}

/// Gets the number of nanoseconds since unix epoch for a given datetime.
///
/// # Arguments
/// * `datetime` - The datetime to convert.
pub fn nanos_since_epoch(datetime: &DateTime<Utc>) -> u64 {
    let timestamp = datetime.timestamp() as u64;
    let nanoseconds = u64::from(datetime.timestamp_subsec_nanos());
    timestamp * 1_000_000_000 + nanoseconds
}

/// Represents JSON values that need to be hashable but are not by default
#[derive(Hash)]
enum SpecialJsonValue {
    Null,
    NaN,
    ArrayPrelude,
    MapPrelude
}

fn hash_json<H: Hasher>(state: &mut H, value: &JsonValue) {
    match value {
        JsonValue::Null => SpecialJsonValue::Null.hash(state),
        JsonValue::Bool(value) => value.hash(state),
        JsonValue::Number(value) => {
            if let Some(value) = value.as_u64() {
                value.hash(state);
            } else if let Some(value) = value.as_i64() {
                value.hash(state);
            } else if let Some(value) = value.as_f64() {
                if value.is_nan() {
                    SpecialJsonValue::NaN.hash(state);
                } else {
                    // Convert the float to a u64 so it is hashable. This
                    // should be safe since we handle `NaN` separately.
                    // See https://doc.rust-lang.org/std/primitive.f64.html#method.from_bits
                    value.to_bits().hash(state);
                }
            } else {
                unreachable!();
            }
        },
        JsonValue::String(ref value) => value.hash(state),
        JsonValue::Array(ref value) => {
            SpecialJsonValue::ArrayPrelude.hash(state);

            for elem in value {
                hash_json(state, &elem);
            }
        },
        JsonValue::Object(ref value) => {
            SpecialJsonValue::MapPrelude.hash(state);

            for (key, value) in value {
                key.hash(state);
                hash_json(state, &value);
            }
        },
    }
}

#[derive(Debug)]
pub(crate) struct InternableJsonValue(pub JsonValue);

impl InternableJsonValue {
    pub fn new(value: JsonValue) -> Self {
        Self { 0: value }
    }
}

impl PartialEq for InternableJsonValue {
    fn eq(&self, other: &InternableJsonValue) -> bool {
        match (&self.0, &other.0) {
            (JsonValue::Null, JsonValue::Null) => true,
            (JsonValue::Bool(first), JsonValue::Bool(second)) => first == second,
            (JsonValue::Number(first), JsonValue::Number(second)) => {
                if let (Some(first), Some(second)) = (first.as_u64(), second.as_u64()) {
                    first == second
                } else if let (Some(first), Some(second)) = (first.as_i64(), second.as_i64()) {
                    first == second
                } else if let (Some(first), Some(second)) = (first.as_f64(), second.as_f64()) {
                    // For the purposes of how we're using equality checking
                    // (i.e., for internment), this is fine
                    first.to_bits() == second.to_bits()
                } else {
                    false
                }
            },
            (JsonValue::String(first), JsonValue::String(second)) => first == second,
            (JsonValue::Array(first), JsonValue::Array(second)) => first == second,
            (JsonValue::Object(first), JsonValue::Object(second)) => first == second,
            _ => false
        }
    }
}

impl Eq for InternableJsonValue {}

impl Hash for InternableJsonValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_json(state, &self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::{generate_random_secret, generate_temporary_path, generate_uuid_v1, nanos_since_epoch, next_uuid};
    use chrono::{DateTime, NaiveDateTime, Utc};
    use core::str::FromStr;
    use regex::Regex;
    use uuid::Uuid;

    #[test]
    fn should_generate_temporary_path() {
        let first = generate_temporary_path();
        let second = generate_temporary_path();
        assert!(first.len() > 0);
        assert!(second.len() > 0);
        assert_ne!(first, second);
    }

    #[test]
    fn should_generate_new_uuid_v1() {
        let first = generate_uuid_v1();
        let second = generate_uuid_v1();
        assert_ne!(first, second);
    }

    #[test]
    fn should_generate_random_secret() {
        let secret = generate_random_secret(8);
        assert!(Regex::new(r"[a-zA-Z0-9]{8}").unwrap().is_match(&secret[..]));
    }

    #[test]
    fn should_generate_next_uuid() {
        let result = next_uuid(Uuid::from_str("16151dea-a538-4bf1-9559-851e256cf139").unwrap());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Uuid::from_str("16151dea-a538-4bf1-9559-851e256cf13a").unwrap()
        );

        let from_uuid = Uuid::from_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        assert!(next_uuid(from_uuid).is_err());
    }

    #[test]
    fn should_generate_nanos_since_epoch() {
        let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 62), Utc);
        assert_eq!(nanos_since_epoch(&datetime), 61000000062);
    }
}
