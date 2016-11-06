use datastore::{Datastore, Transaction};
use traits::Id;
use models;
use rocksdb::uuid::Uuid;
use util::{Error, generate_random_secret};
use serde_json::Value as JsonValue;
use chrono::naive::datetime::NaiveDateTime;

impl Id for Uuid {}

#[derive(Clone, Debug)]
pub struct RocksdbDatastore {
	
}

impl RocksdbDatastore {
	pub fn new() -> RocksdbDatastore {
		RocksdbDatastore{}
	}
}

impl Datastore<RocksdbTransaction, Uuid> for RocksdbDatastore {
	fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn transaction(&self, account_id: Uuid) -> Result<RocksdbTransaction, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}
}

#[derive(Debug)]
pub struct RocksdbTransaction {
	account_id: Uuid
}

impl RocksdbTransaction {
	fn new(account_id: Uuid) -> Result<Self, Error> {
		Ok(RocksdbTransaction {
			account_id: account_id
		})
	}
}

impl Transaction<Uuid> for RocksdbTransaction {
	fn get_vertex(&self, id: Uuid) -> Result<models::Vertex<Uuid>, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn create_vertex(&self, t: String) -> Result<Uuid, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn set_vertex(&self, v: models::Vertex<Uuid>) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<models::Edge<Uuid>, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn set_edge(&self, e: models::Edge<Uuid>) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn delete_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_edge_count(&self, outbound_id: Uuid, t: String) -> Result<i64, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_edge_range(&self, outbound_id: Uuid, t: String, offset: i64, limit: i32) -> Result<Vec<models::Edge<Uuid>>, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_edge_time_range(&self, outbound_id: Uuid, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<models::Edge<Uuid>>, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn set_account_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn get_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn set_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn delete_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn commit(self) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}

	fn rollback(self) -> Result<(), Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}
}
