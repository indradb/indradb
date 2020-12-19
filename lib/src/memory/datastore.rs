use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use crate::util::next_uuid;
use crate::errors::Result;
use crate::{
    Datastore, Edge, EdgeDirection, EdgeProperties, EdgeProperty, EdgePropertyQuery, EdgeQuery, NamedProperty,
    Transaction, Type, Vertex, VertexProperties, VertexProperty, VertexPropertyQuery, VertexQuery, SpecificVertexQuery,
    VertexQueryExt
};

use serde_json::Value as JsonValue;
use uuid::Uuid;

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug, Default)]
struct InternalMemoryDatastore {
    vertices: BTreeMap<Uuid, Type>,
    edges: BTreeSet<Edge>,
    reversed_edges: BTreeSet<Edge>,
    vertex_properties: BTreeMap<(Uuid, String), JsonValue>,
    edge_properties: BTreeMap<(Edge, String), JsonValue>,
}

type QueryIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

impl InternalMemoryDatastore {
    fn get_vertices_by_query<'a>(&'a self, q: VertexQuery) -> Result<QueryIter<'a, Vertex>> {
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

                let iter: QueryIter<Vertex> = Box::new(
                    iter.take(range.limit as usize).map(|(id, t)| Vertex::with_id(*id, t.clone()))
                );

                Ok(iter)
            }
            VertexQuery::Specific(specific) => {
                let iter: QueryIter<Vertex> = Box::new(
                    specific.ids.into_iter().filter_map(move |id| {
                        self.vertices.get(&id).map(|value| Vertex::with_id(id, value.clone()))
                    }),
                );

                Ok(iter)
            }
            VertexQuery::Pipe(pipe) => {
                let edges = self.get_edges_by_query(*pipe.inner)?;

                let iter: QueryIter<Uuid> = match pipe.direction {
                    EdgeDirection::Outbound => Box::new(edges.map(|edge| edge.outbound_id)),
                    EdgeDirection::Inbound => Box::new(edges.map(|edge| edge.inbound_id)),
                };

                let mut iter: QueryIter<(Uuid, &Type)> = Box::new(
                    iter.map(move |id| (id, self.vertices.get(&id)))
                        .filter_map(|(k, v)| Some((k, v?))),
                );

                if let Some(t) = pipe.t {
                    iter = Box::new(iter.filter(move |(_, v)| v == &&t));
                }

                let iter: QueryIter<Vertex> =
                    Box::new(iter.take(pipe.limit as usize).map(|(k, v)| Vertex::with_id(k, v.clone())));

                Ok(iter)
            }
        }
    }

    fn get_edges_by_query<'a>(&'a self, q: EdgeQuery) -> Result<QueryIter<'a, Edge>> {
        match q {
            EdgeQuery::Specific(specific) => {
                let iter: QueryIter<Edge> = Box::new(
                    specific.edges.into_iter().filter(move |edge| self.edges.contains(&edge))
                );
                Ok(iter)
            }
            EdgeQuery::Pipe(pipe) => {
                let iter = self.get_vertices_by_query(*pipe.inner)?;

                let t = pipe.t.clone();
                let direction = pipe.direction;

                let mut iter: QueryIter<&Edge> = Box::new(iter.flat_map(move |vertex| {
                    let from = match &t {
                        Some(t) => Edge::new(vertex.id, t.clone(), Uuid::default()),
                        None => Edge::new(vertex.id, Type::default(), Uuid::default()),
                    };

                    let iter = if direction == EdgeDirection::Outbound {
                        self.edges.range(from..)
                    } else {
                        self.reversed_edges.range(from..)
                    };

                    iter.take_while(move |edge| edge.outbound_id == vertex.id)
                }));

                if let Some(t) = pipe.t {
                    iter = Box::new(iter.filter(move |edge| edge.t == t));
                }

                let iter = Box::new(iter.skip(pipe.offset as usize).take(pipe.limit as usize));

                if direction == EdgeDirection::Outbound {
                    Ok(Box::new(iter.cloned()))
                } else {
                    Ok(Box::new(iter.map(move |edge| edge.reversed())))
                }
            }
        }
    }

    fn delete_vertices(&mut self, vertices: Vec<Uuid>) -> Result<()> {
        for vertex_id in vertices {
            // TODO: get rid of limits
            let mut deletable_edges: Vec<Edge> = Vec::new();
            let vertex_query = SpecificVertexQuery::single(vertex_id);
            for edge in self.get_edges_by_query(vertex_query.clone().outbound(u32::max_value()).into())? {
                deletable_edges.push(edge);
            }
            for edge in self.get_edges_by_query(vertex_query.inbound(u32::max_value()).into())? {
                deletable_edges.push(edge);
            }
            self.delete_edges(deletable_edges);

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

            self.vertices.remove(&vertex_id);
        }

        Ok(())
    }

    fn delete_edges(&mut self, edges: Vec<Edge>) {
        for edge in edges {
            self.edges.remove(&edge);
            self.reversed_edges.remove(&edge.reversed());

            let mut deletable_edge_properties: Vec<(Edge, String)> = Vec::new();

            for (property_key, _) in self.edge_properties.range((edge.clone(), "".to_string())..) {
                let &(ref property_edge_key, _) = property_key;

                if &edge != property_edge_key {
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
            0: Arc::new(RwLock::new(InternalMemoryDatastore::default())),
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
        let iter = datastore.get_vertices_by_query(q.into())?;
        Ok(iter.collect())
    }

    fn delete_vertices<Q: Into<VertexQuery>>(&self, q: Q) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_vertices = datastore
            .get_vertices_by_query(q.into())?
            .map(|vertex| vertex.id)
            .collect();
        datastore.delete_vertices(deletable_vertices)
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

        datastore.reversed_edges.insert(edge.reversed());
        datastore.edges.insert(edge.clone());
        Ok(true)
    }

    fn get_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<Edge>> {
        let datastore = self.datastore.read().unwrap();
        let iter = datastore.get_edges_by_query(q.into())?;
        Ok(iter.collect())
    }

    fn delete_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_edges: Vec<Edge> = datastore.get_edges_by_query(q.into())?.collect();
        datastore.delete_edges(deletable_edges);
        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&Type>, direction: EdgeDirection) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();

        let from = match t {
            Some(t) => Edge::new(id, t.clone(), Uuid::default()),
            None => Edge::new(id, Type::default(), Uuid::default()),
        };

        let range = if direction == EdgeDirection::Outbound {
            datastore.edges.range(from..)
        } else {
            datastore.reversed_edges.range(from..)
        };


        let range = range.take_while(|edge| {
            if let Some(t) = t {
                edge.outbound_id == id && &edge.t == t
            } else {
                edge.outbound_id == id
            }
        });

        Ok(range.count() as u64)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<VertexProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let vertices = datastore.get_vertices_by_query(q.inner)?;

        for vertex in vertices {
            let property_value = datastore.vertex_properties.get(&(vertex.id, q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(VertexProperty::new(vertex.id, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn get_all_vertex_properties<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<VertexProperties>> {
        let datastore = self.datastore.read().unwrap();
        let vertices = datastore.get_vertices_by_query(q.into())?;

        let mut result = Vec::new();
        for vertex in vertices {
            let from = &(vertex.id, "".to_string());
            let to = &(next_uuid(vertex.id).unwrap(), "".to_string());

            let properties = datastore.vertex_properties.range(from..to);
            result.push(VertexProperties::new(
                vertex,
                properties
                    .map(|(n, p)| NamedProperty::new(n.1.clone(), p.clone()))
                    .collect(),
            ));
        }

        Ok(result)
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let vertices: Vec<Vertex> = datastore.get_vertices_by_query(q.inner)?.collect();

        for vertex in vertices {
            datastore.vertex_properties.insert((vertex.id, q.name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let vertices: Vec<Vertex> = datastore.get_vertices_by_query(q.inner)?.collect();

        for vertex in vertices {
            datastore.vertex_properties.remove(&(vertex.id, q.name.clone()));
        }

        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<EdgeProperty>> {
        let mut result = Vec::new();
        let datastore = self.datastore.read().unwrap();
        let edges = datastore.get_edges_by_query(q.inner)?;

        for edge in edges {
            let property_value = datastore.edge_properties.get(&(edge.clone(), q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(EdgeProperty::new(edge, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn get_all_edge_properties<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<EdgeProperties>> {
        let datastore = self.datastore.read().unwrap();
        let edges = datastore.get_edges_by_query(q.into())?;

        let mut result = Vec::new();
        for edge in edges {
            let from = &(edge.clone(), "".to_string());

            let properties = {
                let edge = edge.clone();
                datastore
                    .edge_properties
                    .range(from..)
                    .take_while(move |((property_edge, _name), _value)| &edge == property_edge)
            };

            result.push(EdgeProperties::new(
                edge,
                properties
                    .map(|(n, p)| NamedProperty::new(n.1.clone(), p.clone()))
                    .collect(),
            ));
        }

        Ok(result)
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edges: Vec<Edge> = datastore.get_edges_by_query(q.inner)?.collect();

        for edge in edges {
            datastore.edge_properties.insert((edge, q.name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edges: Vec<Edge> = datastore.get_edges_by_query(q.inner)?.collect();

        for edge in edges {
            datastore.edge_properties.remove(&(edge, q.name.clone()));
        }

        Ok(())
    }
}
