use datastore::{Datastore, Transaction};
use models;
use std::collections::HashSet;
use uuid::Uuid;
use errors::Error;
use util::get_salted_hash;
use serde_json::Value as JsonValue;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc::UTC;
use rocksdb::{DB, Options, WriteBatch, DBCompactionStyle};
use super::models::VertexValue;
use std::sync::Arc;
use std::usize;
use std::i32;
use std::u64;
use super::managers::*;

const CF_NAMES: [&'static str; 9] = [
    "accounts:v1",
    "vertices:v1",
    "edges:v1",
    "edge_ranges:v1",
    "reversed_edge_ranges:v1",
    "global_metadata:v1",
    "account_metadata:v1",
    "vertex_metadata:v1",
    "edge_metadata:v1"
];

fn get_options(max_open_files: Option<i32>) -> Options {
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

    if let Some(max_open_files) = max_open_files {
        opts.set_max_open_files(max_open_files);
    }

    opts
}

pub struct RocksdbDatastore {
    db: Arc<DB>,
}

impl RocksdbDatastore {
    pub fn new(path: &str, max_open_files: Option<i32>) -> Result<RocksdbDatastore, Error> {
        let opts = get_options(max_open_files);

        let db = match DB::open_cf(&opts, path, &CF_NAMES) {
            Ok(db) => db,
            Err(_) => {
                let mut db = DB::open(&opts, path)?;

                for cf_name in CF_NAMES.iter() {
                    db.create_cf(cf_name, &opts)?;
                }

                db
            }
        };

        Ok(RocksdbDatastore { db: Arc::new(db) })
    }

    pub fn repair(path: &str, max_open_files: Option<i32>) -> Result<(), Error> {
        let opts = get_options(max_open_files);
        DB::repair(opts, path)?;
        Ok(())
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

        if !manager.exists(account_id)? {
            return Err(Error::AccountNotFound);
        }

        let mut batch = WriteBatch::default();
        manager.delete(&mut batch, account_id)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
        match AccountManager::new(self.db.clone()).get(account_id)? {
            Some(value) => {
                let expected_hash = get_salted_hash(&value.salt[..], None, &secret[..]);
                Ok(expected_hash == value.hash)
            }
            _ => {
                // Calculate the hash anyways to prevent a timing attack
                get_salted_hash("", None, &secret[..]);
                Ok(false)
            }
        }
    }

    fn transaction(&self, account_id: Uuid) -> Result<RocksdbTransaction, Error> {
        RocksdbTransaction::new(self.db.clone(), account_id)
    }
}

pub struct RocksdbTransaction {
    db: Arc<DB>,
    account_id: Uuid,
}

impl RocksdbTransaction {
    fn new(db: Arc<DB>, account_id: Uuid) -> Result<Self, Error> {
        Ok(RocksdbTransaction {
            db: db,
            account_id: account_id,
        })
    }

    fn handle_get_edge_count(&self, edge_range_manager: EdgeRangeManager, first_id: Uuid, t: models::Type) -> Result<u64, Error> {
        let iterator = edge_range_manager.iterate_for_range(first_id, &t, None)?;
        Ok(iterator.count() as u64)
    }

    fn handle_get_edge_time_range(&self, iterator: Box<Iterator<Item=Result<models::Edge<Uuid>, Error>>>, low: Option<NaiveDateTime>) -> Result<Vec<models::Edge<Uuid>>, Error> {
        let mut edges: Vec<models::Edge<Uuid>> = Vec::new();

        match low {
            Some(low) => {
                // Round down since we only have second accuracy
                let fuzzy_low = NaiveDateTime::from_timestamp(low.timestamp(), 0);

                for item in iterator {
                    let edge = item?;

                    if edge.update_datetime < fuzzy_low {
                        break;
                    } else {
                        edges.push(edge);
                    }
                }
            },
            None => {
                for item in iterator {
                    let edge = item?;
                    edges.push(edge);
                }
            }
        }

        Ok(edges)
    }

