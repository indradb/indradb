use super::super::{Datastore, EdgeQuery, QueryTypeConverter, Transaction, VertexQuery};
use models;
use uuid::Uuid;
use errors::Error;
use util::{get_salted_hash, next_uuid};
use serde_json::Value as JsonValue;
use chrono::offset::Utc;
use rocksdb::{DBCompactionStyle, Options, WriteBatch, DB};
use std::sync::Arc;
use std::usize;
use std::i32;
use std::u64;
use super::managers::*;
use core::fmt::Debug;
use std::collections::HashMap;

const CF_NAMES: [&'static str; 9] = [
    "accounts:v1",
    "vertices:v1",
    "edges:v1",
    "edge_ranges:v1",
    "reversed_edge_ranges:v1",
    "global_metadata:v1",
    "account_metadata:v1",
    "vertex_metadata:v1",
    "edge_metadata:v1",
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

/// A datastore that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbDatastore {
    /// A reference to the rocksdb database.
    db: Arc<DB>,
    /// Whether to use secure UUIDs.
    secure_uuids: bool,
}

impl RocksdbDatastore {
    /// Creates a new rocksdb datastore.
    ///
    /// # Arguments
    /// * `path` - The file path to the rocksdb database.
    /// * `max_open_files` - The maximum number of open files to have. If
    ///   `None`, the default will be used.
    /// * `secure_uuids` - If true, UUIDv4 will be used, which will result in
    ///   difficult to guess UUIDs at the detriment of a more index-optimized
    ///   (and thus faster) variant.
    pub fn new(
        path: &str,
        max_open_files: Option<i32>,
        secure_uuids: bool,
    ) -> Result<RocksdbDatastore, Error> {
        let opts = get_options(max_open_files);

        let db = match DB::open_cf(&opts, path, &CF_NAMES) {
            Ok(db) => db,
            Err(_) => {
                let mut db = DB::open(&opts, path)?;

                for cf_name in &CF_NAMES {
                    db.create_cf(cf_name, &opts)?;
                }

                db
            }
        };

        Ok(RocksdbDatastore {
            db: Arc::new(db),
            secure_uuids: secure_uuids,
        })
    }

    /// Runs a repair operation on the rocksdb database.
    ///
    /// # Arguments
    /// * `path` - The file path to the rocksdb database.
    /// * `max_open_files` - The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn repair(path: &str, max_open_files: Option<i32>) -> Result<(), Error> {
        let opts = get_options(max_open_files);
        DB::repair(opts, path)?;
        Ok(())
    }
}

impl Datastore<RocksdbTransaction> for RocksdbDatastore {
    fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
        AccountManager::new(self.db.clone(), self.secure_uuids).exists(account_id)
    }

    fn create_account(&self) -> Result<(Uuid, String), Error> {
        AccountManager::new(self.db.clone(), self.secure_uuids).create()
    }

    fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
        let manager = AccountManager::new(self.db.clone(), self.secure_uuids);

        if !manager.exists(account_id)? {
            return Err(Error::AccountNotFound);
        }

        let mut batch = WriteBatch::default();
        manager.delete(&mut batch, account_id)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
        match AccountManager::new(self.db.clone(), self.secure_uuids).get(account_id)? {
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
        RocksdbTransaction::new(self.db.clone(), account_id, self.secure_uuids)
    }
}

/// A transaction that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbTransaction {
    /// A reference to the rocksdb database.
    db: Arc<DB>,
    /// The ID of the account that's triggering this transaction.
    account_id: Uuid,
    /// Whether to use secure UUIDs.
    secure_uuids: bool,
}

impl RocksdbTransaction {
    fn new(db: Arc<DB>, account_id: Uuid, secure_uuids: bool) -> Result<Self, Error> {
        Ok(RocksdbTransaction {
            db: db,
            account_id: account_id,
            secure_uuids: secure_uuids,
        })
    }

    fn check_write_permissions(&self, id: Uuid, not_found_err: Error) -> Result<(), Error> {
        let vertex_manager = VertexManager::new(self.db.clone(), self.secure_uuids);
        let vertex_value = vertex_manager.get(id)?;

        match vertex_value {
            None => Err(not_found_err),
            Some(vertex_value) => if vertex_value.owner_id != self.account_id {
                Err(Error::Unauthorized)
            } else {
                Ok(())
            },
        }
    }

