use uuid::Uuid;
use models;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountValue {
    pub email: String,
    pub salt: String,
    pub hash: String,
}

impl AccountValue {
    pub fn new(email: String, salt: String, hash: String) -> Self {
        AccountValue {
            email: email,
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
    pub update_timestamp: i64,
    pub weight: models::Weight,
}

impl EdgeValue {
    pub fn new(update_timestamp: i64, weight: models::Weight) -> Self {
        EdgeValue {
            update_timestamp: update_timestamp,
            weight: weight,
        }
    }
}
