use std::i32;
use std::sync::{Arc, Mutex};
use std::u64;
use std::usize;

use super::managers::*;
use crate::errors::Result;
use crate::{
    BulkInsertItem, BulkInsertResult, Datastore, Edge, EdgeDirection, EdgeProperties, EdgeProperty, EdgePropertyQuery,
    EdgeQuery, NamedProperty, Transaction, Type, Vertex, VertexProperties, VertexProperty, VertexPropertyQuery,
    VertexQuery,
};

use rocksdb::{DBCompactionStyle, MemtableFactory, Options, WriteBatch, WriteOptions, DB};
use serde_json::Value as JsonValue;

const CF_NAMES: [&str; 7] = [
    "meta:v1",
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
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.set_compaction_style(DBCompactionStyle::Level);
    opts.set_write_buffer_size(67_108_864); // 64mb
    opts.set_max_write_buffer_number(3);
    opts.set_target_file_size_base(67_108_864); // 64mb
    opts.set_max_background_compactions(4);
    opts.set_level_zero_file_num_compaction_trigger(8);
    opts.set_level_zero_slowdown_writes_trigger(17);
    opts.set_level_zero_stop_writes_trigger(24);
    opts.set_num_levels(4);
    opts.set_max_bytes_for_level_base(536_870_912); // 512mb
    opts.set_max_bytes_for_level_multiplier(8.0);

    if let Some(max_open_files) = max_open_files {
        opts.set_max_open_files(max_open_files);
    }

    if bulk_load_optimized {
        // Via https://github.com/facebook/rocksdb/wiki/RocksDB-FAQ
        opts.set_allow_concurrent_memtable_write(false);
        opts.set_memtable_factory(MemtableFactory::Vector);
        opts.set_max_background_flushes(8);
        opts.set_disable_auto_compactions(true);
        opts.set_level_zero_file_num_compaction_trigger(1024);
        opts.set_level_zero_slowdown_writes_trigger(1024 * 5);
        opts.set_level_zero_stop_writes_trigger(1024 * 6);
    }

    opts
}

fn execute_vertex_query(db: &DB, q: VertexQuery) -> Result<Vec<VertexItem>> {
    match q {
        VertexQuery::Range(q) => {
            let vertex_manager = VertexManager::new(db);

            let last_id = match q.start_id {
                Some(start_id) => {
                    match start_id.checked_add(1) {
                        Some(last_id) => last_id,
                        // If we get an error back, it's because
                        // `start_id` is the maximum possible value. We
                        // know that no vertices exist whose ID is greater
                        // than the maximum possible value, so just return
                        // an empty list.
                        None => return Ok(vec![]),
                    }
                }
                None => 0,
            };

            let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> =
                Box::new(vertex_manager.iterate_for_range(last_id)?);

            if let Some(ref t) = q.t {
                iter = Box::new(iter.filter(move |item| match item {
                    Ok((_, v)) => v == t,
                    Err(_) => true,
                }));
            }

            let vertices: Result<Vec<VertexItem>> = iter.take(q.limit as usize).collect();
            vertices
        }
        VertexQuery::Specific(q) => {
            let vertex_manager = VertexManager::new(db);

            let iter = q.ids.into_iter().map(move |id| match vertex_manager.get(id)? {
                Some(value) => Ok(Some((id, value))),
                None => Ok(None),
            });

            let iter = iter.filter_map(|item| match item {
                Err(err) => Some(Err(err)),
                Ok(Some(value)) => Some(Ok(value)),
                _ => None,
            });

            let vertices: Result<Vec<VertexItem>> = iter.collect();
            vertices
        }
        VertexQuery::Pipe(q) => {
            let vertex_manager = VertexManager::new(db);
            let iter = execute_edge_query(db, *q.inner)?.into_iter();
            let direction = q.direction;

            let iter = iter.map(move |(out_id, _, in_id)| {
                let id = match direction {
                    EdgeDirection::Outbound => out_id,
                    EdgeDirection::Inbound => in_id,
                };

                match vertex_manager.get(id)? {
                    Some(value) => Ok(Some((id, value))),
                    None => Ok(None),
                }
            });

            let iter = iter.filter_map(|item| match item {
                Err(err) => Some(Err(err)),
                Ok(Some(value)) => Some(Ok(value)),
                _ => None,
            });

            let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> = Box::new(iter);

            if let Some(ref t) = q.t {
                iter = Box::new(iter.filter(move |item| match item {
                    Ok((_, v)) => v == t,
                    Err(_) => true,
                }));
            }

            let vertices: Result<Vec<VertexItem>> = iter.take(q.limit as usize).collect();
            vertices
        }
    }
}

