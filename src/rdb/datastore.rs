use datastore::{Datastore, Transaction};
use traits::Id;
use models;
use uuid::Uuid;
use util::{Error, generate_random_secret, get_salted_hash};
use serde_json::Value as JsonValue;
use chrono::naive::datetime::{NaiveDateTime};
use chrono::offset::utc::UTC;
use rocksdb::{DB, Writable, Options, IteratorMode, Direction, WriteBatch, DBIterator, DBVector, DBCompactionStyle};
use super::models::{AccountValue, EdgeValue, VertexValue};
use bincode::SizeLimit;
use bincode::serde as bincode_serde;
use std::io::Write;
use bincode::serde::{SerializeError, DeserializeError};
use std::sync::Arc;
use std::str;
use std::usize;
use std::i32;
use std::u64;
use std::u8;
use std::io::BufWriter;
use std::str::Utf8Error;
use std::io::Cursor;
use serde_json;
use std::ops::Deref;
use libc;

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

enum KeyComponent {
	Uuid(Uuid),
	String(String),
	Byte(u8)
}

struct KeyBuilder {
	components: Vec<KeyComponent>
}

fn build_key(components: Vec<KeyComponent>) -> Box<[u8]> {
	let mut len = 0;

	for component in components.iter() {
		len += match *component {
			KeyComponent::Uuid(_) => 16,
			KeyComponent::String(ref s) => s.len(),
			KeyComponent::Byte(_) => 1
		};
	}

	let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));

	for component in components.iter() {
		let res = match *component {
			KeyComponent::Uuid(ref uuid) => cursor.write(uuid.as_bytes()),
			KeyComponent::String(ref s) => cursor.write(s.as_bytes()),
			KeyComponent::Byte(b) => cursor.write(&[b])
		};

		if let Err(err) = res {
			panic!("Could not build key: {}", err);
		}

	}

	cursor.into_inner().into_boxed_slice()
}

const ACCOUNT_PRELUDE: u8 = 0;
const VERTEX_PRELUDE: u8 = 1;
const EDGE_PRELUDE: u8 = 2;
const GLOBAL_METADATA_PRELUDE: u8 = 10;
const ACCOUNT_METADATA_PRELUDE: u8 = 11;
const VERTEX_METADATA_PRELUDE: u8 = 12;
const EDGE_METADATA_PRELUDE: u8 = 13;

fn account_key(id: Uuid) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(ACCOUNT_PRELUDE),
		KeyComponent::Uuid(id)
	])
}

fn vertex_key(id: Uuid) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(VERTEX_PRELUDE),
		KeyComponent::Uuid(id)
	])
}

fn edge_key(outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<Box<[u8]>, Error> {
	if t.len() > u8::MAX as usize {
		return Err(Error::Unexpected("`type` is too long".to_string()));
	}

	Ok(build_key(vec![
		KeyComponent::Byte(EDGE_PRELUDE),
		KeyComponent::Uuid(outbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t),
		KeyComponent::Uuid(inbound_id)
	]))
}

fn edge_without_inbound_id_key_pattern(outbound_id: Uuid, t: String) -> Result<Box<[u8]>, Error> {
	if t.len() > u8::MAX as usize {
		return Err(Error::Unexpected("`type` is too long".to_string()));
	}

	Ok(build_key(vec![
		KeyComponent::Byte(EDGE_PRELUDE),
		KeyComponent::Uuid(outbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t)
	]))
}

fn global_metadata_key(key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(GLOBAL_METADATA_PRELUDE),
		KeyComponent::String(key)
	])
}

fn account_metadata_key(id: Uuid, key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(ACCOUNT_METADATA_PRELUDE),
		KeyComponent::Uuid(id),
		KeyComponent::String(key)
	])
}

fn vertex_metadata_key(id: Uuid, key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(VERTEX_METADATA_PRELUDE),
		KeyComponent::Uuid(id),
		KeyComponent::String(key)
	])
}

