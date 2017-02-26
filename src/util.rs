use rand::{Rng, OsRng};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use errors::ValidationError;
use uuid::Uuid;

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
    let mut bytes = uuid.as_bytes().clone();

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
