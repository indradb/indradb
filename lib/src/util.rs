//! Utility functions. These are public because they may be useful for crates
//! that implement Datastore.

use std::u8;

use crate::errors::{ValidationError, ValidationResult};
use crate::models;

use once_cell::sync::Lazy;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

const NODE_ID: [u8; 6] = [0, 0, 0, 0, 0, 0];

static CONTEXT: Lazy<Context> = Lazy::new(|| Context::new(0));

/// Generates a UUID v1. This utility method uses a shared context and node ID
/// to help ensure generated UUIDs are unique.
pub fn generate_uuid_v1() -> Uuid {
    Uuid::new_v1(Timestamp::now(&*CONTEXT), &NODE_ID)
}

/// Gets the next UUID that would occur after the given one.
///
/// # Arguments
/// * `uuid`: The input UUID.
///
/// # Errors
/// Returns a `ValidationError` if the input UUID is the great possible value
/// (i.e., FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF)
pub fn next_uuid(uuid: Uuid) -> ValidationResult<Uuid> {
    let mut bytes = *uuid.as_bytes();

    for i in (0..16).rev() {
        if bytes[i] < 255 {
            bytes[i] += 1;
            return Ok(Uuid::from_slice(&bytes[..]).unwrap());
        } else {
            bytes[i] = 0;
        }
    }

    Err(ValidationError::CannotIncrementUuid)
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
    use super::{
        extract_count, extract_edge_properties, extract_edges, extract_vertex_properties, extract_vertices,
        generate_uuid_v1, next_uuid,
    };
    use core::str::FromStr;
    use uuid::Uuid;

    #[test]
    fn should_generate_new_uuid_v1() {
        let first = generate_uuid_v1();
        let second = generate_uuid_v1();
        assert_ne!(first, second);
    }

    #[test]
    fn should_generate_next_uuid() {
        let result = next_uuid(Uuid::from_str("16151dea-a538-4bf1-9559-851e256cf139").unwrap());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Uuid::from_str("16151dea-a538-4bf1-9559-851e256cf13a").unwrap()
        );

        let from_uuid = Uuid::from_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        assert!(next_uuid(from_uuid).is_err());
    }

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
