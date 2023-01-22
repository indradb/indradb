use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::errors::{Error, Result};
use crate::{Database, Datastore, DynIter, Edge, Identifier, Json, Transaction, ValidationError, Vertex};

use rmp_serde::decode::Error as RmpDecodeError;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
enum IndexedPropertyMember {
    Vertex(u64),
    Edge(Edge),
}

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap a mutex around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug, Default, Serialize, Deserialize)]
struct InternalMemory {
    vertices: BTreeMap<u64, Identifier>,
    edges: BTreeSet<Edge>,
    reversed_edges: BTreeSet<Edge>,
    vertex_properties: BTreeMap<(u64, Identifier), Json>,
    edge_properties: BTreeMap<(Edge, Identifier), Json>,
    property_values: HashMap<Identifier, HashMap<Json, HashSet<IndexedPropertyMember>>>,
}

pub struct MemoryTransaction<'a> {
    internal: MutexGuard<'a, InternalMemory>,
    path: Option<PathBuf>,
}

impl<'a> MemoryTransaction<'a> {
    fn get_property_values(&self, name: &Identifier) -> Result<&HashMap<Json, HashSet<IndexedPropertyMember>>> {
        if let Some(container) = self.internal.property_values.get(name) {
            Ok(container)
        } else {
            Err(Error::NotIndexed)
        }
    }
}

