use std::convert::TryFrom;
use std::str::FromStr;

use crate::errors::{ValidationError, ValidationResult};

use serde::{Deserialize, Serialize};

/// A string that must be less than 256 characters long, and can only contain
/// letters, numbers, dashes and underscores. This is used for vertex and edge
/// types, as well as property names.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Identifier(pub(crate) String);

impl Identifier {
    /// Constructs a new identifier.
    ///
    /// # Arguments
    /// * `s`: The identifier value.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the identifier is longer than 255
    /// characters, or has invalid characters.
    pub fn new<S: Into<String>>(s: S) -> ValidationResult<Self> {
        let s = s.into();

        if s.len() > 255 {
            Err(ValidationError::ValueTooLong)
        } else if !s.chars().all(|c| c == '-' || c == '_' || c.is_alphanumeric()) {
            Err(ValidationError::InvalidValue)
        } else {
            Ok(Identifier(s))
        }
    }

    /// Constructs a new identifier, without any checks that it is valid.
    ///
    /// # Arguments
    /// * `s`: The identifier value.
    ///
    /// # Safety
    /// This function is marked unsafe because there's no verification that
    /// the identifier is valid.
    pub unsafe fn new_unchecked<S: Into<String>>(s: S) -> Self {
        Identifier(s.into())
    }

    /// Gets a reference to the identifier value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the identifier into a string with the underlying value.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self("".to_string())
    }
}

impl FromStr for Identifier {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl TryFrom<String> for Identifier {
    type Error = ValidationError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Identifier;
    use std::str::FromStr;

    #[test]
    fn should_create() {
        assert_eq!(Identifier::new("foo").unwrap().as_str(), "foo");
        let long_t = (0..256).map(|_| "X").collect::<String>();
        assert!(Identifier::new(long_t).is_err());
        assert!(Identifier::new("$").is_err());
    }

    #[test]
    fn should_create_unchecked() {
        unsafe {
            assert_eq!(Identifier::new_unchecked("foo").as_str(), "foo");
            assert_eq!(Identifier::new_unchecked("$").as_str(), "$");
        }
    }

    #[test]
    fn should_try_from_str() {
        assert_eq!(Identifier::try_from("foo".to_string()).unwrap().as_str(), "foo");
        let long_t = (0..256).map(|_| "X").collect::<String>();
        assert!(Identifier::try_from(long_t).is_err());
        assert!(Identifier::try_from("$".to_string()).is_err());
    }

    #[test]
    fn should_convert_between_identifier_and_string() {
        let id = Identifier::new("foo").unwrap();
        assert_eq!(Identifier::from_str("foo").unwrap(), id);
        assert_eq!(id.as_str(), "foo");
        assert_eq!(id.to_string(), "foo".to_string());
        assert_eq!(id.into_string(), "foo".to_string());
    }
}
