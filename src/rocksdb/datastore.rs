use datastore::{Datastore, Transaction};
use traits::Id;
use models;
use rocksdb::uuid::Uuid;
use util::{Error, generate_random_secret, get_salted_hash};
use serde_json::Value as JsonValue;
use chrono::naive::datetime::NaiveDateTime;
use rocksdb::rocksdb::{DB, Writable, Options};
use super::models::{Account};
use rocksdb::bincode::{SizeLimit, serde};
use std::io::Write;
use rocksdb::bincode::serde::{SerializeError, DeserializeError};

impl Id for Uuid {}

fn account_key(account_id: Uuid) -> String {
	format!("accounts:{}:v1", account_id)
}

pub struct RocksdbDatastore {
	db: DB
}

impl RocksdbDatastore {
	pub fn new(path: String) -> Result<RocksdbDatastore, Error> {
		let mut opts = Options::default();
		opts.create_if_missing(true);

		let mut db = try!(DB::open(&opts, &path[..]));

		Ok(RocksdbDatastore{
			db: db
		})
	}
}

impl Datastore<RocksdbTransaction, Uuid> for RocksdbDatastore {
	fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
		match try!(self.db.get(account_key(account_id).as_bytes())) {
			Some(_) => Ok(true),
			None => Ok(false)
		}
	}

	fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
		let account_id = Uuid::new_v4();
		let salt = generate_random_secret();
		let secret = generate_random_secret();
		let hash = get_salted_hash(salt.clone(), None, secret.clone());

		let account = Account {
			email: email,
			salt: salt,
			hash: hash
		};

		let account_bytes = try!(serde::serialize(&account, SizeLimit::Infinite));
		try!(self.db.put(account_key(account_id).as_bytes(), &account_bytes[..]));
		Ok((account_id, secret))
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		try!(self.db.delete(format!("accounts:{}:v1", account_id).as_bytes()));
		Ok(())
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		match try!(self.db.get(account_key(account_id).as_bytes())) {
			Some(account_bytes) => {
				let account = try!(serde::deserialize::<Account>(&account_bytes.to_owned()[..]));
				let expected_hash = get_salted_hash(account.salt, None, secret);
				Ok(expected_hash == account.hash)
			},
			_ => Ok(false)
		}
	}

	fn transaction(&self, account_id: Uuid) -> Result<RocksdbTransaction, Error> {
		Err(Error::Unexpected("Unimplemented".to_string()))
	}
}

impl From<String> for Error {
	fn from(message: String) -> Error {
		Error::Unexpected(message)
	}
}

impl From<SerializeError> for Error {
	fn from(err: SerializeError) -> Error {
		Error::Unexpected(format!("Could not serialize contents: {:?}", err))
	}
}

impl From<DeserializeError> for Error {
	fn from(err: DeserializeError) -> Error {
		Error::Unexpected(format!("Could not deserialize contents: {:?}", err))
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
