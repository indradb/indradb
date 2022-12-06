use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::sync::{Arc, RwLock};

use crate::errors::{Error, Result};
use crate::util;
use crate::{
    Datastore, Edge, EdgeDirection, EdgeKey, EdgeProperties, EdgeProperty, EdgePropertyQuery, EdgeQuery, Identifier,
    Json, NamedProperty, Vertex, VertexProperties, VertexProperty, VertexPropertyQuery, VertexQuery,
};

use bincode::Error as BincodeError;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use uuid::Uuid;

macro_rules! iter_vertex_values {
    ($self:expr, $iter:expr) => {
        Box::new($iter.filter_map(move |id| $self.vertices.get(&id).map(|value| (id, value.clone()))))
    };
}

macro_rules! iter_edge_values {
    ($self:expr, $iter:expr) => {
        Box::new($iter.filter_map(move |key| $self.edges.get(&key).map(|update_datetime| (key, *update_datetime))))
    };
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
enum IndexedPropertyMember {
    Vertex(Uuid),
    Edge(EdgeKey),
}

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug, Default, Serialize, Deserialize)]
struct InternalMemoryDatastore {
    vertices: BTreeMap<Uuid, Identifier>,
    edges: BTreeMap<EdgeKey, DateTime<Utc>>,
    reversed_edges: BTreeMap<EdgeKey, DateTime<Utc>>,
    vertex_properties: BTreeMap<(Uuid, Identifier), Json>,
    edge_properties: BTreeMap<(EdgeKey, Identifier), Json>,
    property_values: HashMap<Identifier, HashMap<Json, HashSet<IndexedPropertyMember>>>,
}

type QueryIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

impl InternalMemoryDatastore {
    fn get_all_vertices_with_property(
        &self,
        property_name: &Identifier,
        error_if_missing: bool,
    ) -> Result<HashSet<Uuid>> {
        let mut vertices = HashSet::<Uuid>::default();
        if let Some(container) = self.property_values.get(property_name) {
            for sub_container in container.values() {
                for member in sub_container {
                    if let IndexedPropertyMember::Vertex(id) = member {
                        vertices.insert(*id);
                    }
                }
            }
        } else if error_if_missing {
            return Err(Error::NotIndexed);
        }
        Ok(vertices)
    }

    fn get_all_edges_with_property(
        &self,
        property_name: &Identifier,
        error_if_missing: bool,
    ) -> Result<HashSet<EdgeKey>> {
        let mut edges = HashSet::<EdgeKey>::default();
        if let Some(container) = self.property_values.get(property_name) {
            for sub_container in container.values() {
                for member in sub_container {
                    if let IndexedPropertyMember::Edge(edge_key) = member {
                        edges.insert(edge_key.clone());
                    }
                }
            }
        } else if error_if_missing {
            return Err(Error::NotIndexed);
        }
        Ok(edges)
    }

