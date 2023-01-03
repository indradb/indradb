use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::errors::{Error, Result};
use crate::util;
use crate::{
    Datastore, DynIter, Edge, EdgeDirection, Identifier, Json, Query, QueryOutputValue, Transaction,
    TransactionBuilder, Vertex,
};

use bincode::Error as BincodeError;
use rmp_serde::decode::Error as RmpDecodeError;
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
        Box::new($iter.filter(move |edge| $self.edges.contains(&edge)))
    };
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
enum IndexedPropertyMember {
    Vertex(Uuid),
    Edge(Edge),
}

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap a mutex around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug, Default, Serialize, Deserialize)]
struct InternalMemory {
    vertices: BTreeMap<Uuid, Identifier>,
    edges: BTreeSet<Edge>,
    reversed_edges: BTreeSet<Edge>,
    vertex_properties: BTreeMap<(Uuid, Identifier), Json>,
    edge_properties: BTreeMap<(Edge, Identifier), Json>,
    property_values: HashMap<Identifier, HashMap<Json, HashSet<IndexedPropertyMember>>>,
}

type QueryIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

impl InternalMemory {
    fn get_property_values(&self, name: &Identifier) -> Result<&HashMap<Json, HashSet<IndexedPropertyMember>>> {
        if let Some(container) = self.property_values.get(name) {
            Ok(container)
        } else {
            Err(Error::NotIndexed)
        }
    }

    fn get_all_vertices_with_property(
        &self,
        property_name: &Identifier,
        error_if_missing: bool,
    ) -> Result<HashSet<Uuid>> {
        let mut vertices = HashSet::<Uuid>::default();
        match self.get_property_values(property_name) {
            Ok(container) => {
                for sub_container in container.values() {
                    for member in sub_container {
                        if let IndexedPropertyMember::Vertex(id) = member {
                            vertices.insert(*id);
                        }
                    }
                }
            }
            Err(err) if error_if_missing => {
                return Err(err);
            }
            _ => {}
        }
        Ok(vertices)
    }

    fn get_all_edges_with_property(&self, property_name: &Identifier, error_if_missing: bool) -> Result<HashSet<Edge>> {
        let mut edges = HashSet::<Edge>::default();
        match self.get_property_values(property_name) {
            Ok(container) => {
                for sub_container in container.values() {
                    for member in sub_container {
                        if let IndexedPropertyMember::Edge(edge) = member {
                            edges.insert(edge.clone());
                        }
                    }
                }
            }
            Err(err) if error_if_missing => {
                return Err(err);
            }
            _ => {}
        }
        Ok(edges)
    }

    // fn query(&self, q: &Query, output: &mut Vec<QueryOutputValue>) -> Result<()> {
    //     let value = match q {
    //         Query::AllVertex(_) => {
    //             let iter = self.vertices.iter().map(|(id, t)| Vertex::with_id(*id, t.clone()));
    //             QueryOutputValue::Vertices(iter.collect())
    //         }
    //         Query::RangeVertex(ref q) => {
    //             let mut iter: QueryIter<(&Uuid, &Identifier)> = if let Some(start_id) = q.start_id {
    //                 Box::new(self.vertices.range(start_id..))
    //             } else {
    //                 Box::new(self.vertices.iter())
    //             };

    //             if let Some(ref t) = q.t {
    //                 iter = Box::new(iter.filter(move |(_, v)| v == &t));
    //             }

    //             iter = Box::new(iter.take(q.limit as usize));
    //             let iter = iter.map(|(id, t)| Vertex::with_id(*id, t.clone()));

    //             QueryOutputValue::Vertices(iter.collect())
    //         }
    //         Query::SpecificVertex(ref q) => {
    //             let iter = iter_vertex_values!(self, q.ids.iter());
    //             let iter = iter.map(|(id, t)| Vertex::with_id(*id, t.clone()));
    //             QueryOutputValue::Vertices(iter.collect())
    //         }
    //         Query::Pipe(ref q) => {
    //             self.query(&*q.inner, output)?;
    //             let piped_values = output.pop().unwrap();

