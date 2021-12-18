//! Utility functions. These are public because they may be useful for crates
//! that implement Datastore/Transaction.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Error as IoError, Read, Write};
use std::{i32, i64, str, u8};

use crate::errors::{ValidationError, ValidationResult};
use crate::models;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::offset::Utc;
use chrono::{DateTime, Duration, NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

const NODE_ID: [u8; 6] = [0, 0, 0, 0, 0, 0];

lazy_static! {
    static ref CONTEXT: Context = Context::new(0);

    /// The maximum possible datetime.
    pub static ref MAX_DATETIME: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp(i64::from(i32::MAX), 0), Utc)
            .with_nanosecond(1_999_999_999u32)
            .unwrap();
}

/// A byte-serializable value, frequently employed in the keys of key/value
/// store.
pub enum Component<'a> {
    Uuid(Uuid),
    FixedLengthString(&'a str),
    Identifier(&'a models::Identifier),
    DateTime(DateTime<Utc>),
    Json(&'a models::Json),
}

impl<'a> Component<'a> {
    // Really just implemented to not set off a clippy warning
    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn len(&self) -> usize {
        match *self {
            Component::Uuid(_) => 16,
            Component::FixedLengthString(s) => s.len(),
            Component::Identifier(t) => t.0.len() + 1,
            Component::DateTime(_) => 8,
            Component::Json(_) => 8,
        }
    }

    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
        match *self {
            Component::Uuid(uuid) => cursor.write_all(uuid.as_bytes()),
            Component::FixedLengthString(s) => cursor.write_all(s.as_bytes()),
            Component::Identifier(i) => {
                cursor.write_all(&[i.0.len() as u8])?;
                cursor.write_all(i.0.as_bytes())
            }
            Component::DateTime(datetime) => {
                let time_to_end = nanos_since_epoch(&MAX_DATETIME) - nanos_since_epoch(&datetime);
                cursor.write_u64::<BigEndian>(time_to_end)
            }
            Component::Json(json) => {
                let mut hasher = DefaultHasher::new();
                json.hash(&mut hasher);
                let hash = hasher.finish();
                cursor.write_u64::<BigEndian>(hash)
            }
        }
    }
}

// Serializes component(s) into bytes.
///
/// # Arguments
/// * `components`: The components to serialize to bytes.
pub fn build(components: &[Component]) -> Vec<u8> {
    let len = components.iter().fold(0, |len, component| len + component.len());
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

    for component in components {
        if let Err(err) = component.write(&mut cursor) {
            panic!("Could not write bytes: {}", err);
        }
    }

    cursor.into_inner()
}

/// Gets the number of nanoseconds since unix epoch for a given datetime.
///
/// # Arguments
/// * `datetime`: The datetime to convert.
fn nanos_since_epoch(datetime: &DateTime<Utc>) -> u64 {
    let timestamp = datetime.timestamp() as u64;
    let nanoseconds = u64::from(datetime.timestamp_subsec_nanos());
    timestamp * 1_000_000_000 + nanoseconds
}

/// Reads a UUID from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_uuid<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> Uuid {
    let mut buf: [u8; 16] = [0; 16];
    cursor.read_exact(&mut buf).unwrap();
    Uuid::from_slice(&buf).unwrap()
}

/// Reads an identifier from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_identifier<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> models::Identifier {
    let t_len = {
        let mut buf: [u8; 1] = [0; 1];
        cursor.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };

    let mut buf = vec![0u8; t_len];
    cursor.read_exact(&mut buf).unwrap();

    unsafe {
        let s = str::from_utf8_unchecked(&buf).to_string();
        models::Identifier::new_unchecked(s)
    }
}

/// Reads a fixed-length string from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_fixed_length_string<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> String {
    let mut buf = String::new();
    cursor.read_to_string(&mut buf).unwrap();
    buf
}

/// Reads a datetime from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_datetime<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> DateTime<Utc> {
    let time_to_end = cursor.read_u64::<BigEndian>().unwrap();
    assert!(time_to_end <= i64::MAX as u64);
    *MAX_DATETIME - Duration::nanoseconds(time_to_end as i64)
}

pub fn read_u64<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> u64 {
    cursor.read_u64::<BigEndian>().unwrap()
}

/// Generates a UUID v1. This utility method uses a shared context and node ID
/// to help ensure generated UUIDs are unique.
pub fn generate_uuid_v1() -> Uuid {
    let now = Utc::now();
    let ts = Timestamp::from_unix(&*CONTEXT, now.timestamp() as u64, now.timestamp_subsec_nanos());
    Uuid::new_v1(ts, &NODE_ID).expect("Expected to be able to generate a UUID")
}

/// Gets the next UUID that would occur after the given one.
///
/// # Arguments
///
/// * `uuid`: The input UUID.
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

    Err(ValidationError::CannotIncrementUuid)
}

#[cfg(test)]
mod tests {
    use super::{generate_uuid_v1, nanos_since_epoch, next_uuid};
    use chrono::{DateTime, NaiveDateTime, Utc};
    use core::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn should_generate_nanos_since_epoch() {
        let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 62), Utc);
        assert_eq!(nanos_since_epoch(&datetime), 61000000062);
    }

    #[test]
    fn should_generate_new_uuid_v1() {
        let first = generate_uuid_v1();
        let second = generate_uuid_v1();
        assert_ne!(first, second);
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
}