    fn vertex_query_to_iterator(
        &self,
        q: VertexQuery,
    ) -> Result<Box<Iterator<Item = VertexItem>>, Error> {
        let vertex_manager = VertexManager::new(self.db.clone(), self.secure_uuids);

        match q {
            VertexQuery::All { start_id, limit } => {
                let next_uuid = match start_id {
                    Some(start_id) => {
                        match next_uuid(start_id) {
                            Ok(next_uuid) => next_uuid,
                            // If we get an error back, it's because
                            // `start_id` is the maximum possible value. We
                            // know that no vertices exist whose ID is greater
                            // than the maximum possible value, so just return
                            // an empty list.
                            Err(_) => return Ok(Box::new(vec![].into_iter())),
                        }
                    }
                    None => Uuid::default(),
                };

                let iterator = vertex_manager.iterate_for_range(next_uuid)?;
                Ok(Box::new(iterator.take(limit as usize)))
            }
            VertexQuery::Vertices { ids } => {
                let iterator = Box::new(ids.into_iter().map(|item| Ok(item)));

                Ok(self.handle_vertex_id_iterator(iterator))
            }
            VertexQuery::Pipe {
                edge_query,
                converter,
                limit,
            } => {
                let edge_iterator = self.edge_query_to_iterator(*edge_query)?;

                let vertex_id_iterator = Box::new(edge_iterator.map(move |item| {
                    let ((outbound_id, _, _, inbound_id), _) = item?;

                    match converter {
                        QueryTypeConverter::Outbound => Ok(outbound_id),
                        QueryTypeConverter::Inbound => Ok(inbound_id),
                    }
                }));

                Ok(Box::new(
                    self.handle_vertex_id_iterator(vertex_id_iterator)
                        .take(limit as usize),
                ))
            }
        }
    }

    fn edge_query_to_iterator(
        &self,
        q: EdgeQuery,
    ) -> Result<Box<Iterator<Item = EdgeRangeItem>>, Error> {
        match q {
            EdgeQuery::Edges { keys } => {
                let edge_manager = EdgeManager::new(self.db.clone());

                let iterator = keys.into_iter().map(move |key| {
                    match edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                        Some(value) => Ok(Some((
                            (
                                key.outbound_id,
                                key.t,
                                value.update_datetime,
                                key.inbound_id,
                            ),
                            value.weight,
                        ))),
                        None => Ok(None),
                    }
                });

                Ok(self.remove_nones_from_iterator(Box::new(iterator)))
            }
            EdgeQuery::Pipe {
                vertex_query,
                converter,
                type_filter,
                high_filter,
                low_filter,
                limit,
            } => {
                let vertex_iterator = self.vertex_query_to_iterator(*vertex_query)?;

                let edge_range_manager = match converter {
                    QueryTypeConverter::Outbound => EdgeRangeManager::new(self.db.clone()),
                    QueryTypeConverter::Inbound => EdgeRangeManager::new_reversed(self.db.clone()),
                };

                // Ideally we'd use iterators all the way down, but things
                // start breaking apart due to conditional expressions not
                // returning the same type signature, issues with `Result`s
                // and some of the iterators, etc. So at this point, we'll
                // just resort to building a vector.
                let mut edges: Vec<EdgeRangeItem> = Vec::new();

                if let Some(low_filter) = low_filter {
                    for item in vertex_iterator {
                        let (id, _) = item?;
                        let edge_iterator =
                            edge_range_manager.iterate_for_range(id, &type_filter, high_filter)?;

                        for item in edge_iterator {
                            match item {
                                Ok(((_, _, edge_range_update_datetime, _), _)) => {
                                    if edge_range_update_datetime >= low_filter {
                                        edges.push(item);
                                    } else {
                                        break;
                                    }
                                }
                                Err(_) => edges.push(item),
                            }

                            if edges.len() == limit as usize {
                                break;
                            }
                        }
                    }
                } else {
                    for item in vertex_iterator {
                        let (id, _) = item?;
                        let edge_iterator =
                            edge_range_manager.iterate_for_range(id, &type_filter, high_filter)?;

                        for edge in edge_iterator {
                            edges.push(edge);

                            if edges.len() == limit as usize {
                                break;
                            }
                        }

                        if edges.len() == limit as usize {
                            break;
                        }
                    }
                }

                Ok(Box::new(edges.into_iter()))
            }
        }
    }

    fn remove_nones_from_iterator<'a, T: Debug + 'a>(
        &self,
        iterator: Box<Iterator<Item = Result<Option<T>, Error>>>,
    ) -> Box<Iterator<Item = Result<T, Error>> + 'a> {
        let filtered = iterator.filter(|item| match *item {
            Err(_) | Ok(Some(_)) => true,
            _ => false,
        });

        let mapped = filtered.map(|item| match item {
            Ok(Some(value)) => Ok(value),
            Err(err) => Err(err),
            _ => panic!("Unexpected item: {:?}", item),
        });

        Box::new(mapped)
    }

    fn handle_vertex_id_iterator(
        &self,
        iterator: Box<Iterator<Item = Result<Uuid, Error>>>,
    ) -> Box<Iterator<Item = VertexItem>> {
        let vertex_manager = VertexManager::new(self.db.clone(), self.secure_uuids);

        let mapped = iterator.map(move |item| {
            let id = item?;
            let value = vertex_manager.get(id)?;

            match value {
                Some(value) => Ok(Some((id, value))),
                None => Ok(None),
            }
        });

        self.remove_nones_from_iterator(Box::new(mapped))
    }
}