    //             let values = match piped_values {
    //                 QueryOutputValue::Edges(ref piped_edges) => {
    //                     let iter: QueryIter<Uuid> = match q.direction {
    //                         EdgeDirection::Outbound => Box::new(piped_edges.iter().map(|e| e.outbound_id)),
    //                         EdgeDirection::Inbound => Box::new(piped_edges.iter().map(|e| e.inbound_id)),
    //                     };

    //                     let mut iter: QueryIter<(Uuid, &Identifier)> = Box::new(
    //                         iter.map(move |id| (id, self.vertices.get(&id)))
    //                             .filter_map(|(k, v)| Some((k, v?))),
    //                     );

    //                     if let Some(ref t) = q.t {
    //                         iter = Box::new(iter.filter(move |(_, v)| v == &t));
    //                     }

    //                     iter = Box::new(iter.take(q.limit as usize));

    //                     let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone())).collect();
    //                     QueryOutputValue::Vertices(iter)
    //                 }
    //                 QueryOutputValue::Vertices(ref piped_vertices) => {
    //                     let mut iter: QueryIter<&Edge> = Box::new(piped_vertices.iter().flat_map(move |v| {
    //                         let lower_bound = match &q.t {
    //                             Some(t) => Edge::new(v.id, t.clone(), Uuid::default()),
    //                             None => Edge::new(v.id, Identifier::default(), Uuid::default()),
    //                         };

    //                         let iter = if q.direction == EdgeDirection::Outbound {
    //                             self.edges.range(lower_bound..)
    //                         } else {
    //                             self.reversed_edges.range(lower_bound..)
    //                         };

    //                         iter.take_while(move |edge| edge.outbound_id == v.id)
    //                     }));

    //                     if let Some(ref t) = q.t {
    //                         iter = Box::new(iter.filter(move |edge| &edge.t == t));
    //                     }

    //                     let iter = iter.take(q.limit as usize);

    //                     let iter: QueryIter<Edge> = if q.direction == EdgeDirection::Outbound {
    //                         Box::new(iter.cloned())
    //                     } else {
    //                         Box::new(iter.map(move |edge| edge.reversed()))
    //                     };

    //                     QueryOutputValue::Edges(iter.collect())
    //                 }
    //                 _ => {
    //                     return Err(Error::Unsupported);
    //                 }
    //             };

    //             if let Query::Include(_) = *q.inner {
    //                 // keep the value exported
    //                 output.push(piped_values);
    //             }

    //             values
    //         }
    //         Query::PipeProperty(ref q) => {
    //             self.query(&*q.inner, output)?;
    //             let piped_values = output.pop().unwrap();

    //             let values = match piped_values {
    //                 QueryOutputValue::Edges(ref piped_edges) => {
    //                     let mut edge_properties = Vec::new();
    //                     for edge in piped_edges.into_iter() {
    //                         if let Some(name) = &q.name {
    //                             if let Some(value) = self.edge_properties.get(&(edge.clone(), name.clone())) {
    //                                 edge_properties.push((edge.clone(), name.clone(), value.0.clone()));
    //                             }
    //                         } else {
    //                             let from = &(edge.clone(), Identifier::default());
    //                             for ((prop_edge, prop_name), prop_value) in self.edge_properties.range(from..) {
    //                                 if &prop_edge != &edge {
    //                                     break;
    //                                 }
    //                                 edge_properties.push((edge.clone(), prop_name.clone(), prop_value.0.clone()));
    //                             }
    //                         }
    //                     }

