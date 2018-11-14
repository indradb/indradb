use core::str::FromStr;
use errors::{ValidationError, ValidationResult};
use std::u16;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Ord, PartialOrd)]
pub struct Id(pub Vec<u8>);

impl Id {
    pub fn new(value: Vec<u8>) -> ValidationResult<Self> {
        if value.len() > u16::max_value() as usize {
            Err("Id is too long".into())
        } else {
            Ok(Id(value))
        }
    }
}

impl FromStr for Id {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.as_bytes().to_vec())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new(Vec::new()).unwrap()
    }
}
