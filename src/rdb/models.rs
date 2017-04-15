use uuid::Uuid;
use models;
use chrono::{DateTime, UTC};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountValue {
    pub salt: String,
    pub hash: String,
}

impl AccountValue {
    pub fn new(salt: String, hash: String) -> Self {
        AccountValue {
            salt: salt,
            hash: hash,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VertexValue {
    pub owner_id: Uuid,
    pub t: models::Type,
}

impl VertexValue {
    pub fn new(owner_id: Uuid, t: models::Type) -> Self {
        VertexValue {
            owner_id: owner_id,
            t: t,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdgeValue {
    pub update_datetime: DateTime<UTC>,
    pub weight: models::Weight,
}

impl EdgeValue {
    pub fn new(update_datetime: DateTime<UTC>, weight: models::Weight) -> Self {
        EdgeValue {
            update_datetime: update_datetime,
            weight: weight,
        }
    }
}
