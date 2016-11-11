use datastore::{Datastore, Transaction};
use traits::Id;
use models;
use rocksdb::uuid::Uuid;
use util::{Error, generate_random_secret, get_salted_hash};
use serde_json::Value as JsonValue;
use chrono::naive::datetime::{NaiveDateTime};
use chrono::offset::utc::UTC;
use rocksdb::rocksdb::{DB, Writable, Options, IteratorMode, Direction, WriteBatch, DBIterator, DBVector};
use super::models::{AccountValue, EdgeValue, VertexValue};
use rocksdb::bincode::SizeLimit;
use rocksdb::bincode::serde as bincode_serde;
use std::io::Write;
use rocksdb::bincode::serde::{SerializeError, DeserializeError};
use std::sync::Arc;
use rocksdb::regex::bytes::Regex;
use std::str;
use std::usize;
use std::i32;
use std::io::BufWriter;
use std::str::Utf8Error;
use serde_json;

lazy_static! {
	static ref VERTEX_KEY_MATCHER: Regex = Regex::new("^v1:(.{32})$").unwrap();
	static ref EDGE_KEY_MATCHER: Regex = Regex::new("^e1:(.{32}):([^:]*):(.{32})$").unwrap();
}

// We use a macro to avoid take_while, and the overhead that closure callbacks would cause
macro_rules! prefix_iterate {
	($db:expr, $prefix:expr, $key:ident, $value:ident, $code:block) => {
		for ($key, $value) in $db.iterator(IteratorMode::From($prefix, Direction::Forward)) {
			if !$key.starts_with($prefix) {
				break;
			}

			$code;
		}
	}
}

impl Id for Uuid {}

fn account_key(id: Uuid) -> String {
	format!("a1:{}", id.simple().to_string())
}

fn vertex_key(id: Uuid) -> String {
	format!("v1:{}", id.simple().to_string())
}

fn edge_key(outbound_id: Uuid, t: String, inbound_id: Uuid) -> String {
	format!("e1:{}:{}:{}", outbound_id.simple().to_string(), t, inbound_id.simple().to_string())
}

fn global_metadata_key(key: String) -> String {
	format!("gm1:{}", key)
}

fn account_metadata_key(id: Uuid, key: String) -> String {
	format!("am1:{}:{}", id.simple().to_string(), key)
}

fn vertex_metadata_key(id: Uuid, key: String) -> String {
	format!("vm1:{}:{}", id.simple().to_string(), key)
}