impl<'a> Transaction<'a> for MemoryTransaction<'a> {
    fn vertex_count(&self) -> u64 {
        self.internal.vertices.len() as u64
    }

    fn all_vertices(&'a self) -> Result<DynIter<'a, Vertex>> {
        let iter = self
            .internal
            .vertices
            .iter()
            .map(|(id, t)| Ok(Vertex::new(*id, t.clone())));
        Ok(Box::new(iter))
    }

    fn range_vertices(&'a self, offset: u64) -> Result<DynIter<'a, Vertex>> {
        let iter = self
            .internal
            .vertices
            .range(offset..)
            .map(|(id, t)| Ok(Vertex::new(*id, t.clone())));
        Ok(Box::new(iter))
    }

    fn specific_vertices(&'a self, ids: Vec<u64>) -> Result<DynIter<'a, Vertex>> {
        let iter = ids.into_iter().filter_map(move |id| {
            self.internal
                .vertices
                .get(&id)
                .map(|value| Ok(Vertex::new(id, value.clone())))
        });
        Ok(Box::new(iter))
    }

    fn vertex_ids_with_property(&'a self, name: &Identifier) -> Result<Option<DynIter<'a, u64>>> {
        if let Some(container) = self.internal.property_values.get(name) {
            let mut vertex_ids = HashSet::<u64>::default();
            for sub_container in container.values() {
                for member in sub_container {
                    if let IndexedPropertyMember::Vertex(id) = member {
                        vertex_ids.insert(*id);
                    }
                }
            }
            Ok(Some(Box::new(vertex_ids.into_iter().map(Ok))))
        } else {
            Ok(None)
        }
    }

    fn vertex_ids_with_property_value(
        &'a self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, u64>>> {
        let container = self.get_property_values(name)?;
        let wrapped_value = Json::new(value.clone());
        if let Some(sub_container) = container.get(&wrapped_value) {
            let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
                IndexedPropertyMember::Vertex(id) => Some(Ok(*id)),
                _ => None,
            }));
            Ok(Some(Box::new(iter)))
        } else {
            Ok(None)
        }
    }

    fn edge_count(&self) -> u64 {
        self.internal.edges.len() as u64
    }

    fn all_edges(&'a self) -> Result<DynIter<'a, Edge>> {
        let iter = self.internal.edges.iter().map(|e| Ok(e.clone()));
        Ok(Box::new(iter))
    }

    fn range_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        let iter = self.internal.edges.range(offset..).map(|e| Ok(e.clone()));
        Ok(Box::new(iter))
    }

    fn range_reversed_edges(&'a self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        let iter = self.internal.reversed_edges.range(offset..).map(|e| Ok(e.clone()));
        Ok(Box::new(iter))
    }

    fn specific_edges(&'a self, edges: Vec<Edge>) -> Result<DynIter<'a, Edge>> {
        let iter = edges
            .into_iter()
            .filter(move |edge| self.internal.edges.contains(edge))
            .map(Ok);
        Ok(Box::new(iter))
    }

    fn edges_with_property(&'a self, name: &Identifier) -> Result<Option<DynIter<'a, Edge>>> {
        if let Some(container) = self.internal.property_values.get(name) {
            let mut edges = HashSet::<Edge>::default();
            for sub_container in container.values() {
                for member in sub_container {
                    if let IndexedPropertyMember::Edge(edge) = member {
                        edges.insert(edge.clone());
                    }
                }
            }
            Ok(Some(Box::new(edges.into_iter().map(Ok))))
        } else {
            Ok(None)
        }
    }

    fn edges_with_property_value(
        &'a self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Edge>>> {
        let container = self.get_property_values(name)?;
        let wrapped_value = Json::new(value.clone());
        if let Some(sub_container) = container.get(&wrapped_value) {
            let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
                IndexedPropertyMember::Edge(edge) if self.internal.edges.contains(edge) => Some(edge),
                _ => None,
            }));
            Ok(Some(Box::new(iter.map(|e| Ok(e.clone())))))
        } else {
            Ok(None)
        }
    }

    fn vertex_property(&self, vertex: &Vertex, name: &Identifier) -> Result<Option<serde_json::Value>> {
        if let Some(value) = self.internal.vertex_properties.get(&(vertex.id, name.clone())) {
            Ok(Some(value.0.clone()))
        } else {
            Ok(None)
        }
    }

    fn all_vertex_properties_for_vertex(
        &'a self,
        vertex: &Vertex,
    ) -> Result<DynIter<'a, (Identifier, serde_json::Value)>> {
        let mut vertex_properties = Vec::new();
        let from = &(vertex.id, Identifier::default());
        let to = &(
            vertex.id.checked_add(1).ok_or(ValidationError::CannotIncrementId)?,
            Identifier::default(),
        );
        for ((_prop_vertex_id, prop_name), prop_value) in self.internal.vertex_properties.range(from..to) {
            vertex_properties.push((prop_name.clone(), prop_value.0.clone()));
        }
        Ok(Box::new(vertex_properties.into_iter().map(Ok)))
    }

    fn edge_property(&self, edge: &Edge, name: &Identifier) -> Result<Option<serde_json::Value>> {
        if let Some(value) = self.internal.edge_properties.get(&(edge.clone(), name.clone())) {
            Ok(Some(value.0.clone()))
        } else {
            Ok(None)
        }
    }

    fn all_edge_properties_for_edge(&'a self, edge: &Edge) -> Result<DynIter<'a, (Identifier, serde_json::Value)>> {
        let mut edge_properties = Vec::new();
        let from = &(edge.clone(), Identifier::default());
        for ((prop_edge, prop_name), prop_value) in self.internal.edge_properties.range(from..) {
            if prop_edge != edge {
                break;
            }
            edge_properties.push((prop_name.clone(), prop_value.0.clone()));
        }
        Ok(Box::new(edge_properties.into_iter().map(Ok)))
    }

    fn delete_vertices(&mut self, vertices: Vec<Vertex>) -> Result<()> {
        for vertex in vertices {
            self.internal.vertices.remove(&vertex.id);

            let mut deletable_vertex_properties: Vec<(u64, Identifier)> = Vec::new();
            for (property_key, _) in self
                .internal
                .vertex_properties
                .range((vertex.id, Identifier::default())..)
            {
                let &(ref property_vertex_id, _) = property_key;

                if &vertex.id != property_vertex_id {
                    break;
                }

                deletable_vertex_properties.push(property_key.clone());
            }
            self.delete_vertex_properties(deletable_vertex_properties)?;

            let mut deletable_edges: Vec<Edge> = Vec::new();
            for edge in self.internal.edges.iter() {
                if edge.outbound_id == vertex.id || edge.inbound_id == vertex.id {
                    deletable_edges.push(edge.clone());
                }
            }
            self.delete_edges(deletable_edges)?;
        }
        Ok(())
    }

    fn delete_edges(&mut self, edges: Vec<Edge>) -> Result<()> {
        for edge in edges {
            self.internal.edges.remove(&edge);
            self.internal.reversed_edges.remove(&edge.reversed());

            let mut deletable_edge_properties: Vec<(Edge, Identifier)> = Vec::new();
            for (property_key, _) in self
                .internal
                .edge_properties
                .range((edge.clone(), Identifier::default())..)
            {
                let &(ref property_edge, _) = property_key;

                if &edge != property_edge {
                    break;
                }

                deletable_edge_properties.push(property_key.clone());
            }
            self.delete_edge_properties(deletable_edge_properties)?;
        }
        Ok(())
    }

    fn delete_vertex_properties(&mut self, props: Vec<(u64, Identifier)>) -> Result<()> {
        for prop in props {
            if let Some(property_value) = self.internal.vertex_properties.remove(&prop) {
                let (property_vertex_id, property_name) = prop;
                if let Some(property_container) = self.internal.property_values.get_mut(&property_name) {
                    debug_assert!(property_container
                        .get_mut(&property_value)
                        .unwrap()
                        .remove(&IndexedPropertyMember::Vertex(property_vertex_id)));
                }
            }
        }
        Ok(())
    }

    fn delete_edge_properties(&mut self, props: Vec<(Edge, Identifier)>) -> Result<()> {
        for prop in props {
            if let Some(property_value) = self.internal.edge_properties.remove(&prop) {
                let (property_edge, property_name) = prop;
                if let Some(property_container) = self.internal.property_values.get_mut(&property_name) {
                    debug_assert!(property_container
                        .get_mut(&property_value)
                        .unwrap()
                        .remove(&IndexedPropertyMember::Edge(property_edge)));
                }
            }
        }
        Ok(())
    }

    fn sync(&self) -> Result<()> {
        if let Some(ref persist_path) = self.path {
            let temp_path = NamedTempFile::new().map_err(|err| Error::Datastore(Box::new(err)))?;
            {
                let mut buf = BufWriter::new(temp_path.as_file());
                rmp_serde::encode::write(&mut buf, &*self.internal)?;
            }
            temp_path
                .persist(persist_path)
                .map_err(|err| Error::Datastore(Box::new(err)))?;
        }
        Ok(())
    }

    fn max_id(&self) -> Result<u64> {
        if let Some((k, _)) = self.internal.vertices.last_key_value() {
            Ok(*k)
        } else {
            Ok(0)
        }
    }

    fn create_vertex(&mut self, vertex: &Vertex) -> Result<bool> {
        let mut inserted = false;

        self.internal.vertices.entry(vertex.id).or_insert_with(|| {
            inserted = true;
            vertex.t.clone()
        });

        Ok(inserted)
    }

    fn create_edge(&mut self, edge: &Edge) -> Result<bool> {
        if !self.internal.vertices.contains_key(&edge.outbound_id)
            || !self.internal.vertices.contains_key(&edge.inbound_id)
        {
            return Ok(false);
        }

        self.internal.edges.insert(edge.clone());
        self.internal.reversed_edges.insert(edge.reversed());
        Ok(true)
    }

    fn index_property(&mut self, name: Identifier) -> Result<()> {
        let mut property_container: HashMap<Json, HashSet<IndexedPropertyMember>> = HashMap::new();
        for id in self.internal.vertices.keys() {
            if let Some(value) = self.internal.vertex_properties.get(&(*id, name.clone())) {
                property_container
                    .entry(value.clone())
                    .or_insert_with(HashSet::new)
                    .insert(IndexedPropertyMember::Vertex(*id));
            }
        }
        for edge in self.internal.edges.iter() {
            if let Some(value) = self.internal.edge_properties.get(&(edge.clone(), name.clone())) {
                property_container
                    .entry(value.clone())
                    .or_insert_with(HashSet::new)
                    .insert(IndexedPropertyMember::Edge(edge.clone()));
            }
        }

        let existing_property_container = self.internal.property_values.entry(name).or_insert_with(HashMap::new);
        for (value, members) in property_container.into_iter() {
            let existing_members = existing_property_container.entry(value).or_insert_with(HashSet::new);
            for member in members {
                existing_members.insert(member);
            }
        }

        Ok(())
    }

    fn set_vertex_properties(
        &mut self,
        vertex_ids: Vec<u64>,
        name: Identifier,
        value: serde_json::Value,
    ) -> Result<()> {
        let mut deletable_vertex_properties = Vec::new();
        for vertex_id in &vertex_ids {
            deletable_vertex_properties.push((*vertex_id, name.clone()));
        }
        self.delete_vertex_properties(deletable_vertex_properties)?;

        let wrapped_value = Json::new(value);
        for vertex_id in &vertex_ids {
            self.internal
                .vertex_properties
                .insert((*vertex_id, name.clone()), wrapped_value.clone());
        }

        if let Some(property_container) = self.internal.property_values.get_mut(&name) {
            let property_container = property_container.entry(wrapped_value).or_insert_with(HashSet::new);
            for vertex_id in vertex_ids.into_iter() {
                property_container.insert(IndexedPropertyMember::Vertex(vertex_id));
            }
        }

        Ok(())
    }

    fn set_edge_properties(&mut self, edges: Vec<Edge>, name: Identifier, value: serde_json::Value) -> Result<()> {
        let mut deletable_edge_properties = Vec::new();
        for edge in &edges {
            deletable_edge_properties.push((edge.clone(), name.clone()));
        }
        self.delete_edge_properties(deletable_edge_properties)?;

        let wrapped_value = Json::new(value);
        for edge in &edges {
            self.internal
                .edge_properties
                .insert((edge.clone(), name.clone()), wrapped_value.clone());
        }

        if let Some(property_container) = self.internal.property_values.get_mut(&name) {
            let property_container = property_container.entry(wrapped_value).or_insert_with(HashSet::new);
            for edge in edges.into_iter() {
                property_container.insert(IndexedPropertyMember::Edge(edge));
            }
        }

        Ok(())
    }
}

