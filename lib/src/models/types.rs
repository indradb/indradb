use crate::errors::{ValidationError, ValidationResult};
use core::str::FromStr;
use lazy_static::lazy_static;
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
            Err(ValidationError::ValueTooLong)
        } else if !TYPE_VALIDATOR.is_match(&s[..]) {
            Err(ValidationError::InvalidValue)
        } else {
            Ok(Type(s))
        }
    }

    /// Constructs a new type, without any checks that the name is valid.
    ///
    /// # Arguments
    ///
    /// * `t` - The type, which must be less than 256 characters long.
    ///
    /// # Safety
    /// This function is marked unsafe because there's no verification that
    /// the type name is valid.
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
        Self::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Type;
    use std::str::FromStr;

    #[test]
    fn should_fail_for_invalid_types() {
        let long_t = (0..256).map(|_| "X").collect::<String>();
        assert!(Type::new(long_t).is_err());
        assert!(Type::new("$").is_err());
    }

    #[test]
    fn should_convert_str_to_type() {
        assert_eq!(Type::from_str("foo").unwrap(), Type::new("foo").unwrap());
    }
}
