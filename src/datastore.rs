use super::util::SimpleError;
use super::responses::{Response, ErrorResponse};
use super::requests::Request;
use std::vec::Vec;

pub trait Datastore<T: Transaction> {
	fn get_account_id(&self, email: String) -> Result<Option<i32>, SimpleError>;
	fn create_account(&self, email: String, secret: String) -> Result<i32, SimpleError>;
	fn delete_account(&self, account_id: i32) -> Result<(), SimpleError>;
	fn auth(&self, email: String, secret: String) -> Result<Option<i32>, SimpleError>;
	fn transaction(&self, account_id: i32) -> T;
}

pub trait Transaction {
	fn request(&mut self, req: Request);
	fn commit(&self) -> Result<Vec<Result<Response, ErrorResponse>>, SimpleError>;
	fn rollback(&self) -> Option<SimpleError>;
}