    fn get_vertex_values_by_query(&self, q: &VertexQuery) -> Result<QueryIter<'_, (Uuid, Identifier)>> {
        match q {
            VertexQuery::Range(range) => {
                let mut iter: QueryIter<(&Uuid, &Identifier)> = if let Some(start_id) = range.start_id {
                    Box::new(self.vertices.range(start_id..))
                } else {
                    Box::new(self.vertices.iter())
                };

                if let Some(ref t) = range.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &t));
                }

                let iter: QueryIter<(Uuid, Identifier)> =
                    Box::new(iter.take(range.limit as usize).map(|(k, v)| (*k, v.clone())));

                Ok(iter)
            }
            VertexQuery::Specific(specific) => Ok(iter_vertex_values!(self, specific.ids.into_iter())),
            VertexQuery::Pipe(pipe) => {
                let edge_values = self.get_edge_values_by_query(&pipe.inner)?;

                let iter: QueryIter<Uuid> = match pipe.direction {
                    EdgeDirection::Outbound => Box::new(edge_values.map(|(key, _)| key.outbound_id)),
                    EdgeDirection::Inbound => Box::new(edge_values.map(|(key, _)| key.inbound_id)),
                };

                let mut iter: QueryIter<(Uuid, &Identifier)> = Box::new(
                    iter.map(move |id| (id, self.vertices.get(&id)))
                        .filter_map(|(k, v)| Some((k, v?))),
                );

                if let Some(t) = pipe.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &&t));
                }

                let iter: QueryIter<(Uuid, Identifier)> =
                    Box::new(iter.take(pipe.limit as usize).map(|(k, v)| (k, v.clone())));

                Ok(iter)
            }
            VertexQuery::PropertyPresence(q) => {
                let vertices = self.get_all_vertices_with_property(&q.name, true)?;
                Ok(iter_vertex_values!(self, vertices.into_iter()))
            }
            VertexQuery::PropertyValue(q) => {
                if let Some(container) = self.property_values.get(&q.name) {
                    if let Some(sub_container) = container.get(&q.value) {
                        let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
                            IndexedPropertyMember::Vertex(id) => {
                                self.vertices.get(id).map(|value| (*id, value.clone()))
                            }
                            _ => None,
                        }));
                        return Ok(iter);
                    }
                    Ok(iter_vertex_values!(self, Vec::default().into_iter()))
                } else {
                    Err(Error::NotIndexed)
                }
            }
            VertexQuery::PipePropertyPresence(q) => {
                let vertices_with_property = self.get_all_vertices_with_property(&q.name, false)?;
                let vertex_values = self.get_vertex_values_by_query(&q.inner)?;

                let iter: QueryIter<(Uuid, Identifier)> = if q.exists {
                    Box::new(vertex_values.filter(move |(id, _)| vertices_with_property.contains(id)))
                } else {
                    Box::new(vertex_values.filter(move |(id, _)| !vertices_with_property.contains(id)))
                };

                Ok(iter)
            }
            VertexQuery::PipePropertyValue(q) => {
                let vertex_values = self.get_vertex_values_by_query(&q.inner)?;

                let ids: HashSet<Uuid> = if let Some(container) = self.property_values.get(&q.name) {
                    if let Some(members) = container.get(&q.value) {
                        members
                            .iter()
                            .filter_map(|member| match member {
                                IndexedPropertyMember::Vertex(id) => Some(*id),
                                _ => None,
                            })
                            .collect()
                    } else {
                        HashSet::default()
                    }
                } else {
                    HashSet::default()
                };

                let iter: QueryIter<(Uuid, Identifier)> = if q.equal {
                    Box::new(vertex_values.filter(move |(id, _)| ids.contains(id)))
                } else {
                    Box::new(vertex_values.filter(move |(id, _)| !ids.contains(id)))
                };

                Ok(iter)
            }
        }
    }

    fn get_edge_values_by_query(&self, q: &EdgeQuery) -> Result<QueryIter<'_, (EdgeKey, DateTime<Utc>)>> {
        match q {
            EdgeQuery::Specific(specific) => Ok(iter_edge_values!(self, specific.keys.into_iter())),
            EdgeQuery::Pipe(pipe) => {
                let iter = self.get_vertex_values_by_query(&pipe.inner)?;

                let t = pipe.t.clone();
                let direction = pipe.direction;

                let mut iter: QueryIter<(&EdgeKey, &DateTime<Utc>)> = Box::new(iter.flat_map(move |(id, _)| {
                    let lower_bound = match &t {
                        Some(t) => EdgeKey::new(id, t.clone(), Uuid::default()),
                        None => EdgeKey::new(id, Identifier::default(), Uuid::default()),
                    };

                    let iter = if direction == EdgeDirection::Outbound {
                        self.edges.range(lower_bound..)
                    } else {
                        self.reversed_edges.range(lower_bound..)
                    };

                    iter.take_while(move |(key, _)| key.outbound_id == id)
                }));

                if let Some(t) = pipe.t {
                    iter = Box::new(iter.filter(move |(key, _)| key.t == t));
                }

                if let Some(high) = pipe.high {
                    iter = Box::new(iter.filter(move |(_, update_datetime)| update_datetime <= &&high));
                }

                if let Some(low) = pipe.low {
                    iter = Box::new(iter.filter(move |(_, update_datetime)| update_datetime >= &&low));
                }

                let iter = iter.take(pipe.limit as usize);

                let iter: QueryIter<(EdgeKey, DateTime<Utc>)> = if direction == EdgeDirection::Outbound {
                    Box::new(iter.map(move |(key, value)| (key.clone(), *value)))
                } else {
                    Box::new(iter.map(move |(key, value)| (key.reversed(), *value)))
                };

                let iter = Box::new(iter);
                Ok(iter)
            }
            EdgeQuery::PropertyPresence(q) => {
                let edges = self.get_all_edges_with_property(&q.name, true)?;
                Ok(iter_edge_values!(self, edges.into_iter()))
            }
            EdgeQuery::PropertyValue(q) => {
                if let Some(container) = self.property_values.get(&q.name) {
                    if let Some(sub_container) = container.get(&q.value) {
                        let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
                            IndexedPropertyMember::Edge(key) => self.edges.get(key).map(|value| (key.clone(), *value)),
                            _ => None,
                        }));
                        return Ok(iter);
                    }
                    Ok(iter_edge_values!(self, Vec::default().into_iter()))
                } else {
                    Err(Error::NotIndexed)
                }
            }
            EdgeQuery::PipePropertyPresence(q) => {
                let edges_with_property = self.get_all_edges_with_property(&q.name, false)?;
                let edge_values = self.get_edge_values_by_query(&q.inner)?;

                let iter: QueryIter<(EdgeKey, DateTime<Utc>)> = if q.exists {
                    Box::new(edge_values.filter(move |(key, _)| edges_with_property.contains(key)))
                } else {
                    Box::new(edge_values.filter(move |(key, _)| !edges_with_property.contains(key)))
                };

                Ok(iter)
            }
            EdgeQuery::PipePropertyValue(q) => {
                let edge_values = self.get_edge_values_by_query(&q.inner)?;

                let keys: HashSet<EdgeKey> = if let Some(container) = self.property_values.get(&q.name) {
                    if let Some(members) = container.get(&q.value) {
                        members
                            .iter()
                            .filter_map(|member| match member {
                                IndexedPropertyMember::Edge(key) => Some(key.clone()),
                                _ => None,
                            })
                            .collect()
                    } else {
                        HashSet::default()
                    }
                } else {
                    HashSet::default()
                };

                let iter: QueryIter<(EdgeKey, DateTime<Utc>)> = if q.equal {
                    Box::new(edge_values.filter(move |(key, _)| keys.contains(key)))
                } else {
                    Box::new(edge_values.filter(move |(key, _)| !keys.contains(key)))
                };

                Ok(iter)
            }
        }
    }

    fn delete_vertices(&mut self, vertices: Vec<Uuid>) {
        for vertex_id in vertices {
            self.vertices.remove(&vertex_id);

            let mut deletable_vertex_properties: Vec<(Uuid, Identifier)> = Vec::new();
            for (property_key, _) in self.vertex_properties.range((vertex_id, Identifier::default())..) {
                let &(ref property_vertex_id, _) = property_key;

                if &vertex_id != property_vertex_id {
                    break;
                }

                deletable_vertex_properties.push(property_key.clone());
            }
            self.delete_vertex_properties(deletable_vertex_properties);

            let mut deletable_edges: Vec<EdgeKey> = Vec::new();
            for edge_key in self.edges.keys() {
                if edge_key.outbound_id == vertex_id || edge_key.inbound_id == vertex_id {
                    deletable_edges.push(edge_key.clone());
                }
            }
            self.delete_edges(deletable_edges);
        }
    }

    fn delete_vertex_properties(&mut self, keys: Vec<(Uuid, Identifier)>) {
        for property_key in keys {
            if let Some(property_value) = self.vertex_properties.remove(&property_key) {
                let (property_vertex_id, property_name) = property_key;
                if let Some(property_container) = self.property_values.get_mut(&property_name) {
                    debug_assert!(property_container
                        .get_mut(&property_value)
                        .unwrap()
                        .remove(&IndexedPropertyMember::Vertex(property_vertex_id)));
                }
            }
        }
    }

    fn delete_edges(&mut self, edges: Vec<EdgeKey>) {
        for edge_key in edges {
            self.edges.remove(&edge_key);
            self.reversed_edges.remove(&edge_key.reversed());

            let mut deletable_edge_properties: Vec<(EdgeKey, Identifier)> = Vec::new();
            for (property_key, _) in self.edge_properties.range((edge_key.clone(), Identifier::default())..) {
                let &(ref property_edge_key, _) = property_key;

                if &edge_key != property_edge_key {
                    break;
                }

                deletable_edge_properties.push(property_key.clone());
            }
            self.delete_edge_properties(deletable_edge_properties)
        }
    }

    fn delete_edge_properties(&mut self, keys: Vec<(EdgeKey, Identifier)>) {
        for property_key in keys {
            if let Some(property_value) = self.edge_properties.remove(&property_key) {
                let (property_edge_key, property_name) = property_key;
                if let Some(property_container) = self.property_values.get_mut(&property_name) {
                    debug_assert!(property_container
                        .get_mut(&property_value)
                        .unwrap()
                        .remove(&IndexedPropertyMember::Edge(property_edge_key)));
                }
            }
        }
    }
}

