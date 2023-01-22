//! Utility functions. These are public because they may be useful for crates
//! that implement Datastore.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Error as IoError, Read, Write};
use std::{str, u8};

use crate::models;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

/// A byte-serializable value, frequently employed in the keys of key/value
/// store.
pub enum Component<'a> {
    /// A u64.
    U64(u64),
    /// A fixed length string.
    FixedLengthString(&'a str),
    /// An identifier.
    Identifier(&'a models::Identifier),
    /// A JSON value.
    Json(&'a models::Json),
}

impl<'a> Component<'a> {
    /// Gets the length of the component. This isn't called `len` to avoid a
    /// clippy warning.
    pub fn byte_len(&self) -> usize {
        match *self {
            Component::U64(_) => 8,
            Component::FixedLengthString(s) => s.len(),
            Component::Identifier(t) => t.0.len() + 1,
            Component::Json(_) => 8,
        }
    }

    /// Writes a component into a cursor of bytes.
    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), IoError> {
        match *self {
            Component::U64(v) => cursor.write_u64::<BigEndian>(v),
            Component::FixedLengthString(s) => cursor.write_all(s.as_bytes()),
            Component::Identifier(i) => {
                cursor.write_all(&[i.0.len() as u8])?;
                cursor.write_all(i.0.as_bytes())
            }
            Component::Json(json) => {
                let mut hasher = DefaultHasher::new();
                json.hash(&mut hasher);
                let hash = hasher.finish();
                cursor.write_u64::<BigEndian>(hash)
            }
        }
    }
}

// Serializes component(s) into bytes.
///
/// # Arguments
/// * `components`: The components to serialize to bytes.
pub fn build(components: &[Component]) -> Vec<u8> {
    let len = components.iter().fold(0, |len, component| len + component.byte_len());
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

    for component in components {
        if let Err(err) = component.write(&mut cursor) {
            panic!("Could not write bytes: {}", err);
        }
    }

    cursor.into_inner()
}

/// Reads an identifier from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_identifier<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> models::Identifier {
    let t_len = {
        let mut buf: [u8; 1] = [0; 1];
        cursor.read_exact(&mut buf).unwrap();
        buf[0] as usize
    };

    let mut buf = vec![0u8; t_len];
    cursor.read_exact(&mut buf).unwrap();

    unsafe {
        let s = str::from_utf8_unchecked(&buf).to_string();
        models::Identifier::new_unchecked(s)
    }
}

/// Reads a fixed-length string from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_fixed_length_string<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> String {
    let mut buf = String::new();
    cursor.read_to_string(&mut buf).unwrap();
    buf
}

/// Reads a `u64` from bytes.
///
/// # Arguments
/// * `cursor`: The bytes to read from.
pub fn read_u64<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> u64 {
    cursor.read_u64::<BigEndian>().unwrap()
}

/// Extracts vertices from the last query output value, or `None`.
///
/// # Arguments
/// * `output`: The query output.
pub fn extract_vertices(mut output: Vec<models::QueryOutputValue>) -> Option<Vec<models::Vertex>> {
    if let Some(models::QueryOutputValue::Vertices(vertices)) = output.pop() {
        Some(vertices)
    } else {
        None
    }
}

/// Extracts edges from the last query output value, or `None`.
///
/// # Arguments
/// * `output`: The query output.
pub fn extract_edges(mut output: Vec<models::QueryOutputValue>) -> Option<Vec<models::Edge>> {
    if let Some(models::QueryOutputValue::Edges(edges)) = output.pop() {
        Some(edges)
    } else {
        None
    }
}

/// Extracts a count from the last query output value, or `None`.
///
/// # Arguments
/// * `output`: The query output.
pub fn extract_count(mut output: Vec<models::QueryOutputValue>) -> Option<u64> {
    if let Some(models::QueryOutputValue::Count(count)) = output.pop() {
        Some(count)
    } else {
        None
    }
}

/// Extracts vertex properties from the last query output value, or `None`.
///
/// # Arguments
/// * `output`: The query output.
pub fn extract_vertex_properties(mut output: Vec<models::QueryOutputValue>) -> Option<Vec<models::VertexProperties>> {
    if let Some(models::QueryOutputValue::VertexProperties(props)) = output.pop() {
        Some(props)
    } else {
        None
    }
}

/// Extracts edge properties from the last query output value, or `None`.
///
/// # Arguments
/// * `output`: The query output.
pub fn extract_edge_properties(mut output: Vec<models::QueryOutputValue>) -> Option<Vec<models::EdgeProperties>> {
    if let Some(models::QueryOutputValue::EdgeProperties(props)) = output.pop() {
        Some(props)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_count, extract_edge_properties, extract_edges, extract_vertex_properties, extract_vertices};
    use core::str::FromStr;

    #[test]
    fn should_not_extract_vertices_on_empty() {
        assert_eq!(extract_vertices(vec![]), None);
    }

    #[test]
    fn should_not_extract_edges_on_empty() {
        assert_eq!(extract_edges(vec![]), None);
    }

    #[test]
    fn should_not_extract_count_on_empty() {
        assert_eq!(extract_count(vec![]), None);
    }

    #[test]
    fn should_not_extract_vertex_properties_on_empty() {
        assert_eq!(extract_vertex_properties(vec![]), None);
    }

    #[test]
    fn should_not_extract_edge_properties_on_empty() {
        assert_eq!(extract_edge_properties(vec![]), None);
    }
}
