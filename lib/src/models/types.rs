use regex::Regex;
use errors::ValidationError;
use core::str::FromStr;

lazy_static! {
    static ref TYPE_VALIDATOR: Regex = Regex::new("^[a-zA-Z0-9-_]+$").unwrap();
}

/// An edge or vertex type.
///
/// Types must be less than 256 characters long, and can only contain letters,
/// numbers, dashes and underscores.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Ord, PartialOrd)]
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
    pub fn new(t: String) -> Result<Self, ValidationError> {
        if t.len() > 255 {
            Err(ValidationError::new("Type is too long".to_string()))
        } else if !TYPE_VALIDATOR.is_match(&t[..]) {
            Err(ValidationError::new("Invalid type".to_string()))
        } else {
            Ok(Type(t))
        }
    }
}

impl FromStr for Type {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string())?)
    }
}