    //                     QueryOutputValue::EdgeProperties(edge_properties)
    //                 }
    //                 QueryOutputValue::Vertices(ref piped_vertices) => {
    //                     let mut vertex_properties = Vec::with_capacity(piped_vertices.len());
    //                     for vertex in piped_vertices.into_iter() {
    //                         if let Some(name) = &q.name {
    //                             if let Some(value) = self.vertex_properties.get(&(vertex.id, name.clone())) {
    //                                 vertex_properties.push((vertex.clone(), name.clone(), value.0.clone()));
    //                             }
    //                         } else {
    //                             let from = &(vertex.id, Identifier::default());
    //                             let to = &(util::next_uuid(vertex.id).unwrap(), Identifier::default());
    //                             for ((_prop_vertex_id, prop_name), prop_value) in self.vertex_properties.range(from..to)
    //                             {
    //                                 vertex_properties.push((vertex.clone(), prop_name.clone(), prop_value.0.clone()));
    //                             }
    //                         }
    //                     }

    //                     QueryOutputValue::VertexProperties(vertex_properties)
    //                 }
    //                 _ => {
    //                     return Err(Error::Unsupported);
    //                 }
    //             };

    //             if let Query::Include(_) = *q.inner {
    //                 // keep the value exported
    //                 output.push(piped_values);
    //             }

    //             values
    //         }
    //         Query::VertexWithPropertyPresence(ref q) => {
    //             let vertices = self.get_all_vertices_with_property(&q.name, true)?;
    //             let iter = iter_vertex_values!(self, vertices.into_iter());
    //             let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone()));
    //             QueryOutputValue::Vertices(iter.collect())
    //         }
    //         Query::VertexWithPropertyValue(ref q) => {
    //             let container = self.get_property_values(&q.name)?;
    //             let wrapped_value = Json::new(q.value.clone());
    //             if let Some(sub_container) = container.get(&wrapped_value) {
    //                 let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
    //                     IndexedPropertyMember::Vertex(id) => self.vertices.get(id).map(|value| (*id, value.clone())),
    //                     _ => None,
    //                 }));
    //                 let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone()));
    //                 QueryOutputValue::Vertices(iter.collect())
    //             } else {
    //                 let iter = iter_vertex_values!(self, Vec::default().into_iter());
    //                 let iter = iter.map(|(id, t)| Vertex::with_id(id, t.clone()));
    //                 QueryOutputValue::Vertices(iter.collect())
    //             }
    //         }
    //         Query::EdgeWithPropertyPresence(ref q) => {
    //             let edges = self.get_all_edges_with_property(&q.name, true)?;
    //             let iter = iter_edge_values!(self, edges.into_iter());
    //             QueryOutputValue::Edges(iter.collect())
    //         }
    //         Query::EdgeWithPropertyValue(ref q) => {
    //             let container = self.get_property_values(&q.name)?;
    //             let wrapped_value = Json::new(q.value.clone());
    //             if let Some(sub_container) = container.get(&wrapped_value) {
    //                 let iter = Box::new(sub_container.iter().filter_map(move |member| match member {
    //                     IndexedPropertyMember::Edge(edge) if self.edges.contains(edge) => Some(edge),
    //                     _ => None,
    //                 }));
    //                 QueryOutputValue::Edges(iter.cloned().collect())
    //             } else {
    //                 let iter = iter_edge_values!(self, Vec::default().into_iter());
    //                 QueryOutputValue::Edges(iter.collect())
    //             }
    //         }
    //         Query::PipeWithPropertyPresence(ref q) => {
    //             self.query(&*q.inner, output)?;
    //             let piped_values = output.pop().unwrap();