impl Transaction for RocksdbTransaction {
    fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error> {
        VertexManager::new(self.db.clone(), self.secure_uuids).create(t, self.account_id)
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<models::Vertex>, Error> {
        let iterator = self.vertex_query_to_iterator(q)?;

        let mapped = iterator.map(move |item| {
            let (id, value) = item?;
            let vertex = models::Vertex::new(id, value.t);
            Ok(vertex)
        });

        mapped.collect()
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let iterator = self.vertex_query_to_iterator(q)?;
        let vertex_manager = VertexManager::new(self.db.clone(), self.secure_uuids);
        let mut batch = WriteBatch::default();

        for item in iterator {
            let (id, old_value) = item?;

            if old_value.owner_id != self.account_id {
                continue;
            }

            vertex_manager.delete(&mut batch, id)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn create_edge(&self, key: models::EdgeKey, weight: models::Weight) -> Result<(), Error> {
        // Verify that the vertices exist and that we own the vertex with the outbound ID
        self.check_write_permissions(key.outbound_id, Error::VertexNotFound)?;
        if !VertexManager::new(self.db.clone(), self.secure_uuids).exists(key.inbound_id)? {
            return Err(Error::VertexNotFound);
        }

        let new_update_datetime = Utc::now();
        let mut batch = WriteBatch::default();
        EdgeManager::new(self.db.clone()).set(
            &mut batch,
            key.outbound_id,
            &key.t,
            key.inbound_id,
            new_update_datetime,
            weight,
        )?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<models::Edge>, Error> {
        let iterator = self.edge_query_to_iterator(q)?;

        let mapped = iterator.map(move |item| {
            let ((outbound_id, t, update_datetime, inbound_id), weight) = item?;
            let key = models::EdgeKey::new(outbound_id, t, inbound_id);
            let edge = models::Edge::new(key, weight, update_datetime);
            Ok(edge)
        });

        mapped.collect()
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let edge_manager = EdgeManager::new(self.db.clone());
        let vertex_manager = VertexManager::new(self.db.clone(), self.secure_uuids);
        let iterator = self.edge_query_to_iterator(q)?;
        let mut batch = WriteBatch::default();

        for item in iterator {
            let ((outbound_id, t, update_datetime, inbound_id), _) = item?;

            if let Some(vertex_value) = vertex_manager.get(outbound_id)? {
                if vertex_value.owner_id == self.account_id {
                    edge_manager.delete(&mut batch, outbound_id, &t, inbound_id, update_datetime)?;
                }
            };
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let iterator = self.edge_query_to_iterator(q)?;
        Ok(iterator.count() as u64)
    }

    fn get_global_metadata(&self, name: String) -> Result<JsonValue, Error> {
        let manager = GlobalMetadataManager::new(self.db.clone());
        manager
            .get(&name[..])?
            .ok_or_else(|| Error::MetadataNotFound)
    }

    fn set_global_metadata(&self, name: String, value: JsonValue) -> Result<(), Error> {
        let manager = GlobalMetadataManager::new(self.db.clone());
        manager.set(&name[..], &value)
    }

    fn delete_global_metadata(&self, name: String) -> Result<(), Error> {
        let mut batch = WriteBatch::default();
        GlobalMetadataManager::new(self.db.clone()).delete(&mut batch, &name[..])?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_account_metadata(&self, owner_id: Uuid, name: String) -> Result<JsonValue, Error> {
        if !AccountManager::new(self.db.clone(), self.secure_uuids).exists(owner_id)? {
            return Err(Error::AccountNotFound);
        }

        let manager = AccountMetadataManager::new(self.db.clone());
        manager
            .get(owner_id, &name[..])?
            .ok_or_else(|| Error::MetadataNotFound)
    }

    fn set_account_metadata(
        &self,
        owner_id: Uuid,
        name: String,
        value: JsonValue,
    ) -> Result<(), Error> {
        if !AccountManager::new(self.db.clone(), self.secure_uuids).exists(owner_id)? {
            return Err(Error::AccountNotFound);
        }

        let manager = AccountMetadataManager::new(self.db.clone());
        manager.set(owner_id, &name[..], &value)?;
        Ok(())
    }

    fn delete_account_metadata(&self, owner_id: Uuid, name: String) -> Result<(), Error> {
        let manager = AccountMetadataManager::new(self.db.clone());

        if !manager.exists(owner_id, &name)? {
            return Err(Error::MetadataNotFound);
        }

        let mut batch = WriteBatch::default();
        manager.delete(&mut batch, owner_id, &name[..])?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_vertex_metadata(
        &self,
        q: VertexQuery,
        name: String,
    ) -> Result<HashMap<Uuid, JsonValue>, Error> {
        let manager = VertexMetadataManager::new(self.db.clone());
        let mut metadata: HashMap<Uuid, JsonValue> = HashMap::new();

        for item in self.vertex_query_to_iterator(q)? {
            let (id, _) = item?;
            let value = manager.get(id, &name[..])?;

            if let Some(value) = value {
                metadata.insert(id, value);
            }
        }

        Ok(metadata)
    }

    fn set_vertex_metadata(
        &self,
        q: VertexQuery,
        name: String,
        value: JsonValue,
    ) -> Result<(), Error> {
        let manager = VertexMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.vertex_query_to_iterator(q)? {
            let (id, _) = item?;
            manager.set(&mut batch, id, &name[..], &value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_metadata(&self, q: VertexQuery, name: String) -> Result<(), Error> {
        let manager = VertexMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.vertex_query_to_iterator(q)? {
            let (id, _) = item?;
            manager.delete(&mut batch, id, &name[..])?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_metadata(
        &self,
        q: EdgeQuery,
        name: String,
    ) -> Result<HashMap<models::EdgeKey, JsonValue>, Error> {
        let manager = EdgeMetadataManager::new(self.db.clone());
        let mut metadata: HashMap<models::EdgeKey, JsonValue> = HashMap::new();

        for item in self.edge_query_to_iterator(q)? {
            let ((outbound_id, t, _, inbound_id), _) = item?;
            let value = manager.get(outbound_id, &t, inbound_id, &name[..])?;

            if let Some(value) = value {
                let key = models::EdgeKey::new(outbound_id, t, inbound_id);
                metadata.insert(key, value);
            }
        }

        Ok(metadata)
    }

    fn set_edge_metadata(&self, q: EdgeQuery, name: String, value: JsonValue) -> Result<(), Error> {
        let manager = EdgeMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.edge_query_to_iterator(q)? {
            let ((outbound_id, t, _, inbound_id), _) = item?;
            manager.set(&mut batch, outbound_id, &t, inbound_id, &name[..], &value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge_metadata(&self, q: EdgeQuery, name: String) -> Result<(), Error> {
        let manager = EdgeMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.edge_query_to_iterator(q)? {
            let ((outbound_id, t, _, inbound_id), _) = item?;
            manager.delete(&mut batch, outbound_id, &t, inbound_id, &name[..])?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn commit(self) -> Result<(), Error> {
        Ok(())
    }

    fn rollback(self) -> Result<(), Error> {
        Err(Error::Unexpected(
            "Transactions cannot be rolled back in the rocksdb datastore implementation"
                .to_string(),
        ))
    }
}
