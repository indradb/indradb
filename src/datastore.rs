use super::util::SimpleError;
use super::responses::{Response, ErrorResponse};
use super::requests::Request;
use std::vec::Vec;
use std::fmt::Debug;
use serde::ser::Serialize;
use serde::de::Deserialize;
use std::default::Default;

pub trait Id: Clone + Debug + Serialize + Deserialize + Eq + Default {}

pub trait Datastore<T: Transaction, I: Id> {
	fn has_account(&self, user_id: I) -> Result<bool, SimpleError>;
	fn create_account(&self, email: String) -> Result<(I, String), SimpleError>;
	fn delete_account(&self, user_id: I) -> Result<(), SimpleError>;
	fn auth(&self, user_id: I, secret: String) -> Result<bool, SimpleError>;
	fn transaction(&self, user_id: I) -> Result<T, SimpleError>;
}

pub trait Transaction {
	fn request(&mut self, req: Request);
	fn commit(&self) -> Result<Vec<Result<Response, ErrorResponse>>, SimpleError>;
	fn rollback(&self) -> Option<SimpleError>;
}
