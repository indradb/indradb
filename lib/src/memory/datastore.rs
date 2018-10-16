use traits::{Datastore, Transaction, VertexIterator, EdgeIterator, VertexMetadataIterator, EdgeMetadataIterator};
use chrono::offset::Utc;
use chrono::DateTime;
use errors::Result;
use models;
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use uuid::Uuid;
use std::iter::Iterator;

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug)]
struct InternalMemoryDatastore {
    edge_metadata: BTreeMap<(models::EdgeKey, String), JsonValue>,
    edges: BTreeMap<models::EdgeKey, DateTime<Utc>>,
    vertex_metadata: BTreeMap<(Uuid, String), JsonValue>,
    vertices: BTreeMap<Uuid, models::Type>,
}

/// An in-memory-only datastore.
#[derive(Debug)]
pub struct MemoryDatastore(Arc<RwLock<InternalMemoryDatastore>>);

impl MemoryDatastore {
    /// Creates a new in-memory datastore.
    pub fn default() -> MemoryDatastore {
        Self {
            0: Arc::new(RwLock::new(InternalMemoryDatastore {
                edge_metadata: BTreeMap::new(),
                edges: BTreeMap::new(),
                vertex_metadata: BTreeMap::new(),
                vertices: BTreeMap::new(),
            })),
        }
    }
}

impl Datastore for MemoryDatastore {
    type Trans = MemoryTransaction;

    fn transaction(&self) -> Result<Self::Trans> {
        Ok(MemoryTransaction {
            datastore: Arc::clone(&self.0),
        })
    }
}

/// A transaction for manipulating in-memory-only datastores.
#[derive(Debug)]
pub struct MemoryTransaction {
    datastore: Arc<RwLock<InternalMemoryDatastore>>,
}

impl Transaction for MemoryTransaction {
    type VertexIterator = MemoryVertexIterator;

    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();
        let mut inserted = false;

        datastore.vertices.entry(vertex.id).or_insert_with(|| {
            inserted = true;
            vertex.t.clone()
        });

        Ok(inserted)
    }

    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();

        if !datastore.vertices.contains_key(&key.outbound_id) || !datastore.vertices.contains_key(&key.inbound_id) {
            return Ok(false);
        }

        datastore.edges.insert(key.clone(), Utc::now());
        Ok(true)
    }

    fn set_vertex_metadata(&self, id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        if datastore.vertices.contains_key(&id) {
            datastore.vertex_metadata.insert((id, name.to_string()), value.clone());
        }

        Ok(())
    }

    fn set_edge_metadata(&self, key: &models::EdgeKey, name: &str, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        if datastore.edges.contains_key(key) {
            datastore.edge_metadata.insert((key.clone(), name.to_string()), value.clone());
        }

        Ok(())
    }

    fn vertices(&self) -> Self::VertexIterator {
        MemoryVertexIterator::new(MemoryVertexIteratorSource::All(Arc::clone(&self.datastore)), None, None)
    }

    fn vertex(&self, id: Uuid) -> Self::VertexIterator {
        MemoryVertexIterator::new(MemoryVertexIteratorSource::Id(Arc::clone(&self.datastore), id), None, None)
    }
}

enum MemoryVertexIteratorSource {
    All(Arc<RwLock<InternalMemoryDatastore>>),
    Id(Arc<RwLock<InternalMemoryDatastore>>, Uuid),
    Pipe(MemoryEdgeIterator, models::EdgeDirection)
}

pub struct MemoryVertexIterator {
    source: MemoryVertexIteratorSource,
    t: Option<models::Type>,
    limit: Option<usize>
}

impl MemoryVertexIterator {
    fn new(source: MemoryVertexIteratorSource, t: Option<models::Type>, limit: Option<usize>) -> Self {
        Self { source, t, limit }
    }

    fn get_datastore(&self) -> &Arc<RwLock<InternalMemoryDatastore>> {
        match self.source {
            MemoryVertexIteratorSource::All(ref datastore) => datastore,
            MemoryVertexIteratorSource::Id(ref datastore, _) => datastore,
            MemoryVertexIteratorSource::Pipe(ref iter, _) => iter.source.get_datastore()
        }
    }
}

impl VertexIterator for MemoryVertexIterator {
    type EdgeIterator = MemoryEdgeIterator;
    type VertexMetadataIterator = MemoryVertexMetadataIterator;

    fn t(self, t: models::Type) -> Self {
        Self::new(self.source, Some(t), self.limit)
    }

    fn limit(self, limit: usize) -> Self {
        Self::new(self.source, self.t, Some(limit))
    }

    fn metadata(self, name: String) -> Self::VertexMetadataIterator {
        MemoryVertexMetadataIterator {
            source: self,
            name: name,
        }
    }

    fn outbound(self) -> Self::EdgeIterator {
        MemoryEdgeIterator::new(Box::new(self), models::EdgeDirection::Outbound, None, None, None, None)
    }

    fn inbound(self) -> Self::EdgeIterator {
        MemoryEdgeIterator::new(Box::new(self), models::EdgeDirection::Inbound, None, None, None, None)
    }

