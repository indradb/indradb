use core::str::FromStr;
use crate::errors::{ValidationError, ValidationResult};
use regex::Regex;

lazy_static! {
    static ref TYPE_VALIDATOR: Regex = Regex::new("^[a-zA-Z0-9-_]+$").unwrap();
}

/// An edge or vertex type.
///
/// Types must be less than 256 characters long, and can only contain letters,
/// numbers, dashes and underscores.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Ord, PartialOrd)]
pub struct Type(pub String);

impl Type {
    /// Constructs a new type.
    ///
    /// # Arguments
    ///
    /// * `t` - The type, which must be less than 256 characters long.
    ///
    /// # Errors
    /// Returns a `ValidationError` if the type is longer than 255 characters,
    /// or has invalid characters.
    pub fn new<S: Into<String>>(s: S) -> ValidationResult<Self> {
        let s = s.into();

        if s.len() > 255 {
            Err("Type is too long".into())
        } else if !TYPE_VALIDATOR.is_match(&s[..]) {
            Err("Invalid type".into())
        } else {
            Ok(Type(s))
        }
    }

    pub unsafe fn new_unchecked<S: Into<String>>(s: S) -> Self {
        Type(s.into())
    }
}

impl Default for Type {
    fn default() -> Self {
        Self { 0: "".to_string() }
    }
}

impl FromStr for Type {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string())?)
    }
}

#[cfg(test)]
mod tests {
    use super::Type;
    use std::str::FromStr;
    use crate::util::generate_random_secret;

    #[test]
    fn should_fail_for_invalid_types() {
        assert!(Type::new(generate_random_secret(256)).is_err());
        assert!(Type::new("$").is_err());
    }

    #[test]
    fn should_convert_str_to_type() {
        assert_eq!(Type::from_str("foo").unwrap(), Type::new("foo").unwrap());
    }
}
