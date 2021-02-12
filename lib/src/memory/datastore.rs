use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::sync::{Arc, RwLock};

use crate::errors::Result;
use crate::{
    Datastore, Edge, EdgeDirection, EdgeKey, EdgeProperties, EdgeProperty, EdgePropertyQuery, EdgeQuery, NamedProperty,
    Transaction, Type, Vertex, VertexProperties, VertexProperty, VertexPropertyQuery, VertexQuery,
};

use bincode::Error as BincodeError;
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tempfile::NamedTempFile;
use uuid::Uuid;

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug, Default, Serialize, Deserialize)]
struct InternalMemoryDatastore {
    vertices: BTreeMap<Uuid, Type>,
    edges: BTreeMap<EdgeKey, DateTime<Utc>>,
    reversed_edges: BTreeMap<EdgeKey, DateTime<Utc>>,
    vertex_properties: BTreeMap<(Uuid, String), JsonValue>,
    edge_properties: BTreeMap<(EdgeKey, String), JsonValue>,
}

type QueryIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

impl InternalMemoryDatastore {
    fn get_vertex_values_by_query(&self, q: VertexQuery) -> Result<QueryIter<'_, (Uuid, Type)>> {
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
                    EdgeDirection::Outbound => Box::new(edge_values.map(|(key, _)| key.outbound_id)),
                    EdgeDirection::Inbound => Box::new(edge_values.map(|(key, _)| key.inbound_id)),
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

    fn get_edge_values_by_query(&self, q: EdgeQuery) -> Result<QueryIter<'_, (EdgeKey, DateTime<Utc>)>> {
        match q {
            EdgeQuery::Specific(specific) => {
                let iter: QueryIter<(EdgeKey, DateTime<Utc>)> = Box::new(
                    specific
                        .keys
                        .into_iter()
                        .filter_map(move |key| self.edges.get(&key).map(|update_datetime| (key, *update_datetime))),
                );

                Ok(iter)
            }
            EdgeQuery::Pipe(pipe) => {
                let iter = self.get_vertex_values_by_query(*pipe.inner)?;

                let t = pipe.t.clone();
                let direction = pipe.direction;

                let mut iter: QueryIter<(&EdgeKey, &DateTime<Utc>)> = Box::new(iter.flat_map(move |(id, _)| {
                    let lower_bound = match &t {
                        Some(t) => EdgeKey::new(id, t.clone(), Uuid::default()),
                        None => EdgeKey::new(id, Type::default(), Uuid::default()),
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

            let mut deletable_edges: Vec<EdgeKey> = Vec::new();

            for edge_key in self.edges.keys() {
                if edge_key.outbound_id == vertex_id || edge_key.inbound_id == vertex_id {
                    deletable_edges.push(edge_key.clone());
                }
            }

            self.delete_edges(deletable_edges);
        }
    }

    fn delete_edges(&mut self, edges: Vec<EdgeKey>) {
        for edge_key in edges {
            self.edges.remove(&edge_key);
            self.reversed_edges.remove(&edge_key.reversed());

            let mut deletable_edge_properties: Vec<(EdgeKey, String)> = Vec::new();

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
    type Trans = MemoryTransaction;

    fn sync(&self) -> Result<()> {
        if let Some(ref persist_path) = self.path {
            let temp_path = NamedTempFile::new()?;
            let buf = BufWriter::new(temp_path.as_file());
            let datastore = self.datastore.read().unwrap();
            bincode::serialize_into(buf, &*datastore)?;
            temp_path.persist(persist_path)?;
        }
        Ok(())
    }

    fn transaction(&self) -> Result<Self::Trans> {
        Ok(MemoryTransaction {
            datastore: Arc::clone(&self.datastore),
        })
    }
}

/// A transaction for manipulating in-memory datastores.
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

    fn create_edge(&self, key: &EdgeKey) -> Result<bool> {
        let mut datastore = self.datastore.write().unwrap();

        if !datastore.vertices.contains_key(&key.outbound_id) || !datastore.vertices.contains_key(&key.inbound_id) {
            return Ok(false);
        }

        datastore.edges.insert(key.clone(), Utc::now());
        datastore.reversed_edges.insert(key.reversed(), Utc::now());
        Ok(true)
    }

    fn get_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<Edge>> {
        let edge_values: Vec<(EdgeKey, DateTime<Utc>)> = {
            let datastore = self.datastore.read().unwrap();
            let iter = datastore.get_edge_values_by_query(q.into())?;
            iter.collect()
        };

        let iter = edge_values
            .into_iter()
            .map(|(key, update_datetime)| Edge::new(key, update_datetime));
        Ok(iter.collect())
    }

    fn delete_edges<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let deletable_edges: Vec<EdgeKey> = datastore.get_edge_values_by_query(q.into())?.map(|(k, _)| k).collect();
        datastore.delete_edges(deletable_edges);
        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&Type>, direction: EdgeDirection) -> Result<u64> {
        let datastore = self.datastore.read().unwrap();

        let lower_bound = match t {
            Some(t) => EdgeKey::new(id, t.clone(), Uuid::default()),
            None => EdgeKey::new(id, Type::default(), Uuid::default()),
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

        for (key, _) in edge_values {
            let property_value = datastore.edge_properties.get(&(key.clone(), q.name.clone()));

            if let Some(property_value) = property_value {
                result.push(EdgeProperty::new(key, property_value.clone()));
            }
        }

        Ok(result)
    }

    fn get_all_edge_properties<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<EdgeProperties>> {
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(q.into())?;

        let mut result = Vec::new();
        for (id, t) in edge_values {
            let from = &(id.clone(), "".to_string());

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

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edge_values: Vec<(EdgeKey, DateTime<Utc>)> = datastore.get_edge_values_by_query(q.inner)?.collect();

        for (key, _) in edge_values.into_iter() {
            datastore.edge_properties.insert((key, q.name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let mut datastore = self.datastore.write().unwrap();
        let edge_values: Vec<(EdgeKey, DateTime<Utc>)> = datastore.get_edge_values_by_query(q.inner)?.collect();

        for (key, _) in edge_values {
            datastore.edge_properties.remove(&(key, q.name.clone()));
        }

        Ok(())
    }
}
