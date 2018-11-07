use super::super::{Datastore, EdgeDirection, EdgeQuery, Transaction, VertexQuery, VertexPropertyQuery, EdgePropertyQuery};
use super::managers::*;
use chrono::offset::Utc;
use errors::Result;
use models;
use rocksdb::{DBCompactionStyle, Options, WriteBatch, DB, WriteOptions};
use serde_json::Value as JsonValue;
use std::i32;
use std::sync::Arc;
use std::u64;
use std::usize;
use util::next_uuid;
use uuid::Uuid;

const CF_NAMES: [&str; 6] = [
    "vertices:v1",
    "edges:v1",
    "edge_ranges:v1",
    "reversed_edge_ranges:v1",
    "vertex_properties:v1",
    "edge_properties:v1",
];

fn get_options(max_open_files: Option<i32>, bulk_load_optimized: bool) -> Options {
    // Current tuning based off of the total ordered example, flash
    // storage example on
    // https://github.com/facebook/rocksdb/wiki/RocksDB-Tuning-Guide
    // Some of the options for it were not available
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_compaction_style(DBCompactionStyle::Level);
    opts.set_write_buffer_size(67_108_864); // 64mb
    opts.set_max_write_buffer_number(3);
    opts.set_target_file_size_base(67_108_864); // 64mb
    opts.set_level_zero_file_num_compaction_trigger(8);
    opts.set_level_zero_slowdown_writes_trigger(17);
    opts.set_level_zero_stop_writes_trigger(24);
    opts.set_num_levels(4);
    opts.set_max_bytes_for_level_base(536_870_912); // 512mb
    opts.set_max_bytes_for_level_multiplier(8.0);
    opts.set_max_background_compactions(4);

    if let Some(max_open_files) = max_open_files {
        opts.set_max_open_files(max_open_files);
    }

    if bulk_load_optimized {
        // Via https://github.com/facebook/rocksdb/wiki/RocksDB-FAQ
        opts.set_allow_concurrent_memtable_write(false);
        // opts.set_memtable_factory(MemtableFactory::Vector); // disabled as this seems to stall writes
        opts.set_max_background_flushes(8);
        opts.set_disable_auto_compactions(true);
        opts.set_level_zero_file_num_compaction_trigger(1024);
        opts.set_level_zero_slowdown_writes_trigger(1024 * 5);
        opts.set_level_zero_stop_writes_trigger(1024 * 6);
    }

    opts
}

fn remove_nones_from_iterator<I, T>(iter: I) -> impl Iterator<Item = Result<T>>
where
    I: Iterator<Item = Result<Option<T>>>,
{
    iter.filter_map(|item| match item {
        Err(err) => Some(Err(err)),
        Ok(Some(value)) => Some(Ok(value)),
        _ => None
    })
}

/// A datastore that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbDatastore {
    db: Arc<DB>,
}

impl RocksdbDatastore {
    /// Creates a new rocksdb datastore.
    ///
    /// # Arguments
    /// * `path` - The file path to the rocksdb database.
    /// * `max_open_files` - The maximum number of open files to have. If
    ///   `None`, the default will be used.
    /// * `bulk_load_optimized` - Whether to configure the database to
    ///   optimize for bulk loading, based off of suggestions from the RocksDB
    ///   FAQ.
    pub fn new(path: &str, max_open_files: Option<i32>, bulk_load_optimized: bool) -> Result<RocksdbDatastore> {
        let opts = get_options(max_open_files, bulk_load_optimized);

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

        Ok(RocksdbDatastore { db: Arc::new(db) })
    }

    /// Runs a repair operation on the rocksdb database.
    ///
    /// # Arguments
    /// * `path` - The file path to the rocksdb database.
    /// * `max_open_files` - The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn repair(path: &str, max_open_files: Option<i32>) -> Result<()> {
        let opts = get_options(max_open_files, false);
        DB::repair(opts, path)?;
        Ok(())
    }
}

impl Datastore for RocksdbDatastore {
    type Trans = RocksdbTransaction;

    // We override the default `bulk_insert` implementation because further
    // optimization can be done by using `WriteBatch`s.
    fn bulk_insert<I>(&self, items: I) -> Result<()>
    where
        I: Iterator<Item = models::BulkInsertItem>,
    {
        let vertex_manager = VertexManager::new(self.db.clone());
        let edge_manager = EdgeManager::new(self.db.clone());
        let vertex_property_manager = VertexPropertyManager::new(self.db.clone());
        let edge_property_manager = EdgePropertyManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in items {
            match item {
                models::BulkInsertItem::Vertex(ref vertex) => {
                    vertex_manager.create(&mut batch, vertex)?;
                }
                models::BulkInsertItem::Edge(ref key) => {
                    edge_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, Utc::now())?;
                }
                models::BulkInsertItem::VertexProperty(id, ref name, ref value) => {
                    vertex_property_manager.set(&mut batch, id, name, value)?;
                }
                models::BulkInsertItem::EdgeProperty(ref key, ref name, ref value) => {
                    edge_property_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, name, value)?;
                }
            }
        }

        // NOTE: syncing and WAL are disabled for bulk inserts to maximimze
        // performance
        let mut opts = WriteOptions::default();
        opts.set_sync(false);
        opts.disable_wal(true);

        self.db.write_opt(batch, &opts)?;
        Ok(())
    }

    fn transaction(&self) -> Result<Self::Trans> {
        RocksdbTransaction::new(self.db.clone())
    }
}