fn execute_edge_query(db: &DB, q: EdgeQuery) -> Result<Vec<EdgeRangeItem>> {
    match q {
        EdgeQuery::Specific(q) => {
            let edge_manager = EdgeManager::new(&db);

            let iter = q.edges.into_iter().map(move |edge| -> Result<Option<EdgeRangeItem>> {
                if edge_manager.exists(edge.outbound_id, &edge.t, edge.inbound_id)? {
                    Ok(Some((edge.outbound_id, edge.t, edge.inbound_id)))
                } else {
                    Ok(None)
                }
            });

            let iter = iter.filter_map(|item| match item {
                Err(err) => Some(Err(err)),
                Ok(Some(value)) => Some(Ok(value)),
                _ => None,
            });

            let edges: Result<Vec<EdgeRangeItem>> = iter.collect();
            edges
        }
        EdgeQuery::Pipe(q) => {
            let vertices = execute_vertex_query(db, *q.inner)?;

            let edge_range_manager = match q.direction {
                EdgeDirection::Outbound => EdgeRangeManager::new(&db),
                EdgeDirection::Inbound => EdgeRangeManager::new_reversed(&db),
            };

            // Ideally we'd use iterators all the way down, but things
            // start breaking apart due to conditional expressions not
            // returning the same type signature, issues with `Result`s
            // and some of the iterators, etc. So at this point, we'll
            // just resort to building a vector.
            let mut edges: Vec<EdgeRangeItem> = Vec::new();

            for (id, _) in vertices.into_iter() {
                let edge_iterator = edge_range_manager.iterate_for_range(id, q.t.as_ref())?;

                for item in edge_iterator {
                    let (edge_range_first_id, edge_range_t, edge_range_second_id) = item?;

                    edges.push(match q.direction {
                        EdgeDirection::Outbound => (edge_range_first_id, edge_range_t, edge_range_second_id),
                        EdgeDirection::Inbound => (edge_range_second_id, edge_range_t, edge_range_first_id),
                    });

                    if edges.len() == q.limit as usize {
                        break;
                    }
                }
            }

            Ok(edges)
        }
    }
}

/// A datastore that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbDatastore {
    db: Arc<DB>,
    last_id: Arc<Mutex<u64>>,
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

        let meta_manager = MetaManager::new(&db);
        let last_id = meta_manager.get_last_id()?;

        Ok(RocksdbDatastore {
            db: Arc::new(db),
            last_id: Arc::new(Mutex::new(last_id)),
        })
    }

    /// Runs a repair operation on the rocksdb database.
    ///
    /// # Arguments
    /// * `path` - The file path to the rocksdb database.
    /// * `max_open_files` - The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn repair(path: &str, max_open_files: Option<i32>) -> Result<()> {
        let opts = get_options(max_open_files, false);
        DB::repair(&opts, path)?;
        Ok(())
    }
}

impl Datastore for RocksdbDatastore {
    type Trans = RocksdbTransaction;

