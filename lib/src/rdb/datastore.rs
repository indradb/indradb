use std::collections::HashSet;
use std::i32;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::u64;
use std::usize;

use super::managers::*;
use crate::errors::Result;
use crate::util::next_uuid;
use crate::{
    BulkInsertItem, Datastore, Edge, EdgeDirection, EdgeKey, EdgeProperties, EdgeProperty, EdgePropertyQuery,
    EdgeQuery, JsonValue, NamedProperty, Transaction, Type, Vertex, VertexProperties, VertexProperty,
    VertexPropertyQuery, VertexQuery,
};

use chrono::offset::Utc;
use rocksdb::{DBCompactionStyle, Options, WriteBatch, WriteOptions, DB};
use uuid::Uuid;

const CF_NAMES: [&str; 9] = [
    "vertices:v1",
    "edges:v1",
    "edge_ranges:v1",
    "reversed_edge_ranges:v1",
    "vertex_properties:v1",
    "edge_properties:v1",
    "vertex_property_values:v1",
    "edge_property_values:v1",
    "metadata:v1",
];

fn get_options(max_open_files: Option<i32>) -> Options {
    // Current tuning based off of the total ordered example, flash
    // storage example on
    // https://github.com/facebook/rocksdb/wiki/RocksDB-Tuning-Guide
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

    if let Some(max_open_files) = max_open_files {
        opts.set_max_open_files(max_open_files);
    }

    opts
}

