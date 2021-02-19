use std::i32;
use std::path::Path;
use std::sync::Arc;
use std::u64;
use std::usize;

use super::managers;
use crate::errors::Result;
use crate::util::next_uuid;
use crate::{
    BulkInsertItem, Datastore, Edge, EdgeDirection, EdgeKey, EdgeProperties, EdgeProperty, EdgePropertyQuery,
    EdgeQuery, NamedProperty, Transaction, Type, Vertex, VertexProperties, VertexProperty, VertexPropertyQuery,
    VertexQuery,
};
use crate::tree;
use crate::tree::TreeLikeDatastore;

use chrono::offset::Utc;
use rocksdb::{DBCompactionStyle, Options, WriteOptions, DB};
use serde_json::Value as JsonValue;
use uuid::Uuid;

const CF_NAMES: [&str; 6] = [
    "vertices:v1",
    "edges:v1",
    "edge_ranges:v1",
    "reversed_edge_ranges:v1",
    "vertex_properties:v1",
    "edge_properties:v1",
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

impl tree::WriteBatch for rocksdb::WriteBatch {}

#[derive(Clone, Debug)]
struct InternalRocksdbDatastore {
    db: Arc<DB>,
}

impl TreeLikeDatastore for InternalRocksdbDatastore {
    type VertexManager = managers::VertexManager;
    fn vertex_manager(&self) -> Self::VertexManager {
        managers::VertexManager::new(&self.db)
    }

    type EdgeManager = managers::EdgeManager;
    fn edge_manager(&self) -> Self::EdgeManager {
        managers::EdgeManager::new(&self.db)
    }

    type EdgeRangeManager = managers::EdgeRangeManager;
    fn edge_range_manager(&self) -> Self::EdgeRangeManager {
        managers::EdgeRangeManager::new(&self.db)
    }
    fn reversed_edge_range_manager(&self) -> Self::EdgeRangeManager {
        managers::EdgeRangeManager::new_reversed(&self.db)
    }

    type VertexPropertyManager = managers::VertexPropertyManager;
    fn vertex_property_manager(&self) -> Self::VertexPropertyManager {
        managers::VertexPropertyManager::new(&self.db)
    }

    type EdgePropertyManager = managers::EdgePropertyManager;
    fn edge_property_manager(&self) -> Self::EdgePropertyManager {
        managers::EdgePropertyManager::new(&self.db)
    }

    type WriteBatch = rocksdb::WriteBatch;
    fn write_batch(&self) -> Self::WriteBatch {
        rocksdb::WriteBatch::default()
    }
    fn write(&self, batch: Self::WriteBatch) -> Result<()> {
        self.db.write(batch)?;
        Ok(())
    }
}

/// A datastore that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbDatastore {
    db: InternalRocksdbDatastore
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

        Ok(RocksdbDatastore { db: InternalRocksdbDatastore { db: Arc::new(db) } })
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
        let db = self.db.db.clone();
        managers::VertexManager::new(&db).compact();
        managers::EdgeManager::new(&db).compact();
        managers::EdgeRangeManager::new(&db).compact();
        managers::EdgeRangeManager::new_reversed(&db).compact();
        managers::VertexPropertyManager::new(&db).compact();
        managers::EdgePropertyManager::new(&db).compact();
        db.flush()?;
        Ok(())
    }

    // We override the default `bulk_insert` implementation because further
    // optimization can be done by using `WriteBatch`s.
    fn bulk_insert<I>(&self, items: I) -> Result<()>
    where
        I: Iterator<Item = BulkInsertItem>,
    {
        let batch = self.db.bulk_insert(items)?;
        // NOTE: syncing and WAL are disabled for bulk inserts to maximize
        // performance
        let mut opts = WriteOptions::default();
        opts.set_sync(false);
        opts.disable_wal(true);
        self.db.db.write_opt(batch, &opts)?;

        Ok(())
    }

    fn transaction(&self) -> Result<Self::Trans> {
        Ok(RocksdbTransaction::new(self.db.clone()))
    }
}

/// A transaction that is backed by rocksdb.
#[derive(Debug)]
pub struct RocksdbTransaction {
    db: InternalRocksdbDatastore,
}

impl RocksdbTransaction {
    fn new(db: InternalRocksdbDatastore) -> Self {
        RocksdbTransaction { db }
    }
}

impl Transaction for RocksdbTransaction {
    fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        if let Some(batch) = self.db.create_vertex(vertex)? {
            self.db.write(batch)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<Vertex>> {
        self.db.get_vertices(q)
    }

    fn delete_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<()> {
        let batch = self.db.delete_vertices(q)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        self.db.get_vertex_count()
    }

    fn create_edge(&self, key: &EdgeKey) -> Result<bool> {
        if let Some(batch) = self.db.create_edge(key)? {
            self.db.write(batch)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<Edge>> {
        self.db.get_edges(q)
    }

    fn delete_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<()> {
        let batch = self.db.delete_edges(q)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&Type>, direction: EdgeDirection) -> Result<u64> {
        self.db.get_edge_count(id, t, direction)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<VertexProperty>> {
        self.db.get_vertex_properties(q)
    }

    fn get_all_vertex_properties<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<VertexProperties>> {
        self.db.get_all_vertex_properties(q)
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let batch = self.db.set_vertex_properties(q, value)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let batch = self.db.delete_vertex_properties(q)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<EdgeProperty>> {
        self.db.get_edge_properties(q)
    }

    fn get_all_edge_properties<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<EdgeProperties>> {
        self.db.get_all_edge_properties(q)
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let batch = self.db.set_edge_properties(q, value)?;
        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let batch = self.db.delete_edge_properties(q)?;
        self.db.write(batch)?;
        Ok(())
    }
}
