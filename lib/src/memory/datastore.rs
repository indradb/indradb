
use ::{Datastore, EdgeQuery, Transaction, VertexQuery, VertexPropertyQuery, EdgePropertyQuery};
use ::util::InternableJsonValue;
use chrono::offset::Utc;
use chrono::DateTime;
use errors::Result;
use models;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use hashbrown::{HashMap, HashSet};
use std::hash::Hash;
use internment::ArcIntern;

#[derive(Debug)]
struct PropertyMap<K: Clone + Eq + Hash> {
    key_map: HashMap<K, HashMap<ArcIntern<String>, ArcIntern<InternableJsonValue>>>,
    value_map: HashMap<ArcIntern<String>, HashMap<ArcIntern<InternableJsonValue>, HashSet<K>>>
}

impl<K: Clone + Eq + Hash> Default for PropertyMap<K> {
    fn default() -> Self {
        Self {
            key_map: HashMap::new(),
            value_map: HashMap::new()
        }
    }
}

impl<K: Clone + Eq + Hash> PropertyMap<K> {
    fn remove_range(&mut self, key: K) {
        if let Some(values) = self.key_map.remove(&key) {
            for (name, value) in values {
                if let Some(container) = self.value_map.get_mut(&name) {
                    if let Some(container) = container.get_mut(&value) {
                        container.remove(&key);
                    }
                }
            }
        }
    }

    fn remove(&mut self, key: K, name: String) {
        let name = ArcIntern::new(name);

        if let Some(container) = self.key_map.get_mut(&key) {
            if let Some(value) = container.remove(&name) {
                self.value_map.get_mut(&name).unwrap().remove(&value);
            }
        }
    }

    fn insert(&mut self, key: K, name: String, value: JsonValue) {
        let name = ArcIntern::new(name);
        let value = ArcIntern::new(InternableJsonValue::new(value));
        self.key_map.entry(key.clone()).or_insert_with(HashMap::new).insert(name.clone(), value.clone());
        let container = self.value_map.entry(name).or_insert_with(HashMap::new);
        container.entry(value).or_insert_with(HashSet::new).insert(key);
    }

    fn by_key(&self, key: K, name: String) -> Option<&JsonValue> {
        let name = ArcIntern::new(name);
        Some(&self.key_map.get(&key)?.get(&name)?.0)
    }

    fn by_value(&self, name: String, value: JsonValue) -> Option<&HashSet<K>> {
        let name = ArcIntern::new(name);
        let value = ArcIntern::new(InternableJsonValue::new(value));
        self.value_map.get(&name)?.get(&value)
    }
}

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug)]
struct InternalMemoryDatastore {
    edge_properties: PropertyMap<models::EdgeKey>,
    edges: BTreeMap<models::EdgeKey, DateTime<Utc>>,
    vertex_properties: PropertyMap<Uuid>,
    vertices: BTreeMap<Uuid, models::Type>,
}

impl InternalMemoryDatastore {
    fn get_vertex_values_by_query(&self, q: VertexQuery) -> Result<Vec<(Uuid, models::Type)>> {
        match q {
            VertexQuery::Range(q) => {
                let mut iter: Box<dyn Iterator<Item=(&Uuid, &models::Type)>> = if let Some(start_id) = q.start_id {
                    Box::new(self.vertices.range(start_id..))
                } else {
                    Box::new(self.vertices.iter())
                };

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &t));
                }

