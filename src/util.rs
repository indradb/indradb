use std::error::Error;
use std::fmt;
use std::collections::BTreeMap;
use serde_json::Value as JsonValue;
use serde_json;
use super::responses::ErrorResponse;
use rand::{Rng, OsRng};

#[derive(Debug)]
pub struct SimpleError {
	description: String
}

impl SimpleError {
	pub fn new(description: &str) -> SimpleError {
		SimpleError {
			description: description.to_string()
		}
	}

	pub fn new_from_string(description: String) -> SimpleError {
		SimpleError {
			description: description
		}
	}
}

impl Error for SimpleError {
	fn description(&self) -> &str {
		&self.description[..]
	}

	fn cause(&self) -> Option<&Error> {
		None
	}
}

impl fmt::Display for SimpleError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.description)
	}
}

pub fn parse_json_object(s: String) -> Result<BTreeMap<String, JsonValue>, ErrorResponse> {
	let serialized = serde_json::from_str(&s[..]);

	if serialized.is_ok() {
		let json: JsonValue = serialized.unwrap();
		let obj = json.as_object();

		if obj.is_some() {
			return Ok(obj.unwrap().clone())
		}
	}

	Err(ErrorResponse::Unexpected("Did not get a JSON object back".to_string()))
}

pub fn generate_random_secret() -> String {
    let mut chars = vec![];
	let options = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let mut rng = OsRng::new().unwrap();

    for _ in 0..32 {
		let c: u8 = *rng.choose(options).unwrap();
        chars.push(c);
    }

    String::from_utf8(chars).unwrap()
}
