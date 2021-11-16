//! Utility functions. These are public because they may be useful for crates
//! that implement Datastore/Transaction.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Error as IoError, Read, Write};
use std::{i32, i64, str, u8};

use crate::models;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::offset::Utc;
use chrono::{DateTime, Duration, NaiveDateTime, Timelike};
use lazy_static::lazy_static;

lazy_static! {
    /// The maximum possible datetime.
    pub static ref MAX_DATETIME: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp(i64::from(i32::MAX), 0), Utc)
            .with_nanosecond(1_999_999_999u32)
            .unwrap();
}

/// A byte-serializable value, frequently employed in the keys of key/value
/// store.
pub enum Component<'a> {
    U64(u64),
    FixedLengthString(&'a str),
    Identifier(&'a models::Identifier),
    DateTime(DateTime<Utc>),
    JsonValue(&'a models::JsonValue),
}

impl<'a> Component<'a> {
    // Really just implemented to not set off a clippy warning
    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn len(&self) -> usize {
        match *self {
            Component::U64(_) => 8,
            Component::FixedLengthString(s) => s.len(),
            Component::Identifier(t) => t.0.len() + 1,
            Component::DateTime(_) => 8,
            Component::JsonValue(_) => 8,
        }
    }

    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
        match *self {
            Component::U64(v) => cursor.write_u64::<BigEndian>(v),
            Component::FixedLengthString(s) => cursor.write_all(s.as_bytes()),
            Component::Identifier(i) => {
                cursor.write_all(&[i.0.len() as u8])?;
                cursor.write_all(i.0.as_bytes())
            }
            Component::DateTime(datetime) => {
                let time_to_end = nanos_since_epoch(&MAX_DATETIME) - nanos_since_epoch(&datetime);
                cursor.write_u64::<BigEndian>(time_to_end)
            }
            Component::JsonValue(json) => {
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
    // TODO: replace this w/ chrono's `timestamp_nanos`
    let timestamp = datetime.timestamp() as u64;
    let nanoseconds = u64::from(datetime.timestamp_subsec_nanos());
    timestamp * 1_000_000_000 + nanoseconds
}

/// Reads a u64 from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_u64<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> u64 {
    cursor.read_u64::<BigEndian>().unwrap()
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

#[cfg(test)]
mod tests {
    use super::nanos_since_epoch;
    use chrono::{DateTime, NaiveDateTime, Utc};

    #[test]
    fn should_generate_nanos_since_epoch() {
        let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 62), Utc);
        assert_eq!(nanos_since_epoch(&datetime), 61000000062);
    }
}
