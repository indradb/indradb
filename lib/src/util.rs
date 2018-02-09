//! Utility functions.

use rand::{OsRng, Rng};
use errors::ValidationError;
use uuid::{Uuid, UuidV1Context};
use chrono::DateTime;
use chrono::offset::Utc;

const NODE_ID: [u8; 6] = [0, 0, 0, 0, 0, 0];

lazy_static! {
    static ref CONTEXT: UuidV1Context = UuidV1Context::new(0);
}

#[derive(Debug)]
pub enum UuidGenerator {
    V1,
    V4
}

impl UuidGenerator {
    pub fn new(secure: bool) -> Self {
        if secure {
            UuidGenerator::V4
        } else {
            UuidGenerator::V1
        }
    }

    pub fn next(&self) -> Uuid {
        match self {
            &UuidGenerator::V1 => {
                let now = Utc::now();

                Uuid::new_v1(
                    &CONTEXT,
                    now.timestamp() as u64,
                    now.timestamp_subsec_nanos(),
                    &NODE_ID,
                ).expect("Expected to be able to generate a UUID")
            }
            &UuidGenerator::V4 => Uuid::new_v4(),
        }
    }
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
pub fn next_uuid(uuid: Uuid) -> Result<Uuid, ValidationError> {
    let mut bytes = *uuid.as_bytes();

    for i in (0..16).rev() {
        if bytes[i] < 255 {
            bytes[i] += 1;
            return Ok(Uuid::from_bytes(&bytes[..]).unwrap());
        } else {
            bytes[i] = 0;
        }
    }

    Err(ValidationError::new(
        "Could not increment the UUID".to_string(),
    ))
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

#[cfg(test)]
mod tests {
    use super::{generate_random_secret, nanos_since_epoch, next_uuid, UuidGenerator};
    use regex::Regex;
    use uuid::Uuid;
    use core::str::FromStr;
    use chrono::{DateTime, NaiveDateTime, Utc};

    #[test]
    fn should_generate_uuids() {
        let generator = UuidGenerator::new(false);
        let first = generator.next();
        let second = generator.next();
        assert_ne!(first, second);
        let generator = UuidGenerator::new(true);
        let first = generator.next();
        let second = generator.next();
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
