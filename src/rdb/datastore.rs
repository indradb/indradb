use datastore::{Datastore, Transaction};
use models;
use uuid::Uuid;
use errors::Error;
use util::get_salted_hash;
use serde_json::Value as JsonValue;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc::UTC;
use rocksdb::{DB, Options, IteratorMode, Direction, WriteBatch, DBCompactionStyle};
use super::models::VertexValue;
use std::sync::Arc;
use std::usize;
use std::i32;
use std::u64;
use super::keys::*;
use super::managers::*;

pub struct RocksdbDatastore {
	db: Arc<DB>
}

impl RocksdbDatastore {
	pub fn new(path: String, max_open_files: Option<i32>) -> Result<RocksdbDatastore, Error> {
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
		try!(db.create_cf("accounts:v1", &opts));
		try!(db.create_cf("vertices:v1", &opts));
		try!(db.create_cf("edges:v1", &opts));
		try!(db.create_cf("edge_ranges:v1", &opts));
		try!(db.create_cf("reversed_edge_ranges:v1", &opts));
		try!(db.create_cf("global_metadata:v1", &opts));
		try!(db.create_cf("account_metadata:v1", &opts));
		try!(db.create_cf("vertex_metadata:v1", &opts));
		try!(db.create_cf("edge_metadata:v1", &opts));

		Ok(RocksdbDatastore{
			db: Arc::new(db)
		})
	}
}

impl Datastore<RocksdbTransaction, Uuid> for RocksdbDatastore {
	fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
		AccountManager::new(self.db.clone()).exists(account_id)
	}

	fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
		AccountManager::new(self.db.clone()).create(email)
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		let manager = AccountManager::new(self.db.clone());

		if !try!(manager.exists(account_id)) {
			return Err(Error::AccountNotFound);
		}

		let mut batch = WriteBatch::default();
		try!(manager.delete(&mut batch, account_id));
		try!(self.db.write(batch));
		Ok(())
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		match try!(AccountManager::new(self.db.clone()).get(account_id)) {
			Some(value) => {
				let expected_hash = get_salted_hash(&value.salt[..], None, &secret[..]);
				Ok(expected_hash == value.hash)
			},
			_ => Ok(false)
		}
	}

	fn transaction(&self, account_id: Uuid) -> Result<RocksdbTransaction, Error> {
		RocksdbTransaction::new(self.db.clone(), account_id)
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

	fn handle_get_edge_count(&self, edge_range_manager: EdgeRangeManager, first_id: Uuid, t: models::Type) -> Result<u64, Error> {
		let mut count: u64 = 0;
		let edge_range_prefix_key = edge_range_manager.prefix_key(first_id, t);

		prefix_iterate!(edge_range_manager, &edge_range_prefix_key, key, value, {
			count += 1;

			if count == u64::MAX {
				break;
			}
		});

		Ok(count)
	}

	fn check_write_permissions(&self, id: Uuid, err: Error) -> Result<(), Error> {
		let vertex_manager = VertexManager::new(self.db.clone());
		let vertex_value = try!(vertex_manager.get(id));

		if vertex_value.is_none() || vertex_value.unwrap().owner_id != self.account_id {
			Err(err)
		} else {
			Ok(())
		}
	}
}

impl Transaction<Uuid> for RocksdbTransaction {
	fn get_vertex(&self, id: Uuid) -> Result<models::Vertex<Uuid>, Error> {
		match try!(VertexManager::new(self.db.clone()).get(id)) {
			Some(value) => {
				let vertex = models::Vertex::new(id, value.t);
				Ok(vertex)
			},
			None => Err(Error::VertexNotFound)
		}
	}

	fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error> {
		VertexManager::new(self.db.clone()).create(t, self.account_id.clone())
	}

	fn set_vertex(&self, vertex: models::Vertex<Uuid>) -> Result<(), Error> {
		try!(self.check_write_permissions(vertex.id, Error::VertexNotFound));
		let value = VertexValue::new(self.account_id, vertex.t);
		VertexManager::new(self.db.clone()).update(vertex.id, &value)
	}

	fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
		try!(self.check_write_permissions(id, Error::VertexNotFound));
		let mut batch = WriteBatch::default();
		try!(VertexManager::new(self.db.clone()).delete(&mut batch, id));
		try!(self.db.write(batch));
		Ok(())
	}

	fn get_edge(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<models::Edge<Uuid>, Error> {
		match try!(EdgeManager::new(self.db.clone()).get(outbound_id, t.clone(), inbound_id)) {
			Some(value) => Ok(models::Edge::new(outbound_id, t, inbound_id, value.weight)),
			None => Err(Error::EdgeNotFound)
		}
	}

	fn set_edge(&self, edge: models::Edge<Uuid>) -> Result<(), Error> {
		// Verify that the vertices exist and that we own the vertex with the outbound ID
		try!(self.check_write_permissions(edge.outbound_id, Error::VertexNotFound));
		if !try!(VertexManager::new(self.db.clone()).exists(edge.inbound_id)) {
			return Err(Error::VertexNotFound);
		}

		let edge_manager = EdgeManager::new(self.db.clone());
		let old_edge = try!(edge_manager.get(edge.outbound_id, edge.t.clone(), edge.inbound_id));
		let old_update_datetime = old_edge.and_then(|edge| Some(NaiveDateTime::from_timestamp(edge.update_timestamp, 0)));

		let mut batch = WriteBatch::default();
		let new_update_datetime = UTC::now().naive_utc();
		try!(edge_manager.set(&mut batch, edge.outbound_id, edge.t, edge.inbound_id, old_update_datetime, new_update_datetime, edge.weight));
		try!(self.db.write(batch));
		Ok(())
	}

	fn delete_edge(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<(), Error> {
		// Verify that the edge exists and that we own it
		let edge_manager = EdgeManager::new(self.db.clone());

		match try!(edge_manager.get(outbound_id, t.clone(), inbound_id)) {
			Some(value) => {
				try!(self.check_write_permissions(outbound_id, Error::EdgeNotFound));
				let mut batch = WriteBatch::default();
				try!(edge_manager.delete(&mut batch, outbound_id, t, inbound_id, &value));
				try!(self.db.write(batch));
				Ok(())
			},
			None => Err(Error::EdgeNotFound)
		}
	}

	fn get_edge_count(&self, outbound_id: Uuid, t: models::Type) -> Result<u64, Error> {
		let edge_range_manager = EdgeRangeManager::new(self.db.clone());
		self.handle_get_edge_count(edge_range_manager, outbound_id, t)
	}

	fn get_edge_range(&self, outbound_id: Uuid, t: models::Type, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
		let edge_range_manager = EdgeRangeManager::new(self.db.clone());
		let edge_range_prefix_key = edge_range_manager.prefix_key(outbound_id, t.clone());
		let edge_range_max_key = edge_range_manager.max_key_in_range(outbound_id, t);
		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();
		let mut i = 0;

		reverse_iterate!(edge_range_manager, &edge_range_max_key, &edge_range_prefix_key, key, value, {
			i += 1;

			if i <= offset {
				continue;
			} else if edges.len() >= limit as usize {
				break;
			}

			let (edge_outbound_id, edge_type, _) = parse_edge_range_key(&key);
			let edge_value = try!(edge_range_manager.deserialize_value(&value));
			let edge = models::Edge::new(edge_outbound_id, edge_type, edge_value.other_id, edge_value.weight);
			edges.push(edge);
		});

		Ok(edges)
	}

	fn get_edge_time_range(&self, outbound_id: Uuid, t: models::Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
		let edge_range_manager = EdgeRangeManager::new(self.db.clone());
		let edge_range_prefix_key = edge_range_manager.prefix_key(outbound_id, t.clone());

		let edge_range_max_key = match high {
			Some(high) => edge_range_manager.key(outbound_id, t, high),
			None => edge_range_manager.max_key_in_range(outbound_id, t)
		};

		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

		reverse_iterate!(edge_range_manager, &edge_range_max_key, &edge_range_prefix_key, key, value, {
			if edges.len() >= limit as usize {
				break;
			}

			let (edge_outbound_id, edge_type, update_datetime) = parse_edge_range_key(&key);

			if let Some(low) = low {
				if low > update_datetime {
					break;
				}
			}

			let edge_value = try!(edge_range_manager.deserialize_value(&value));
			let edge = models::Edge::new(edge_outbound_id, edge_type, edge_value.other_id, edge_value.weight);
			edges.push(edge);
		});

		Ok(edges)
	}

	fn get_reversed_edge_count(&self, inbound_id: Uuid, t: models::Type) -> Result<u64, Error> {
		let edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
		self.handle_get_edge_count(edge_range_manager, inbound_id, t)
	}

	fn get_reversed_edge_range(&self, inbound_id: Uuid, t: models::Type, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
		let edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
		let edge_range_prefix_key = edge_range_manager.prefix_key(inbound_id, t.clone());
		let edge_range_max_key = edge_range_manager.max_key_in_range(inbound_id, t);
		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();
		let mut i = 0;

		reverse_iterate!(edge_range_manager, &edge_range_max_key, &edge_range_prefix_key, key, value, {
			i += 1;

			if i <= offset {
				continue;
			} else if edges.len() >= limit as usize {
				break;
			}

			let (edge_inbound_id, edge_type, _) = parse_edge_range_key(&key);
			let edge_value = try!(edge_range_manager.deserialize_value(&value));
			let edge = models::Edge::new(edge_value.other_id, edge_type, edge_inbound_id, edge_value.weight);
			edges.push(edge);
		});

		Ok(edges)
	}

	fn get_reversed_edge_time_range(&self, inbound_id: Uuid, t: models::Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
		let edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
		let edge_range_prefix_key = edge_range_manager.prefix_key(inbound_id, t.clone());

		let edge_range_max_key = match high {
			Some(high) => edge_range_manager.key(inbound_id, t.clone(), high),
			None => edge_range_manager.max_key_in_range(inbound_id, t)
		};

		let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

		reverse_iterate!(edge_range_manager, &edge_range_max_key, &edge_range_prefix_key, key, value, {
			if edges.len() >= limit as usize {
				break;
			}

			let (edge_inbound_id, edge_type, update_datetime) = parse_edge_range_key(&key);

			if let Some(low) = low {
				if low > update_datetime {
					break;
				}
			}

			let edge_value = try!(edge_range_manager.deserialize_value(&value));
			let edge = models::Edge::new(edge_value.other_id, edge_type, edge_inbound_id, edge_value.weight);
			edges.push(edge);
		});

		Ok(edges)
	}

	fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
		let manager = GlobalMetadataManager::new(self.db.clone());
		try!(manager.get(key)).ok_or_else(|| Error::MetadataNotFound)
	}

	fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
		let manager = GlobalMetadataManager::new(self.db.clone());
		manager.set(key, &value)
	}

	fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
		let mut batch = WriteBatch::default();
		try!(GlobalMetadataManager::new(self.db.clone()).delete(&mut batch, key));		try!(self.db.write(batch));
		Ok(())
	}

	fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let manager = AccountMetadataManager::new(self.db.clone());
		try!(manager.get(owner_id, key)).ok_or_else(|| Error::MetadataNotFound)
	}

	fn set_account_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		if !try!(AccountManager::new(self.db.clone()).exists(owner_id)) {
			return Err(Error::AccountNotFound);
		}

		let manager = AccountMetadataManager::new(self.db.clone());
		try!(manager.set(owner_id, key, &value));
		Ok(())
	}

	fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		let mut batch = WriteBatch::default();
		try!(AccountMetadataManager::new(self.db.clone()).delete(&mut batch, owner_id, key));
		try!(self.db.write(batch));
		Ok(())
	}

	fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let manager = VertexMetadataManager::new(self.db.clone());
		try!(manager.get(owner_id, key)).ok_or_else(|| Error::MetadataNotFound)
	}

	fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		if !try!(VertexManager::new(self.db.clone()).exists(owner_id)) {
			return Err(Error::VertexNotFound);
		}

		let manager = VertexMetadataManager::new(self.db.clone());
		manager.set(owner_id, key, &value)
	}

	fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		let mut batch = WriteBatch::default();
		try!(VertexMetadataManager::new(self.db.clone()).delete(&mut batch, owner_id, key));
		try!(self.db.write(batch));
		Ok(())
	}

	fn get_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
		let manager = EdgeMetadataManager::new(self.db.clone());
		try!(manager.get(outbound_id, t, inbound_id, key)).ok_or_else(|| Error::MetadataNotFound)
	}

	fn set_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		if !try!(EdgeManager::new(self.db.clone()).exists(outbound_id, t.clone(), inbound_id)) {
			return Err(Error::EdgeNotFound);
		}

		let manager = EdgeMetadataManager::new(self.db.clone());
		manager.set(outbound_id, t, inbound_id, key, &value)
	}

	fn delete_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String) -> Result<(), Error> {
		let mut batch = WriteBatch::default();
		try!(EdgeMetadataManager::new(self.db.clone()).delete(&mut batch, outbound_id, t, inbound_id, key));
		try!(self.db.write(batch));
		Ok(())
	}

	fn commit(self) -> Result<(), Error> {
		Ok(())
	}

	fn rollback(self) -> Result<(), Error> {
		Err(Error::Unexpected("Transactions cannot be rolled back in the rocksdb datastore implementation".to_string()))
	}
}
