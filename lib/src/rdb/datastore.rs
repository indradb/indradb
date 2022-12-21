use std::collections::{HashMap, HashSet};
use std::i32;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::u64;
use std::usize;

use super::managers::*;
use crate::errors::{Error, Result};
use crate::{
    BulkInsertItem, Datastore, Edge, EdgeDirection, EdgeKey, EdgeProperties, EdgeProperty, EdgePropertyQuery,
    EdgeQuery, Identifier, Json, NamedProperty, PropertyPresenceEdgeQuery, PropertyPresenceVertexQuery,
    PropertyValueEdgeQuery, PropertyValueVertexQuery, Vertex, VertexProperties, VertexProperty, VertexPropertyQuery,
    VertexQuery,
};

use chrono::offset::Utc;
use chrono::DateTime;
use rocksdb::{DBCompactionStyle, Options, WriteBatch, DB};

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

fn guard_indexed_property(db_ref: DBRef<'_>, property: &Identifier) -> Result<()> {
    if !db_ref.indexed_properties.contains(property) {
        Err(Error::NotIndexed)
    } else {
        Ok(())
    }
}

fn vertices_from_property_value_iterator<'a>(
    db_ref: DBRef<'a>,
    iter: impl Iterator<Item = Result<VertexPropertyValueKey>> + 'a,
) -> Result<Vec<VertexItem>> {
    let vertex_manager = VertexManager::new(db_ref);

    let mut vertices = Vec::new();
    for item in iter {
        let (_, _, id) = item?;
        if let Some(t) = vertex_manager.get(id)? {
            vertices.push((id, t));
        }
    }

    Ok(vertices)
}

fn vertices_from_piped_property_query(
    db_ref: DBRef<'_>,
    inner_query: VertexQuery,
    property_query: VertexQuery,
    intersection: bool,
) -> Result<Vec<VertexItem>> {
    let mut piped_vertices_mapping: HashMap<u64, Identifier> =
        execute_vertex_query(db_ref, inner_query)?.into_iter().collect();
    let piped_vertices: HashSet<u64> = piped_vertices_mapping.keys().cloned().collect();

    let property_vertices: HashSet<u64> = {
        execute_vertex_query(db_ref, property_query)?
            .into_iter()
            .map(move |item| {
                let (id, _) = item;
                id
            })
            .collect()
    };

    let merged_vertices: Box<dyn Iterator<Item = &u64>> = if intersection {
        Box::new(piped_vertices.intersection(&property_vertices))
    } else {
        Box::new(piped_vertices.difference(&property_vertices))
    };

    Ok(merged_vertices
        .map(move |id| (*id, piped_vertices_mapping.remove(id).unwrap()))
        .collect())
}

fn edges_from_property_value_iterator<'a>(
    db_ref: DBRef<'a>,
    iter: impl Iterator<Item = Result<EdgePropertyValueKey>> + 'a,
) -> Result<Vec<EdgeRangeItem>> {
    let edge_manager = EdgeManager::new(db_ref);

    let mut edges = Vec::new();
    for item in iter {
        let (_, _, (out_id, t, in_id)) = item?;
        if let Some(dt) = edge_manager.get(out_id, &t, in_id)? {
            edges.push((out_id, t, dt, in_id));
        }
    }

    Ok(edges)
}

fn edges_from_piped_property_query(
    db_ref: DBRef<'_>,
    inner_query: EdgeQuery,
    property_query: EdgeQuery,
    intersection: bool,
) -> Result<Vec<EdgeRangeItem>> {
    let mut piped_edges_mapping: HashMap<EdgeKey, DateTime<Utc>> = execute_edge_query(db_ref, inner_query)?
        .into_iter()
        .map(move |item| {
            let (out_id, t, dt, in_id) = item;
            (EdgeKey::new(out_id, t, in_id), dt)
        })
        .collect();
    let piped_edges: HashSet<EdgeKey> = piped_edges_mapping.keys().cloned().collect();

    let property_edges: HashSet<EdgeKey> = {
        execute_edge_query(db_ref, property_query)?
            .into_iter()
            .map(move |item| {
                let (out_id, t, _, in_id) = item;
                EdgeKey::new(out_id, t, in_id)
            })
            .collect()
    };

    let merged_edges: Box<dyn Iterator<Item = &EdgeKey>> = if intersection {
        Box::new(piped_edges.intersection(&property_edges))
    } else {
        Box::new(piped_edges.difference(&property_edges))
    };

    Ok(merged_edges
        .map(move |key| {
            let dt = piped_edges_mapping.remove(key).unwrap();
            (key.outbound_id, key.t.clone(), dt, key.inbound_id)
        })
        .collect())
}

