use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, RwLock};

use super::managers::*;
use crate::errors::Result;
use crate::{BulkInsertItem, Database, Datastore, DynIter, Edge, Identifier, Json, Transaction, Vertex};

use rocksdb::{DBCompactionStyle, Options, WriteBatch, DB};
use uuid::Uuid;

const CF_NAMES: [&str; 8] = [
    "vertices:v2",
    "edge_ranges:v2",
    "reversed_edge_ranges:v2",
    "vertex_properties:v2",
    "edge_properties:v2",
    "vertex_property_values:v2",
    "edge_property_values:v2",
    "metadata:v2",
];

pub struct RocksdbTransaction<'a> {
    db: &'a DB,
    indexed_properties: Arc<RwLock<HashSet<Identifier>>>,
    vertex_manager: VertexManager<'a>,
    edge_manager: EdgeManager<'a>,
    edge_range_manager: EdgeRangeManager<'a>,
    reversed_edge_range_manager: EdgeRangeManager<'a>,
    vertex_property_manager: VertexPropertyManager<'a>,
    edge_property_manager: EdgePropertyManager<'a>,
    vertex_property_value_manager: VertexPropertyValueManager<'a>,
    edge_property_value_manager: EdgePropertyValueManager<'a>,
    metadata_manager: MetadataManager<'a>,
}

impl<'a> RocksdbTransaction<'a> {
    fn vertex_ids_from_property_value_iterator(
        &'a self,
        iter: impl Iterator<Item = Result<VertexPropertyValueKey>> + 'a,
    ) -> impl Iterator<Item = Result<Uuid>> + 'a {
        iter.filter_map(|item| match item {
            Ok((_, _, id)) => match self.vertex_manager.exists(id) {
                Ok(true) => Some(Ok(id)),
                Ok(false) => None,
                Err(err) => Some(Err(err)),
            },
            Err(err) => Some(Err(err)),
        })
    }
}

