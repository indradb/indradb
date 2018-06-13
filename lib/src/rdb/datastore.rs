use super::managers::*;
use super::super::{Datastore, EdgeDirection, EdgeQuery, Transaction, VertexQuery};
use chrono::DateTime;
use chrono::offset::Utc;
use core::fmt::Debug;
use errors::Result;
use models;
use rocksdb::{DBCompactionStyle, Options, WriteBatch, DB};
use serde_json::Value as JsonValue;
use std::i32;
use std::sync::Arc;
use std::u64;
use std::usize;
use util::next_uuid;
use uuid::Uuid;

const CF_NAMES: [&'static str; 6] = [
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

impl Datastore<RocksdbTransaction> for RocksdbDatastore {
    fn transaction(&self) -> Result<RocksdbTransaction> {
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
        Ok(RocksdbTransaction { db: db })
    }

    fn vertex_query_to_iterator(&self, q: &VertexQuery) -> Result<Box<Iterator<Item = VertexItem>>> {
        let vertex_manager = VertexManager::new(self.db.clone());

        match q {
            &VertexQuery::All { start_id, limit } => {
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
            &VertexQuery::Vertices { ref ids } => {
                let vertices: Vec<Result<Uuid>> = ids.into_iter().map(|id| Ok(*id)).collect();
                let iterator = Box::new(vertices.into_iter());
                Ok(self.handle_vertex_id_iterator(iterator))
            }
            &VertexQuery::Pipe {
                ref edge_query,
                converter,
                limit,
            } => {
                let edge_iterator = self.edge_query_to_iterator(edge_query)?;

                let vertex_id_iterator = Box::new(edge_iterator.map(move |item| {
                    let (outbound_id, _, _, inbound_id) = item?;

                    match converter {
                        EdgeDirection::Outbound => Ok(outbound_id),
                        EdgeDirection::Inbound => Ok(inbound_id),
                    }
                }));

                Ok(Box::new(
                    self.handle_vertex_id_iterator(vertex_id_iterator)
                        .take(limit as usize),
                ))
            }
        }
    }

    fn edge_query_to_iterator(&self, q: &EdgeQuery) -> Result<Box<Iterator<Item = EdgeRangeItem>>> {
        match q {
            &EdgeQuery::Edges { ref keys } => {
                let edge_manager = EdgeManager::new(self.db.clone());

                let edges: Vec<Result<Option<(Uuid, models::Type, DateTime<Utc>, Uuid)>>> = keys.into_iter()
                    .map(
                        move |key| match edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                            Some(update_datetime) => Ok(Some((
                                key.outbound_id,
                                key.t.clone(),
                                update_datetime,
                                key.inbound_id,
                            ))),
                            None => Ok(None),
                        },
                    )
                    .collect();

                let iterator = self.remove_nones_from_iterator(Box::new(edges.into_iter()));
                Ok(iterator)
            }
            &EdgeQuery::Pipe {
                ref vertex_query,
                converter,
                ref type_filter,
                high_filter,
                low_filter,
                limit,
            } => {
                let vertex_iterator = self.vertex_query_to_iterator(&*vertex_query)?;

                let edge_range_manager = match converter {
                    EdgeDirection::Outbound => EdgeRangeManager::new(self.db.clone()),
                    EdgeDirection::Inbound => EdgeRangeManager::new_reversed(self.db.clone()),
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
                            edge_range_manager.iterate_for_range(id, type_filter.as_ref(), high_filter)?;

                        for item in edge_iterator {
                            match item {
                                Ok((
                                    edge_range_first_id,
                                    edge_range_t,
                                    edge_range_update_datetime,
                                    edge_range_second_id,
                                )) => {
                                    if edge_range_update_datetime >= low_filter {
                                        edges.push(match converter {
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
                                        });
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
                            edge_range_manager.iterate_for_range(id, type_filter.as_ref(), high_filter)?;

                        for item in edge_iterator {
                            match item {
                                Ok((
                                    edge_range_first_id,
                                    edge_range_t,
                                    edge_range_update_datetime,
                                    edge_range_second_id,
                                )) => {
                                    edges.push(match converter {
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
                                    });
                                }
                                Err(_) => edges.push(item),
                            }

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
        iterator: Box<Iterator<Item = Result<Option<T>>>>,
    ) -> Box<Iterator<Item = Result<T>> + 'a> {
        let filtered = iterator.filter(|item| match *item {
            Err(_) | Ok(Some(_)) => true,
            _ => false,
        });

        let mapped = filtered.map(|item| match item {
            Ok(Some(value)) => Ok(value),
            Err(err) => Err(err),
            _ => unreachable!(),
        });

        Box::new(mapped)
    }

    fn handle_vertex_id_iterator(
        &self,
        iterator: Box<Iterator<Item = Result<Uuid>>>,
    ) -> Box<Iterator<Item = VertexItem>> {
        let vertex_manager = VertexManager::new(self.db.clone());

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
    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool> {
        let vertex_manager = VertexManager::new(self.db.clone());

        if vertex_manager.exists(vertex.id)? {
            Ok(false)
        } else {
            vertex_manager.create(vertex)?;
            Ok(true)
        }
    }

    fn get_vertices(&self, q: &VertexQuery) -> Result<Vec<models::Vertex>> {
        let iterator = self.vertex_query_to_iterator(q)?;

        let mapped = iterator.map(move |item| {
            let (id, t) = item?;
            let vertex = models::Vertex::with_id(id, t);
            Ok(vertex)
        });

        mapped.collect()
    }

    fn delete_vertices(&self, q: &VertexQuery) -> Result<()> {
        let iterator = self.vertex_query_to_iterator(q)?;
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

    fn get_edges(&self, q: &EdgeQuery) -> Result<Vec<models::Edge>> {
        let iterator = self.edge_query_to_iterator(q)?;

        let mapped = iterator.map(move |item| {
            let (outbound_id, t, update_datetime, inbound_id) = item?;
            let key = models::EdgeKey::new(outbound_id, t, inbound_id);
            let edge = models::Edge::new(key, update_datetime);
            Ok(edge)
        });

        mapped.collect()
    }

    fn delete_edges(&self, q: &EdgeQuery) -> Result<()> {
        let edge_manager = EdgeManager::new(self.db.clone());
        let vertex_manager = VertexManager::new(self.db.clone());
        let iterator = self.edge_query_to_iterator(q)?;
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
        type_filter: Option<&models::Type>,
        direction: models::EdgeDirection,
    ) -> Result<u64> {
        let edge_range_manager = match direction {
            EdgeDirection::Outbound => EdgeRangeManager::new(self.db.clone()),
            EdgeDirection::Inbound => EdgeRangeManager::new_reversed(self.db.clone()),
        };

        let count = edge_range_manager
            .iterate_for_range(id, type_filter, None)?
            .count();

        Ok(count as u64)
    }

    fn get_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<Vec<models::VertexMetadata>> {
        let manager = VertexMetadataManager::new(self.db.clone());
        let mut metadata = Vec::new();

        for item in self.vertex_query_to_iterator(q)? {
            let (id, _) = item?;
            let value = manager.get(id, &name[..])?;

            if let Some(value) = value {
                metadata.push(models::VertexMetadata::new(id, value));
            }
        }

        Ok(metadata)
    }

    fn set_vertex_metadata(&self, q: &VertexQuery, name: &str, value: &JsonValue) -> Result<()> {
        let manager = VertexMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.vertex_query_to_iterator(q)? {
            let (id, _) = item?;
            manager.set(&mut batch, id, &name[..], value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_metadata(&self, q: &VertexQuery, name: &str) -> Result<()> {
        let manager = VertexMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.vertex_query_to_iterator(q)? {
            let (id, _) = item?;
            manager.delete(&mut batch, id, &name[..])?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn get_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<Vec<models::EdgeMetadata>> {
        let manager = EdgeMetadataManager::new(self.db.clone());
        let mut metadata = Vec::new();

        for item in self.edge_query_to_iterator(q)? {
            let (outbound_id, t, _, inbound_id) = item?;
            let value = manager.get(outbound_id, &t, inbound_id, &name[..])?;

            if let Some(value) = value {
                let key = models::EdgeKey::new(outbound_id, t, inbound_id);
                metadata.push(models::EdgeMetadata::new(key, value));
            }
        }

        Ok(metadata)
    }

    fn set_edge_metadata(&self, q: &EdgeQuery, name: &str, value: &JsonValue) -> Result<()> {
        let manager = EdgeMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.edge_query_to_iterator(q)? {
            let (outbound_id, t, _, inbound_id) = item?;
            manager.set(&mut batch, outbound_id, &t, inbound_id, &name[..], value)?;
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn delete_edge_metadata(&self, q: &EdgeQuery, name: &str) -> Result<()> {
        let manager = EdgeMetadataManager::new(self.db.clone());
        let mut batch = WriteBatch::default();

        for item in self.edge_query_to_iterator(q)? {
            let (outbound_id, t, _, inbound_id) = item?;
            manager.delete(&mut batch, outbound_id, &t, inbound_id, &name[..])?;
        }

        self.db.write(batch)?;
        Ok(())
    }
}