fn edge_metadata_key(outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Box<[u8]> {
	build_key(vec![
		KeyComponent::Byte(EDGE_METADATA_PRELUDE),
		KeyComponent::Uuid(outbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t),
		KeyComponent::Uuid(inbound_id),
		KeyComponent::String(key)
	])
}

fn parse_edge_key(key: &[u8]) -> (Uuid, String, Uuid) {
	if key.len() < 34 {
		panic!("Unexpected key length: {}", key.len());
	} else if key[0] != EDGE_PRELUDE {
		panic!("Unexpected prelude: {:x}", key[0]);
	}

	let outbound_id = Uuid::from_bytes(&key[1..17]).unwrap();
	let t_len = key[17] as usize;
	let t = str::from_utf8(&key[18..t_len+18]).unwrap();
	let inbound_id = Uuid::from_bytes(&key[t_len+18..key.len()]).unwrap();
	(outbound_id, t.to_string(), inbound_id)
}

fn parse_vertex_key(key: &[u8]) -> Uuid {
	if key.len() != 17 {
		panic!("Unexpected key length: {}", key.len());
	} else if key[0] != VERTEX_PRELUDE {
		panic!("Unexpected prelude: {:x}", key[0]);
	}

	Uuid::from_bytes(&key[1..17]).unwrap()
}

// Abstracted out so we can use it both for `RocksdbDatastore::has_account`,
// and for account metadata sanity checks
fn has_account(db: &DB, id: Uuid) -> Result<bool, Error> {
	match try!(db.get(&account_key(id))) {
		Some(_) => Ok(true),
		None => Ok(false)
	}
}

fn delete_vertex<W: Writable>(id: Uuid, db: &DB, mut w: &mut W) -> Result<(), Error> {
	try!(w.delete(&vertex_key(id)));

	let vertex_metadata_key_prefix = build_key(vec![
		KeyComponent::Byte(VERTEX_METADATA_PRELUDE),
		KeyComponent::Uuid(id)
	]);

	prefix_iterate!(db, &vertex_metadata_key_prefix, key, value, {
		try!(w.delete(&key));
	});

	let edge_key_prefix = build_key(vec![
		KeyComponent::Byte(EDGE_PRELUDE),
		KeyComponent::Uuid(id)
	]);

	prefix_iterate!(db, &edge_key_prefix, key, value, {
		let (_, t, inbound_id) = parse_edge_key(&key);
		try!(delete_edge(id.clone(), t, inbound_id, db, w));
	});

	Ok(())
}

fn delete_edge<W: Writable>(outbound_id: Uuid, t: String, inbound_id: Uuid, db: &DB, mut w: &mut W) -> Result<(), Error> {
	try!(w.delete(&try!(edge_key(outbound_id, t.clone(), inbound_id))));

	let edge_metadata_key_prefix = build_key(vec![
		KeyComponent::Byte(EDGE_METADATA_PRELUDE),
		KeyComponent::Uuid(outbound_id),
		KeyComponent::Byte(t.len() as u8),
		KeyComponent::String(t),
		KeyComponent::Uuid(inbound_id)
	]);

	prefix_iterate!(db, &edge_metadata_key_prefix, key, value, {
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
		None => Err(Error::MetadataNotFound)
	}
}

fn set_metadata(db: &DB, key: Box<[u8]>, value: JsonValue) -> Result<(), Error> {
	let json_bytes = try!(serde_json::to_vec(&value));
	try!(db.put(&key, &json_bytes[..]));
	Ok(())
}

fn get_vertex_value(db: &DB, id: Uuid) -> Result<VertexValue, Error> {
	match try!(db.get(&vertex_key(id))) {
		Some(vertex_bytes) => {
			let vertex_value = try!(bincode_serde::deserialize::<VertexValue>(&vertex_bytes.to_owned()[..]));
			Ok(vertex_value)
		},
		None => Err(Error::VertexNotFound)
	}
}

pub struct RocksdbDatastore {
	db: Arc<DB>
}

impl RocksdbDatastore {
	pub fn new(path: String, max_open_files: Option<i32>) -> Result<RocksdbDatastore, Error> {
		// NOTE: the rocksdb lib currently doesn't support prefix databases.
		// Once it does, we could use that to speed things up quite a bit.
		// Current tuning based off of the total ordered example, flash
		// storage example on
		// https://github.com/facebook/rocksdb/wiki/RocksDB-Tuning-Guide
		// Some of the options for it were not available 

		let mut opts = Options::default();
		opts.create_if_missing(true);
		opts.set_compaction_style(DBCompactionStyle::Level);
		opts.set_write_buffer_size(67108864); //64mb
		opts.set_max_write_buffer_number(3);
		opts.set_target_file_size_base(67108864); //64mb
		opts.set_max_background_compactions(4);
		opts.set_level_zero_slowdown_writes_trigger(17);
		opts.set_level_zero_stop_writes_trigger(24);

		if max_open_files.is_some() {
			opts.set_max_open_files(max_open_files.unwrap());
		}

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
		try!(self.db.put(&account_key(account_id), &account_bytes[..]));
		Ok((account_id, secret))
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		if !try!(self.has_account(account_id)) {
			return Err(Error::AccountNotFound);
		}

		let mut batch = WriteBatch::default();
		batch.delete(&account_key(account_id));

		// NOTE: This currently does a sequential scan through all keys to
		// find which vertices to delete. This could be more efficient.
		prefix_iterate!(self.db, "v1:".as_bytes(), key, value, {
			let vertex_value = try!(bincode_serde::deserialize::<VertexValue>(&value.to_owned()[..]));

			if vertex_value.owner_id == account_id {
				batch.delete(&key);
			}

			let vertex_id = parse_vertex_key(&key);
			delete_vertex(vertex_id, &self.db, &mut batch);
		});

		let account_metadata_key_prefix = build_key(vec![
			KeyComponent::Byte(ACCOUNT_METADATA_PRELUDE),
			KeyComponent::Uuid(account_id)
		]);

		prefix_iterate!(self.db, &account_metadata_key_prefix, key, value, {
			batch.delete(&key);
		});

		try!(self.db.write(batch));
		Ok(())
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		match try!(self.db.get(&account_key(account_id))) {
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
		try!(self.db.put(&vertex_key(id), &vertex_value_bytes[..]));
		Ok(id)
	}

	fn set_vertex(&self, vertex: models::Vertex<Uuid>) -> Result<(), Error> {
		let mut vertex_value = try!(get_vertex_value(&self.db, vertex.id));
		if vertex_value.owner_id != self.account_id {
			return Err(Error::VertexNotFound);
		}

		vertex_value.t = vertex.t;
		let vertex_value_bytes = try!(bincode_serde::serialize(&vertex_value, SizeLimit::Infinite));
		try!(self.db.put(&vertex_key(vertex.id), &vertex_value_bytes[..]));
		Ok(())
	}

	fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
		let vertex_value = try!(get_vertex_value(&self.db, id));

		if vertex_value.owner_id != self.account_id {
			return Err(Error::VertexNotFound);
		}

		let mut batch = WriteBatch::default();
		try!(delete_vertex(id, &self.db, &mut batch));
		try!(self.db.write(batch));
		Ok(())
	}

	fn get_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<models::Edge<Uuid>, Error> {
		match try!(self.db.get(&try!(edge_key(outbound_id, t.clone(), inbound_id)))) {
			Some(edge_value_bytes) => {
				let edge_value = try!(bincode_serde::deserialize::<EdgeValue>(&edge_value_bytes.to_owned()[..]));
				Ok(models::Edge::new(outbound_id, t, inbound_id, edge_value.weight))
			},
			None => Err(Error::EdgeNotFound)
		}
	}

	fn set_edge(&self, edge: models::Edge<Uuid>) -> Result<(), Error> {
		if edge.weight < -1.0 || edge.weight > 1.0 {
			return Err(Error::OutOfRange("weight".to_string()));
		}

		let outbound_vertex_value = try!(get_vertex_value(&self.db, edge.outbound_id));
		if outbound_vertex_value.owner_id != self.account_id {
			return Err(Error::VertexNotFound);
		}

		try!(get_vertex_value(&self.db, edge.inbound_id));

		let edge_value = EdgeValue::new(UTC::now().timestamp(), edge.weight);
		let edge_value_bytes = try!(bincode_serde::serialize(&edge_value, SizeLimit::Infinite));
		try!(self.db.put(&try!(edge_key(edge.outbound_id, edge.t, edge.inbound_id)), &edge_value_bytes[..]));
		Ok(())
	}

	fn delete_edge(&self, outbound_id: Uuid, t: String, inbound_id: Uuid) -> Result<(), Error> {
		// NOTE: currently doing a double-lookup of the edge: once to make
		// sure it exists, and once to check the account ID. This could be
		// optimized.

		try!(self.get_edge(outbound_id, t.clone(), inbound_id));

		let outbound_vertex_value = try!(get_vertex_value(&self.db, outbound_id));
		if outbound_vertex_value.owner_id != self.account_id {
			return Err(Error::EdgeNotFound);
		}

		let mut batch = WriteBatch::default();
		try!(delete_edge(outbound_id, t, inbound_id, &self.db, &mut batch));
		try!(self.db.write(batch));
		Ok(())
	}

	fn get_edge_count(&self, outbound_id: Uuid, t: String) -> Result<u64, Error> {
		let edge_key_prefix = try!(edge_without_inbound_id_key_pattern(outbound_id, t));
		let mut count: u64 = 0;

		prefix_iterate!(self.db, &edge_key_prefix, key, value, {
			count += 1;

			if count == u64::MAX {
				break;
			}
		});

		Ok(count)
	}

	fn get_edge_range(&self, outbound_id: Uuid, t: String, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
		if offset > usize::MAX as u64 {
			return Err(Error::OutOfRange("offset".to_string()));
		}

		let edge_key_prefix = try!(edge_without_inbound_id_key_pattern(outbound_id, t));
		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();
		let mut i = 0;

		prefix_iterate!(self.db, &edge_key_prefix, key, value, {
			if i < offset as usize {
				continue;
			} else if edges.len() >= limit as usize {
				break;
			}

			let (_, t, inbound_id) = parse_edge_key(&key);
			let edge_value = try!(bincode_serde::deserialize::<EdgeValue>(&value.to_owned()[..]));
			let edge = models::Edge::new(outbound_id.clone(), t, inbound_id, edge_value.weight);

			edges.push(edge);
			i += 1;
		});

		Ok(edges)
	}

	fn get_edge_time_range(&self, outbound_id: Uuid, t: String, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
		let edge_key_prefix = try!(edge_without_inbound_id_key_pattern(outbound_id, t));
		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

		prefix_iterate!(self.db, &edge_key_prefix, key, value, {
			if edges.len() >= limit as usize {
				break;
			}

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

			let (_, t, inbound_id) = parse_edge_key(&key);
			let edge = models::Edge::new(outbound_id, t.to_string(), inbound_id, edge_value.weight);
			edges.push(edge)
		});

		Ok(edges)
	}

	fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(&global_metadata_key(key)));
		get_metadata(result)
	}

	fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
		set_metadata(&self.db, global_metadata_key(key), value)
	}

	fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
		try!(self.get_global_metadata(key.clone()));
		try!(self.db.delete(&global_metadata_key(key)));
		Ok(())
	}

	fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(&account_metadata_key(owner_id, key)));
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
		try!(self.db.delete(&account_metadata_key(owner_id, key)));
		Ok(())
	}

	fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(&vertex_metadata_key(owner_id, key)));
		get_metadata(result)
	}

	fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		try!(self.get_vertex(owner_id));
		set_metadata(&self.db, vertex_metadata_key(owner_id, key), value)
	}

	fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		try!(self.get_vertex_metadata(owner_id, key.clone()));
		try!(self.db.delete(&vertex_metadata_key(owner_id, key)));
		Ok(())
	}

	fn get_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let result = try!(self.db.get(&edge_metadata_key(outbound_id, t, inbound_id, key)));
		get_metadata(result)
	}

	fn set_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		try!(self.get_edge(outbound_id, t.clone(), inbound_id));
		set_metadata(&self.db, edge_metadata_key(outbound_id, t, inbound_id, key), value)
	}

	fn delete_edge_metadata(&self, outbound_id: Uuid, t: String, inbound_id: Uuid, key: String) -> Result<(), Error> {
		try!(self.get_edge_metadata(outbound_id, t.clone(), inbound_id, key.clone()));
		try!(self.db.delete(&edge_metadata_key(outbound_id, t, inbound_id, key)));
		Ok(())
	}

	fn commit(self) -> Result<(), Error> {
		Ok(())
	}

	fn rollback(self) -> Result<(), Error> {
		Err(Error::Unexpected("Transactions cannot be rolled back in the rocksdb datastore implementation".to_string()))
	}
}
