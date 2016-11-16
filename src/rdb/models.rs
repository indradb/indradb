use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountValue {
    pub email: String,
    pub salt: String,
    pub hash: String
}

impl AccountValue {
    pub fn new(email: String, salt: String, hash: String) -> Self {
        AccountValue {
            email: email,
            salt: salt,
            hash: hash
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VertexValue {
    pub owner_id: Uuid,
    pub t: String
}

impl VertexValue {
    pub fn new(owner_id: Uuid, t: String) -> Self {
        VertexValue {
            owner_id: owner_id,
            t: t
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdgeValue {
    pub update_date: i64,
    pub weight: f32
}

impl EdgeValue {
    pub fn new(update_date: i64, weight: f32) -> Self {
        EdgeValue {
            update_date: update_date,
            weight: weight
        }
    }
}