    fn get(&self) -> Result<Vec<models::Vertex>> {
        let datastore = self.get_datastore().read().unwrap();

        match self.source {
            MemoryVertexIteratorSource::All(_) => {
                let iter = datastore.vertices.iter();

                if let Some(limit) = self.limit {
                    Ok(iter.take(limit).map(|(k, v)| models::Vertex::with_id(*k, v.clone())).collect())
                } else {
                    Ok(iter.map(|(k, v)| models::Vertex::with_id(*k, v.clone())).collect())
                }
            },
            MemoryVertexIteratorSource::Id(_, id) => {
                let results = if let Some(value) = datastore.vertices.get(&id) {
                    vec![models::Vertex::with_id(id, value.clone())]
                } else {
                    vec![]
                };

                Ok(results)
            },
            MemoryVertexIteratorSource::Pipe(ref iter, direction) => {
                let edges = iter.get()?;

                let edges = match self.limit {
                    Some(limit) if limit <= edges.len() => edges.get(0..limit),
                    _ => edges.get(..)
                };

                Ok(edges.unwrap().into_iter().filter_map(|edge| {
                    let id = match direction {
                        models::EdgeDirection::Outbound => edge.key.outbound_id,
                        models::EdgeDirection::Inbound => edge.key.inbound_id,
                    };

                    datastore.vertices.get(&id).map(|value| {
                        models::Vertex::with_id(id, value.clone())
                    })
                }).collect())
            }
        }
    }

    fn len(&self) -> Result<u64> {
        Ok(self.get()?.len() as u64)
    }

    fn delete(&self) -> Result<()> {
        let vertices = self.get()?;
        let mut datastore = self.get_datastore().write().unwrap();

        for vertex in vertices {
            datastore.vertices.remove(&vertex.id);

            let deletable_vertex_metadata = datastore.vertex_metadata
                .range((vertex.id, "".to_string())..)
                .take_while(|((metadata_vertex_id, _), _)| &vertex.id == metadata_vertex_id)
                .map(|(metadata_key, _)| metadata_key.clone())
                .collect();

            delete_vertex_metadata(&mut datastore, deletable_vertex_metadata);

            let deletable_edges = datastore.edges.keys()
                .filter(|edge_key| edge_key.outbound_id == vertex.id || edge_key.inbound_id == vertex.id)
                .cloned()
                .collect();

            delete_edges(&mut datastore, deletable_edges);
        }

        Ok(())
    }
}

pub struct MemoryVertexMetadataIterator {
    source: MemoryVertexIterator,
    name: String
}

impl VertexMetadataIterator for MemoryVertexMetadataIterator {
    fn get(&self) -> Result<Vec<models::VertexMetadata>> {
        let vertices = self.source.get()?;
        let datastore = self.source.get_datastore().read().unwrap();
        
        Ok(vertices.into_iter().filter_map(|vertex| {
            let key = (vertex.id, self.name.clone());
            datastore.vertex_metadata.get(&key).map(|value| {
                models::VertexMetadata::new(vertex.id, value.clone())
            })
        }).collect())
    }

    fn delete(&self) -> Result<()> {
        let deletable_vertex_metadata = self.get()?.into_iter()
            .map(|metadata| (metadata.id, self.name.clone()))
            .collect();
        let mut datastore = self.source.get_datastore().write().unwrap();
        delete_vertex_metadata(&mut datastore, deletable_vertex_metadata);
        Ok(())
    }
}

pub struct MemoryEdgeIterator {
    source: Box<MemoryVertexIterator>,
    direction: models::EdgeDirection,
    t: Option<models::Type>,
    high: Option<DateTime<Utc>>,
    low: Option<DateTime<Utc>>,
    limit: Option<usize>
}

impl MemoryEdgeIterator {
    fn new(source: Box<MemoryVertexIterator>, direction: models::EdgeDirection, t: Option<models::Type>, high: Option<DateTime<Utc>>, low: Option<DateTime<Utc>>, limit: Option<usize>) -> Self {
        Self { source, direction, t, high, low, limit }
    }
}

impl EdgeIterator for MemoryEdgeIterator {
    type VertexIterator = MemoryVertexIterator;
    type EdgeMetadataIterator = MemoryEdgeMetadataIterator;

    fn t(self, t: models::Type) -> Self {
        Self::new(self.source, self.direction, Some(t), self.high, self.low, self.limit)
    }

    fn high(self, dt: DateTime<Utc>) -> Self {
        Self::new(self.source, self.direction, self.t, Some(dt), self.low, self.limit)
    }

    fn low(self, dt: DateTime<Utc>) -> Self {
        Self::new(self.source, self.direction, self.t, self.high, Some(dt), self.limit)
    }

    fn limit(self, limit: usize) -> Self {
        Self::new(self.source, self.direction, self.t, self.high, self.low, Some(limit))
    }

    fn metadata(self, name: String) -> Self::EdgeMetadataIterator {
        MemoryEdgeMetadataIterator {
            source: self,
            name: name,
        }
    }