    // We override the default `bulk_insert` implementation because further
    // optimization can be done by using `WriteBatch`s.
    fn bulk_insert<I>(&self, items: I) -> Result<BulkInsertResult>
    where
        I: Iterator<Item = BulkInsertItem>,
    {
        let db = self.db.clone();
        let vertex_manager = VertexManager::new(&db);
        let edge_manager = EdgeManager::new(&db);
        let vertex_property_manager = VertexPropertyManager::new(&db);
        let edge_property_manager = EdgePropertyManager::new(&db);
        let meta_manager = MetaManager::new(&db);
        let mut batch = WriteBatch::default();
        let mut inserted_vertices = false;
        let mut inserted_edges = false;
        let mut inserted_vertex_properties = false;
        let mut inserted_edge_properties = false;

        let id_lock = self.last_id.clone();
        let mut id = id_lock.lock().unwrap();
        let mut id_range = None;

        for item in items {
            match item {
                BulkInsertItem::Vertex(t) => {
                    *id += 1;
                    vertex_manager.create(&mut batch, &Vertex::new(*id, t))?;
                    inserted_vertices = true;
                    id_range = match id_range {
                        Some((start_id, _)) => Some((start_id, *id)),
                        None => Some((*id, *id)),
                    };
                }
                BulkInsertItem::Edge(ref edge) => {
                    edge_manager.set(&mut batch, edge.outbound_id, &edge.t, edge.inbound_id)?;
                    inserted_edges = true;
                }
                BulkInsertItem::VertexProperty(id, ref name, ref value) => {
                    vertex_property_manager.set(&mut batch, id, name, value)?;
                    inserted_vertex_properties = true;
                }
                BulkInsertItem::EdgeProperty(ref edge, ref name, ref value) => {
                    edge_property_manager.set(&mut batch, edge.outbound_id, &edge.t, edge.inbound_id, name, value)?;
                    inserted_edge_properties = true;
                }
            }
        }

        if inserted_vertices {
            meta_manager.set_last_id(&mut batch, *id)?;
        }

        // NOTE: syncing and WAL are disabled for bulk inserts to maximize
        // performance
        let mut opts = WriteOptions::default();
        opts.set_sync(false);
        opts.disable_wal(true);
        self.db.write_opt(batch, &opts)?;

        // manually compact
        if inserted_vertices {
            vertex_manager.compact();
            meta_manager.compact();
        }
        if inserted_edges {
            edge_manager.compact();
        }
        if inserted_vertex_properties {
            vertex_property_manager.compact();
        }
        if inserted_edge_properties {
            edge_property_manager.compact();
        }

        Ok(BulkInsertResult { id_range })
    }

    fn transaction(&self) -> Result<Self::Trans> {
        RocksdbTransaction::new(self.db.clone(), self.last_id.clone())
    }
}

/// A transaction that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbTransaction {
    db: Arc<DB>,
    last_id: Arc<Mutex<u64>>,
}

impl RocksdbTransaction {
    fn new(db: Arc<DB>, last_id: Arc<Mutex<u64>>) -> Result<Self> {
        Ok(RocksdbTransaction { db, last_id })
    }
}

impl Transaction for RocksdbTransaction {
    fn create_vertex(&self, t: &Type) -> Result<u64> {
        let db = self.db.clone();
        let meta_manager = MetaManager::new(&db);
        let vertex_manager = VertexManager::new(&db);

        let id = {
            let mut batch = WriteBatch::default();
            let id_lock = self.last_id.clone();
            let mut id = id_lock.lock().unwrap();
            *id += 1;
            meta_manager.set_last_id(&mut batch, *id)?;
            vertex_manager.create(&mut batch, &Vertex::new(*id, t.clone()))?;
            self.db.write(batch)?;
            *id
        };

        Ok(id)
    }

    fn get_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<Vertex>> {
        let db = self.db.clone();
        let iter = execute_vertex_query(&db, q.into())?.into_iter();

        let iter = iter.map(move |(id, t)| {
            let vertex = Vertex::new(id, t);
            Ok(vertex)
        });

        iter.collect()
    }

