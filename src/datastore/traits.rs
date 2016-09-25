use util::SimpleError;
use responses::{Response, ErrorResponse};
use requests::Request;
use traits::Id;
use std::vec::Vec;

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
