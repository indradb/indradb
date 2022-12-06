use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;

use crate::errors::{ValidationError, ValidationResult};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as SerdeDeserializeError;

/// A string that must be less than 256 characters long, and can only contain
/// letters, numbers, dashes and underscores. This is used for vertex and edge
/// types, as well as property names.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Ord, PartialOrd)]
pub struct Identifier(pub(crate) Arc<String>);

impl Identifier {
    /// Constructs a new identifier.
    ///
    /// # Arguments
    /// * `s`: The identifier value.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the identifier is longer than 255
    /// characters, or has invalid characters.
    pub fn new<S: Into<Arc<String>>>(s: S) -> ValidationResult<Self> {
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
    pub unsafe fn new_unchecked<S: Into<Arc<String>>>(s: S) -> Self {
        Identifier(s.into())
    }

    /// Gets a reference to the identifier value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self(Arc::new("".to_string()))
    }
}

impl FromStr for Identifier {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(Arc::new(s.to_string()))
    }
}

impl TryFrom<String> for Identifier {
    type Error = ValidationError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(Arc::new(s))
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Identifier, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Identifier::new(s).map_err(|err| SerdeDeserializeError::custom(format!("{}", err)))
    }
}

#[cfg(test)]
mod tests {
    use super::Identifier;
    use std::str::FromStr;

    #[test]
    fn should_fail_for_invalid_identifiers() {
        let long_t = (0..256).map(|_| "X").collect::<String>();
        assert!(Identifier::new(long_t).is_err());
        assert!(Identifier::new("$").is_err());
    }

    #[test]
    fn should_convert_str_to_identifier() {
        assert_eq!(Identifier::from_str("foo").unwrap(), Identifier::new("foo").unwrap());
    }
}
