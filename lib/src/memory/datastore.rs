use super::super::{Id, Datastore, EdgePropertyQuery, EdgeQuery, Transaction, VertexPropertyQuery, VertexQuery};
use chrono::offset::Utc;
use chrono::DateTime;
use errors::Result;
use models;
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, RwLock};

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug)]
struct InternalMemoryDatastore {
    edge_properties: BTreeMap<(models::EdgeKey, String), JsonValue>,
    edges: BTreeMap<models::EdgeKey, DateTime<Utc>>,
    vertex_properties: BTreeMap<(Id, String), JsonValue>,
    vertices: BTreeMap<Id, models::Type>,
}

impl InternalMemoryDatastore {
    fn get_vertex_values_by_query(&self, q: VertexQuery) -> Result<Vec<(Id, models::Type)>> {
        match q {
            VertexQuery::Range(range) => {
                let mut iter: Box<dyn Iterator<Item = (&Id, &models::Type)>> = if let Some(start_id) = range.start_id
                {
                    Box::new(self.vertices.range(start_id..))
                } else {
                    Box::new(self.vertices.iter())
                };

                if let Some(ref t) = range.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &t));
                }

                Ok(iter.take(range.limit as usize).map(|(k, v)| (k.clone(), v.clone())).collect())
            }
            VertexQuery::Specific(specific) => {
                let mut results = Vec::new();

                for id in specific.ids {
                    let value = self.vertices.get(&id);

                    if let Some(value) = value {
                        results.push((id, value.clone()));
                    }
                }

                Ok(results)
            }
            VertexQuery::Pipe(pipe) => {
                let edge_values = self.get_edge_values_by_query(*pipe.inner)?.into_iter();

                let iter: Box<dyn Iterator<Item = Id>> = match pipe.direction {
                    models::EdgeDirection::Outbound => Box::new(edge_values.map(|(key, _)| key.outbound_id)),
                    models::EdgeDirection::Inbound => Box::new(edge_values.map(|(key, _)| key.inbound_id)),
                };

                let mut iter: Box<dyn Iterator<Item = (Id, &models::Type)>> = Box::new(
                    iter.map(|id| (id.clone(), self.vertices.get(&id)))
                        .filter_map(|(k, v)| Some((k, v?))),
                );

                if let Some(ref t) = pipe.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &t));
                }

                Ok(iter.take(pipe.limit as usize).map(|(k, v)| (k, v.clone())).collect())
            }
        }
    }

    fn get_edge_values_by_query(&self, q: EdgeQuery) -> Result<Vec<(models::EdgeKey, DateTime<Utc>)>> {
        match q {
            EdgeQuery::Specific(specific) => {
                let mut results = Vec::new();

                for key in specific.keys {
                    let value = self.edges.get(&key);

                    if let Some(update_datetime) = value {
                        results.push((key.clone(), *update_datetime));
                    }
                }

                Ok(results)
            }
            EdgeQuery::Pipe(pipe) => {
                let vertex_values = self.get_vertex_values_by_query(*pipe.inner)?;
                let mut results = Vec::new();

                if pipe.limit == 0 {
                    return Ok(results);
                }

                match pipe.direction {
                    models::EdgeDirection::Outbound => {
                        for (id, _) in vertex_values {
                            let lower_bound = match &pipe.t {
                                Some(t) => models::EdgeKey::new(id.clone(), t.clone(), Id::default()),
                                None => {
                                    let empty_type = models::Type::default();
                                    models::EdgeKey::new(id.clone(), empty_type, Id::default())
                                }
                            };

                            for (key, update_datetime) in self.edges.range(lower_bound..) {
                                if key.outbound_id != id {
                                    break;
                                }

                                if let Some(t) = &pipe.t {
                                    if &key.t != t {
                                        break;
                                    }
                                }

                                if let Some(high) = &pipe.high {
                                    if update_datetime > high {
                                        continue;
                                    }
                                }

                                if let Some(low) = &pipe.low {
                                    if update_datetime < low {
                                        continue;
                                    }
                                }

                                results.push((key.clone(), *update_datetime));

                                if results.len() == pipe.limit as usize {
                                    return Ok(results);
                                }
                            }
                        }
                    }
                    models::EdgeDirection::Inbound => {
                        let mut candidate_ids = HashSet::new();
                        for (id, _) in vertex_values {
                            candidate_ids.insert(id);
                        }

                        for (key, update_datetime) in &self.edges {
                            if !candidate_ids.contains(&key.inbound_id) {
                                continue;
                            }

                            if let Some(t) = &pipe.t {
                                if &key.t != t {
                                    continue;
                                }
                            }

                            if let Some(high) = &pipe.high {
                                if update_datetime > high {
                                    continue;
                                }
                            }

                            if let Some(low) = &pipe.low {
                                if update_datetime < low {
                                    continue;
                                }
                            }

                            results.push((key.clone(), *update_datetime));

                            if results.len() == pipe.limit as usize {
                                return Ok(results);
                            }
                        }
                    }
                }

                Ok(results)
            }
        }
    }

    fn delete_vertices(&mut self, vertices: Vec<Id>) {
        for vertex_id in vertices {
            self.vertices.remove(&vertex_id);

            let mut deletable_vertex_properties: Vec<(Id, String)> = Vec::new();

            for (property_key, _) in self.vertex_properties.range((vertex_id.clone(), "".to_string())..) {
                let &(ref property_vertex_id, _) = property_key;

                if &vertex_id != property_vertex_id {
                    break;
                }

                deletable_vertex_properties.push(property_key.clone());
            }

            for property_key in deletable_vertex_properties {
                self.vertex_properties.remove(&property_key);
            }

            let mut deletable_edges: Vec<models::EdgeKey> = Vec::new();

            for edge_key in self.edges.keys() {
                if edge_key.outbound_id == vertex_id || edge_key.inbound_id == vertex_id {
                    deletable_edges.push(edge_key.clone());
                }
            }

            self.delete_edges(deletable_edges);
        }
    }

    fn delete_edges(&mut self, edges: Vec<models::EdgeKey>) {
        for edge_key in edges {
            self.edges.remove(&edge_key);

            let mut deletable_edge_properties: Vec<(models::EdgeKey, String)> = Vec::new();

            for (property_key, _) in self.edge_properties.range((edge_key.clone(), "".to_string())..) {
                let &(ref property_edge_key, _) = property_key;

                if &edge_key != property_edge_key {
                    break;
                }

                deletable_edge_properties.push(property_key.clone());
            }

            for property_key in deletable_edge_properties {
                self.edge_properties.remove(&property_key);
            }
        }
    }
}

