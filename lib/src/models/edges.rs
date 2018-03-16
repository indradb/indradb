use super::types::Type;
use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;

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
            outbound_id: outbound_id,
            t: t,
            inbound_id: inbound_id,
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
        Edge {
            key: key,
            created_datetime: created_datetime,
        }
    }
}