fn edge_metadata_key(outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> String {
	format!("em1:{}:{}:{}:{}", outbound_id.simple().to_string(), t, inbound_id.simple().to_string(), key)
}

// Abstracted out so we can use it both for `RocksdbDatastore::has_account`,
// and for account metadata sanity checks
fn has_account(db: &DB, id: Uuid) -> Result<bool, Error> {
	match try!(db.get(account_key(id).as_bytes())) {
		Some(_) => Ok(true),
		None => Ok(false)
	}
}

fn delete_vertex<W: Writable>(id: Uuid, db: &DB, mut w: &mut W) -> Result<(), Error> {
	try!(w.delete(vertex_key(id).as_bytes()));

	prefix_iterate!(db, format!("vm1:{}:", id.simple().to_string()).as_bytes(), key, value, {
		try!(w.delete(&key));
	});

	prefix_iterate!(db, format!("e1:{}:", id.simple().to_string()).as_bytes(), key, value, {
		let caps = EDGE_KEY_MATCHER.captures(&key).unwrap();
		let outbound_id_str = try!(str::from_utf8(caps.at(1).unwrap()));
		let outbound_id = Uuid::parse_str(outbound_id_str).unwrap();
		let t = str::from_utf8(caps.at(2).unwrap()).unwrap();
		let inbound_id_str = try!(str::from_utf8(caps.at(3).unwrap()));
		let inbound_id = Uuid::parse_str(inbound_id_str).unwrap();
		try!(delete_edge(outbound_id, t.to_string(), inbound_id, db, w));
	});

	Ok(())
}

fn delete_edge<W: Writable>(outbound_id: Uuid, t: String, inbound_id: Uuid, db: &DB, mut w: &mut W) -> Result<(), Error> {
	try!(w.delete(edge_key(outbound_id, t.clone(), inbound_id).as_bytes()));

	prefix_iterate!(db, format!("em1:{}:{}:{}:", outbound_id.simple().to_string(), t, inbound_id.simple().to_string()).as_bytes(), key, value, {
		try!(w.delete(&key));
	});

	Ok(())
}

fn get_metadata(result: Option<DBVector>) -> Result<JsonValue, Error> {
	match result {
		Some(json_bytes) => {
			let json = try!(serde_json::from_slice::<JsonValue>(&json_bytes.to_owned()[..]));
			Ok(json)
		},
		None => Err(Error::MetadataDoesNotExist)
	}
}

fn set_metadata(db: &DB, key: String, value: JsonValue) -> Result<(), Error> {
	let json_bytes = try!(serde_json::to_vec(&value));
	try!(db.put(key.as_bytes(), &json_bytes[..]));
	Ok(())
}

fn get_vertex_value(db: &DB, id: Uuid) -> Result<VertexValue, Error> {
	match try!(db.get(vertex_key(id).as_bytes())) {
		Some(vertex_bytes) => {
			let vertex_value = try!(bincode_serde::deserialize::<VertexValue>(&vertex_bytes.to_owned()[..]));
			Ok(vertex_value)
		},
		None => Err(Error::VertexDoesNotExist)
	}
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
		has_account(&self.db, account_id)
	}

	fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
		let account_id = Uuid::new_v4();
		let salt = generate_random_secret();
		let secret = generate_random_secret();
		let hash = get_salted_hash(salt.clone(), None, secret.clone());
		let account = AccountValue::new(email, salt, hash);
		let account_bytes = try!(bincode_serde::serialize(&account, SizeLimit::Infinite));
		try!(self.db.put(account_key(account_id).as_bytes(), &account_bytes[..]));
		Ok((account_id, secret))
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		if !try!(self.has_account(account_id)) {
			return Err(Error::AccountNotFound);
		}

		let mut batch = WriteBatch::default();
		batch.delete(account_key(account_id).as_bytes());

		// NOTE: This currently does a sequential scan through all keys to
		// find which vertices to delete. This could be more efficient.
		prefix_iterate!(self.db, "v1:".as_bytes(), key, value, {
			let vertex_value = try!(bincode_serde::deserialize::<VertexValue>(&value.to_owned()[..]));

			if vertex_value.owner_id == account_id {
				batch.delete(&key);
			}

			let caps = VERTEX_KEY_MATCHER.captures(&key).unwrap();
			let vertex_id_str = try!(str::from_utf8(caps.at(1).unwrap()));
			let vertex_id = Uuid::parse_str(vertex_id_str).unwrap();
			delete_vertex(vertex_id, &self.db, &mut batch);
		});

		prefix_iterate!(self.db, format!("a1:{}", account_id.simple().to_string()).as_bytes(), key, value, {
			batch.delete(&key);
		});

		try!(self.db.write(batch));
		Ok(())
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		match try!(self.db.get(account_key(account_id).as_bytes())) {
			Some(account_bytes) => {
				let account = try!(bincode_serde::deserialize::<AccountValue>(&account_bytes.to_owned()[..]));
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

impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Error {
		Error::Unexpected(format!("Could not (de-)serialize json: {:?}", err))
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
}

impl Transaction<Uuid> for RocksdbTransaction {
	fn get_vertex(&self, id: Uuid) -> Result<models::Vertex<Uuid>, Error> {
		let vertex_value = try!(get_vertex_value(&self.db, id));
		let vertex = models::Vertex::new(id, vertex_value.t);
		Ok(vertex)
	}

	fn create_vertex(&self, t: String) -> Result<Uuid, Error> {
		let id = Uuid::new_v4();
		let vertex_value = VertexValue::new(self.account_id.clone(), t);
		let vertex_value_bytes = try!(bincode_serde::serialize(&vertex_value, SizeLimit::Infinite));
		try!(self.db.put(vertex_key(id).as_bytes(), &vertex_value_bytes[..]));
		Ok(id)
	}

	fn set_vertex(&self, vertex: models::Vertex<Uuid>) -> Result<(), Error> {
		let mut vertex_value = try!(get_vertex_value(&self.db, vertex.id));
		if vertex_value.owner_id != self.account_id {
			return Err(Error::VertexDoesNotExist);
		}

		vertex_value.t = vertex.t;
		let vertex_value_bytes = try!(bincode_serde::serialize(&vertex_value, SizeLimit::Infinite));
		try!(self.db.put(vertex_key(vertex.id).as_bytes(), &vertex_value_bytes[..]));
		Ok(())
	}

	fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
		let vertex_value = try!(get_vertex_value(&self.db, id));

		if vertex_value.owner_id != self.account_id {
			return Err(Error::VertexDoesNotExist);
		}

		let mut batch = WriteBatch::default();
		try!(delete_vertex(id, &self.db, &mut batch));
		try!(self.db.write(batch));
		Ok(())
	}

	fn get_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<models::Edge<Uuid>, Error> {
		match try!(self.db.get(edge_key(outbound_id, t.clone(), inbound_id).as_bytes())) {
			Some(edge_value_bytes) => {
				let edge_value = try!(bincode_serde::deserialize::<EdgeValue>(&edge_value_bytes.to_owned()[..]));
				Ok(models::Edge::new(outbound_id, t, inbound_id, edge_value.weight))
			},
			None => Err(Error::EdgeDoesNotExist)
		}
	}

	fn set_edge(&self, edge: models::Edge<Uuid>) -> Result<(), Error> {
		if edge.weight < -1.0 || edge.weight > 1.0 {
			return Err(Error::WeightOutOfRange);
		}

		let outbound_vertex_value = try!(get_vertex_value(&self.db, edge.outbound_id));
		if outbound_vertex_value.owner_id != self.account_id {
			return Err(Error::VertexDoesNotExist);
		}

		try!(get_vertex_value(&self.db, edge.inbound_id));

		let edge_value = EdgeValue::new(UTC::now().timestamp(), edge.weight);
		let edge_value_bytes = try!(bincode_serde::serialize(&edge_value, SizeLimit::Infinite));
		try!(self.db.put(edge_key(edge.outbound_id, edge.t, edge.inbound_id).as_bytes(), &edge_value_bytes[..]));
		Ok(())
	}

	fn delete_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<(), Error> {
		// NOTE: currently doing a double-lookup of the edge: once to make
		// sure it exists, and once to check the account ID. This could be
		// optimized.

		try!(self.get_edge(outbound_id, t.clone(), inbound_id));

		let outbound_vertex_value = try!(get_vertex_value(&self.db, outbound_id));
		if outbound_vertex_value.owner_id != self.account_id {
			return Err(Error::EdgeDoesNotExist);
		}

		let mut batch = WriteBatch::default();
		try!(delete_edge(outbound_id, t, inbound_id, &self.db, &mut batch));
		try!(self.db.write(batch));
		Ok(())
	}

	fn get_edge_count(&self, outbound_id: Uuid, t: String) -> Result<i64, Error> {
		let key_prefix = format!("e1:{}:{}:", outbound_id.simple().to_string(), t);
		let mut count = 0;

		prefix_iterate!(self.db, key_prefix.as_bytes(), key, value, {
			count += 1;
		});

		Ok(count)
	}

	fn get_edge_range(&self, outbound_id: Uuid, t: String, offset: i64, limit: i32) -> Result<Vec<models::Edge<Uuid>>, Error> {
		if offset > i32::MAX as i64 || offset < 0 {
			return Err(Error::OffsetOutOfRange);
		} else if limit < 0 {
			return Err(Error::LimitOutOfRange);
		}

		let key_prefix = format!("e1:{}:{}:", outbound_id.simple().to_string(), t);
		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();
		let mut i = 0;

		prefix_iterate!(self.db, key_prefix.as_bytes(), key, value, {
			if i < offset as usize {
				continue;
			}

			let caps = EDGE_KEY_MATCHER.captures(&key).unwrap();
			let t = str::from_utf8(caps.at(2).unwrap()).unwrap();
			let inbound_id_str = try!(str::from_utf8(caps.at(3).unwrap()));
			let inbound_id = Uuid::parse_str(inbound_id_str).unwrap();

			let edge_value = try!(bincode_serde::deserialize::<EdgeValue>(&value.to_owned()[..]));
			let edge = models::Edge::new(outbound_id.clone(), t.to_string(), inbound_id, edge_value.weight);

			edges.push(edge);
			i += 1;
		});

		Ok(edges)
	}

	fn get_edge_time_range(&self, outbound_id: Uuid, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: i32) -> Result<Vec<models::Edge<Uuid>>, Error> {
		if limit < 0 {
			return Err(Error::LimitOutOfRange);
		}

		let key_prefix = format!("e1:{}:{}:", outbound_id.simple().to_string(), t);
		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

		prefix_iterate!(self.db, key_prefix.as_bytes(), key, value, {
			if edges.len() >= limit as usize {
				break;
			}

			let caps = EDGE_KEY_MATCHER.captures(&key).unwrap();
			let t = str::from_utf8(caps.at(2).unwrap()).unwrap();
			let inbound_id_str = try!(str::from_utf8(caps.at(3).unwrap()));
			let inbound_id = Uuid::parse_str(inbound_id_str).unwrap();

			let edge_value = try!(bincode_serde::deserialize::<EdgeValue>(&value.to_owned()[..]));

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
		});

		Ok(edges)
	}

	fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(global_metadata_key(key).as_bytes()));
		get_metadata(result)
	}

	fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
		set_metadata(&self.db, global_metadata_key(key), value)
	}

	fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
		try!(self.get_global_metadata(key.clone()));
		try!(self.db.delete(global_metadata_key(key).as_bytes()));
		Ok(())
	}

	fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(account_metadata_key(owner_id, key).as_bytes()));
		get_metadata(result)
	}

	fn set_account_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		if !try!(has_account(&self.db, owner_id)) {
			return Err(Error::AccountNotFound);
		}

		set_metadata(&self.db, account_metadata_key(owner_id, key), value)
	}

	fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		try!(self.get_account_metadata(owner_id, key.clone()));
		try!(self.db.delete(account_metadata_key(owner_id, key).as_bytes()));
		Ok(())
	}

	fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(vertex_metadata_key(owner_id, key).as_bytes()));
		get_metadata(result)
	}

	fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		try!(self.get_vertex(owner_id));
		set_metadata(&self.db, vertex_metadata_key(owner_id, key), value)
	}

	fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		try!(self.get_vertex_metadata(owner_id, key.clone()));
		try!(self.db.delete(vertex_metadata_key(owner_id, key).as_bytes()));
		Ok(())
	}

	fn get_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(edge_metadata_key(outbound_id, t, inbound_id, key).as_bytes()));
		get_metadata(result)
	}

	fn set_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		try!(self.get_edge(outbound_id, t.clone(), inbound_id));
		set_metadata(&self.db, edge_metadata_key(outbound_id, t, inbound_id, key), value)
	}

	fn delete_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<(), Error> {
		try!(self.get_edge_metadata(outbound_id, t.clone(), inbound_id, key.clone()));
		try!(self.db.delete(edge_metadata_key(outbound_id, t, inbound_id, key).as_bytes()));
		Ok(())
	}

	fn commit(self) -> Result<(), Error> {
		Ok(())
	}

	fn rollback(self) -> Result<(), Error> {
		Err(Error::Unexpected("Transactions cannot be rolled back in the rocksdb datastore implementation".to_string()))
	}
}
