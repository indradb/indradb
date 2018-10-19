use super::super::{VertexIterator, EdgeIterator, VertexMetadataIterator, EdgeMetadataIterator, Datastore, Transaction};
use super::managers::*;
use chrono::offset::Utc;
use errors::Result;
use models;
use rocksdb::{DBCompactionStyle, Options, WriteBatch, DB};
use serde_json::Value as JsonValue;
use std::i32;
use std::sync::Arc;
use uuid::Uuid;
use std::vec::IntoIter;
use chrono::DateTime;

const CF_NAMES: [&str; 6] = [
    "vertices:v1",
    "edges:v1",
    "edge_ranges:v1",
    "reversed_edge_ranges:v1",
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
    opts.set_write_buffer_size(67_108_864); //64mb
    opts.set_max_write_buffer_number(3);
    opts.set_target_file_size_base(67_108_864); //64mb
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
    db: Arc<DB>,
}

impl RocksdbDatastore {
    /// Creates a new rocksdb datastore.
    ///
    /// # Arguments
    /// * `path` - The file path to the rocksdb database.
    /// * `max_open_files` - The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn new(path: &str, max_open_files: Option<i32>) -> Result<RocksdbDatastore> {
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

        Ok(RocksdbDatastore { db: Arc::new(db) })
    }

    /// Runs a repair operation on the rocksdb database.
    ///
    /// # Arguments
    /// * `path` - The file path to the rocksdb database.
    /// * `max_open_files` - The maximum number of open files to have. If
    ///   `None`, the default will be used.
    pub fn repair(path: &str, max_open_files: Option<i32>) -> Result<()> {
        let opts = get_options(max_open_files);
        DB::repair(opts, path)?;
        Ok(())
    }
}

impl Datastore for RocksdbDatastore {
    type Trans = RocksdbTransaction;

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
}

impl Transaction for RocksdbTransaction {
    type VertexIterator = RocksdbVertexIterator;

    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool> {
        let vertex_manager = VertexManager::new(self.db.clone());

        if vertex_manager.exists(vertex.id)? {
            Ok(false)
        } else {
            vertex_manager.create(vertex)?;
            Ok(true)
        }
    }

    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool> {
        // Verify that the vertices exist and that we own the vertex with the outbound ID
        if !VertexManager::new(self.db.clone()).exists(key.inbound_id)? {
            return Ok(false);
        }

        let new_update_datetime = Utc::now();
        let mut batch = WriteBatch::default();
        EdgeManager::new(self.db.clone()).set(
            &mut batch,
            key.outbound_id,
            &key.t,
            key.inbound_id,
            new_update_datetime,
        )?;
        self.db.write(batch)?;
        Ok(true)
    }

    fn set_vertex_metadata(&self, id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let manager = VertexMetadataManager::new(self.db.clone());
        manager.set(id, name, value)
    }

    fn set_edge_metadata(&self, key: &models::EdgeKey, name: &str, value: &JsonValue) -> Result<()> {
        let manager = EdgeMetadataManager::new(self.db.clone());
        manager.set(key.outbound_id, &key.t, key.inbound_id, name, value)
    }

    fn vertices(&self) -> Self::VertexIterator {
        RocksdbVertexIterator::new(RocksdbVertexIteratorSource::All(Arc::clone(&self.db)), None)
    }

    fn vertex(&self, id: Uuid) -> Self::VertexIterator {
        RocksdbVertexIterator::new(RocksdbVertexIteratorSource::Id(Arc::clone(&self.db), id), None)
    }
}

enum RocksdbVertexIteratorSource {
    All(Arc<DB>),
    Id(Arc<DB>, Uuid),
    Pipe(RocksdbEdgeIterator, models::EdgeDirection)
}

pub struct RocksdbVertexIterator {
    source: RocksdbVertexIteratorSource,
    t: Option<models::Type>
}

impl RocksdbVertexIterator {
    fn new(source: RocksdbVertexIteratorSource, t: Option<models::Type>) -> Self {
        Self { source, t }
    }

    fn get_datastore(&self) -> Arc<DB> {
        match self.source {
            RocksdbVertexIteratorSource::All(ref datastore) => datastore.clone(),
            RocksdbVertexIteratorSource::Id(ref datastore, _) => datastore.clone(),
            RocksdbVertexIteratorSource::Pipe(ref iter, _) => iter.source.get_datastore()
        }
    }
}

impl VertexIterator for RocksdbVertexIterator {
    type EdgeIterator = RocksdbEdgeIterator;
    type VertexMetadataIterator = RocksdbVertexMetadataIterator;
    type Iterator = IntoIter<models::Vertex>;

    fn t(self, t: models::Type) -> Self {
        Self::new(self.source, Some(t))
    }

    fn metadata(self, name: String) -> Self::VertexMetadataIterator {
        RocksdbVertexMetadataIterator {
            source: self,
            name: name,
        }
    }

