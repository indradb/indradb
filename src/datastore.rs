use super::util::SimpleError;
use super::responses::{Response, ErrorResponse};
use super::requests::Request;
use std::vec::Vec;
use std::fmt::Debug;
use serde::ser::Serialize;
use serde::de::Deserialize;
use std::default::Default;
use core::hash::Hash;
use std::marker::Copy;

pub trait Id: Clone + Debug + Serialize + Deserialize + Eq + Default + Hash + Copy {}

pub trait Datastore<T: Transaction<I>, I: Id> {
	fn has_account(&self, user_id: I) -> Result<bool, SimpleError>;
	fn create_account(&self, email: String) -> Result<(I, String), SimpleError>;
	fn delete_account(&self, user_id: I) -> Result<(), SimpleError>;
	fn auth(&self, user_id: I, secret: String) -> Result<bool, SimpleError>;
	fn transaction(&self, user_id: I) -> Result<T, SimpleError>;
}

pub trait Transaction<I: Id> {
	fn request(&mut self, req: Request<I>);
	fn commit(&self) -> Result<Vec<Result<Response<I>, ErrorResponse<I>>>, SimpleError>;
	fn rollback(&self) -> Option<SimpleError>;
}
