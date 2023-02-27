use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Deref;
use std::str::FromStr;

use crate::errors::{ValidationError, ValidationResult};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A string that must be less than 256 characters long, and can only contain
/// letters, numbers, dashes and underscores. This is used for vertex and edge
/// types, as well as property names.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Ord, PartialOrd)]
pub struct Identifier(pub(crate) Intern);

static INTERNAL_CONTAINERS: Lazy<Mutex<HashSet<&'static str>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Ord, PartialOrd)]
pub(crate) struct Intern {
    pointer: &'static str,
}

impl Intern {
    pub(crate) const fn new(pointer: &'static str) -> Self {
        Intern { pointer }
    }

    pub(crate) fn len(&self) -> usize {
        self.pointer.len()
    }
}

impl Deref for Intern {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.pointer
    }
}

impl<S: Into<String>> From<S> for Intern {
    fn from(s: S) -> Self {
        let s = s.into();

        let mut guard = INTERNAL_CONTAINERS.lock();
        if let Some(&p) = guard.get(s.as_str()) {
            Intern { pointer: p }
        } else {
            let p = Box::leak(Box::from(s));
            guard.insert(p);
            Intern { pointer: p }
        }
    }
}

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
            Ok(Self(s.into()))
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
        Self(Intern::from(s))
    }

    /// Constructs a new identifier constantly, without any checks that it is valid.
    ///
    /// # Arguments
    /// * `s`: The identifier value.
    ///
    /// # Safety
    /// This function is marked unsafe because there's no verification that
    /// the identifier is valid.
    pub const unsafe fn const_new_unchecked(s: &'static str) -> Self {
        Self(Intern::new(s))
    }

    /// Gets a reference to the identifier value.
    pub fn as_str(&self) -> &str {
        self.0.deref()
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self(Intern::new(""))
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
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

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Identifier, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: String = Deserialize::deserialize(deserializer)?;
        let id = unsafe { Identifier::new_unchecked(v) };
        Ok(id)
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
            assert_eq!(Identifier::const_new_unchecked("foo").as_str(), "foo");
            assert_eq!(Identifier::const_new_unchecked("$").as_str(), "$");
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
    }
}