                Ok(iter.take(q.limit as usize).map(|(k, v)| (*k, v.clone())).collect())
            }
            VertexQuery::WithId(q) => {
                let mut results = Vec::new();

                for id in q.ids {
                    let value = self.vertices.get(&id);

                    if let Some(value) = value {
                        results.push((id, value.clone()));
                    }
                }

                Ok(results)
            }
            VertexQuery::WithProp(q) => {
                if let Some(candidates) = self.vertex_properties.by_value(q.name, q.value) {
                    Ok(candidates.iter().take(q.limit as usize).map(|id| {
                        let value = self.vertices.get(&id).unwrap();
                        (*id, value.clone())
                    }).collect())
                } else {
                    Ok(vec![])
                }
            }
            VertexQuery::Pipe(q) => {
                let edge_values = self.get_edge_values_by_query(*q.inner)?.into_iter();

                let iter: Box<dyn Iterator<Item=Uuid>> = match q.direction {
                    models::EdgeDirection::Outbound => Box::new(edge_values.map(|(key, _)| key.outbound_id)),
                    models::EdgeDirection::Inbound => Box::new(edge_values.map(|(key, _)| key.inbound_id)),
                };

                let mut iter: Box<dyn Iterator<Item=(Uuid, &models::Type)>> = Box::new(
                    iter.map(|id| (id, self.vertices.get(&id))).filter_map(|(k, v)| Some((k, v?)))
                );

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &t));
                }

                Ok(iter.take(q.limit as usize).map(|(k, v)| (k, v.clone())).collect())
            }
        }
    }

    fn get_edge_values_by_query(&self, q: EdgeQuery) -> Result<Vec<(models::EdgeKey, DateTime<Utc>)>> {
        match q {
            EdgeQuery::WithKey(q) => {
                let mut results = Vec::new();

                for key in q.keys {
                    let value = self.edges.get(&key);

                    if let Some(update_datetime) = value {
                        results.push((key.clone(), *update_datetime));
                    }
                }

                Ok(results)
            }
            EdgeQuery::WithProp(q) => {
                if let Some(candidates) = self.edge_properties.by_value(q.name, q.value) {
                    Ok(candidates.iter().take(q.limit as usize).map(|key| {
                        let value = self.edges.get(&key).unwrap();
                        (key.clone(), *value)
                    }).collect())
                } else {
                    Ok(vec![])
                }
            }
            EdgeQuery::Pipe(q) => {
                let vertex_values = self.get_vertex_values_by_query(*q.inner)?;
                let mut results = Vec::new();

                if q.limit == 0 {
                    return Ok(results);
                }

                match q.direction {
                    models::EdgeDirection::Outbound => {
                        for (id, _) in vertex_values {
                            let lower_bound = match &q.t {
                                Some(t) => models::EdgeKey::new(id, t.clone(), Uuid::default()),
                                None => {
                                    let empty_type = models::Type::default();
                                    models::EdgeKey::new(id, empty_type, Uuid::default())
                                }
                            };

                            for (key, update_datetime) in self.edges.range(lower_bound..) {
                                if key.outbound_id != id {
                                    break;
                                }

                                if let Some(t) = &q.t {
                                    if &key.t != t {
                                        break;
                                    }
                                }

                                if let Some(high) = &q.high {
                                    if update_datetime > high {
                                        continue;
                                    }
                                }

                                if let Some(low) = &q.low {
                                    if update_datetime < low {
                                        continue;
                                    }
                                }

                                results.push((key.clone(), *update_datetime));

                                if results.len() == q.limit as usize {
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

                            if let Some(t) = &q.t {
                                if &key.t != t {
                                    continue;
                                }
                            }

                            if let Some(high) = &q.high {
                                if update_datetime > high {
                                    continue;
                                }
                            }

                            if let Some(low) = &q.low {
                                if update_datetime < low {
                                    continue;
                                }
                            }

                            results.push((key.clone(), *update_datetime));

                            if results.len() == q.limit as usize {
                                return Ok(results);
                            }
                        }
                    }
                }

                Ok(results)
            }
        }
    }

    fn delete_vertices(&mut self, vertices: Vec<Uuid>) {
        for vertex_id in vertices {
            self.vertices.remove(&vertex_id);
            self.vertex_properties.remove_range(vertex_id);

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
            self.edge_properties.remove_range(edge_key);
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
                edge_properties: PropertyMap::default(),
                edges: BTreeMap::new(),
                vertex_properties: PropertyMap::default(),
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

        datastore.vertices.entry(vertex.id).or_insert_with(|| {
            inserted = true;
            vertex.t.clone()
        });

        Ok(inserted)
    }

    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>> {
        let vertex_values = self.datastore.read().unwrap().get_vertex_values_by_query(q.into())?;
        let iter = vertex_values
            .into_iter()
            .map(|(uuid, t)| models::Vertex::with_id(uuid, t));
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

    fn get_edge_count(
        &self,
        id: Uuid,
        type_filter: Option<&models::Type>,
        direction: models::EdgeDirection,
    ) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();

        if direction == models::EdgeDirection::Outbound {
            let lower_bound = match type_filter {
                Some(type_filter) => models::EdgeKey::new(id, type_filter.clone(), Uuid::default()),
                None => {
                    let empty_type = models::Type::default();
                    models::EdgeKey::new(id, empty_type, Uuid::default())
                }
            };
            let range = datastore.edges.range(lower_bound..);

            let range = range.take_while(|&(k, _)| {
                if let Some(type_filter) = type_filter {
                    k.outbound_id == id && &k.t == type_filter
                } else {
                    k.outbound_id == id
                }
            });

            Ok(range.count() as u64)
        } else {
            let range = datastore.edges.iter().filter(|&(k, _)| {
                if let Some(type_filter) = type_filter {
                    k.inbound_id == id && &k.t == type_filter
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
            let property_value = datastore.vertex_properties.by_key(id, q.name.clone());

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
            datastore
                .vertex_properties
                .insert(id, q.name.clone(), value.clone());
        }

        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let vertex_values = datastore.get_vertex_values_by_query(q.inner)?;

        for (id, _) in vertex_values {
            datastore.vertex_properties.remove(id, q.name.clone());
        }

        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(q.inner)?;

        for (key, _) in edge_values {
            let property_value = datastore.edge_properties.by_key(key.clone(), q.name.clone());

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
            datastore.edge_properties.insert(key, q.name.clone(), value.clone());
        }

        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let edge_values = datastore.get_edge_values_by_query(q.inner)?;

        for (key, _) in edge_values {
            datastore.edge_properties.remove(key, q.name.clone());
        }

        Ok(())
    }
}
