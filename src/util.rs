use rand::{Rng, OsRng};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use errors::ValidationError;
use uuid::Uuid;
use chrono::{DateTime, UTC};
use byteorder::BigEndian;
use std::io::Cursor;
use chrono::Timelike;
use std::io::Write;
use byteorder::WriteBytesExt;

/// Generates a securely random string consisting of letters (uppercase and
/// lowercase) and digits.
pub fn generate_random_secret() -> String {
    let mut chars = vec![];
    let options = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = OsRng::new().unwrap();

    for _ in 0..32 {
        let c: u8 = *rng.choose(options).unwrap();
        chars.push(c);
    }

    String::from_utf8(chars).unwrap()
}

/// Generates a SHA256 hash, based off of a salt, an optional pepper, and a
/// secret value.
pub fn get_salted_hash(salt: &str, pepper: Option<&str>, secret: &str) -> String {
    let mut sha = Sha256::new();
    sha.input(salt.as_bytes());

    if let Some(pepper) = pepper {
        sha.input(pepper.as_bytes());
    }

    sha.input(secret.as_bytes());
    return format!("1:{}", sha.result_str());
}

/// Gets the next UUID that would occur after the given one
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

    Err(ValidationError::new("Could not increment the UUID".to_string()))
}

/// Gets the number of nanoseconds since unix epoch for a given datetime
pub fn nanos_since_epoch(datetime: &DateTime<UTC>) -> u64 {
    let timestamp: u64 = datetime.timestamp() as u64;
    let nanoseconds: u64 = datetime.timestamp_subsec_nanos() as u64;
    timestamp * 1000000000 + nanoseconds
}

/// Returns a new UUID.
pub fn parent_uuid() -> Uuid {
    loop {
        let id = Uuid::new_v4();

        if id != Uuid::default() {
            return id;
        }
    }
}

/// Creates a new ID that is based in part off a parent ID.
///
/// # Arguments
/// 
/// * `parent` - The ID of the parent.
pub fn child_uuid(parent: Uuid) -> Uuid {
    let now = UTC::now();
    let mut buf = [0u8; 16];
    let mut cursor: Cursor<&mut [u8]> = Cursor::new(&mut buf);
    cursor.write(&parent.as_bytes()[12..]).unwrap();
    cursor.write_i64::<BigEndian>(now.timestamp()).unwrap();
    cursor.write_u32::<BigEndian>(now.nanosecond()).unwrap();
    Uuid::from_bytes(&cursor.into_inner()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{generate_random_secret, get_salted_hash, next_uuid, nanos_since_epoch, parent_uuid, child_uuid};
    use regex::Regex;
    use uuid::Uuid;
    use core::str::FromStr;
    use chrono::{DateTime, NaiveDateTime, UTC};

    #[test]
    fn should_generate_random_secret() {
        let secret = generate_random_secret();
        assert!(Regex::new(r"[a-zA-Z0-9]{32}").unwrap().is_match(&secret[..]));
    }

    #[test]
    fn should_generate_salted_hash() {
        let hash = get_salted_hash("a", Some("b"), "c");
        assert_eq!(hash, "1:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        let hash = get_salted_hash("a", None, "c");
        assert_eq!(hash, "1:f45de51cdef30991551e41e882dd7b5404799648a0a00753f44fc966e6153fc1");
    }

    #[test]
    fn should_generate_next_uuid() {
        let result = next_uuid(Uuid::from_str("16151dea-a538-4bf1-9559-851e256cf139").unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Uuid::from_str("16151dea-a538-4bf1-9559-851e256cf13a").unwrap());

        let from_uuid = Uuid::from_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        assert!(next_uuid(from_uuid).is_err());
    }

    #[test]
    fn should_generate_nanos_since_epoch() {
        let datetime = DateTime::<UTC>::from_utc(NaiveDateTime::from_timestamp(61, 62), UTC);
        assert_eq!(nanos_since_epoch(&datetime), 61000000062);
    }

    #[test]
    fn should_generate_parent_uuid() {
        let uuid = parent_uuid();
        assert!(uuid != Uuid::default());
    }

    #[test]
    fn should_generate_child_uuid() {
        let pid = parent_uuid();
        let cid = child_uuid(pid);
        assert!(pid != cid);
        assert!(pid.as_bytes()[12..] == cid.as_bytes()[0..4]);
    }
}