fn execute_vertex_query(db: &DB, indexed_properties: &HashSet<Type>, q: VertexQuery) -> Result<Vec<VertexItem>> {
    match q {
        VertexQuery::Range(q) => {
            let vertex_manager = VertexManager::new(db, indexed_properties);

            let next_uuid = match q.start_id {
                Some(start_id) => {
                    match next_uuid(start_id) {
                        Ok(next_uuid) => next_uuid,
                        // If we get an error back, it's because
                        // `start_id` is the maximum possible value. We
                        // know that no vertices exist whose ID is greater
                        // than the maximum possible value, so just return
                        // an empty list.
                        Err(_) => return Ok(vec![]),
                    }
                }
                None => Uuid::default(),
            };

            let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> =
                Box::new(vertex_manager.iterate_for_range(next_uuid));

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
            let vertex_manager = VertexManager::new(db, indexed_properties);

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
            let vertex_manager = VertexManager::new(db, indexed_properties);
            let iter = execute_edge_query(db, indexed_properties, *q.inner)?.into_iter();
            let direction = q.direction;

            let iter = iter.map(move |(out_id, _, _, in_id)| {
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

fn execute_edge_query(db: &DB, indexed_properties: &HashSet<Type>, q: EdgeQuery) -> Result<Vec<EdgeRangeItem>> {
    match q {
        EdgeQuery::Specific(q) => {
            let edge_manager = EdgeManager::new(db, indexed_properties);

            let iter = q.keys.into_iter().map(move |key| -> Result<Option<EdgeRangeItem>> {
                match edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                    Some(update_datetime) => {
                        Ok(Some((key.outbound_id, key.t.clone(), update_datetime, key.inbound_id)))
                    }
                    None => Ok(None),
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
            let vertices = execute_vertex_query(db, indexed_properties, *q.inner)?;

            let edge_range_manager = match q.direction {
                EdgeDirection::Outbound => EdgeRangeManager::new(db, indexed_properties),
                EdgeDirection::Inbound => EdgeRangeManager::new_reversed(db, indexed_properties),
            };

            // Ideally we'd use iterators all the way down, but things
            // start breaking apart due to conditional expressions not
            // returning the same type signature, issues with `Result`s
            // and some of the iterators, etc. So at this point, we'll
            // just resort to building a vector.
            let mut edges: Vec<EdgeRangeItem> = Vec::new();

            for (id, _) in vertices.into_iter() {
                let edge_iterator = edge_range_manager.iterate_for_range(id, q.t.as_ref(), q.high)?;

                for item in edge_iterator {
                    let (edge_range_first_id, edge_range_t, edge_range_update_datetime, edge_range_second_id) = item?;

                    if let Some(low) = q.low {
                        if edge_range_update_datetime < low {
                            break;
                        }
                    }

                    edges.push(match q.direction {
                        EdgeDirection::Outbound => (
                            edge_range_first_id,
                            edge_range_t,
                            edge_range_update_datetime,
                            edge_range_second_id,
                        ),
                        EdgeDirection::Inbound => (
                            edge_range_second_id,
                            edge_range_t,
                            edge_range_update_datetime,
                            edge_range_first_id,
                        ),
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
    indexed_properties: Arc<RwLock<HashSet<Type>>>,
}

impl RocksdbDatastore {
    /// Creates a new rocksdb datastore.
    ///
    /// # Arguments
    /// * `path`: The file path to the rocksdb database.
    /// * `max_open_files`: The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn new<P: AsRef<Path>>(path: P, max_open_files: Option<i32>) -> Result<RocksdbDatastore> {
        let opts = get_options(max_open_files);
        let path = path.as_ref();

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

        let metadata_manager = MetadataManager::new(&db);
        let indexed_properties = metadata_manager.get_indexed_properties()?;

        Ok(RocksdbDatastore {
            db: Arc::new(db),
            indexed_properties: Arc::new(RwLock::new(indexed_properties)),
        })
    }

    /// Runs a repair operation on the rocksdb database.
    ///
    /// # Arguments
    /// * `path`: The file path to the rocksdb database.
    /// * `max_open_files`: The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn repair<P: AsRef<Path>>(path: P, max_open_files: Option<i32>) -> Result<()> {
        let opts = get_options(max_open_files);
        DB::repair(&opts, path)?;
        Ok(())
    }
}

impl Datastore for RocksdbDatastore {
    type Trans = RocksdbTransaction;

    fn sync(&self) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        VertexManager::new(&db, &indexed_properties).compact();
        EdgeManager::new(&db, &indexed_properties).compact();
        EdgeRangeManager::new(&db, &indexed_properties).compact();
        EdgeRangeManager::new_reversed(&db, &indexed_properties).compact();
        VertexPropertyManager::new(&db, &indexed_properties).compact();
        EdgePropertyManager::new(&db, &indexed_properties).compact();
        VertexPropertyValueManager::new(&db).compact();
        EdgePropertyValueManager::new(&db).compact();
        MetadataManager::new(&db).compact();
        db.flush()?;
        Ok(())
    }

    // We override the default `bulk_insert` implementation because further
    // optimization can be done by using `WriteBatch`s.
    fn bulk_insert<I>(&self, items: I) -> Result<()>
    where
        I: Iterator<Item = BulkInsertItem>,
    {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let vertex_manager = VertexManager::new(&db, &indexed_properties);
        let edge_manager = EdgeManager::new(&db, &indexed_properties);
        let vertex_property_manager = VertexPropertyManager::new(&db, &indexed_properties);
        let edge_property_manager = EdgePropertyManager::new(&db, &indexed_properties);
        let mut batch = WriteBatch::default();

        for item in items {
            match item {
                BulkInsertItem::Vertex(ref vertex) => {
                    vertex_manager.create(&mut batch, vertex)?;
                }
                BulkInsertItem::Edge(ref key) => {
                    edge_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, Utc::now())?;
                }
                BulkInsertItem::VertexProperty(id, ref name, ref value) => {
                    vertex_property_manager.set(&mut batch, id, name, value)?;
                }
                BulkInsertItem::EdgeProperty(ref key, ref name, ref value) => {
                    edge_property_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, name, value)?;
                }
            }
        }

        // NOTE: syncing and WAL are disabled for bulk inserts to maximize
        // performance
        let mut opts = WriteOptions::default();
        opts.set_sync(false);
        opts.disable_wal(true);
        self.db.write_opt(batch, &opts)?;

        Ok(())
    }

    fn transaction(&self) -> Result<Self::Trans> {
        Ok(RocksdbTransaction::new(self.db.clone(), self.indexed_properties.clone()))
    }

    fn index_property<T: Into<Type>>(&mut self, name: T) -> Result<()> {
        let name = name.into();

        let mut indexed_properties = self.indexed_properties.write().unwrap();
        if !indexed_properties.insert(name.clone()) {
            return Ok(());
        }

        let db = self.db.clone();
        let mut batch = WriteBatch::default();
        let vertex_manager = VertexManager::new(&db, &indexed_properties);
        let edge_range_manager = EdgeRangeManager::new(&db, &indexed_properties);
        let vertex_property_manager = VertexPropertyManager::new(&db, &indexed_properties);
        let edge_property_manager = EdgePropertyManager::new(&db, &indexed_properties);
        let vertex_property_value_manager = VertexPropertyValueManager::new(&db);
        let edge_property_value_manager = EdgePropertyValueManager::new(&db);
        let metadata_manager = MetadataManager::new(&db);
        metadata_manager.set_indexed_properties(&mut batch, &indexed_properties)?;
        
        for item in vertex_manager.iterate_for_range(Uuid::default()) {
            let (vertex_id, _) = item?;
            if let Some(property_value) = vertex_property_manager.get(vertex_id, &name)? {
                vertex_property_value_manager.set(&mut batch, vertex_id, &name, &property_value);
            }
        }

        for item in edge_range_manager.iterate_for_range(Uuid::default(), None, None)? {
            let (out_id, t, _, in_id) = item?;
            if let Some(property_value) = edge_property_manager.get(out_id, &t, in_id, &name)? {
                edge_property_value_manager.set(&mut batch, out_id, &t, in_id, &name, &property_value);
            }
        }

        Ok(())
    }
}

/// A transaction that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbTransaction {
    db: Arc<DB>,
    indexed_properties: Arc<RwLock<HashSet<Type>>>,
}

impl RocksdbTransaction {
    fn new(db: Arc<DB>, indexed_properties: Arc<RwLock<HashSet<Type>>>) -> Self {
        RocksdbTransaction { db, indexed_properties }
    }
}

impl Transaction for RocksdbTransaction {
    fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let vertex_manager = VertexManager::new(&db, &indexed_properties);

        if vertex_manager.exists(vertex.id)? {
            Ok(false)
        } else {
            let mut batch = WriteBatch::default();
            vertex_manager.create(&mut batch, vertex)?;
            self.db.write(batch)?;
            Ok(true)
        }
    }

    fn get_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<Vertex>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let iter = execute_vertex_query(&db, &indexed_properties, q.into())?.into_iter();

        let iter = iter.map(move |(id, t)| {
            let vertex = Vertex::with_id(id, t);
            Ok(vertex)
        });

        iter.collect()
    }

    fn delete_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let iter = execute_vertex_query(&db, &indexed_properties, q.into())?.into_iter();
        let vertex_manager = VertexManager::new(&db, &indexed_properties);
        let mut batch = WriteBatch::default();

        for (id, _) in iter {
            vertex_manager.delete(&mut batch, id)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let vertex_manager = VertexManager::new(&db, &indexed_properties);
        let iterator = vertex_manager.iterate_for_range(Uuid::default());
        Ok(iterator.count() as u64)
    }

    fn create_edge(&self, key: &EdgeKey) -> Result<bool> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let vertex_manager = VertexManager::new(&db, &indexed_properties);

        if !vertex_manager.exists(key.outbound_id)? || !vertex_manager.exists(key.inbound_id)? {
            Ok(false)
        } else {
            let edge_manager = EdgeManager::new(&db, &indexed_properties);
            let mut batch = WriteBatch::default();
            edge_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, Utc::now())?;
            self.db.write(batch)?;
            Ok(true)
        }
    }

    fn get_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<Edge>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let iter = execute_edge_query(&db, &indexed_properties, q.into())?.into_iter();

        let iter = iter.map(move |(out_id, t, update_datetime, in_id)| {
            let key = EdgeKey::new(out_id, t, in_id);
            let edge = Edge::new(key, update_datetime);
            Ok(edge)
        });

        iter.collect()
    }

    fn delete_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let edge_manager = EdgeManager::new(&db, &indexed_properties);
        let vertex_manager = VertexManager::new(&db, &indexed_properties);
        let iter = execute_edge_query(&db, &indexed_properties, q.into())?;
        let mut batch = WriteBatch::default();

        for (out_id, t, update_datetime, in_id) in iter {
            if vertex_manager.get(out_id)?.is_some() {
                edge_manager.delete(&mut batch, out_id, &t, in_id, update_datetime)?;
            };
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&Type>, direction: EdgeDirection) -> Result<u64> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();

        let edge_range_manager = match direction {
            EdgeDirection::Outbound => EdgeRangeManager::new(&db, &indexed_properties),
            EdgeDirection::Inbound => EdgeRangeManager::new_reversed(&db, &indexed_properties),
        };

        let count = edge_range_manager.iterate_for_range(id, t, None)?.count();

        Ok(count as u64)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<VertexProperty>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let manager = VertexPropertyManager::new(&db, &indexed_properties);
        let mut properties = Vec::new();

        for (id, _) in execute_vertex_query(&db, &indexed_properties, q.inner)?.into_iter() {
            let value = manager.get(id, &q.name)?;

            if let Some(value) = value {
                properties.push(VertexProperty::new(id, value));
            }
        }

        Ok(properties)
    }

    fn get_all_vertex_properties<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<VertexProperties>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let iter = execute_vertex_query(&db, &indexed_properties, q.into())?.into_iter();
        let manager = VertexPropertyManager::new(&db, &indexed_properties);

        let iter = iter.map(move |(id, t)| {
            let vertex = Vertex::with_id(id, t);

            let it = manager.iterate_for_owner(id)?;
            let props: Result<Vec<_>> = it.collect();
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
        let indexed_properties = self.indexed_properties.read().unwrap();
        let manager = VertexPropertyManager::new(&db, &indexed_properties);
        let mut batch = WriteBatch::default();

        for (id, _) in execute_vertex_query(&db, &indexed_properties, q.inner)?.into_iter() {
            manager.set(&mut batch, id, &q.name, value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let manager = VertexPropertyManager::new(&db, &indexed_properties);
        let mut batch = WriteBatch::default();

        for (id, _) in execute_vertex_query(&db, &indexed_properties, q.inner)?.into_iter() {
            manager.delete(&mut batch, id, &q.name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<EdgeProperty>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let manager = EdgePropertyManager::new(&db, &indexed_properties);
        let mut properties = Vec::new();

        for (out_id, t, _, in_id) in execute_edge_query(&db, &indexed_properties, q.inner)?.into_iter() {
            let value = manager.get(out_id, &t, in_id, &q.name)?;

            if let Some(value) = value {
                let key = EdgeKey::new(out_id, t, in_id);
                properties.push(EdgeProperty::new(key, value));
            }
        }

        Ok(properties)
    }

    fn get_all_edge_properties<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<EdgeProperties>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let iter = execute_edge_query(&db, &indexed_properties, q.into())?.into_iter();
        let manager = EdgePropertyManager::new(&db, &indexed_properties);

        let iter = iter.map(move |(out_id, t, time, in_id)| {
            let edge = Edge::new(EdgeKey::new(out_id, t.clone(), in_id), time);
            let it = manager.iterate_for_owner(out_id, &t, in_id)?;
            let props: Result<Vec<_>> = it.collect();
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
        let indexed_properties = self.indexed_properties.read().unwrap();
        let manager = EdgePropertyManager::new(&db, &indexed_properties);
        let mut batch = WriteBatch::default();

        for (out_id, t, _, in_id) in execute_edge_query(&db, &indexed_properties, q.inner)?.into_iter() {
            manager.set(&mut batch, out_id, &t, in_id, &q.name, value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let manager = EdgePropertyManager::new(&db, &indexed_properties);
        let mut batch = WriteBatch::default();

        for (out_id, t, _, in_id) in execute_edge_query(&db, &indexed_properties, q.inner)?.into_iter() {
            manager.delete(&mut batch, out_id, &t, in_id, &q.name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }
}
