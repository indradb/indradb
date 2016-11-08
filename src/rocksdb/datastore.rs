use datastore::{Datastore, Transaction};
use traits::Id;
use models;
use rocksdb::uuid::Uuid;
use util::{Error, generate_random_secret, get_salted_hash};
use serde_json::Value as JsonValue;
use chrono::naive::datetime::{NaiveDateTime};
use chrono::offset::utc::UTC;
use rocksdb::rocksdb::{DB, Writable, Options, IteratorMode, Direction, WriteBatch};
use super::models::{AccountValue, EdgeValue, VertexValue};
use rocksdb::bincode::{SizeLimit, serde};
use std::io::Write;
use rocksdb::bincode::serde::{SerializeError, DeserializeError};
use std::sync::Arc;
use rocksdb::regex::bytes::Regex;
use std::str;
use std::usize;
use std::i32;
use std::io::BufWriter;
use std::str::Utf8Error;

lazy_static! {
	static ref EDGE_KEY_MATCHER: Regex = Regex::new("^e1:.{32}:([^:]*):(.{32})$").unwrap();
}

impl Id for Uuid {}

fn account_key(id: Uuid) -> String {
	format!("a1:{}", id)
}

fn vertex_key(id: Uuid) -> String {
	format!("v1:{}", id)
}

fn edge_key(outbound_id: Uuid, t: String, inbound_id: Uuid) -> String {
	format!("e1:{}:{}:{}", outbound_id.simple().to_string(), t, inbound_id.simple().to_string())
}

fn global_metadata_key(key: String) -> String {
	format!("gm1:{}", key)
}

fn account_metadata_key(id: Uuid, key: String) -> String {
	format!("am1:{}:{}", id, key)
}

fn vertex_metadata_key(id: Uuid, key: String) -> String {
	format!("vm1:{}:{}", id, key)
}

