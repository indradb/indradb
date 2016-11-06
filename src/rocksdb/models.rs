#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub salt: String,
    pub hash: String
}