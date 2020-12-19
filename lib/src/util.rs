//! Utility functions.

use crate::errors::Result;
use crate::errors::{ValidationError, ValidationResult};
use chrono::offset::Utc;
use chrono::DateTime;
use rand::prelude::*;
use rand::rngs::OsRng;
use std::env;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

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
    let ts = Timestamp::from_unix(&*CONTEXT, now.timestamp() as u64, now.timestamp_subsec_nanos());
    Uuid::new_v1(ts, &NODE_ID).expect("Expected to be able to generate a UUID")
}

/// Generates a securely random string consisting of letters (uppercase and
/// lowercase) and digits.
pub fn generate_random_secret(count: usize) -> String {
    let mut chars = vec![];
    let options = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for _ in 0..count {
        // Don't use `choose_multiple`, because it shuffles the list (i.e.
        // prevents duplicates)
        let c: u8 = *options.choose(&mut OsRng).unwrap();
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

    Err(ValidationError::CannotIncrementUuid)
}

// TODO: remove
/// Gets the number of nanoseconds since unix epoch for a given datetime.
///
/// # Arguments
/// * `datetime` - The datetime to convert.
pub fn nanos_since_epoch(datetime: &DateTime<Utc>) -> u64 {
    let timestamp = datetime.timestamp() as u64;
    let nanoseconds = u64::from(datetime.timestamp_subsec_nanos());
    timestamp * 1_000_000_000 + nanoseconds
}

pub fn remove_nones_from_iterator<I, T>(iter: I) -> impl Iterator<Item = Result<T>>
where
    I: Iterator<Item = Result<Option<T>>>,
{
    iter.filter_map(|item| match item {
        Err(err) => Some(Err(err)),
        Ok(Some(value)) => Some(Ok(value)),
        _ => None,
    })
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
        let secret = generate_random_secret(62);
        assert!(Regex::new(r"[a-zA-Z0-9]{62}").unwrap().is_match(&secret[..]));
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