    //             let values = match piped_values {
    //                 QueryOutputValue::Edges(ref piped_edges) => {
    //                     let edges_with_property = self.get_all_edges_with_property(&q.name, false)?;
    //                     let iter = piped_edges.iter().filter(move |e| {
    //                         let contains = edges_with_property.contains(&e);
    //                         (q.exists && contains) || (!q.exists && !contains)
    //                     });
    //                     QueryOutputValue::Edges(iter.cloned().collect())
    //                 }
    //                 QueryOutputValue::Vertices(ref piped_vertices) => {
    //                     let vertices_with_property = self.get_all_vertices_with_property(&q.name, false)?;
    //                     let iter = piped_vertices.iter().filter(move |v| {
    //                         let contains = vertices_with_property.contains(&v.id);
    //                         (q.exists && contains) || (!q.exists && !contains)
    //                     });
    //                     QueryOutputValue::Vertices(iter.cloned().collect())
    //                 }
    //                 _ => {
    //                     return Err(Error::Unsupported);
    //                 }
    //             };

    //             if let Query::Include(_) = *q.inner {
    //                 // keep the value exported
    //                 output.push(piped_values);
    //             }

    //             values
    //         }
    //         Query::PipeWithPropertyValue(ref q) => {
    //             self.query(&*q.inner, output)?;
    //             let piped_values = output.pop().unwrap();

    //             let empty_hashset = HashSet::default();
    //             let indexed_members: &HashSet<IndexedPropertyMember> =
    //                 if let Some(container) = self.property_values.get(&q.name) {
    //                     let wrapped_value = Json::new(q.value.clone());
    //                     if let Some(ref members) = container.get(&wrapped_value) {
    //                         members
    //                     } else {
    //                         &empty_hashset
    //                     }
    //                 } else {
    //                     &empty_hashset
    //                 };

    //             let values = match piped_values {
    //                 QueryOutputValue::Edges(ref piped_edges) => {
    //                     let iter = piped_edges.iter().filter(move |e| {
    //                         let contains = indexed_members.contains(&IndexedPropertyMember::Edge((**e).clone()));
    //                         (q.equal && contains) || (!q.equal && !contains)
    //                     });
    //                     QueryOutputValue::Edges(iter.cloned().collect())
    //                 }
    //                 QueryOutputValue::Vertices(ref piped_vertices) => {
    //                     let iter = piped_vertices.iter().filter(move |v| {
    //                         let contains = indexed_members.contains(&IndexedPropertyMember::Vertex(v.id));
    //                         (q.equal && contains) || (!q.equal && !contains)
    //                     });
    //                     QueryOutputValue::Vertices(iter.cloned().collect())
    //                 }
    //                 _ => {
    //                     return Err(Error::Unsupported);
    //                 }
    //             };

    //             if let Query::Include(_) = *q.inner {
    //                 // keep the value exported
    //                 output.push(piped_values);
    //             }

    //             values
    //         }
    //         Query::AllEdge(_) => QueryOutputValue::Edges(self.edges.iter().cloned().collect()),
    //         Query::SpecificEdge(ref q) => {
    //             let iter = iter_edge_values!(self, q.edges.clone().into_iter());
    //             QueryOutputValue::Edges(iter.collect())
    //         }
    //         Query::Include(ref q) => {
    //             self.query(&*q.inner, output)?;
    //             output.pop().unwrap()
    //         }
    //         Query::Count(ref q) => {
    //             let count = match &*q.inner {
    //                 // These paths are optimized
    //                 Query::AllVertex(_) => self.vertices.len(),
    //                 Query::AllEdge(_) => self.edges.len(),
    //                 q => {
    //                     self.query(q, output)?;
    //                     let piped_values = output.pop().unwrap();
    //                     match piped_values {
    //                         QueryOutputValue::Vertices(v) => v.len(),
    //                         QueryOutputValue::Edges(v) => v.len(),
    //                         QueryOutputValue::VertexProperties(v) => v.len(),
    //                         QueryOutputValue::EdgeProperties(v) => v.len(),
    //                         _ => return Err(Error::Unsupported),
    //                     }
    //                 }
    //             };
    //             QueryOutputValue::Count(count as u64)
    //         }
    //     };

    //     output.push(value);
    //     Ok(())
    // }

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