/// An in-memory datastore.
#[derive(Debug, Clone)]
pub struct MemoryDatastore {
    datastore: Arc<RwLock<InternalMemoryDatastore>>,
    path: Option<PathBuf>,
}

impl Default for MemoryDatastore {
    fn default() -> MemoryDatastore {
        Self {
            datastore: Arc::new(RwLock::new(InternalMemoryDatastore::default())),
            path: None,
        }
    }
}

impl MemoryDatastore {
    /// Reads a persisted image from disk. Calls to sync will overwrite the
    /// file at the specified path.
    ///
    /// # Arguments
    /// * `path`: The path to the persisted image.
    pub fn read<P: Into<PathBuf>>(path: P) -> StdResult<MemoryDatastore, BincodeError> {
        let path = path.into();
        let buf = BufReader::new(File::open(&path)?);
        let datastore = bincode::deserialize_from(buf)?;
        Ok(MemoryDatastore {
            datastore: Arc::new(RwLock::new(datastore)),
            path: Some(path),
        })
    }

    /// Creates a new datastore. Calls to sync will overwrite the file at the
    /// specified path, but as opposed to `read`, this will not read the file
    /// first.
    ///
    /// # Arguments
    /// * `path`: The path to the persisted image.
    pub fn create<P: Into<PathBuf>>(path: P) -> StdResult<MemoryDatastore, BincodeError> {
        Ok(MemoryDatastore {
            datastore: Arc::new(RwLock::new(InternalMemoryDatastore::default())),
            path: Some(path.into()),
        })
    }
}