fn execute_vertex_query(db_ref: DBRef<'_>, q: VertexQuery) -> Result<Vec<VertexItem>> {
    match q {
        VertexQuery::Range(q) => {
            let vertex_manager = VertexManager::new(db_ref);

            let next_id = match q.start_id {
                Some(start_id) if start_id == u64::max_value() => return Ok(vec![]),
                Some(start_id) => start_id + 1,
                None => 0,
            };

            let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> =
                Box::new(vertex_manager.iterate_for_range(next_id));

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
            let vertex_manager = VertexManager::new(db_ref);

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
            let vertex_manager = VertexManager::new(db_ref);
            let iter = execute_edge_query(db_ref, *q.inner)?.into_iter();
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
        VertexQuery::PropertyPresence(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let vertex_property_value_manager = VertexPropertyValueManager::new(db_ref);
            let iter = vertex_property_value_manager.iterate_for_name(&q.name);
            vertices_from_property_value_iterator(db_ref, iter)
        }
        VertexQuery::PropertyValue(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let vertex_property_value_manager = VertexPropertyValueManager::new(db_ref);
            let iter = vertex_property_value_manager.iterate_for_value(&q.name, &Json::new(q.value));
            vertices_from_property_value_iterator(db_ref, iter)
        }
        VertexQuery::PipePropertyPresence(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let property_query = PropertyPresenceVertexQuery::new(q.name).into();
            vertices_from_piped_property_query(db_ref, *q.inner, property_query, q.exists)
        }
        VertexQuery::PipePropertyValue(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let property_query = PropertyValueVertexQuery::new(q.name, q.value).into();
            vertices_from_piped_property_query(db_ref, *q.inner, property_query, q.equal)
        }
    }
}

fn execute_edge_query(db_ref: DBRef<'_>, q: EdgeQuery) -> Result<Vec<EdgeRangeItem>> {
    match q {
        EdgeQuery::Specific(q) => {
            let edge_manager = EdgeManager::new(db_ref);

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
            let vertices = execute_vertex_query(db_ref, *q.inner)?;

            let edge_range_manager = match q.direction {
                EdgeDirection::Outbound => EdgeRangeManager::new(db_ref),
                EdgeDirection::Inbound => EdgeRangeManager::new_reversed(db_ref),
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
        EdgeQuery::PropertyPresence(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let edge_property_value_manager = EdgePropertyValueManager::new(db_ref);
            let iter = edge_property_value_manager.iterate_for_name(&q.name);
            edges_from_property_value_iterator(db_ref, iter)
        }
        EdgeQuery::PropertyValue(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let edge_property_value_manager = EdgePropertyValueManager::new(db_ref);
            let iter = edge_property_value_manager.iterate_for_value(&q.name, &Json::new(q.value));
            edges_from_property_value_iterator(db_ref, iter)
        }
        EdgeQuery::PipePropertyPresence(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let property_query = PropertyPresenceEdgeQuery::new(q.name).into();
            edges_from_piped_property_query(db_ref, *q.inner, property_query, q.exists)
        }
        EdgeQuery::PipePropertyValue(q) => {
            guard_indexed_property(db_ref, &q.name)?;
            let property_query = PropertyValueEdgeQuery::new(q.name, q.value).into();
            edges_from_piped_property_query(db_ref, *q.inner, property_query, q.equal)
        }
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
    fn sync(&self) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        VertexManager::new(db_ref).compact();
        EdgeManager::new(db_ref).compact();
        EdgeRangeManager::new(db_ref).compact();
        EdgeRangeManager::new_reversed(db_ref).compact();
        VertexPropertyManager::new(db_ref).compact();
        EdgePropertyManager::new(db_ref).compact();
        VertexPropertyValueManager::new(db_ref).compact();
        EdgePropertyValueManager::new(db_ref).compact();
        MetadataManager::new(&db).compact();
        db.flush()?;
        Ok(())
    }

    fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let vertex_manager = VertexManager::new(db_ref);

        if vertex_manager.exists(vertex.id)? {
            Ok(false)
        } else {
            let mut batch = WriteBatch::default();
            vertex_manager.create(&mut batch, vertex)?;
            db.write(batch)?;
            Ok(true)
        }
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let iter = execute_vertex_query(db_ref, q)?.into_iter();

        let iter = iter.map(move |(id, t)| {
            let vertex = Vertex::with_id(id, t);
            Ok(vertex)
        });

        iter.collect()
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let iter = execute_vertex_query(db_ref, q)?.into_iter();
        let vertex_manager = VertexManager::new(db_ref);
        let mut batch = WriteBatch::default();

        for (id, _) in iter {
            vertex_manager.delete(&mut batch, id)?;
        }

        db.write(batch)?;
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let vertex_manager = VertexManager::new(db_ref);
        let iterator = vertex_manager.iterate_for_range(0);
        Ok(iterator.count() as u64)
    }

    fn create_edge(&self, key: &EdgeKey) -> Result<bool> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let vertex_manager = VertexManager::new(db_ref);

        if !vertex_manager.exists(key.outbound_id)? || !vertex_manager.exists(key.inbound_id)? {
            Ok(false)
        } else {
            let edge_manager = EdgeManager::new(db_ref);
            let mut batch = WriteBatch::default();
            edge_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, Utc::now())?;
            db.write(batch)?;
            Ok(true)
        }
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let iter = execute_edge_query(db_ref, q)?.into_iter();

        let iter = iter.map(move |(out_id, t, update_datetime, in_id)| {
            let key = EdgeKey::new(out_id, t, in_id);
            let edge = Edge::new(key, update_datetime);
            Ok(edge)
        });

        iter.collect()
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let edge_manager = EdgeManager::new(db_ref);
        let vertex_manager = VertexManager::new(db_ref);
        let iter = execute_edge_query(db_ref, q)?;
        let mut batch = WriteBatch::default();

        for (out_id, t, update_datetime, in_id) in iter {
            if vertex_manager.get(out_id)?.is_some() {
                edge_manager.delete(&mut batch, out_id, &t, in_id, update_datetime)?;
            };
        }

        db.write(batch)?;
        Ok(())
    }

    fn get_edge_count(&self, id: u64, t: Option<&Identifier>, direction: EdgeDirection) -> Result<u64> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);

        let edge_range_manager = match direction {
            EdgeDirection::Outbound => EdgeRangeManager::new(db_ref),
            EdgeDirection::Inbound => EdgeRangeManager::new_reversed(db_ref),
        };

        let count = edge_range_manager.iterate_for_range(id, t, None)?.count();

        Ok(count as u64)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<VertexProperty>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let manager = VertexPropertyManager::new(db_ref);
        let mut properties = Vec::new();

        for (id, _) in execute_vertex_query(db_ref, q.inner)?.into_iter() {
            let value = manager.get(id, &q.name)?;

            if let Some(value) = value {
                properties.push(VertexProperty::new(id, value.0));
            }
        }

        Ok(properties)
    }

    fn get_all_vertex_properties(&self, q: VertexQuery) -> Result<Vec<VertexProperties>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let iter = execute_vertex_query(db_ref, q)?.into_iter();
        let manager = VertexPropertyManager::new(db_ref);

        let iter = iter.map(move |(id, t)| {
            let vertex = Vertex::with_id(id, t);

            let it = manager.iterate_for_owner(id)?;
            let props: Result<Vec<_>> = it.collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, name), value)| NamedProperty::new(name, value.0))
                .collect();

            Ok(VertexProperties::new(vertex, props))
        });

        iter.collect()
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: serde_json::Value) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let manager = VertexPropertyManager::new(db_ref);
        let mut batch = WriteBatch::default();

        let wrapped_value = Json::new(value);
        for (id, _) in execute_vertex_query(db_ref, q.inner)?.into_iter() {
            manager.set(&mut batch, id, &q.name, &wrapped_value)?;
        }

        db.write(batch)?;
        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let manager = VertexPropertyManager::new(db_ref);
        let mut batch = WriteBatch::default();

        for (id, _) in execute_vertex_query(db_ref, q.inner)?.into_iter() {
            manager.delete(&mut batch, id, &q.name)?;
        }

        db.write(batch)?;
        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<EdgeProperty>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let manager = EdgePropertyManager::new(db_ref);
        let mut properties = Vec::new();

        for (out_id, t, _, in_id) in execute_edge_query(db_ref, q.inner)?.into_iter() {
            let value = manager.get(out_id, &t, in_id, &q.name)?;

            if let Some(value) = value {
                let key = EdgeKey::new(out_id, t, in_id);
                properties.push(EdgeProperty::new(key, value.0));
            }
        }

        Ok(properties)
    }

    fn get_all_edge_properties(&self, q: EdgeQuery) -> Result<Vec<EdgeProperties>> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let iter = execute_edge_query(db_ref, q)?.into_iter();
        let manager = EdgePropertyManager::new(db_ref);

        let iter = iter.map(move |(out_id, t, time, in_id)| {
            let edge = Edge::new(EdgeKey::new(out_id, t.clone(), in_id), time);
            let it = manager.iterate_for_owner(out_id, &t, in_id)?;
            let props: Result<Vec<_>> = it.collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, _, _, name), value)| NamedProperty::new(name, value.0))
                .collect();

            Ok(EdgeProperties::new(edge, props))
        });

        iter.collect()
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: serde_json::Value) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let manager = EdgePropertyManager::new(db_ref);
        let mut batch = WriteBatch::default();

        let wrapped_value = Json::new(value);
        for (out_id, t, _, in_id) in execute_edge_query(db_ref, q.inner)?.into_iter() {
            manager.set(&mut batch, out_id, &t, in_id, &q.name, &wrapped_value)?;
        }

        db.write(batch)?;
        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let manager = EdgePropertyManager::new(db_ref);
        let mut batch = WriteBatch::default();

        for (out_id, t, _, in_id) in execute_edge_query(db_ref, q.inner)?.into_iter() {
            manager.delete(&mut batch, out_id, &t, in_id, &q.name)?;
        }

        db.write(batch)?;
        Ok(())
    }

    // We override the default `bulk_insert` implementation because further
    // optimization can be done by using `WriteBatch`s.
    fn bulk_insert(&self, items: Vec<BulkInsertItem>) -> Result<()> {
        let db = self.db.clone();
        let indexed_properties = self.indexed_properties.read().unwrap();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let vertex_manager = VertexManager::new(db_ref);
        let edge_manager = EdgeManager::new(db_ref);
        let vertex_property_manager = VertexPropertyManager::new(db_ref);
        let edge_property_manager = EdgePropertyManager::new(db_ref);
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
                    vertex_property_manager.set(&mut batch, id, name, &Json::new(value.clone()))?;
                }
                BulkInsertItem::EdgeProperty(ref key, ref name, ref value) => {
                    edge_property_manager.set(
                        &mut batch,
                        key.outbound_id,
                        &key.t,
                        key.inbound_id,
                        name,
                        &Json::new(value.clone()),
                    )?;
                }
            }
        }

        self.db.write(batch)?;
        Ok(())
    }

    fn index_property(&self, name: Identifier) -> Result<()> {
        let mut indexed_properties = self.indexed_properties.write().unwrap();
        if !indexed_properties.insert(name.clone()) {
            return Ok(());
        }

        let db = self.db.clone();
        let db_ref = DBRef::new(&db, &indexed_properties);
        let mut batch = WriteBatch::default();
        let vertex_manager = VertexManager::new(db_ref);
        let edge_range_manager = EdgeRangeManager::new(db_ref);
        let vertex_property_manager = VertexPropertyManager::new(db_ref);
        let edge_property_manager = EdgePropertyManager::new(db_ref);
        let vertex_property_value_manager = VertexPropertyValueManager::new(db_ref);
        let edge_property_value_manager = EdgePropertyValueManager::new(db_ref);
        let metadata_manager = MetadataManager::new(&db);
        metadata_manager.set_indexed_properties(&mut batch, &indexed_properties)?;

        for item in vertex_manager.iterate_for_range(0) {
            let (vertex_id, _) = item?;
            if let Some(property_value) = vertex_property_manager.get(vertex_id, &name)? {
                vertex_property_value_manager.set(&mut batch, vertex_id, &name, &property_value);
            }
        }

        for item in edge_range_manager.iterate_for_all() {
            let (out_id, t, _, in_id) = item?;
            if let Some(property_value) = edge_property_manager.get(out_id, &t, in_id, &name)? {
                edge_property_value_manager.set(&mut batch, out_id, &t, in_id, &name, &property_value);
            }
        }

        db.write(batch)?;
        Ok(())
    }
}
