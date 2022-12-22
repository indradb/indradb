use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::sync::{Arc, RwLock};

use crate::errors::{Error, Result};
use crate::util;
use crate::{Datastore, Edge, EdgeDirection, EdgeKey, Identifier, Json, Query, QueryOutputValue, Vertex};

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

    fn query(&self, q: &Query, output: &mut Vec<QueryOutputValue>) -> Result<()> {
        let value = match q {
            Query::AllVertices(_) => {
                let iter = self.vertices.iter().map(|(id, t)| Vertex::with_id(*id, t.clone()));
                QueryOutputValue::Vertices(iter.collect())
            }
            Query::RangeVertex(ref q) => {
                let mut iter: QueryIter<(&Uuid, &Identifier)> = if let Some(start_id) = q.start_id {
                    Box::new(self.vertices.range(start_id..))
                } else {
                    Box::new(self.vertices.iter())
                };

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &t));
                }

                iter = Box::new(iter.take(q.limit as usize));
                let iter = iter.map(|(id, t)| Vertex::with_id(*id, t.clone()));

                QueryOutputValue::Vertices(iter.collect())
            }
            Query::SpecificVertex(ref q) => {
                let iter = iter_vertex_values!(self, q.ids.iter());
                let iter = iter.map(|(id, t)| Vertex::with_id(*id, t.clone()));
                QueryOutputValue::Vertices(iter.collect())
            }
            Query::Pipe(ref q) => {
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        let iter: QueryIter<Uuid> = match q.direction {
                            EdgeDirection::Outbound => Box::new(piped_edges.iter().map(|e| e.key.outbound_id)),
                            EdgeDirection::Inbound => Box::new(piped_edges.iter().map(|e| e.key.inbound_id)),
                        };

                        let mut iter: QueryIter<(Uuid, &Identifier)> = Box::new(
                            iter.map(move |id| (id, self.vertices.get(&id)))
                                .filter_map(|(k, v)| Some((k, v?))),
                        );

                        if let Some(ref t) = q.t {
                            iter = Box::new(iter.filter(move |(_, v)| v == &t));
                        }

                        iter = Box::new(iter.take(q.limit as usize));

                        let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone())).collect();
                        QueryOutputValue::Vertices(iter)
                    }
                    QueryOutputValue::Vertices(ref piped_vertices) => {
                        let mut iter: QueryIter<(&EdgeKey, &DateTime<Utc>)> =
                            Box::new(piped_vertices.iter().flat_map(move |v| {
                                let lower_bound = match &q.t {
                                    Some(t) => EdgeKey::new(v.id, t.clone(), Uuid::default()),
                                    None => EdgeKey::new(v.id, Identifier::default(), Uuid::default()),
                                };

                                let iter = if q.direction == EdgeDirection::Outbound {
                                    self.edges.range(lower_bound..)
                                } else {
                                    self.reversed_edges.range(lower_bound..)
                                };

                                iter.take_while(move |(key, _)| key.outbound_id == v.id)
                            }));

                        if let Some(ref t) = q.t {
                            iter = Box::new(iter.filter(move |(key, _)| &key.t == t));
                        }

                        let iter = iter.take(q.limit as usize);

                        let iter: QueryIter<(EdgeKey, DateTime<Utc>)> = if q.direction == EdgeDirection::Outbound {
                            Box::new(iter.map(move |(key, value)| (key.clone(), *value)))
                        } else {
                            Box::new(iter.map(move |(key, value)| (key.reversed(), *value)))
                        };

                        let iter = iter.map(move |(key, value)| Edge::new(key, value)).collect();
                        QueryOutputValue::Edges(iter)
                    }
                    _ => {
                        return Err(Error::Unsupported);
                    }
                };

                if let Query::Include(_) = *q.inner {
                    // keep the value exported
                    output.push(piped_values);
                }

                values
            }
            Query::PipeProperty(ref q) => {
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        let mut edge_properties = Vec::new();
                        for edge in piped_edges.into_iter() {
                            if let Some(name) = &q.name {
                                if let Some(value) = self.edge_properties.get(&(edge.key.clone(), name.clone())) {
                                    edge_properties.push((edge.clone(), name.clone(), value.0.clone()));
                                }
                            } else {
                                let from = &(edge.key.clone(), Identifier::default());
                                for ((prop_edge_key, prop_name), prop_value) in self.edge_properties.range(from..) {
                                    if prop_edge_key != &edge.key {
                                        break;
                                    }
                                    edge_properties.push((edge.clone(), prop_name.clone(), prop_value.0.clone()));
                                }
                            }
                        }

                        QueryOutputValue::EdgeProperties(edge_properties)
                    }
                    QueryOutputValue::Vertices(ref piped_vertices) => {
                        let mut vertex_properties = Vec::with_capacity(piped_vertices.len());
                        for vertex in piped_vertices.into_iter() {
                            if let Some(name) = &q.name {
                                if let Some(value) = self.vertex_properties.get(&(vertex.id, name.clone())) {
                                    vertex_properties.push((vertex.clone(), name.clone(), value.0.clone()));
                                }
                            } else {
                                let from = &(vertex.id, Identifier::default());
                                let to = &(util::next_uuid(vertex.id).unwrap(), Identifier::default());
                                for ((_prop_vertex_id, prop_name), prop_value) in self.vertex_properties.range(from..to)
                                {
                                    vertex_properties.push((vertex.clone(), prop_name.clone(), prop_value.0.clone()));
                                }
                            }
                        }

                        QueryOutputValue::VertexProperties(vertex_properties)
                    }
                    _ => {
                        return Err(Error::Unsupported);
                    }
                };

                if let Query::Include(_) = *q.inner {
                    // keep the value exported
                    output.push(piped_values);
                }

                values
            }
            Query::VertexWithPropertyPresence(ref q) => {
                let vertices = self.get_all_vertices_with_property(&q.name, true)?;
                let iter = iter_vertex_values!(self, vertices.into_iter());
                let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone()));
                QueryOutputValue::Vertices(iter.collect())
            }
            Query::VertexWithPropertyValue(ref q) => {
                if let Some(container) = self.property_values.get(&q.name) {
                    let wrapped_value = Json::new(q.value.clone());
                    if let Some(sub_container) = container.get(&wrapped_value) {
                        let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
                            IndexedPropertyMember::Vertex(id) => {
                                self.vertices.get(id).map(|value| (*id, value.clone()))
                            }
                            _ => None,
                        }));
                        let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone()));
                        QueryOutputValue::Vertices(iter.collect())
                    } else {
                        let iter = iter_vertex_values!(self, Vec::default().into_iter());
                        let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone()));
                        QueryOutputValue::Vertices(iter.collect())
                    }
                } else {
                    return Err(Error::NotIndexed);
                }
            }
            Query::EdgeWithPropertyPresence(ref q) => {
                let edges = self.get_all_edges_with_property(&q.name, true)?;
                let iter = iter_edge_values!(self, edges.into_iter());
                let iter = iter.map(|(key, dt)| Edge::new(key.clone(), dt.clone()));
                QueryOutputValue::Edges(iter.collect())
            }
            Query::EdgeWithPropertyValue(ref q) => {
                if let Some(container) = self.property_values.get(&q.name) {
                    let wrapped_value = Json::new(q.value.clone());
                    if let Some(sub_container) = container.get(&wrapped_value) {
                        let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
                            IndexedPropertyMember::Edge(key) => self.edges.get(key).map(|dt| (key.clone(), dt.clone())),
                            _ => None,
                        }));
                        let iter = iter.map(|(key, dt)| Edge::new(key, dt));
                        QueryOutputValue::Edges(iter.collect())
                    } else {
                        let iter = iter_edge_values!(self, Vec::default().into_iter());
                        let iter = iter.map(|(key, dt)| Edge::new(key, dt));
                        QueryOutputValue::Edges(iter.collect())
                    }
                } else {
                    return Err(Error::NotIndexed);
                }
            }
            Query::PipeWithPropertyPresence(ref q) => {
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        let edges_with_property = self.get_all_edges_with_property(&q.name, false)?;
                        let iter = piped_edges.iter().filter(move |e| {
                            let contains = edges_with_property.contains(&e.key);
                            (q.exists && contains) || (!q.exists && !contains)
                        });
                        QueryOutputValue::Edges(iter.cloned().collect())
                    }
                    QueryOutputValue::Vertices(ref piped_vertices) => {
                        let vertices_with_property = self.get_all_vertices_with_property(&q.name, false)?;
                        let iter = piped_vertices.iter().filter(move |v| {
                            let contains = vertices_with_property.contains(&v.id);
                            (q.exists && contains) || (!q.exists && !contains)
                        });
                        QueryOutputValue::Vertices(iter.cloned().collect())
                    }
                    _ => {
                        return Err(Error::Unsupported);
                    }
                };

                if let Query::Include(_) = *q.inner {
                    // keep the value exported
                    output.push(piped_values);
                }

                values
            }
            Query::PipeWithPropertyValue(ref q) => {
                self.query(&*q.inner, output)?;
                let piped_values = output.pop().unwrap();

                let empty_hashset = HashSet::default();
                let indexed_members: &HashSet<IndexedPropertyMember> =
                    if let Some(container) = self.property_values.get(&q.name) {
                        let wrapped_value = Json::new(q.value.clone());
                        if let Some(ref members) = container.get(&wrapped_value) {
                            members
                        } else {
                            &empty_hashset
                        }
                    } else {
                        &empty_hashset
                    };

                let values = match piped_values {
                    QueryOutputValue::Edges(ref piped_edges) => {
                        let iter = piped_edges.iter().filter(move |e| {
                            let contains = indexed_members.contains(&IndexedPropertyMember::Edge(e.key.clone()));
                            (q.equal && contains) || (!q.equal && !contains)
                        });
                        QueryOutputValue::Edges(iter.cloned().collect())
                    }
                    QueryOutputValue::Vertices(ref piped_vertices) => {
                        let iter = piped_vertices.iter().filter(move |v| {
                            let contains = indexed_members.contains(&IndexedPropertyMember::Vertex(v.id));
                            (q.equal && contains) || (!q.equal && !contains)
                        });
                        QueryOutputValue::Vertices(iter.cloned().collect())
                    }
                    _ => {
                        return Err(Error::Unsupported);
                    }
                };

                if let Query::Include(_) = *q.inner {
                    // keep the value exported
                    output.push(piped_values);
                }

                values
            }
            Query::AllEdges(_) => {
                let iter = self
                    .edges
                    .iter()
                    .map(move |(key, value)| Edge::new(key.clone(), value.clone()));
                QueryOutputValue::Edges(iter.collect())
            }
            Query::SpecificEdge(ref q) => {
                let iter = iter_edge_values!(self, q.keys.clone().into_iter());
                let iter = iter.map(move |(key, value)| Edge::new(key, value));
                QueryOutputValue::Edges(iter.collect())
            }
            Query::Include(ref q) => {
                self.query(&*q.inner, output)?;
                output.pop().unwrap()
            }
            Query::Count(ref q) => {
                let count = match &*q.inner {
                    // These paths are optimized
                    Query::AllVertices(_) => self.vertices.len(),
                    Query::AllEdges(_) => self.edges.len(),
                    q => {
                        self.query(q, output)?;
                        let piped_values = output.pop().unwrap();
                        match piped_values {
                            QueryOutputValue::Vertices(v) => v.len(),
                            QueryOutputValue::Edges(v) => v.len(),
                            QueryOutputValue::VertexProperties(v) => v.len(),
                            QueryOutputValue::EdgeProperties(v) => v.len(),
                            _ => return Err(Error::Unsupported),
                        }
                    }
                };
                QueryOutputValue::Count(count as u64)
            }
        };

        output.push(value);
        Ok(())
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

    fn create_edge(&self, key: &EdgeKey) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();

        if !datastore.vertices.contains_key(&key.outbound_id) || !datastore.vertices.contains_key(&key.inbound_id) {
            return Ok(false);
        }

        datastore.edges.insert(key.clone(), Utc::now());
        datastore.reversed_edges.insert(key.reversed(), Utc::now());
        Ok(true)
    }

    fn get(&self, q: Query) -> Result<Vec<QueryOutputValue>> {
        let mut output = Vec::new();
        let datastore = self.datastore.read().unwrap();
        datastore.query(&q, &mut output)?;
        Ok(output)
    }

    fn delete(&self, q: Query) -> Result<()> {
        let mut output = Vec::new();
        let mut datastore = self.datastore.write().unwrap();
        datastore.query(&q, &mut output)?;
        match output.pop().unwrap() {
            QueryOutputValue::Vertices(vertices) => {
                datastore.delete_vertices(vertices.into_iter().map(|v| v.id).collect());
            }
            QueryOutputValue::Edges(edges) => {
                datastore.delete_edges(edges.into_iter().map(|e| e.key).collect());
            }
            QueryOutputValue::VertexProperties(vertex_properties) => datastore.delete_vertex_properties(
                vertex_properties
                    .into_iter()
                    .map(|(vertex, prop_name, _prop_value)| (vertex.id, prop_name.clone()))
                    .collect(),
            ),
            QueryOutputValue::EdgeProperties(edge_properties) => datastore.delete_edge_properties(
                edge_properties
                    .into_iter()
                    .map(|(edge, prop_name, _prop_value)| (edge.key, prop_name.clone()))
                    .collect(),
            ),
            QueryOutputValue::Count(_) => return Err(Error::Unsupported),
        }
        Ok(())
    }

    fn set_properties(&self, q: Query, name: Identifier, value: serde_json::Value) -> Result<()> {
        let mut output = Vec::new();
        let mut datastore = self.datastore.write().unwrap();
        datastore.query(&q, &mut output)?;
        match output.pop().unwrap() {
            QueryOutputValue::Vertices(vertices) => {
                let mut deletable_vertex_properties = Vec::new();
                for vertex in &vertices {
                    deletable_vertex_properties.push((vertex.id, name.clone()));
                }
                datastore.delete_vertex_properties(deletable_vertex_properties);

                let wrapped_value = Json::new(value);
                for vertex in &vertices {
                    datastore
                        .vertex_properties
                        .insert((vertex.id, name.clone()), wrapped_value.clone());
                }

                if let Some(property_container) = datastore.property_values.get_mut(&name) {
                    let property_container = property_container.entry(wrapped_value).or_insert_with(HashSet::new);
                    for vertex in vertices.into_iter() {
                        property_container.insert(IndexedPropertyMember::Vertex(vertex.id));
                    }
                }
            }
            QueryOutputValue::Edges(edges) => {
                let mut deletable_edge_properties = Vec::new();
                for edge in &edges {
                    deletable_edge_properties.push((edge.key.clone(), name.clone()));
                }
                datastore.delete_edge_properties(deletable_edge_properties);

                let wrapped_value = Json::new(value);
                for edge in &edges {
                    datastore
                        .edge_properties
                        .insert((edge.key.clone(), name.clone()), wrapped_value.clone());
                }

                if let Some(property_container) = datastore.property_values.get_mut(&name) {
                    let property_container = property_container.entry(wrapped_value).or_insert_with(HashSet::new);
                    for edge in edges.into_iter() {
                        property_container.insert(IndexedPropertyMember::Edge(edge.key));
                    }
                }
            }
            _ => return Err(Error::Unsupported),
        }
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

impl crate::compat::DatastoreV3CompatExt for MemoryDatastore {}