impl Datastore for MemoryDatastore {
    fn sync(&self) -> Result<()> {
        if let Some(ref persist_path) = self.path {
            let temp_path = NamedTempFile::new().map_err(|err| Error::Datastore(Box::new(err)))?;
            let buf = BufWriter::new(temp_path.as_file());
            let datastore = self.datastore.read().unwrap();
            bincode::serialize_into(buf, &*datastore)?;
            temp_path
                .persist(persist_path)
                .map_err(|err| Error::Datastore(Box::new(err)))?;
        }
        Ok(())
    }

    fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();
        let mut inserted = false;

        datastore.vertices.entry(vertex.id).or_insert_with(|| {
            inserted = true;
            vertex.t.clone()
        });

        Ok(inserted)
    }

    fn get_vertices(&self, q: &VertexQuery) -> Result<Vec<Vertex>> {
        let datastore = self.datastore.read().unwrap();
        let iter = datastore.get_vertex_values_by_query(q)?;
        let iter = iter.map(|(uuid, t)| Vertex::with_id(uuid, t));
        Ok(iter.collect())
    }

    fn delete_vertices(&self, q: &VertexQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_vertices = datastore.get_vertex_values_by_query(q)?.map(|(k, _)| k).collect();
        datastore.delete_vertices(deletable_vertices);
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();
        Ok(datastore.vertices.len() as u64)
    }

    fn create_edge(&self, key: &EdgeKey) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();

        if !datastore.vertices.contains_key(&key.outbound_id) || !datastore.vertices.contains_key(&key.inbound_id) {
            return Ok(false);
        }

        datastore.edges.insert(key.clone(), Utc::now());
        datastore.reversed_edges.insert(key.reversed(), Utc::now());
        Ok(true)
    }

    fn get_edges(&self, q: &EdgeQuery) -> Result<Vec<Edge>> {
        let edge_values: Vec<(EdgeKey, DateTime<Utc>)> = {
            let datastore = self.datastore.read().unwrap();
            let iter = datastore.get_edge_values_by_query(q)?;
            iter.collect()
        };

        let iter = edge_values
            .into_iter()
            .map(|(key, update_datetime)| Edge::new(key, update_datetime));
        Ok(iter.collect())
    }

    fn delete_edges(&self, q: &EdgeQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_edges: Vec<EdgeKey> = datastore.get_edge_values_by_query(q)?.map(|(k, _)| k).collect();
        datastore.delete_edges(deletable_edges);
        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&Identifier>, direction: EdgeDirection) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();

        let lower_bound = match t {
            Some(t) => EdgeKey::new(id, t.clone(), Uuid::default()),
            None => EdgeKey::new(id, Identifier::default(), Uuid::default()),
        };

        let range = if direction == EdgeDirection::Outbound {
            datastore.edges.range(lower_bound..)
        } else {
            datastore.reversed_edges.range(lower_bound..)
        };

        let range = range.take_while(|&(k, _)| {
            if let Some(t) = t {
                k.outbound_id == id && &k.t == t
            } else {
                k.outbound_id == id
            }
        });

        Ok(range.count() as u64)
    }

    fn get_vertex_properties(&self, q: &VertexPropertyQuery) -> Result<Vec<VertexProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let vertex_values = datastore.get_vertex_values_by_query(&q.inner)?;

        for (id, _) in vertex_values {
            let property_value = datastore.vertex_properties.get(&(id, q.name.clone()));
            if let Some(property_value) = property_value {
                result.push(VertexProperty::new(id, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn get_all_vertex_properties(&self, q: &VertexQuery) -> Result<Vec<VertexProperties>> {
        let datastore = self.datastore.read().unwrap();
        let vertex_values = datastore.get_vertex_values_by_query(q)?;

        let mut result = Vec::new();
        for (id, t) in vertex_values {
            let from = &(id, Identifier::default());
            let to = &(util::next_uuid(id).unwrap(), Identifier::default());

            let properties = datastore.vertex_properties.range(from..to);
            result.push(VertexProperties::new(
                Vertex::with_id(id, t),
                properties
                    .map(|(n, p)| NamedProperty::new(n.1.clone(), p.clone()))
                    .collect(),
            ));
        }

        Ok(result)
    }

    fn set_vertex_properties(&self, q: &VertexPropertyQuery, value: Json) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let vertex_values: Vec<(Uuid, Identifier)> = datastore.get_vertex_values_by_query(&q.inner)?.collect();

        let mut deletable_vertex_properties = Vec::<(Uuid, Identifier)>::new();
        for (id, _) in &vertex_values {
            deletable_vertex_properties.push((*id, q.name.clone()));
        }
        datastore.delete_vertex_properties(deletable_vertex_properties);

        for (id, _) in &vertex_values {
            datastore
                .vertex_properties
                .insert((*id, q.name.clone()), value.clone());
        }

        if let Some(property_container) = datastore.property_values.get_mut(&q.name) {
            let property_container = property_container.entry(value).or_insert_with(HashSet::new);
            for (id, _) in vertex_values.into_iter() {
                property_container.insert(IndexedPropertyMember::Vertex(id));
            }
        }

        Ok(())
    }

    fn delete_vertex_properties(&self, q: &VertexPropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let mut deletable_vertex_properties = Vec::<(Uuid, Identifier)>::new();
        for (id, _) in datastore.get_vertex_values_by_query(&q.inner)? {
            deletable_vertex_properties.push((id, q.name.clone()));
        }
        datastore.delete_vertex_properties(deletable_vertex_properties);
        Ok(())
    }

    fn get_edge_properties(&self, q: &EdgePropertyQuery) -> Result<Vec<EdgeProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(&q.inner)?;

        for (key, _) in edge_values {
            let property_value = datastore.edge_properties.get(&(key.clone(), q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(EdgeProperty::new(key, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn get_all_edge_properties(&self, q: &EdgeQuery) -> Result<Vec<EdgeProperties>> {
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(q)?;

        let mut result = Vec::new();
        for (id, t) in edge_values {
            let from = &(id.clone(), Identifier::default());

            let properties = datastore
                .edge_properties
                .range(from..)
                .take_while(|((key, _name), _value)| *key == id);
            result.push(EdgeProperties::new(
                Edge::new(id.clone(), t),
                properties
                    .map(|(n, p)| NamedProperty::new(n.1.clone(), p.clone()))
                    .collect(),
            ));
        }

        Ok(result)
    }

    fn set_edge_properties(&self, q: &EdgePropertyQuery, value: Json) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edge_values: Vec<(EdgeKey, DateTime<Utc>)> = datastore.get_edge_values_by_query(&q.inner)?.collect();

        let mut deletable_edge_properties = Vec::<(EdgeKey, Identifier)>::new();
        for (key, _) in &edge_values {
            deletable_edge_properties.push((key.clone(), q.name.clone()));
        }
        datastore.delete_edge_properties(deletable_edge_properties);

        for (key, _) in &edge_values {
            datastore
                .edge_properties
                .insert((key.clone(), q.name.clone()), value.clone());
        }

        if let Some(property_container) = datastore.property_values.get_mut(&q.name) {
            let property_container = property_container.entry(value).or_insert_with(HashSet::new);
            for (key, _) in edge_values.into_iter() {
                property_container.insert(IndexedPropertyMember::Edge(key));
            }
        }

        Ok(())
    }

    fn delete_edge_properties(&self, q: &EdgePropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edge_values: Vec<(EdgeKey, DateTime<Utc>)> = datastore.get_edge_values_by_query(&q.inner)?.collect();
        let mut deletable_edge_properties = Vec::<(EdgeKey, Identifier)>::new();
        for (key, _) in edge_values {
            deletable_edge_properties.push((key, q.name.clone()));
        }
        datastore.delete_edge_properties(deletable_edge_properties);
        Ok(())
    }

    fn index_property(&self, name: Identifier) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let mut property_container: HashMap<Json, HashSet<IndexedPropertyMember>> = HashMap::new();
        for id in datastore.vertices.keys() {
            if let Some(value) = datastore.vertex_properties.get(&(*id, name.clone())) {
                property_container
                    .entry(value.clone())
                    .or_insert_with(HashSet::new)
                    .insert(IndexedPropertyMember::Vertex(*id));
            }
        }
        for key in datastore.edges.keys() {
            if let Some(value) = datastore.edge_properties.get(&(key.clone(), name.clone())) {
                property_container
                    .entry(value.clone())
                    .or_insert_with(HashSet::new)
                    .insert(IndexedPropertyMember::Edge(key.clone()));
            }
        }

        let existing_property_container = datastore.property_values.entry(name).or_insert_with(HashMap::new);
        for (value, members) in property_container.into_iter() {
            let existing_members = existing_property_container.entry(value).or_insert_with(HashSet::new);
            for member in members {
                existing_members.insert(member);
            }
        }

        Ok(())
    }
}