            let mut deletable_edges: Vec<Edge> = Vec::new();
            for edge in self.edges.iter() {
                if edge.outbound_id == vertex_id || edge.inbound_id == vertex_id {
                    deletable_edges.push(edge.clone());
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

    fn delete_edges(&mut self, edges: Vec<Edge>) {
        for edge in edges {
            self.edges.remove(&edge);
            self.reversed_edges.remove(&edge.reversed());

            let mut deletable_edge_properties: Vec<(Edge, Identifier)> = Vec::new();
            for (property_key, _) in self.edge_properties.range((edge.clone(), Identifier::default())..) {
                let &(ref property_edge, _) = property_key;

                if &edge != property_edge {
                    break;
                }

                deletable_edge_properties.push(property_key.clone());
            }
            self.delete_edge_properties(deletable_edge_properties)
        }
    }

    fn delete_edge_properties(&mut self, keys: Vec<(Edge, Identifier)>) {
        for property_key in keys {
            if let Some(property_value) = self.edge_properties.remove(&property_key) {
                let (property_edge, property_name) = property_key;
                if let Some(property_container) = self.property_values.get_mut(&property_name) {
                    debug_assert!(property_container
                        .get_mut(&property_value)
                        .unwrap()
                        .remove(&IndexedPropertyMember::Edge(property_edge)));
                }
            }
        }
    }
}

pub struct MemoryTransaction<'a> {
    internal: MutexGuard<'a, InternalMemory>,
    path: Option<PathBuf>,
}

impl<'a> Transaction<'a> for MemoryTransaction<'a> {
    fn vertex_count(&self) -> u64 {
        self.internal.vertices.len() as u64
    }

