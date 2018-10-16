use super::types::Type;
use chrono::offset::Utc;
use chrono::DateTime;
use uuid::Uuid;

/// Specifies what kind of items should be piped from one type of query to
/// another.
///
/// Edge and vertex queries can build off of one another via pipes - e.g. you
/// can get the outbound edges of a set of vertices by piping from a vertex
/// query to an edge query. `EdgeDirection`s are used to specify which
/// end of things you want to pipe - either the outbound items or the inbound
/// items.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Hash, Copy)]
pub enum EdgeDirection {
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "inbound")]
    Inbound,
}

/// Represents a uniquely identifiable key to an edge.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EdgeKey {
    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The type of the edge.
    #[serde(rename = "type")]
    pub t: Type,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,
}

impl EdgeKey {
    /// Creates a new edge key.
    ///
    /// # Arguments
    ///
    /// * `outbound_id` - The id of the outbound vertex.
    /// * `t` - The type of the edge.
    /// * `inbound_id` - The id of the inbound vertex.
    pub fn new(outbound_id: Uuid, t: Type, inbound_id: Uuid) -> EdgeKey {
        EdgeKey {
            outbound_id,
            t,
            inbound_id,
        }
    }
}

/// An edge.
///
/// Edges are how you would represent a verb or a relationship in the
/// datastore. An example might be "liked" or "reviewed". Edges are typed
/// and directed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    /// The key to the edge.
    pub key: EdgeKey,

    /// When the edge was created.
    pub created_datetime: DateTime<Utc>,
}

impl Edge {
    /// Creates a new edge with the current datetime in UTC.
    ///
    /// # Arguments
    /// * `key` - The key to the edge.
    pub fn new_with_current_datetime(key: EdgeKey) -> Edge {
        Self::new(key, Utc::now())
    }

    /// Creates a new edge with a specified datetime.
    ///
    /// # Arguments
    /// * `key` - The key to the edge.
    /// * `created_datetime` - When the edge was created.
    pub fn new(key: EdgeKey, created_datetime: DateTime<Utc>) -> Edge {
        Edge { key, created_datetime }
    }
}

#[cfg(test)]
mod tests {
    use super::{Edge, EdgeKey};
    use chrono::Utc;
    use models::Type;
    use uuid::Uuid;

    #[test]
    fn should_create_edge_with_current_datetime() {
        let start_datetime = Utc::now();

        let edge = Edge::new_with_current_datetime(EdgeKey::new(Uuid::default(), Type::default(), Uuid::default()));

        let end_datetime = Utc::now();

        assert!(edge.created_datetime >= start_datetime);
        assert!(edge.created_datetime <= end_datetime);
    }
}