/// A transaction that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbTransaction {
    db: Arc<DB>,
}

impl RocksdbTransaction {
    fn new(db: Arc<DB>) -> Result<Self> {
        Ok(RocksdbTransaction { db })
    }

    fn vertex_query_to_iterator(&self, q: VertexQuery) -> Result<Box<dyn Iterator<Item=Result<VertexItem>>>> {
        match q {
            VertexQuery::Range(q) => {
                let vertex_manager = VertexManager::new(self.db.clone());

                let next_uuid = match q.start_id {
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

                let mut iter: Box<dyn Iterator<Item=Result<VertexItem>>> = Box::new(vertex_manager.iterate_for_range(next_uuid)?);

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |item| match item {
                        Ok((_, v)) => v == t,
                        Err(_) => true
                    }));
                }

                let results: Vec<Result<VertexItem>> = iter.take(q.limit as usize).collect();
                Ok(Box::new(results.into_iter()))
            }
            VertexQuery::Specific(q) => {
                let vertex_manager = VertexManager::new(self.db.clone());

                let iter = q.ids.into_iter().map(move |id| match vertex_manager.get(id)? {
                    Some(value) => Ok(Some((id, value))),
                    None => Ok(None)
                });
                
                Ok(Box::new(remove_nones_from_iterator(iter)))
            }
            VertexQuery::Pipe(q) => {
                let vertex_manager = VertexManager::new(self.db.clone());
                let edge_iterator = self.edge_query_to_iterator(*q.inner)?;
                let direction = q.direction;

                let iter = edge_iterator.map(move |item| {
                    let (outbound_id, _, _, inbound_id) = item?;

                    let id = match direction {
                        EdgeDirection::Outbound => outbound_id,
                        EdgeDirection::Inbound => inbound_id,
                    };

                    match vertex_manager.get(id)? {
                        Some(value) => Ok(Some((id, value))),
                        None => Ok(None)
                    }
                });

                let mut iter: Box<dyn Iterator<Item=Result<VertexItem>>> = Box::new(remove_nones_from_iterator(iter));

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |item| match item {
                        Ok((_, v)) => v == t,
                        Err(_) => true
                    }));
                }

                let results: Vec<Result<VertexItem>> = iter.take(q.limit as usize).collect();
                Ok(Box::new(results.into_iter()))
            }
        }
    }

    fn edge_query_to_iterator(&self, q: EdgeQuery) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>>>> {
        match q {
            EdgeQuery::Specific(q) => {
                let edge_manager = EdgeManager::new(self.db.clone());

                let edges = q.keys.into_iter().map(move |key| {
                    match edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                        Some(update_datetime) => {
                            Ok(Some((key.outbound_id, key.t.clone(), update_datetime, key.inbound_id)))
                        }
                        None => Ok(None),
                    }
                });

                let iterator = remove_nones_from_iterator(edges);
                Ok(Box::new(iterator))
            }
            EdgeQuery::Pipe(q) => {
                let vertex_iterator = self.vertex_query_to_iterator(*q.inner)?;

                let edge_range_manager = match q.direction {
                    EdgeDirection::Outbound => EdgeRangeManager::new(self.db.clone()),
                    EdgeDirection::Inbound => EdgeRangeManager::new_reversed(self.db.clone()),
                };

                // Ideally we'd use iterators all the way down, but things
                // start breaking apart due to conditional expressions not
                // returning the same type signature, issues with `Result`s
                // and some of the iterators, etc. So at this point, we'll
                // just resort to building a vector.
                let mut edges: Vec<Result<EdgeRangeItem>> = Vec::new();

                for item in vertex_iterator {
                    let (id, _) = item?;
                    let edge_iterator = edge_range_manager.iterate_for_range(id, q.t.as_ref(), q.high)?;

                    for item in edge_iterator {
                        match item {
                            Ok((
                                edge_range_first_id,
                                edge_range_t,
                                edge_range_update_datetime,
                                edge_range_second_id,
                            )) => {
                                if let Some(low) = q.low {
                                    if edge_range_update_datetime < low {
                                        break;
                                    }
                                }

                                edges.push(match q.direction {
                                    EdgeDirection::Outbound => Ok((
                                        edge_range_first_id,
                                        edge_range_t,
                                        edge_range_update_datetime,
                                        edge_range_second_id,
                                    )),
                                    EdgeDirection::Inbound => Ok((
                                        edge_range_second_id,
                                        edge_range_t,
                                        edge_range_update_datetime,
                                        edge_range_first_id,
                                    )),
                                })
                            }
                            Err(_) => edges.push(item),
                        }

                        if edges.len() == q.limit as usize {
                            break;
                        }
                    }
                }

                Ok(Box::new(edges.into_iter()))
            }
        }
    }
}