    fn delete_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<()> {
        let db = self.db.clone();
        let iter = execute_vertex_query(&db, q.into())?.into_iter();
        let db = self.db.clone();
        let vertex_manager = VertexManager::new(&db);
        let mut batch = WriteBatch::default();

        for (id, _) in iter {
            vertex_manager.delete(&mut batch, id)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let db = self.db.clone();
        let vertex_manager = VertexManager::new(&db);
        let iterator = vertex_manager.iterate_for_range(0)?;
        Ok(iterator.count() as u64)
    }

    fn create_edge(&self, edge: &Edge) -> Result<bool> {
        let db = self.db.clone();
        let vertex_manager = VertexManager::new(&db);

        if !vertex_manager.exists(edge.outbound_id)? || !vertex_manager.exists(edge.inbound_id)? {
            Ok(false)
        } else {
            let edge_manager = EdgeManager::new(&db);
            let mut batch = WriteBatch::default();
            edge_manager.set(&mut batch, edge.outbound_id, &edge.t, edge.inbound_id)?;
            self.db.write(batch)?;
            Ok(true)
        }
    }

    fn get_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<Edge>> {
        let db = self.db.clone();
        let iter = execute_edge_query(&db, q.into())?.into_iter();

        let iter = iter.map(move |(out_id, t, in_id)| {
            let edge = Edge::new(out_id, t, in_id);
            Ok(edge)
        });

        iter.collect()
    }

    fn delete_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<()> {
        let db = self.db.clone();
        let edge_manager = EdgeManager::new(&db);
        let vertex_manager = VertexManager::new(&db);
        let iter = execute_edge_query(&db, q.into())?;
        let mut batch = WriteBatch::default();

        for (out_id, t, in_id) in iter {
            if vertex_manager.get(out_id)?.is_some() {
                edge_manager.delete(&mut batch, out_id, &t, in_id)?;
            };
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_count(&self, id: u64, t: Option<&Type>, direction: EdgeDirection) -> Result<u64> {
        let db = self.db.clone();

        let edge_range_manager = match direction {
            EdgeDirection::Outbound => EdgeRangeManager::new(&db),
            EdgeDirection::Inbound => EdgeRangeManager::new_reversed(&db),
        };

        let count = edge_range_manager.iterate_for_range(id, t)?.count();

        Ok(count as u64)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<VertexProperty>> {
        let db = self.db.clone();
        let manager = VertexPropertyManager::new(&db);
        let mut properties = Vec::new();

        for (id, _) in execute_vertex_query(&db, q.inner)?.into_iter() {
            let value = manager.get(id, &q.name)?;

            if let Some(value) = value {
                properties.push(VertexProperty::new(id, value));
            }
        }

        Ok(properties)
    }

    fn get_all_vertex_properties<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<VertexProperties>> {
        let db = self.db.clone();
        let iter = execute_vertex_query(&db, q.into())?.into_iter();
        let manager = VertexPropertyManager::new(&db);

        let iter = iter.map(move |(id, t)| {
            let vertex = Vertex::new(id, t);

            let it = manager.iterate_for_owner(id)?;
            let props: Result<Vec<_>> = it.map(|r| r).collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, name), value)| NamedProperty::new(name, value))
                .collect();

            Ok(VertexProperties::new(vertex, props))
        });

        iter.collect()
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let db = self.db.clone();
        let manager = VertexPropertyManager::new(&db);
        let mut batch = WriteBatch::default();

        for (id, _) in execute_vertex_query(&db, q.inner)?.into_iter() {
            manager.set(&mut batch, id, &q.name, value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let db = self.db.clone();
        let manager = VertexPropertyManager::new(&db);
        let mut batch = WriteBatch::default();

        for (id, _) in execute_vertex_query(&db, q.inner)?.into_iter() {
            manager.delete(&mut batch, id, &q.name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<EdgeProperty>> {
        let db = self.db.clone();
        let manager = EdgePropertyManager::new(&db);
        let mut properties = Vec::new();

        for (out_id, t, in_id) in execute_edge_query(&db, q.inner)?.into_iter() {
            let value = manager.get(out_id, &t, in_id, &q.name)?;

            if let Some(value) = value {
                let edge = Edge::new(out_id, t, in_id);
                properties.push(EdgeProperty::new(edge, value));
            }
        }

        Ok(properties)
    }

    fn get_all_edge_properties<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<EdgeProperties>> {
        let db = self.db.clone();
        let iter = execute_edge_query(&db, q.into())?.into_iter();
        let manager = EdgePropertyManager::new(&db);

        let iter = iter.map(move |(out_id, t, in_id)| {
            let edge = Edge::new(out_id, t.clone(), in_id);
            let it = manager.iterate_for_owner(out_id, &t, in_id)?;
            let props: Result<Vec<_>> = it.map(|r| r).collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, _, _, name), value)| NamedProperty::new(name, value))
                .collect();

            Ok(EdgeProperties::new(edge, props))
        });

        iter.collect()
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let db = self.db.clone();
        let manager = EdgePropertyManager::new(&db);
        let mut batch = WriteBatch::default();

        for (out_id, t, in_id) in execute_edge_query(&db, q.inner)?.into_iter() {
            manager.set(&mut batch, out_id, &t, in_id, &q.name, value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let db = self.db.clone();
        let manager = EdgePropertyManager::new(&db);
        let mut batch = WriteBatch::default();

        for (out_id, t, in_id) in execute_edge_query(&db, q.inner)?.into_iter() {
            manager.delete(&mut batch, out_id, &t, in_id, &q.name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }
}