    fn outbound(self) -> Self::VertexIterator {
        MemoryVertexIterator::new(MemoryVertexIteratorSource::Pipe(self, models::EdgeDirection::Outbound), None, None)
    }

    fn inbound(self) -> Self::VertexIterator {
        MemoryVertexIterator::new(MemoryVertexIteratorSource::Pipe(self, models::EdgeDirection::Inbound), None, None)
    }

    fn get(&self) -> Result<Vec<models::Edge>> {
        let vertices = self.source.get()?;
        let datastore = self.source.get_datastore().read().unwrap();
        let mut results = Vec::new();

        match self.direction {
            models::EdgeDirection::Outbound => {
                for vertex in vertices.into_iter() {
                    let lower_bound = match self.t {
                        Some(ref type_filter) => models::EdgeKey::new(vertex.id, type_filter.clone(), Uuid::default()),
                        None => {
                            let empty_type = models::Type::default();
                            models::EdgeKey::new(vertex.id, empty_type, Uuid::default())
                        }
                    };

                    for (key, update_datetime) in datastore.edges.range(lower_bound..) {
                        if key.outbound_id != vertex.id {
                            break;
                        }

                        if let Some(ref t) = self.t {
                            if &key.t != t {
                                break;
                            }
                        }

                        if let Some(dt) = self.high {
                            if *update_datetime > dt {
                                continue;
                            }
                        }

                        if let Some(dt) = self.low {
                            if *update_datetime < dt {
                                continue;
                            }
                        }

                        results.push(models::Edge::new(key.clone(), *update_datetime));

                        if let Some(limit) = self.limit {
                            if results.len() == limit {
                                return Ok(results);
                            }
                        }
                    }
                }
            }
            models::EdgeDirection::Inbound => {
                let mut candidate_ids = HashSet::new();
                for vertex in vertices.into_iter() {
                    candidate_ids.insert(vertex.id);
                }

                for (key, update_datetime) in &datastore.edges {
                    if !candidate_ids.contains(&key.inbound_id) {
                        continue;
                    }

                    if let Some(ref t) = self.t {
                        if &key.t != t {
                            continue;
                        }
                    }

                    if let Some(dt) = self.high {
                        if *update_datetime > dt {
                            continue;
                        }
                    }

                    if let Some(dt) = self.low {
                        if *update_datetime < dt {
                            continue;
                        }
                    }

                    results.push(models::Edge::new(key.clone(), *update_datetime));

                    if let Some(limit) = self.limit {
                        if results.len() == limit {
                            return Ok(results);
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn len(&self) -> Result<u64> {
        Ok(self.get()?.len() as u64)
    }

    fn delete(&self) -> Result<()> {
        let edges = self.get()?.into_iter().map(|edge| edge.key).collect();
        let mut datastore = self.source.get_datastore().write().unwrap();
        delete_edges(&mut datastore, edges);
        Ok(())
    }
}

pub struct MemoryEdgeMetadataIterator {
    source: MemoryEdgeIterator,
    name: String
}

impl EdgeMetadataIterator for MemoryEdgeMetadataIterator {
    fn get(&self) -> Result<Vec<models::EdgeMetadata>> {
        let edges = self.source.get()?;
        let datastore = self.source.source.get_datastore().read().unwrap();
        
        Ok(edges.into_iter().filter_map(|edge| {
            let key = (edge.key.clone(), self.name.clone());
            datastore.edge_metadata.get(&key).map(|value| {
                models::EdgeMetadata::new(edge.key, value.clone())
            })
        }).collect())
    }

    fn delete(&self) -> Result<()> {
        let deletable_edge_metadata = self.get()?.into_iter()
            .map(|metadata| (metadata.key, self.name.clone()))
            .collect();
        let mut datastore = self.source.source.get_datastore().write().unwrap();
        delete_edge_metadata(&mut datastore, deletable_edge_metadata);
        Ok(())
    }
}

fn delete_vertex_metadata(datastore: &mut RwLockWriteGuard<'_, InternalMemoryDatastore>, vertex_metadata: Vec<(Uuid, String)>) {
    for metadata_key in vertex_metadata {
        datastore.vertex_metadata.remove(&metadata_key);
    }
}

fn delete_edge_metadata(datastore: &mut RwLockWriteGuard<'_, InternalMemoryDatastore>, edge_metadata: Vec<(models::EdgeKey, String)>) {
    for metadata_key in edge_metadata {
        datastore.edge_metadata.remove(&metadata_key);
    }
}

fn delete_edges(datastore: &mut RwLockWriteGuard<'_, InternalMemoryDatastore>, edges: Vec<models::EdgeKey>) {
    for edge_key in &edges {
        datastore.edges.remove(&edge_key);

        let deletable_edge_metadata = datastore.edge_metadata
            .range((edge_key.clone(), "".to_string())..)
            .take_while(|((metadata_edge_key, _), _)| edge_key == metadata_edge_key)
            .map(|(metadata_key, _)| metadata_key.clone())
            .collect();

        delete_edge_metadata(datastore, deletable_edge_metadata);
    }
}

