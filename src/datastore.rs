use super::util::SimpleError;
use super::responses::{Response, ErrorResponse};
use super::requests::Request;
use std::vec::Vec;

pub trait Datastore<T: Transaction> {
	fn has_account(&self, email: String) -> Result<bool, SimpleError>;
	fn create_account(&self, email: String) -> Result<String, SimpleError>;
	fn delete_account(&self, email: String) -> Result<(), SimpleError>;
	fn auth(&self, email: String, secret: String) -> Result<bool, SimpleError>;
	fn transaction(&self, email: String) -> Result<T, SimpleError>;
}

pub trait Transaction {
	fn request(&mut self, req: Request);
	fn commit(&self) -> Result<Vec<Result<Response, ErrorResponse>>, SimpleError>;
	fn rollback(&self) -> Option<SimpleError>;
}
