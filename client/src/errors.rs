use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct VertexInvalidError;

impl StdError for VertexInvalidError {}

impl fmt::Display for VertexInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid vertex")
    }
}