fn edge_metadata_key(outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> String {
	format!("em1:{}:{}:{}:{}", outbound_id.simple().to_string(), t, inbound_id.simple().to_string(), key)
}

pub struct RocksdbDatastore {
	db: Arc<DB>
}

impl RocksdbDatastore {
	pub fn new(path: String) -> Result<RocksdbDatastore, Error> {
		let mut opts = Options::default();
		opts.create_if_missing(true);

		let mut db = try!(DB::open(&opts, &path[..]));

		Ok(RocksdbDatastore{
			db: Arc::new(db)
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
		let account = AccountValue::new(email, salt, hash);
		let account_bytes = try!(serde::serialize(&account, SizeLimit::Infinite));
		try!(self.db.put(account_key(account_id).as_bytes(), &account_bytes[..]));
		Ok((account_id, secret))
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		if !try!(self.has_account(account_id)) {
			return Err(Error::AccountNotFound);
		}

		let key_prefix = "v1:".as_bytes();

		let mut batch = WriteBatch::default();
		batch.delete(account_key(account_id).as_bytes());

		let iter = self.db.iterator(IteratorMode::From(key_prefix, Direction::Forward));

		// NOTE: This currently does a sequential scan through all keys to
		// find which vertices to delete. This could be more efficient.
		for (key, value) in iter {
			if !key.starts_with(key_prefix) {
				break;
			}

			let vertex_value = try!(serde::deserialize::<VertexValue>(&value.to_owned()[..]));
			
			if vertex_value.owner_id == account_id {
				batch.delete(&key);
			}
		}

		try!(self.db.write(batch));
		Ok(())
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		match try!(self.db.get(account_key(account_id).as_bytes())) {
			Some(account_bytes) => {
				let account = try!(serde::deserialize::<AccountValue>(&account_bytes.to_owned()[..]));
				let expected_hash = get_salted_hash(account.salt, None, secret);
				Ok(expected_hash == account.hash)
			},
			_ => Ok(false)
		}
	}

	fn transaction(&self, account_id: Uuid) -> Result<RocksdbTransaction, Error> {
		RocksdbTransaction::new(self.db.clone(), account_id)
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

impl From<Utf8Error> for Error {
	fn from(_: Utf8Error) -> Error {
		Error::Unexpected(format!("Could not parse utf-8 contents"))
	}
}

pub struct RocksdbTransaction {
	db: Arc<DB>,
	account_id: Uuid
}

impl RocksdbTransaction {
	fn new(db: Arc<DB>, account_id: Uuid) -> Result<Self, Error> {
		Ok(RocksdbTransaction {
			db: db,
			account_id: account_id
		})
	}

	fn get_vertex_value(&self, id: Uuid) -> Result<VertexValue, Error> {
		match try!(self.db.get(vertex_key(id).as_bytes())) {
			Some(vertex_bytes) => {
				let vertex_value = try!(serde::deserialize::<VertexValue>(&vertex_bytes.to_owned()[..]));
				Ok(vertex_value)
			},
			None => Err(Error::VertexDoesNotExist)
		}
	}
}

impl Transaction<Uuid> for RocksdbTransaction {
	fn get_vertex(&self, id: Uuid) -> Result<models::Vertex<Uuid>, Error> {
		let vertex_value = try!(self.get_vertex_value(id));
		let vertex = models::Vertex::new(id, vertex_value.t);
		Ok(vertex)
	}

	fn create_vertex(&self, t: String) -> Result<Uuid, Error> {
		let id = Uuid::new_v4();
		let vertex_value = VertexValue::new(self.account_id.clone(), t);
		let vertex_value_bytes = try!(serde::serialize(&vertex_value, SizeLimit::Infinite));
		try!(self.db.put(vertex_key(id).as_bytes(), &vertex_value_bytes[..]));
		Ok(id)
	}

	fn set_vertex(&self, vertex: models::Vertex<Uuid>) -> Result<(), Error> {
		let mut vertex_value = try!(self.get_vertex_value(vertex.id));
		if vertex_value.owner_id != self.account_id {
			return Err(Error::VertexDoesNotExist);
		}

		vertex_value.t = vertex.t;
		let vertex_value_bytes = try!(serde::serialize(&vertex_value, SizeLimit::Infinite));
		try!(self.db.put(vertex_key(vertex.id).as_bytes(), &vertex_value_bytes[..]));
		Ok(())
	}

	fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
		let vertex_value = try!(self.get_vertex_value(id));
		if vertex_value.owner_id != self.account_id {
			return Err(Error::VertexDoesNotExist);
		}
		//TODO: Delete edges
		Ok(())
	}

	fn get_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<models::Edge<Uuid>, Error> {
		match try!(self.db.get(edge_key(outbound_id, t.clone(), inbound_id).as_bytes())) {
			Some(edge_value_bytes) => {
				let edge_value = try!(serde::deserialize::<EdgeValue>(&edge_value_bytes.to_owned()[..]));
				Ok(models::Edge::new(outbound_id, t, inbound_id, edge_value.weight))
			},
			None => Err(Error::EdgeDoesNotExist)
		}
	}

	fn set_edge(&self, edge: models::Edge<Uuid>) -> Result<(), Error> {
		if edge.weight < -1.0 || edge.weight > 1.0 {
			return Err(Error::WeightOutOfRange);
		}

		let outbound_vertex_value = try!(self.get_vertex_value(edge.outbound_id));
		if outbound_vertex_value.owner_id != self.account_id {
			return Err(Error::VertexDoesNotExist);
		}

		try!(self.get_vertex_value(edge.inbound_id));

		let edge_value = EdgeValue::new(UTC::now().timestamp(), edge.weight);
		let edge_value_bytes = try!(serde::serialize(&edge_value, SizeLimit::Infinite));
		try!(self.db.put(edge_key(edge.outbound_id, edge.t, edge.inbound_id).as_bytes(), &edge_value_bytes[..]));
		Ok(())
	}

	fn delete_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<(), Error> {
		let outbound_vertex_value = try!(self.get_vertex_value(outbound_id));
		if outbound_vertex_value.owner_id != self.account_id {
			return Err(Error::EdgeDoesNotExist);
		}

		try!(self.get_edge(outbound_id, t.clone(), inbound_id));
		try!(self.db.delete(edge_key(outbound_id, t, inbound_id).as_bytes()));
		Ok(())
	}

	fn get_edge_count(&self, outbound_id: Uuid, t: String) -> Result<i64, Error> {
		let key_prefix = format!("e1:{}:{}:", outbound_id.simple().to_string(), t);
		let key_prefix_bytes = key_prefix.as_bytes();
		let iter = self.db.iterator(IteratorMode::From(key_prefix_bytes, Direction::Forward));
		let mut count = 0;

		for (key, value) in iter {
			if !key.starts_with(key_prefix_bytes) {
				break;
			} else {
				count += 1;
			}
		}

		Ok(count)
	}

	fn get_edge_range(&self, outbound_id: Uuid, t: String, offset: i64, limit: i32) -> Result<Vec<models::Edge<Uuid>>, Error> {
		if offset > i32::MAX as i64 || offset < 0 {
			return Err(Error::OffsetOutOfRange);
		} else if limit < 0 {
			return Err(Error::LimitOutOfRange);
		}

		let key_prefix = format!("e1:{}:{}:", outbound_id.simple().to_string(), t);
		let key_prefix_bytes = key_prefix.as_bytes();

		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();
		let iter = self.db.iterator(IteratorMode::From(key_prefix_bytes, Direction::Forward));

		for (i, (key, value)) in iter.enumerate() {
			if !key.starts_with(key_prefix_bytes) || edges.len() >= limit as usize {
				break;
			} else if i < offset as usize {
				continue;
			}

			let caps = EDGE_KEY_MATCHER.captures(&key).unwrap();
			let t = str::from_utf8(caps.at(1).unwrap()).unwrap();
			let inbound_id_str = try!(str::from_utf8(caps.at(2).unwrap()));
			let inbound_id = Uuid::parse_str(inbound_id_str).unwrap();
			
			let edge_value = try!(serde::deserialize::<EdgeValue>(&value.to_owned()[..]));
			let edge = models::Edge::new(outbound_id.clone(), t.to_string(), inbound_id, edge_value.weight);
			
			edges.push(edge)
		}

		Ok(edges)
	}

	fn get_edge_time_range(&self, outbound_id: Uuid, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<models::Edge<Uuid>>, Error> {
		if limit < 0 {
			return Err(Error::LimitOutOfRange);
		}

		let key_prefix = format!("e1:{}:{}:", outbound_id.simple().to_string(), t);
		let key_prefix_bytes = key_prefix.as_bytes();

		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();
		let iter = self.db.iterator(IteratorMode::From(key_prefix_bytes, Direction::Forward));

		for (key, value) in iter {
			if !key.starts_with(key_prefix_bytes) || edges.len() >= limit as usize {
				break;
			}

			let caps = EDGE_KEY_MATCHER.captures(&key).unwrap();
			let t = str::from_utf8(caps.at(1).unwrap()).unwrap();
			let inbound_id_str = try!(str::from_utf8(caps.at(2).unwrap()));
			let inbound_id = Uuid::parse_str(inbound_id_str).unwrap();
			
			let edge_value = try!(serde::deserialize::<EdgeValue>(&value.to_owned()[..]));

			// Filter out items out of the date range
			// NOTE: This currently involves a sequential scan through all
			// relevant edges, and could be made more efficient by indexing
			// the edge update date.
			let update_date = NaiveDateTime::from_timestamp(edge_value.update_date, 0);
			if high.is_some() && update_date > high.unwrap() {
				continue;
			}
			if low.is_some() && update_date < low.unwrap() {
				continue;
			}

			let edge = models::Edge::new(outbound_id, t.to_string(), inbound_id, edge_value.weight);
			edges.push(edge)
		}

		Ok(edges)
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
		Ok(())
	}

	fn rollback(self) -> Result<(), Error> {
		Err(Error::Unexpected("Transactions cannot be rolled back in the rocksdb datastore implementation".to_string()))
	}
}
