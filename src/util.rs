use std::error::Error as StdError;
use std::fmt;
use rand::{Rng, OsRng};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Error {
	AccountNotFound,
	VertexNotFound,
	EdgeNotFound,
	MetadataNotFound,
	OutOfRange(String),
	Unexpected(String),
}

impl Error {
	pub fn description_to_error(message: &str) -> Self {
		match &message[..] {
	        "Account not found" => Error::AccountNotFound,
	        "Vertex does not exist" => Error::VertexNotFound,
	        "Edge does not exist" => Error::EdgeNotFound,
	        "Metadata does not exist" => Error::MetadataNotFound,
	        _ => {
				if message.starts_with("Value out of range: ") {
					Error::OutOfRange(message[20..message.len()].to_string())
				} else {
					Error::Unexpected(message.to_string())
				}
			}
	    }
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		match *self {
			Error::AccountNotFound => "Account not found",
			Error::VertexNotFound => "Vertex does not exist",
			Error::EdgeNotFound => "Edge does not exist",
			Error::MetadataNotFound => "Metadata does not exist",
			Error::OutOfRange(_) => "Value out of range",
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
			Error::OutOfRange(ref name) => write!(f, "Value out of range: {}", name), 
			_ => write!(f, "{}", self.description())
		}
	}
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

pub fn get_salted_hash(salt: String, pepper: Option<String>, secret: String) -> String {
	let mut sha = Sha256::new();
	sha.input(salt.as_bytes());

	if pepper.is_some() {
		sha.input(pepper.unwrap().as_bytes());
	}

	sha.input(secret.as_bytes());
	return format!("1:{}", sha.result_str());
}