    fn outbound(self) -> Self::EdgeIterator {
        RocksdbEdgeIterator::new(Box::new(self), models::EdgeDirection::Outbound, None, None, None)
    }

    fn inbound(self) -> Self::EdgeIterator {
        RocksdbEdgeIterator::new(Box::new(self), models::EdgeDirection::Inbound, None, None, None)
    }

    fn get(&self) -> Result<Self::Iterator> {
        unimplemented!();
    }

    fn delete(&self) -> Result<()> {
        let iterator = self.get()?;
        let vertex_manager = VertexManager::new(self.get_datastore());
        let mut batch = WriteBatch::default();

        for vertex in iterator {
            vertex_manager.delete(&mut batch, vertex.id)?;
        }

        self.get_datastore().write(batch)?;
        Ok(())
    }
}

pub struct RocksdbVertexMetadataIterator {
    source: RocksdbVertexIterator,
    name: String
}

impl VertexMetadataIterator for RocksdbVertexMetadataIterator {
    type Iterator = IntoIter<models::VertexMetadata>;

    fn get(&self) -> Result<Self::Iterator> {
        unimplemented!();
    }

    fn delete(&self) -> Result<()> {
        let iterator = self.get()?;
        let vertex_metadata_manager = VertexMetadataManager::new(self.source.get_datastore());
        let mut batch = WriteBatch::default();

        for metadata in iterator {
            vertex_metadata_manager.delete(&mut batch, metadata.id, &self.name)?;
        }

        self.source.get_datastore().write(batch)?;
        Ok(())
    }
}

pub struct RocksdbEdgeIterator {
    source: Box<RocksdbVertexIterator>,
    direction: models::EdgeDirection,
    t: Option<models::Type>,
    high: Option<DateTime<Utc>>,
    low: Option<DateTime<Utc>>
}

impl RocksdbEdgeIterator {
    fn new(source: Box<RocksdbVertexIterator>, direction: models::EdgeDirection, t: Option<models::Type>, high: Option<DateTime<Utc>>, low: Option<DateTime<Utc>>) -> Self {
        Self { source, direction, t, high, low }
    }
}

impl EdgeIterator for RocksdbEdgeIterator {
    type VertexIterator = RocksdbVertexIterator;
    type EdgeMetadataIterator = RocksdbEdgeMetadataIterator;
    type Iterator = IntoIter<models::Edge>;

    fn t(self, t: models::Type) -> Self {
        Self::new(self.source, self.direction, Some(t), self.high, self.low)
    }

    fn high(self, dt: DateTime<Utc>) -> Self {
        Self::new(self.source, self.direction, self.t, Some(dt), self.low)
    }

    fn low(self, dt: DateTime<Utc>) -> Self {
        Self::new(self.source, self.direction, self.t, self.high, Some(dt))
    }

    fn metadata(self, name: String) -> Self::EdgeMetadataIterator {
        RocksdbEdgeMetadataIterator {
            source: self,
            name: name,
        }
    }

    fn outbound(self) -> Self::VertexIterator {
        RocksdbVertexIterator::new(RocksdbVertexIteratorSource::Pipe(self, models::EdgeDirection::Outbound), None)
    }

    fn inbound(self) -> Self::VertexIterator {
        RocksdbVertexIterator::new(RocksdbVertexIteratorSource::Pipe(self, models::EdgeDirection::Inbound), None)
    }

    fn get(&self) -> Result<Self::Iterator> {
        unimplemented!();
    }

    fn delete(&self) -> Result<()> {
        let iterator = self.get()?;
        let edge_manager = EdgeManager::new(self.source.get_datastore());
        let mut batch = WriteBatch::default();

        for edge in iterator {
            edge_manager.delete(&mut batch, edge.key.outbound_id, &edge.key.t, edge.key.inbound_id, edge.created_datetime)?;
        }

        self.source.get_datastore().write(batch)?;
        Ok(())
    }
}

pub struct RocksdbEdgeMetadataIterator {
    source: RocksdbEdgeIterator,
    name: String
}

impl EdgeMetadataIterator for RocksdbEdgeMetadataIterator {
    type Iterator = IntoIter<models::EdgeMetadata>;

    fn get(&self) -> Result<Self::Iterator> {
        unimplemented!();
    }

    fn delete(&self) -> Result<()> {
        let iterator = self.get()?;
        let edge_metadata_manager = EdgeMetadataManager::new(self.source.source.get_datastore());
        let mut batch = WriteBatch::default();

        for metadata in iterator {
            edge_metadata_manager.delete(&mut batch, metadata.key.outbound_id, &metadata.key.t, metadata.key.inbound_id, &self.name)?;
        }

        self.source.source.get_datastore().write(batch)?;
        Ok(())
    }
}