/// An in-memory-only datastore.
#[derive(Debug)]
pub struct MemoryDatastore(Arc<RwLock<InternalMemoryDatastore>>);

impl MemoryDatastore {
    /// Creates a new in-memory datastore.
    pub fn default() -> MemoryDatastore {
        Self {
            0: Arc::new(RwLock::new(InternalMemoryDatastore {
                edge_properties: BTreeMap::new(),
                edges: BTreeMap::new(),
                vertex_properties: BTreeMap::new(),
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
    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();
        let mut inserted = false;

        datastore.vertices.entry(vertex.id.clone()).or_insert_with(|| {
            inserted = true;
            vertex.t.clone()
        });

        Ok(inserted)
    }

    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>> {
        let vertex_values = self.datastore.read().unwrap().get_vertex_values_by_query(q.into())?;
        let iter = vertex_values
            .into_iter()
            .map(|(id, t)| models::Vertex::with_id(id, t));
        Ok(iter.collect())
    }

    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_vertices = datastore
            .get_vertex_values_by_query(q.into())?
            .into_iter()
            .map(|(k, _)| k)
            .collect();
        datastore.delete_vertices(deletable_vertices);
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();
        Ok(datastore.vertices.len() as u64)
    }

    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();

        if !datastore.vertices.contains_key(&key.outbound_id) || !datastore.vertices.contains_key(&key.inbound_id) {
            return Ok(false);
        }

        datastore.edges.insert(key.clone(), Utc::now());
        Ok(true)
    }

    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>> {
        let edge_values = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_edge_values_by_query(q.into())?
        };

        let iter = edge_values
            .into_iter()
            .map(|(key, update_datetime)| models::Edge::new(key, update_datetime));
        Ok(iter.collect())
    }

    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_edges: Vec<models::EdgeKey> = datastore
            .get_edge_values_by_query(q.into())?
            .into_iter()
            .map(|(k, _)| k)
            .collect();
        datastore.delete_edges(deletable_edges);
        Ok(())
    }

    fn get_edge_count(&self, id: Id, t: Option<&models::Type>, direction: models::EdgeDirection) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();

        if direction == models::EdgeDirection::Outbound {
            let lower_bound = match t {
                Some(t) => models::EdgeKey::new(id.clone(), t.clone(), Id::default()),
                None => {
                    let empty_type = models::Type::default();
                    models::EdgeKey::new(id.clone(), empty_type, Id::default())
                }
            };
            let range = datastore.edges.range(lower_bound..);

            let range = range.take_while(|&(k, _)| {
                if let Some(t) = t {
                    k.outbound_id == id && &k.t == t
                } else {
                    k.outbound_id == id
                }
            });

            Ok(range.count() as u64)
        } else {
            let range = datastore.edges.iter().filter(|&(k, _)| {
                if let Some(t) = t {
                    k.inbound_id == id && &k.t == t
                } else {
                    k.inbound_id == id
                }
            });

            Ok(range.count() as u64)
        }
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<models::VertexProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let vertex_values = datastore.get_vertex_values_by_query(q.inner)?;

        for (id, _) in vertex_values {
            let property_value = datastore.vertex_properties.get(&(id.clone(), q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(models::VertexProperty::new(id, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let vertex_values = datastore.get_vertex_values_by_query(q.inner)?;

        for (id, _) in vertex_values {
            datastore.vertex_properties.insert((id, q.name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let vertex_values = datastore.get_vertex_values_by_query(q.inner)?;

        for (id, _) in vertex_values {
            datastore.vertex_properties.remove(&(id, q.name.clone()));
        }

        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(q.inner)?;

        for (key, _) in edge_values {
            let property_value = datastore.edge_properties.get(&(key.clone(), q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(models::EdgeProperty::new(key, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let edge_values = datastore.get_edge_values_by_query(q.inner)?;

        for (key, _) in edge_values {
            datastore.edge_properties.insert((key, q.name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let edge_values = datastore.get_edge_values_by_query(q.inner)?;

        for (key, _) in edge_values {
            datastore.edge_properties.remove(&(key, q.name.clone()));
        }

        Ok(())
    }
}