impl<'a> Transaction<'a> for RocksdbTransaction<'a> {
    fn vertex_count(&self) -> u64 {
        let iter = self.vertex_manager.iterate_for_range(Uuid::default());
        iter.count() as u64
    }

    fn all_vertices(&'a self) -> Result<DynIter<'a, Vertex>> {
        let iter = self.vertex_manager.iterate_for_range(Uuid::default());
        Ok(Box::new(iter))
    }

    fn range_vertices(&'a self, offset: Uuid) -> Result<DynIter<'a, Vertex>> {
        let iter = self.vertex_manager.iterate_for_range(offset);
        Ok(Box::new(iter))
    }

    fn specific_vertices(&'a self, ids: Vec<Uuid>) -> Result<DynIter<'a, Vertex>> {
        let iter = ids.into_iter().filter_map(move |id| match self.vertex_manager.get(id) {
            Ok(Some(t)) => Some(Ok(Vertex::with_id(id, t))),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        });

        Ok(Box::new(iter))
    }

    fn vertex_ids_with_property(&'a self, name: Identifier) -> Result<Option<DynIter<'a, Uuid>>> {
        if self.indexed_properties.read().unwrap().contains(&name) {
            let iter = self.vertex_property_value_manager.iterate_for_name(name);
            let iter = self.vertex_ids_from_property_value_iterator(iter);
            Ok(Some(Box::new(iter)))
        } else {
            Ok(None)
        }
    }

    fn vertex_ids_with_property_value(&'a self, name: Identifier, value: &Json) -> Result<Option<DynIter<'a, Uuid>>> {
        if self.indexed_properties.read().unwrap().contains(&name) {
            let iter = self.vertex_property_value_manager.iterate_for_value(name, value);
            let iter = self.vertex_ids_from_property_value_iterator(iter);
            Ok(Some(Box::new(iter)))
        } else {
            Ok(None)
        }
    }

    fn edge_count(&self) -> u64 {
        let iter = self.edge_range_manager.iterate_for_all();
        iter.count() as u64
    }

    fn all_edges(&'a self) -> Result<DynIter<'a, Edge>> {
        let iter = self.edge_range_manager.iterate_for_all();
        Ok(Box::new(iter))
    }

    fn range_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        let iter = self
            .edge_range_manager
            .iterate_for_range(offset.outbound_id, offset.t, offset.inbound_id)?;
        Ok(Box::new(iter))
    }

    fn range_reversed_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        let iter =
            self.reversed_edge_range_manager
                .iterate_for_range(offset.outbound_id, offset.t, offset.inbound_id)?;
        Ok(Box::new(iter))
    }

    fn specific_edges(&'a self, edges: Vec<Edge>) -> Result<DynIter<'a, Edge>> {
        let iter = edges
            .into_iter()
            .filter_map(move |e| match self.edge_range_manager.contains(&e) {
                Ok(true) => Some(Ok(e)),
                Ok(false) => None,
                Err(err) => Some(Err(err)),
            });

        Ok(Box::new(iter))
    }

    fn edges_with_property(&'a self, name: Identifier) -> Result<Option<DynIter<'a, Edge>>> {
        if self.indexed_properties.read().unwrap().contains(&name) {
            let iter = self
                .edge_property_value_manager
                .iterate_for_name(name)
                .map(|r| match r {
                    Ok((_, _, e)) => Ok(e),
                    Err(err) => Err(err),
                });
            Ok(Some(Box::new(iter)))
        } else {
            Ok(None)
        }
    }

    fn edges_with_property_value(&'a self, name: Identifier, value: &Json) -> Result<Option<DynIter<'a, Edge>>> {
        if self.indexed_properties.read().unwrap().contains(&name) {
            let iter = self
                .edge_property_value_manager
                .iterate_for_value(name, value)
                .map(|r| match r {
                    Ok((_, _, e)) => Ok(e),
                    Err(err) => Err(err),
                });
            Ok(Some(Box::new(iter)))
        } else {
            Ok(None)
        }
    }

    fn vertex_property(&self, vertex: &Vertex, name: Identifier) -> Result<Option<Json>> {
        match self.vertex_property_manager.get(vertex.id, name)? {
            None => Ok(None),
            Some(value) => Ok(Some(value)),
        }
    }

    fn all_vertex_properties_for_vertex(&'a self, vertex: &Vertex) -> Result<DynIter<'a, (Identifier, Json)>> {
        let iter = self.vertex_property_manager.iterate_for_owner(vertex.id)?;
        let props: Result<Vec<_>> = iter.collect();
        let iter = props?.into_iter().map(|(_, name, value)| Ok((name, value)));
        Ok(Box::new(iter))
    }

    fn edge_property(&self, edge: &Edge, name: Identifier) -> Result<Option<Json>> {
        match self.edge_property_manager.get(edge, name)? {
            None => Ok(None),
            Some(value) => Ok(Some(value)),
        }
    }

    fn all_edge_properties_for_edge(&'a self, edge: &Edge) -> Result<DynIter<'a, (Identifier, Json)>> {
        let iter = self.edge_property_manager.iterate_for_owner(edge)?;
        let props: Result<Vec<_>> = iter.collect();
        let iter = props?.into_iter().map(|(_, name, value)| Ok((name, value)));
        Ok(Box::new(iter))
    }

    fn delete_vertices(&mut self, vertices: Vec<Vertex>) -> Result<()> {
        let indexed_properties = self.indexed_properties.read().unwrap();
        let mut batch = WriteBatch::default();

        for vertex in vertices.into_iter() {
            self.vertex_manager.delete(&mut batch, &indexed_properties, vertex.id)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edges(&mut self, edges: Vec<Edge>) -> Result<()> {
        let indexed_properties = self.indexed_properties.read().unwrap();
        let mut batch = WriteBatch::default();

        for edge in edges.into_iter() {
            if self.vertex_manager.get(edge.outbound_id)?.is_some() {
                self.edge_manager.delete(&mut batch, &indexed_properties, &edge)?;
            };
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_properties(&mut self, props: Vec<(Uuid, Identifier)>) -> Result<()> {
        let indexed_properties = self.indexed_properties.read().unwrap();
        let mut batch = WriteBatch::default();

        for (id, name) in props.into_iter() {
            self.vertex_property_manager
                .delete(&mut batch, &indexed_properties, id, name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge_properties(&mut self, props: Vec<(Edge, Identifier)>) -> Result<()> {
        let indexed_properties = self.indexed_properties.read().unwrap();
        let mut batch = WriteBatch::default();

        for (edge, name) in props.into_iter() {
            self.edge_property_manager
                .delete(&mut batch, &indexed_properties, &edge, name)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn sync(&self) -> Result<()> {
        self.vertex_manager.compact();
        self.edge_range_manager.compact();
        self.edge_range_manager.compact();
        self.vertex_property_manager.compact();
        self.edge_property_manager.compact();
        self.vertex_property_value_manager.compact();
        self.edge_property_value_manager.compact();
        self.metadata_manager.compact();
        self.db.flush()?;
        Ok(())
    }

    fn create_vertex(&mut self, vertex: &Vertex) -> Result<bool> {
        if self.vertex_manager.exists(vertex.id)? {
            Ok(false)
        } else {
            let mut batch = WriteBatch::default();
            self.vertex_manager.create(&mut batch, vertex)?;
            self.db.write(batch)?;
            Ok(true)
        }
    }

    fn create_edge(&mut self, edge: &Edge) -> Result<bool> {
        if !self.vertex_manager.exists(edge.outbound_id)? || !self.vertex_manager.exists(edge.inbound_id)? {
            Ok(false)
        } else {
            let mut batch = WriteBatch::default();
            self.edge_manager.set(&mut batch, edge)?;
            self.db.write(batch)?;
            Ok(true)
        }
    }

    // We override the default `bulk_insert` implementation because further
    // optimization can be done by using `WriteBatch`s.
    fn bulk_insert(&mut self, items: Vec<BulkInsertItem>) -> Result<()> {
        let indexed_properties = self.indexed_properties.read().unwrap();
        let mut batch = WriteBatch::default();

        for item in items {
            match item {
                BulkInsertItem::Vertex(ref vertex) => {
                    self.vertex_manager.create(&mut batch, vertex)?;
                }
                BulkInsertItem::Edge(ref edge) => {
                    self.edge_manager.set(&mut batch, edge)?;
                }
                BulkInsertItem::VertexProperty(id, ref name, ref value) => {
                    self.vertex_property_manager
                        .set(&mut batch, &indexed_properties, id, *name, value)?;
                }
                BulkInsertItem::EdgeProperty(ref edge, ref name, ref value) => {
                    self.edge_property_manager
                        .set(&mut batch, &indexed_properties, edge, *name, value)?;
                }
            }
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn index_property(&mut self, name: Identifier) -> Result<()> {
        let mut indexed_properties = self.indexed_properties.write().unwrap();
        if !indexed_properties.insert(name) {
            return Ok(());
        }

        let mut batch = WriteBatch::default();
        self.metadata_manager
            .set_indexed_properties(&mut batch, &indexed_properties)?;

        for item in self.vertex_manager.iterate_for_range(Uuid::default()) {
            let vertex = item?;
            if let Some(property_value) = self.vertex_property_manager.get(vertex.id, name)? {
                self.vertex_property_value_manager
                    .set(&mut batch, vertex.id, name, &property_value);
            }
        }

        for item in self.edge_range_manager.iterate_for_all() {
            let edge = item?;
            if let Some(property_value) = self.edge_property_manager.get(&edge, name)? {
                self.edge_property_value_manager
                    .set(&mut batch, &edge, name, &property_value);
            }
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn set_vertex_properties(&mut self, vertices: Vec<Uuid>, name: Identifier, value: &Json) -> Result<()> {
        let indexed_properties = self.indexed_properties.read().unwrap();
        let mut batch = WriteBatch::default();
        for id in vertices.into_iter() {
            self.vertex_property_manager
                .set(&mut batch, &indexed_properties, id, name, value)?;
        }
        self.db.write(batch)?;
        Ok(())
    }

    fn set_edge_properties(&mut self, edges: Vec<Edge>, name: Identifier, value: &Json) -> Result<()> {
        let indexed_properties = self.indexed_properties.read().unwrap();
        let mut batch = WriteBatch::default();
        for edge in edges.into_iter() {
            self.edge_property_manager
                .set(&mut batch, &indexed_properties, &edge, name, value)?;
        }
        self.db.write(batch)?;
        Ok(())
    }
}

/// A datastore that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbDatastore {
    db: Arc<DB>,
    indexed_properties: Arc<RwLock<HashSet<Identifier>>>,
}

impl RocksdbDatastore {
    /// Creates a new rocksdb datastore.
    ///
    /// # Arguments
    /// * `path`: The file path to the rocksdb database.
    pub fn new_db<P: AsRef<Path>>(path: P) -> Result<Database<RocksdbDatastore>> {
        let opts = RocksdbDatastore::get_options(None);
        let path = path.as_ref();

        let db = match DB::open_cf(&opts, path, CF_NAMES) {
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

        Ok(Database::new(RocksdbDatastore {
            db: Arc::new(db),
            indexed_properties: Arc::new(RwLock::new(indexed_properties)),
        }))
    }

    /// Creates a new rocksdb datastore with user-tuned rocksdb Option.
    ///
    /// # Arguments
    /// * `path`: The file path to the rocksdb database.
    /// * `opts`: The user-tuned rocksdb options.
    pub fn new_db_with_options<P: AsRef<Path>>(path: P, opts: &Options) -> Result<Database<RocksdbDatastore>> {
        let path = path.as_ref();

        let db = match DB::open_cf(opts, path, CF_NAMES) {
            Ok(db) => db,
            Err(_) => {
                let mut db = DB::open(opts, path)?;

                for cf_name in &CF_NAMES {
                    db.create_cf(cf_name, opts)?;
                }

                db
            }
        };

        let metadata_manager = MetadataManager::new(&db);
        let indexed_properties = metadata_manager.get_indexed_properties()?;

        Ok(Database::new(RocksdbDatastore {
            db: Arc::new(db),
            indexed_properties: Arc::new(RwLock::new(indexed_properties)),
        }))
    }

    /// Runs a repair operation on the rocksdb database.
    ///
    /// # Arguments
    /// * `path`: The file path to the rocksdb database.
    /// * `opts`: The rocksdb options used on datastore.
    pub fn repair<P: AsRef<Path>>(path: P, opts: &Options) -> Result<()> {
        DB::repair(opts, path)?;
        Ok(())
    }

    /// Creates a new rocksdb options with indra's default values.
    /// The returned value can serve as the `options` argument in `RocksdbDatastore::new_db_with_options`.
    ///
    /// # Arguments
    /// * `max_open_files`: The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn get_options(max_open_files: Option<i32>) -> Options {
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
}

impl Datastore for RocksdbDatastore {
    type Transaction<'a>
        = RocksdbTransaction<'a>
    where
        Self: 'a;
    fn transaction(&'_ self) -> Self::Transaction<'_> {
        RocksdbTransaction {
            db: &self.db,
            indexed_properties: self.indexed_properties.clone(),
            vertex_manager: VertexManager::new(&self.db),
            edge_manager: EdgeManager::new(&self.db),
            edge_range_manager: EdgeRangeManager::new(&self.db),
            reversed_edge_range_manager: EdgeRangeManager::new_reversed(&self.db),
            vertex_property_manager: VertexPropertyManager::new(&self.db),
            edge_property_manager: EdgePropertyManager::new(&self.db),
            vertex_property_value_manager: VertexPropertyValueManager::new(&self.db),
            edge_property_value_manager: EdgePropertyValueManager::new(&self.db),
            metadata_manager: MetadataManager::new(&self.db),
        }
    }
}
