use core::str::FromStr;
use errors::{ValidationError, ValidationResult};
use std::u16;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Ord, PartialOrd)]
pub struct Id(pub String);

impl Id {
    pub fn new<S: Into<String>>(s: S) -> ValidationResult<Self> {
        let s = s.into();

        if s.len() > u16::max_value() as usize {
            Err("Id is too long".into())
        } else {
            Ok(Id(s))
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self { 0: "".to_string() }
    }
}

impl FromStr for Id {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string())?)
    }
}