    fn all_vertices(&self) -> Result<DynIter<'a, Vertex>> {
        let iter = self
            .internal
            .vertices
            .iter()
            .map(|(id, t)| Vertex::with_id(*id, t.clone()));
        Ok(Box::new(iter))
    }

    fn range_vertices(&self, offset: Uuid) -> Result<DynIter<'a, Vertex>> {
        todo!();
    }

    fn specific_vertices(&self, ids: &Vec<Uuid>) -> Result<DynIter<'a, Vertex>> {
        todo!();
    }

    fn vertex_ids_with_property(&self, name: &Identifier) -> Result<Option<DynIter<'a, Uuid>>> {
        todo!();
    }

    fn vertex_ids_with_property_value(
        &self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Uuid>>> {
        todo!();
    }

    fn edge_count(&self) -> u64 {
        self.internal.edges.len() as u64
    }

    fn all_edges(&self) -> Result<DynIter<'a, Edge>> {
        todo!();
    }

    fn range_edges(&self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        todo!();
    }

    fn range_reversed_edges(&self, offset: Edge) -> Result<DynIter<'a, Edge>> {
        todo!();
    }

    fn specific_edges(&self, edges: &Vec<Edge>) -> Result<DynIter<'a, Edge>> {
        todo!();
    }

    fn edges_with_property(&self, name: &Identifier) -> Result<Option<DynIter<'a, Edge>>> {
        todo!();
    }

    fn edges_with_property_value(
        &self,
        name: &Identifier,
        value: &serde_json::Value,
    ) -> Result<Option<DynIter<'a, Edge>>> {
        todo!();
    }

    fn vertex_property(&self, vertex: &Vertex, name: &Identifier) -> Result<Option<serde_json::Value>> {
        todo!();
    }

    fn all_vertex_properties_for_vertex(
        &self,
        vertex: &Vertex,
    ) -> Result<DynIter<'a, (Identifier, serde_json::Value)>> {
        todo!();
    }

    fn edge_property(&self, edge: &Edge, name: &Identifier) -> Result<Option<serde_json::Value>> {
        todo!();
    }

    fn all_edge_properties_for_edge(&self, edge: &Edge) -> Result<DynIter<'a, (Identifier, serde_json::Value)>> {
        todo!();
    }

    fn delete_vertices(&self, vertices: Vec<Vertex>) -> Result<()> {
        todo!();
    }

    fn delete_edges(&self, edges: Vec<Edge>) -> Result<()> {
        todo!();
    }

    fn delete_vertex_properties(&self, props: Vec<(Vertex, Identifier, serde_json::Value)>) -> Result<()> {
        todo!();
    }

    fn delete_edge_properties(&self, props: Vec<(Edge, Identifier, serde_json::Value)>) -> Result<()> {
        todo!();
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

    fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        let mut inserted = false;

        self.internal.vertices.entry(vertex.id).or_insert_with(|| {
            inserted = true;
            vertex.t.clone()
        });

        Ok(inserted)
    }

    fn create_edge(&self, edge: &Edge) -> Result<bool> {
        if !self.internal.vertices.contains_key(&edge.outbound_id)
            || !self.internal.vertices.contains_key(&edge.inbound_id)
        {
            return Ok(false);
        }

        self.internal.edges.insert(edge.clone());
        self.internal.reversed_edges.insert(edge.reversed());
        Ok(true)
    }

    fn index_property(&self, name: Identifier) -> Result<()> {
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

    fn set_vertex_properties(&self, vertex_ids: Vec<Uuid>, name: Identifier, value: serde_json::Value) -> Result<()> {
        let mut deletable_vertex_properties = Vec::new();
        for vertex_id in &vertex_ids {
            deletable_vertex_properties.push((*vertex_id, name.clone()));
        }
        self.internal.delete_vertex_properties(deletable_vertex_properties);

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

    fn set_edge_properties(&self, edges: Vec<Edge>, name: Identifier, value: serde_json::Value) -> Result<()> {
        let mut deletable_edge_properties = Vec::new();
        for edge in &edges {
            deletable_edge_properties.push((edge.clone(), name.clone()));
        }
        self.internal.delete_edge_properties(deletable_edge_properties);

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
pub struct MemoryTransactionBuilder {
    internal: Arc<Mutex<InternalMemory>>,
    path: Option<PathBuf>,
}

impl TransactionBuilder for MemoryTransactionBuilder {
    type Transaction<'a> = MemoryTransaction<'a>;
    fn transaction<'a>(&'a self) -> Self::Transaction<'a> {
        MemoryTransaction {
            internal: self.internal.lock().unwrap(),
            path: self.path.clone(),
        }
    }
}

pub fn default() -> Datastore<MemoryTransactionBuilder> {
    Datastore::new(MemoryTransactionBuilder {
        internal: Arc::new(Mutex::new(InternalMemory::default())),
        path: None,
    })
}

/// Reads a persisted image from disk. Calls to sync will overwrite the
/// file at the specified path. Uses msgpack, which unlike bincode
/// supports properties.
///
/// # Arguments
/// * `path`: The path to the persisted image.
pub fn read_msgpack<P: Into<PathBuf>>(path: P) -> StdResult<Datastore<MemoryTransactionBuilder>, RmpDecodeError> {
    let path = path.into();
    let f = File::open(&path).map_err(RmpDecodeError::InvalidDataRead)?;
    let buf = BufReader::new(f);
    let internal: InternalMemory = rmp_serde::from_read(buf)?;
    Ok(Datastore::new(MemoryTransactionBuilder {
        internal: Arc::new(Mutex::new(internal)),
        path: Some(path),
    }))
}

/// Creates a new datastore. Calls to sync will overwrite the file at the
/// specified path, but as opposed to `read`, this will not read the file
/// first. Uses msgpack, which unlike bincode supports properties.
///
/// # Arguments
/// * `path`: The path to the persisted image.
pub fn create_msgpack<P: Into<PathBuf>>(path: P) -> StdResult<Datastore<MemoryTransactionBuilder>, BincodeError> {
    Ok(Datastore::new(MemoryTransactionBuilder {
        internal: Arc::new(Mutex::new(InternalMemory::default())),
        path: Some(path.into()),
    }))
}
