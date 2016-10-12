use std::error::Error;
use std::fmt;

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