impl Transaction for RocksdbTransaction {
    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool> {
        let vertex_manager = VertexManager::new(self.db.clone());

        if vertex_manager.exists(vertex.id)? {
            Ok(false)
        } else {
            let mut batch = WriteBatch::default();
            vertex_manager.create(&mut batch, vertex)?;
            self.db.write(batch)?;
            Ok(true)
        }
    }

    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>> {
        let iterator = self.vertex_query_to_iterator(q.into())?;

        let mapped = iterator.map(move |item| {
            let (id, t) = item?;
            let vertex = models::Vertex::with_id(id, t);
            Ok(vertex)
        });

        mapped.collect()
    }

    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<()> {
        let iterator = self.vertex_query_to_iterator(q.into())?;
        let vertex_manager = VertexManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in iterator {
            let (id, _) = item?;
            vertex_manager.delete(&mut batch, id)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let vertex_manager = VertexManager::new(self.db.clone());
        let iterator = vertex_manager.iterate_for_range(Uuid::default())?;
        Ok(iterator.count() as u64)
    }

    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool> {
        let vertex_manager = VertexManager::new(self.db.clone());

        if !vertex_manager.exists(key.outbound_id)? || !vertex_manager.exists(key.inbound_id)? {
            Ok(false)
        } else {
            let edge_manager = EdgeManager::new(self.db.clone());
            let mut batch = WriteBatch::default();
            edge_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, Utc::now())?;
            self.db.write(batch)?;
            Ok(true)
        }
    }

    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>> {
        let iterator = self.edge_query_to_iterator(q.into())?;

        let mapped = iterator.map(move |item| {
            let (outbound_id, t, update_datetime, inbound_id) = item?;
            let key = models::EdgeKey::new(outbound_id, t, inbound_id);
            let edge = models::Edge::new(key, update_datetime);
            Ok(edge)
        });

        mapped.collect()
    }

    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<()> {
        let edge_manager = EdgeManager::new(self.db.clone());
        let vertex_manager = VertexManager::new(self.db.clone());
        let iterator = self.edge_query_to_iterator(q.into())?;
        let mut batch = WriteBatch::default();

        for item in iterator {
            let (outbound_id, t, update_datetime, inbound_id) = item?;

            if vertex_manager.get(outbound_id)?.is_some() {
                edge_manager.delete(&mut batch, outbound_id, &t, inbound_id, update_datetime)?;
            };
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_count(
        &self,
        id: Uuid,
        t: Option<&models::Type>,
        direction: models::EdgeDirection,
    ) -> Result<u64> {
        let edge_range_manager = match direction {
            EdgeDirection::Outbound => EdgeRangeManager::new(self.db.clone()),
            EdgeDirection::Inbound => EdgeRangeManager::new_reversed(self.db.clone()),
        };

        let count = edge_range_manager.iterate_for_range(id, t, None)?.count();

        Ok(count as u64)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<models::VertexProperty>> {
        let manager = VertexPropertyManager::new(self.db.clone());
        let mut properties = Vec::new();

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            let value = manager.get(id, &q.name)?;

            if let Some(value) = value {
                properties.push(models::VertexProperty::new(id, value));
            }
        }

        Ok(properties)
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let manager = VertexPropertyManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            manager.set(&mut batch, id, &q.name, value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let manager = VertexPropertyManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            manager.delete(&mut batch, id, &q.name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        let manager = EdgePropertyManager::new(self.db.clone());
        let mut properties = Vec::new();

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, _, inbound_id) = item?;
            let value = manager.get(outbound_id, &t, inbound_id, &q.name)?;

            if let Some(value) = value {
                let key = models::EdgeKey::new(outbound_id, t, inbound_id);
                properties.push(models::EdgeProperty::new(key, value));
            }
        }

        Ok(properties)
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let manager = EdgePropertyManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, _, inbound_id) = item?;
            manager.set(&mut batch, outbound_id, &t, inbound_id, &q.name, value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let manager = EdgePropertyManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, _, inbound_id) = item?;
            manager.delete(&mut batch, outbound_id, &t, inbound_id, &q.name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }
}