    fn check_write_permissions(&self, id: Uuid, not_found_err: Error) -> Result<(), Error> {
        let vertex_manager = VertexManager::new(self.db.clone());
        let vertex_value = vertex_manager.get(id)?;

        match vertex_value {
            None => Err(not_found_err),
            Some(vertex_value) => {
                if vertex_value.owner_id != self.account_id {
                    Err(Error::Unauthorized)
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl Transaction<Uuid> for RocksdbTransaction {
    fn get_vertex_range(&self, start_id: Uuid, limit: u16) -> Result<Vec<models::Vertex<Uuid>>, Error> {
        let vertex_manager = VertexManager::new(self.db.clone());
        let iterator = vertex_manager.iterate_for_range(start_id)?;

        let mapped = iterator.take(limit as usize).map(move |item| {
            let (id, value) = item?;
            let vertex = models::Vertex::new(id, value.t);
            Ok(vertex)
        });

        let result: Result<Vec<models::Vertex<Uuid>>, Error> = mapped.collect();
        result
    }

    fn get_vertex(&self, id: Uuid) -> Result<models::Vertex<Uuid>, Error> {
        match VertexManager::new(self.db.clone()).get(id)? {
            Some(value) => {
                let vertex = models::Vertex::new(id, value.t);
                Ok(vertex)
            }
            None => Err(Error::VertexNotFound),
        }
    }

    fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error> {
        VertexManager::new(self.db.clone()).create(t, self.account_id.clone())
    }

    fn set_vertex(&self, vertex: models::Vertex<Uuid>) -> Result<(), Error> {
        self.check_write_permissions(vertex.id, Error::VertexNotFound)?;
        let value = VertexValue::new(self.account_id, vertex.t);
        VertexManager::new(self.db.clone()).update(vertex.id, &value)
    }

    fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
        self.check_write_permissions(id, Error::VertexNotFound)?;
        let mut batch = WriteBatch::default();
        VertexManager::new(self.db.clone()).delete(&mut batch, id)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_types(&self, id: Uuid) -> Result<HashSet<models::Type>, Error> {
        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        let iterator = edge_range_manager.iterate_for_owner(id)?;
        let mut result = HashSet::new();

        for item in iterator {
            let ((_, edge_t, _, _), _) = item?;
            result.insert(edge_t);
        }

        Ok(result)
    }

    fn get_edge(&self,
                outbound_id: Uuid,
                t: models::Type,
                inbound_id: Uuid)
                -> Result<models::Edge<Uuid>, Error> {
        match EdgeManager::new(self.db.clone()).get(outbound_id, &t, inbound_id)? {
            Some(value) => {
                let datetime = NaiveDateTime::from_timestamp(value.update_timestamp, 0);
                Ok(models::Edge::new(outbound_id, t, inbound_id, value.weight, datetime))
            },
            None => Err(Error::EdgeNotFound),
        }
    }

    fn set_edge(&self, edge: models::Edge<Uuid>) -> Result<(), Error> {
        // Verify that the vertices exist and that we own the vertex with the outbound ID
        self.check_write_permissions(edge.outbound_id, Error::VertexNotFound)?;
        if !VertexManager::new(self.db.clone()).exists(edge.inbound_id)? {
            return Err(Error::VertexNotFound);
        }

        let new_update_datetime = UTC::now().naive_utc();
        let mut batch = WriteBatch::default();
        EdgeManager::new(self.db.clone()).set(&mut batch,
                                                   edge.outbound_id,
                                                   &edge.t,
                                                   edge.inbound_id,
                                                   new_update_datetime,
                                                   edge.weight)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<(), Error> {
        // Verify that the edge exists and that we own it
        let edge_manager = EdgeManager::new(self.db.clone());

        match edge_manager.get(outbound_id, &t, inbound_id)? {
            Some(value) => {
                self.check_write_permissions(outbound_id, Error::EdgeNotFound)?;
                let update_datetime = NaiveDateTime::from_timestamp(value.update_timestamp, 0);
                let mut batch = WriteBatch::default();
                edge_manager.delete(&mut batch, outbound_id, &t, inbound_id, update_datetime)?;
                self.db.write(batch)?;
                Ok(())
            }
            None => Err(Error::EdgeNotFound),
        }
    }

    fn get_edge_count(&self, outbound_id: Uuid, t: models::Type) -> Result<u64, Error> {
        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        self.handle_get_edge_count(edge_range_manager, outbound_id, t)
    }

    fn get_edge_range(&self, outbound_id: Uuid, t: models::Type, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        if offset > usize::MAX as u64 {
            return Err(Error::Unexpected("Offset out of range".to_string()));
        }

        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        let iterator = edge_range_manager.iterate_for_range(outbound_id, &t, None)?;

        let mapped = iterator.skip(offset as usize).take(limit as usize).map(move |item| {
            let ((edge_range_outbound_id, edge_range_t, edge_range_update_timestamp, edge_range_inbound_id),
                 edge_range_weight) = item?;
            debug_assert_eq!(edge_range_outbound_id, outbound_id);
            debug_assert_eq!(edge_range_t, t);
            Ok(models::Edge::new(
                edge_range_outbound_id,
                edge_range_t,
                edge_range_inbound_id,
                edge_range_weight,
                edge_range_update_timestamp
            ))
        });

        let result: Result<Vec<models::Edge<Uuid>>, Error> = mapped.collect();
        result
    }

    fn get_edge_time_range(&self, outbound_id: Uuid, t: models::Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        let iterator = edge_range_manager.iterate_for_range(outbound_id, &t, high)?;

        let mapped = iterator.take(limit as usize).map(move |item| {
            let ((edge_range_outbound_id,
                  edge_range_t,
                  edge_range_update_datetime,
                  edge_range_inbound_id),
                 edge_range_weight) = item?;
            debug_assert_eq!(edge_range_outbound_id, outbound_id);
            debug_assert_eq!(edge_range_t, t);
            Ok((models::Edge::new(
                edge_range_outbound_id,
                edge_range_t,
                edge_range_inbound_id,
                edge_range_weight,
                edge_range_update_datetime
            )))
        });

        self.handle_get_edge_time_range(Box::new(mapped), low)
    }

    fn get_reversed_edge_count(&self, inbound_id: Uuid, t: models::Type) -> Result<u64, Error> {
        let edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        self.handle_get_edge_count(edge_range_manager, inbound_id, t)
    }

    fn get_reversed_edge_range(&self, inbound_id: Uuid, t: models::Type, offset: u64, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        if offset > usize::MAX as u64 {
            return Err(Error::Unexpected("Offset out of range".to_string()));
        }

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        let iterator = reversed_edge_range_manager.iterate_for_range(inbound_id, &t, None)?;

        let mapped = iterator.skip(offset as usize).take(limit as usize).map(move |item| {
            let ((edge_range_inbound_id, edge_range_t, edge_range_update_timestamp, edge_range_outbound_id),
                 edge_range_weight) = item?;
            debug_assert_eq!(edge_range_inbound_id, inbound_id);
            debug_assert_eq!(edge_range_t, t);
            Ok(models::Edge::new(
                edge_range_outbound_id,
                edge_range_t,
                edge_range_inbound_id,
                edge_range_weight,
                edge_range_update_timestamp
            ))
        });

        let result: Result<Vec<models::Edge<Uuid>>, Error> = mapped.collect();
        result
    }

    fn get_reversed_edge_time_range(&self, inbound_id: Uuid, t: models::Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<models::Edge<Uuid>>, Error> {
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        let iterator = reversed_edge_range_manager.iterate_for_range(inbound_id, &t, high)?;

        let mapped = iterator.take(limit as usize).map(move |item| {
            let ((edge_range_inbound_id,
                  edge_range_t,
                  edge_range_update_datetime,
                  edge_range_outbound_id),
                 edge_range_weight) = item?;
            debug_assert_eq!(edge_range_inbound_id, inbound_id);
            debug_assert_eq!(edge_range_t, t);
            Ok((models::Edge::new(
                edge_range_outbound_id,
                edge_range_t,
                edge_range_inbound_id,
                edge_range_weight,
                edge_range_update_datetime
            )))
        });

        self.handle_get_edge_time_range(Box::new(mapped), low)
    }

    fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
        let manager = GlobalMetadataManager::new(self.db.clone());
        manager.get(&key[..])?.ok_or_else(|| Error::MetadataNotFound)
    }

    fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
        let manager = GlobalMetadataManager::new(self.db.clone());
        manager.set(&key[..], &value)
    }

    fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
        let mut batch = WriteBatch::default();
        GlobalMetadataManager::new(self.db.clone()).delete(&mut batch, &key[..])?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
        if !AccountManager::new(self.db.clone()).exists(owner_id)? {
            return Err(Error::AccountNotFound);
        }

        let manager = AccountMetadataManager::new(self.db.clone());
        manager.get(owner_id, &key[..])?.ok_or_else(|| Error::MetadataNotFound)
    }

    fn set_account_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
        if !AccountManager::new(self.db.clone()).exists(owner_id)? {
            return Err(Error::AccountNotFound);
        }

        let manager = AccountMetadataManager::new(self.db.clone());
        manager.set(owner_id, &key[..], &value)?;
        Ok(())
    }

    fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
        let manager = AccountMetadataManager::new(self.db.clone());

        if !manager.exists(owner_id, &key)? {
            return Err(Error::MetadataNotFound);
        }

        let mut batch = WriteBatch::default();
        manager.delete(&mut batch, owner_id, &key[..])?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
        if !VertexManager::new(self.db.clone()).exists(owner_id)? {
            return Err(Error::VertexNotFound);
        }

        let manager = VertexMetadataManager::new(self.db.clone());
        manager.get(owner_id, &key[..])?.ok_or_else(|| Error::MetadataNotFound)
    }

    fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
        if !VertexManager::new(self.db.clone()).exists(owner_id)? {
            return Err(Error::VertexNotFound);
        }

        let manager = VertexMetadataManager::new(self.db.clone());
        manager.set(owner_id, &key[..], &value)
    }

    fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
        let manager = VertexMetadataManager::new(self.db.clone());

        if !manager.exists(owner_id, &key)? {
            return Err(Error::MetadataNotFound);
        }

        let mut batch = WriteBatch::default();
        manager.delete(&mut batch, owner_id, &key[..])?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
        if !EdgeManager::new(self.db.clone()).exists(outbound_id, &t, inbound_id)? {
            return Err(Error::EdgeNotFound);
        }

        let manager = EdgeMetadataManager::new(self.db.clone());
        manager.get(outbound_id, &t, inbound_id, &key[..])?
            .ok_or_else(|| Error::MetadataNotFound)
    }

    fn set_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
        if !EdgeManager::new(self.db.clone()).exists(outbound_id, &t, inbound_id)? {
            return Err(Error::EdgeNotFound);
        }

        let manager = EdgeMetadataManager::new(self.db.clone());
        manager.set(outbound_id, &t, inbound_id, &key[..], &value)
    }

    fn delete_edge_metadata(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, key: String) -> Result<(), Error> {
        let manager = EdgeMetadataManager::new(self.db.clone());

        if !manager.exists(outbound_id, &t, inbound_id, &key)? {
            return Err(Error::MetadataNotFound);
        }

        let mut batch = WriteBatch::default();
        manager.delete(&mut batch, outbound_id, &t, inbound_id, &key[..])?;
        self.db.write(batch)?;
        Ok(())
    }

    fn commit(self) -> Result<(), Error> {
        Ok(())
    }

    fn rollback(self) -> Result<(), Error> {
        Err(Error::Unexpected(
            "Transactions cannot be rolled back in the rocksdb datastore implementation"
        .to_string()))
    }
}
