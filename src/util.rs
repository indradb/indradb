use std::error::Error as StdError;
use std::fmt;
use std::collections::BTreeMap;
use serde_json::Value as JsonValue;
use serde_json;
use rand::{Rng, OsRng};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Error {
	AccountNotFound,
	VertexDoesNotExist,
	EdgeDoesNotExist,
	LimitOutOfRange,
	OffsetOutOfRange,
	WeightOutOfRange,
	MetadataDoesNotExist,
	Unexpected(String),
}

impl Error {
	pub fn description_to_error(message: &str) -> Self {
		match &message[..] {
	        "Account not found" => Error::AccountNotFound,
	        "Vertex does not exist" => Error::VertexDoesNotExist,
	        "Edge does not exist" => Error::EdgeDoesNotExist,
	        "Metadata does not exist" => Error::MetadataDoesNotExist,
	        "Weight out of range" => Error::WeightOutOfRange,
	        "Limit out of range" => Error::LimitOutOfRange,
	        "Offset out of range" => Error::OffsetOutOfRange,
	        _ => Error::Unexpected(format!("Unexpected error message: {}", message))
	    }
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::AccountNotFound => "Account not found",
			Error::VertexDoesNotExist => "Vertex does not exist",
			Error::EdgeDoesNotExist => "Edge does not exist",
			Error::LimitOutOfRange => "Limit out of range",
			Error::OffsetOutOfRange => "Offset out of range",
			Error::WeightOutOfRange => "Weight out of range",
			Error::MetadataDoesNotExist => "Metadata does not exist",
			Error::Unexpected(_) => "Unexpected error"
		}
	}

	fn cause(&self) -> Option<&StdError> {
		None
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Unexpected(ref msg) => write!(f, "{}", msg),
			_ => write!(f, "{}", self.description())
		}
	}
}

pub fn parse_json_object(s: String) -> Result<BTreeMap<String, JsonValue>, Error> {
	let serialized = serde_json::from_str(&s[..]);

	if serialized.is_ok() {
		let json: JsonValue = serialized.unwrap();
		let obj = json.as_object();

		if obj.is_some() {
			return Ok(obj.unwrap().clone())
		}
	}

	Err(Error::Unexpected("Did not get a JSON object back".to_string()))
}

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
