use errors::Error;
use traits::Id;
use std::vec::Vec;
use serde_json::value::Value as JsonValue;
use models;
use std::option::Option;
use chrono::naive::datetime::NaiveDateTime;

pub trait Datastore<T: Transaction<I>, I: Id> {
	fn has_account(&self, account_id: I) -> Result<bool, Error>;
	fn create_account(&self, email: String) -> Result<(I, String), Error>;
	fn delete_account(&self, account_id: I) -> Result<(), Error>;
	fn auth(&self, account_id: I, secret: String) -> Result<bool, Error>;
	fn transaction(&self, account_id: I) -> Result<T, Error>;
}

pub trait Transaction<I: Id> {
	fn get_vertex(&self, I) -> Result<models::Vertex<I>, Error>;
	fn create_vertex(&self, models::Type) -> Result<I, Error>;
	fn set_vertex(&self, models::Vertex<I>) -> Result<(), Error>;
	fn delete_vertex(&self, I) -> Result<(), Error>;

	fn get_edge(&self, I, models::Type, I) -> Result<models::Edge<I>, Error>;
	fn set_edge(&self, models::Edge<I>) -> Result<(), Error>;
	fn delete_edge(&self, I, models::Type, I) -> Result<(), Error>;

	fn get_edge_count(&self, I, models::Type) -> Result<u64, Error>;
	fn get_edge_range(&self, I, models::Type, u64, u16) -> Result<Vec<models::Edge<I>>, Error>;
	fn get_edge_time_range(&self, I, models::Type, Option<NaiveDateTime>, Option<NaiveDateTime>, u16) -> Result<Vec<models::Edge<I>>, Error>;

	fn get_reversed_edge_count(&self, I, models::Type) -> Result<u64, Error>;
	fn get_reversed_edge_range(&self, I, models::Type, u64, u16) -> Result<Vec<models::Edge<I>>, Error>;
	fn get_reversed_edge_time_range(&self, I, models::Type, Option<NaiveDateTime>, Option<NaiveDateTime>, u16) -> Result<Vec<models::Edge<I>>, Error>;

	fn get_global_metadata(&self, String) -> Result<JsonValue, Error>;
	fn set_global_metadata(&self, String, JsonValue) -> Result<(), Error>;
	fn delete_global_metadata(&self, String) -> Result<(), Error>;

	fn get_account_metadata(&self, I, String) -> Result<JsonValue, Error>;
	fn set_account_metadata(&self, I, String, JsonValue) -> Result<(), Error>;
	fn delete_account_metadata(&self, I, String) -> Result<(), Error>;

	fn get_vertex_metadata(&self, I, String) -> Result<JsonValue, Error>;
	fn set_vertex_metadata(&self, I, String, JsonValue) -> Result<(), Error>;
	fn delete_vertex_metadata(&self, I, String) -> Result<(), Error>;

	fn get_edge_metadata(&self, I, models::Type, I, String) -> Result<JsonValue, Error>;
	fn set_edge_metadata(&self, I, models::Type, I, String, JsonValue) -> Result<(), Error>;
	fn delete_edge_metadata(&self, I, models::Type, I, String) -> Result<(), Error>;

	fn commit(self) -> Result<(), Error>;
	fn rollback(self) -> Result<(), Error>;
}
