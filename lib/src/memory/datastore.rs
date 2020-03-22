use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::sync::{Arc, RwLock};

use crate::errors::Result;
use crate::{
    Datastore, Edge, EdgeDirection, EdgeProperties, EdgeProperty, EdgePropertyQuery, EdgeQuery, NamedProperty,
    Transaction, Type, Vertex, VertexProperties, VertexProperty, VertexPropertyQuery, VertexQuery,
};

use serde_json::Value as JsonValue;
use uuid::Uuid;

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug)]
struct InternalMemoryDatastore {
    edge_properties: BTreeMap<(Edge, String), JsonValue>,
    edges: BTreeSet<Edge>,
    vertex_properties: BTreeMap<(Uuid, String), JsonValue>,
    vertices: BTreeMap<Uuid, Type>,
}

type QueryIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

impl InternalMemoryDatastore {
    fn get_vertex_values_by_query<'a>(&'a self, q: VertexQuery) -> Result<QueryIter<'a, (Uuid, Type)>> {
        match q {
            VertexQuery::Range(range) => {
                let mut iter: QueryIter<(&Uuid, &Type)> = if let Some(start_id) = range.start_id {
                    Box::new(self.vertices.range(start_id..))
                } else {
                    Box::new(self.vertices.iter())
                };

                if let Some(t) = range.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &&t));
                }

                let iter: QueryIter<(Uuid, Type)> =
                    Box::new(iter.take(range.limit as usize).map(|(k, v)| (*k, v.clone())));

                Ok(iter)
            }
            VertexQuery::Specific(specific) => {
                let iter: QueryIter<(Uuid, Type)> = Box::new(
                    specific
                        .ids
                        .into_iter()
                        .filter_map(move |id| self.vertices.get(&id).map(|value| (id, value.clone()))),
                );

                Ok(iter)
            }
            VertexQuery::Pipe(pipe) => {
                let edge_values = self.get_edge_values_by_query(*pipe.inner)?;

                let iter: QueryIter<Uuid> = match pipe.direction {
                    EdgeDirection::Outbound => Box::new(edge_values.map(|edge| edge.outbound_id)),
                    EdgeDirection::Inbound => Box::new(edge_values.map(|edge| edge.inbound_id)),
                };

                let mut iter: QueryIter<(Uuid, &Type)> = Box::new(
                    iter.map(move |id| (id, self.vertices.get(&id)))
                        .filter_map(|(k, v)| Some((k, v?))),
                );

                if let Some(t) = pipe.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &&t));
                }

                let iter: QueryIter<(Uuid, Type)> =
                    Box::new(iter.take(pipe.limit as usize).map(|(k, v)| (k, v.clone())));

                Ok(iter)
            }
        }
    }

    fn get_edge_values_by_query<'a>(&'a self, q: EdgeQuery) -> Result<QueryIter<'a, Edge>> {
        match q {
            EdgeQuery::Specific(specific) => {
                let iter: QueryIter<Edge> = Box::new(
                    specific
                        .edges
                        .into_iter()
                        .filter_map(move |edge| self.edges.get(&edge))
                        .cloned()
                );

                Ok(iter)
            }
            EdgeQuery::Pipe(pipe) => {
                let iter = self.get_vertex_values_by_query(*pipe.inner)?;

                let mut iter: QueryIter<Edge> = match pipe.direction {
                    EdgeDirection::Outbound => {
                        let t = pipe.t.clone();

                        let iter = iter.flat_map(move |(id, _)| {
                            let lower_bound = match &t {
                                Some(t) => Edge::new(id, t.clone(), Uuid::default()),
                                None => {
                                    let empty_type = Type::default();
                                    Edge::new(id, empty_type, Uuid::default())
                                }
                            };

                            self.edges
                                .range(lower_bound..)
                                .take_while(move |edge| edge.outbound_id == id)
                                .cloned()
                        });

                        Box::new(iter)
                    }
                    EdgeDirection::Inbound => {
                        let mut candidate_ids = HashSet::new();
                        for (id, _) in iter {
                            candidate_ids.insert(id);
                        }

                        let iter = self
                            .edges
                            .iter()
                            .filter(move |edge| candidate_ids.contains(&edge.inbound_id))
                            .cloned();

                        Box::new(iter)
                    }
                };

                if let Some(t) = pipe.t {
                    iter = Box::new(iter.filter(move |edge| edge.t == t));
                }

                let iter = iter.take(pipe.limit as usize);
                let iter = Box::new(iter);
                Ok(iter)
            }
        }
    }

    fn delete_vertices(&mut self, vertices: Vec<Uuid>) {
        for vertex_id in vertices {
            self.vertices.remove(&vertex_id);

            let mut deletable_vertex_properties: Vec<(Uuid, String)> = Vec::new();

            for (property_key, _) in self.vertex_properties.range((vertex_id, "".to_string())..) {
                let &(ref property_vertex_id, _) = property_key;

                if &vertex_id != property_vertex_id {
                    break;
                }

                deletable_vertex_properties.push(property_key.clone());
            }

            for property_key in deletable_vertex_properties {
                self.vertex_properties.remove(&property_key);
            }

            let mut deletable_edges: Vec<Edge> = Vec::new();

            for edge_key in self.edges.iter() {
                if edge_key.outbound_id == vertex_id || edge_key.inbound_id == vertex_id {
                    deletable_edges.push(edge_key.clone());
                }
            }

            self.delete_edges(deletable_edges);
        }
    }

    fn delete_edges(&mut self, edges: Vec<Edge>) {
        for edge_key in edges {
            self.edges.remove(&edge_key);

            let mut deletable_edge_properties: Vec<(Edge, String)> = Vec::new();

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
#[derive(Debug, Clone)]
pub struct MemoryDatastore(Arc<RwLock<InternalMemoryDatastore>>);

impl MemoryDatastore {
    /// Creates a new in-memory datastore.
    pub fn default() -> MemoryDatastore {
        Self {
            0: Arc::new(RwLock::new(InternalMemoryDatastore {
                edge_properties: BTreeMap::new(),
                edges: BTreeSet::new(),
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
    fn create_vertex(&self, vertex: &Vertex) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();
        let mut inserted = false;

        datastore.vertices.entry(vertex.id).or_insert_with(|| {
            inserted = true;
            vertex.t.clone()
        });

        Ok(inserted)
    }

    fn get_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<Vertex>> {
        let datastore = self.datastore.read().unwrap();
        let iter = datastore.get_vertex_values_by_query(q.into())?;
        let iter = iter.map(|(uuid, t)| Vertex::with_id(uuid, t));
        Ok(iter.collect())
    }

    fn delete_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_vertices = datastore
            .get_vertex_values_by_query(q.into())?
            .map(|(k, _)| k)
            .collect();
        datastore.delete_vertices(deletable_vertices);
        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();
        Ok(datastore.vertices.len() as u64)
    }

    fn create_edge(&self, edge: &Edge) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();

        if !datastore.vertices.contains_key(&edge.outbound_id) || !datastore.vertices.contains_key(&edge.inbound_id) {
            return Ok(false);
        }

        datastore.edges.insert(edge.clone());
        Ok(true)
    }

    fn get_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<Edge>> {
        let datastore = self.datastore.read().unwrap();
        let iter = datastore.get_edge_values_by_query(q.into())?;
        Ok(iter.collect())
    }

    fn delete_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_edges: Vec<Edge> = datastore.get_edge_values_by_query(q.into())?.collect();
        datastore.delete_edges(deletable_edges);
        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&Type>, direction: EdgeDirection) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();

        if direction == EdgeDirection::Outbound {
            let lower_bound = match t {
                Some(t) => Edge::new(id, t.clone(), Uuid::default()),
                None => {
                    let empty_type = Type::default();
                    Edge::new(id, empty_type, Uuid::default())
                }
            };
            let range = datastore.edges.range(lower_bound..);

            let range = range.take_while(|&edge| {
                if let Some(t) = t {
                    edge.outbound_id == id && &edge.t == t
                } else {
                    edge.outbound_id == id
                }
            });

            Ok(range.count() as u64)
        } else {
            let range = datastore.edges.iter().filter(|&edge| {
                if let Some(t) = t {
                    edge.inbound_id == id && &edge.t == t
                } else {
                    edge.inbound_id == id
                }
            });

            Ok(range.count() as u64)
        }
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<VertexProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let vertex_values = datastore.get_vertex_values_by_query(q.inner)?;

        for (id, _) in vertex_values {
            let property_value = datastore.vertex_properties.get(&(id, q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(VertexProperty::new(id, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn get_all_vertex_properties<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<VertexProperties>> {
        let datastore = self.datastore.read().unwrap();
        let vertex_values = datastore.get_vertex_values_by_query(q.into())?;

        let mut result = Vec::new();
        for (id, t) in vertex_values {
            let from = &(id, "".to_string());
            let to = &(crate::util::next_uuid(id).unwrap(), "".to_string());

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

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let vertex_values: Vec<(Uuid, Type)> = datastore.get_vertex_values_by_query(q.inner)?.collect();

        for (id, _) in vertex_values.into_iter() {
            datastore.vertex_properties.insert((id, q.name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();

        let vertex_values: Vec<(Uuid, Type)> = datastore.get_vertex_values_by_query(q.inner)?.collect();

        for (id, _) in vertex_values.into_iter() {
            datastore.vertex_properties.remove(&(id, q.name.clone()));
        }

        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<EdgeProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(q.inner)?;

        for edge in edge_values {
            let property_value = datastore.edge_properties.get(&(edge.clone(), q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(EdgeProperty::new(edge, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn get_all_edge_properties<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<EdgeProperties>> {
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(q.into())?;

        let mut result = Vec::new();
        for edge in edge_values {
            let from = &(edge.clone(), "".to_string());

            let properties = datastore
                .edge_properties
                .range(from..)
                .take_while(|((other_edge, _), _)| *other_edge == edge);
            result.push(EdgeProperties::new(
                edge.clone(),
                properties
                    .map(|(n, p)| NamedProperty::new(n.1.clone(), p.clone()))
                    .collect(),
            ));
        }

        Ok(result)
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edge_values: Vec<Edge> = datastore.get_edge_values_by_query(q.inner)?.collect();

        for edge in edge_values.into_iter() {
            datastore.edge_properties.insert((edge, q.name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edge_values: Vec<Edge> = datastore.get_edge_values_by_query(q.inner)?.collect();

        for edge in edge_values {
            datastore.edge_properties.remove(&(edge, q.name.clone()));
        }

        Ok(())
    }
}