/// An in-memory datastore.
#[derive(Debug, Clone)]
pub struct MemoryDatastore {
    internal: Arc<Mutex<InternalMemory>>,
    path: Option<PathBuf>,
}

impl MemoryDatastore {
    /// Creates a new in-memory database with no persistence.
    pub fn new_db() -> Database<MemoryDatastore> {
        Database::new(MemoryDatastore {
            internal: Arc::new(Mutex::new(InternalMemory::default())),
            path: None,
        })
    }

    /// Reads a persisted image from disk. Calls to sync will overwrite the
    /// file at the specified path.
    ///
    /// # Arguments
    /// * `path`: The path to the persisted image.
    pub fn read_msgpack_db<P: Into<PathBuf>>(path: P) -> StdResult<Database<MemoryDatastore>, RmpDecodeError> {
        let path = path.into();
        let f = File::open(&path).map_err(RmpDecodeError::InvalidDataRead)?;
        let buf = BufReader::new(f);
        let internal: InternalMemory = rmp_serde::from_read(buf)?;
        Ok(Database::new(MemoryDatastore {
            internal: Arc::new(Mutex::new(internal)),
            path: Some(path),
        }))
    }

    /// Creates a new datastore. Calls to sync will overwrite the file at the
    /// specified path, but as opposed to `read`, this will not read the file
    /// first.
    ///
    /// # Arguments
    /// * `path`: The path to the persisted image.
    pub fn create_msgpack_db<P: Into<PathBuf>>(path: P) -> Database<MemoryDatastore> {
        Database::new(MemoryDatastore {
            internal: Arc::new(Mutex::new(InternalMemory::default())),
            path: Some(path.into()),
        })
    }
}

impl Datastore for MemoryDatastore {
    type Transaction<'a> = MemoryTransaction<'a>;
    fn transaction(&'_ self) -> Self::Transaction<'_> {
        MemoryTransaction {
            internal: self.internal.lock().unwrap(),
            path: self.path.clone(),
        }
    }
}
