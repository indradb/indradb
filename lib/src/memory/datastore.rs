use super::super::{Datastore, EdgeQuery, Transaction, VertexQuery};
use models;
use uuid::Uuid;
use std::collections::{BTreeMap, HashMap, HashSet};
use chrono::DateTime;
use chrono::offset::Utc;
use std::sync::{Arc, RwLock};
use serde_json::Value as JsonValue;
use errors::Error;
use util::UuidGenerator;

// All of the data is actually stored in this struct, which is stored
// internally to the datastore itself. This way, we can wrap an rwlock around
// the entire datastore, rather than on a per-data structure basis, as the
// latter approach would risk deadlocking without extreme care.
#[derive(Debug)]
struct InternalMemoryDatastore {
    edge_metadata: BTreeMap<(models::EdgeKey, String), JsonValue>,
    edges: BTreeMap<models::EdgeKey, DateTime<Utc>>,
    global_metadata: BTreeMap<String, JsonValue>,
    vertex_metadata: BTreeMap<(Uuid, String), JsonValue>,
    vertices: BTreeMap<Uuid, models::Type>,
    uuid_generator: UuidGenerator,
}

impl InternalMemoryDatastore {
    fn get_vertex_values_by_query(
        &self,
        q: VertexQuery,
    ) -> Result<Vec<(Uuid, models::Type)>, Error> {
        match q {
            VertexQuery::All { start_id, limit } => if let Some(start_id) = start_id {
                Ok(self.vertices
                    .range(start_id..)
                    .take(limit as usize)
                    .map(|(k, v)| (*k, v.clone()))
                    .collect())
            } else {
                Ok(self.vertices
                    .iter()
                    .take(limit as usize)
                    .map(|(k, v)| (*k, v.clone()))
                    .collect())
            },
            VertexQuery::Vertices { ids } => {
                let mut results = Vec::new();

                for id in ids {
                    let value = self.vertices.get(&id);

                    if let Some(value) = value {
                        results.push((id, value.clone()));
                    }
                }

                Ok(results)
            }
            VertexQuery::Pipe {
                edge_query,
                converter,
                limit,
            } => {
                let edge_values = self.get_edge_values_by_query(*edge_query)?;

                let ids: Vec<Uuid> = match converter.clone() {
                    models::QueryTypeConverter::Outbound => edge_values
                        .clone()
                        .into_iter()
                        .take(limit as usize)
                        .map(|(key, _)| key.outbound_id)
                        .collect(),
                    models::QueryTypeConverter::Inbound => edge_values
                        .clone()
                        .into_iter()
                        .take(limit as usize)
                        .map(|(key, _)| key.inbound_id)
                        .collect(),
                };

                let mut results = Vec::new();

                for id in ids {
                    let value = self.vertices.get(&id);
                    if let Some(value) = value {
                        results.push((id, value.clone()));
                    }
                }

                Ok(results)
            }
        }
    }

    fn get_edge_values_by_query(
        &self,
        q: EdgeQuery,
    ) -> Result<Vec<(models::EdgeKey, DateTime<Utc>)>, Error> {
        match q {
            EdgeQuery::Edges { keys } => {
                let mut results = Vec::new();

                for key in keys {
                    let value = self.edges.get(&key);

                    if let Some(update_datetime) = value {
                        results.push((key, *update_datetime));
                    }
                }

                Ok(results)
            }
            EdgeQuery::Pipe {
                vertex_query,
                converter,
                type_filter,
                high_filter,
                low_filter,
                limit,
            } => {
                let vertex_values = self.get_vertex_values_by_query(*vertex_query)?;
                let mut results = Vec::new();

                match converter {
                    models::QueryTypeConverter::Outbound => {
                        for (id, _) in vertex_values {
                            let lower_bound = match type_filter {
                                Some(ref type_filter) => {
                                    models::EdgeKey::new(id, type_filter.clone(), Uuid::default())
                                }
                                None => {
                                    // NOTE: Circumventing the constructor for
                                    // `Type` because it doesn't allow empty
                                    // values, yet we need to use one for
                                    // comparison
                                    let empty_type = models::Type("".to_string());
                                    models::EdgeKey::new(id, empty_type, Uuid::default())
                                }
                            };

                            for (key, update_datetime) in self.edges.range(lower_bound..) {
                                if key.outbound_id != id {
                                    break;
                                }

                                if let Some(ref type_filter) = type_filter {
                                    if &key.t != type_filter {
                                        break;
                                    }
                                }

                                if let Some(high_filter) = high_filter {
                                    if *update_datetime > high_filter {
                                        continue;
                                    }
                                }

                                if let Some(low_filter) = low_filter {
                                    if *update_datetime < low_filter {
                                        continue;
                                    }
                                }

                                results.push((key.clone(), *update_datetime));

                                if results.len() == limit as usize {
                                    return Ok(results);
                                }
                            }
                        }
                    }
                    models::QueryTypeConverter::Inbound => {
                        let mut candidate_ids = HashSet::new();
                        for (id, _) in vertex_values {
                            candidate_ids.insert(id);
                        }

                        for (key, update_datetime) in &self.edges {
                            if !candidate_ids.contains(&key.inbound_id) {
                                continue;
                            }

                            if let Some(ref type_filter) = type_filter {
                                if &key.t != type_filter {
                                    continue;
                                }
                            }

                            if let Some(high_filter) = high_filter {
                                if *update_datetime > high_filter {
                                    continue;
                                }
                            }

                            if let Some(low_filter) = low_filter {
                                if *update_datetime < low_filter {
                                    continue;
                                }
                            }

                            results.push((key.clone(), *update_datetime));

                            if results.len() == limit as usize {
                                return Ok(results);
                            }
                        }
                    }
                }

                Ok(results)
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
                edge_metadata: BTreeMap::new(),
                edges: BTreeMap::new(),
                global_metadata: BTreeMap::new(),
                vertex_metadata: BTreeMap::new(),
                vertices: BTreeMap::new(),
                uuid_generator: UuidGenerator::new(false),
            })),
        }
    }
}

impl Datastore<MemoryTransaction> for MemoryDatastore {
    fn transaction(&self) -> Result<MemoryTransaction, Error> {
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
    fn create_vertex(&self, t: models::Type) -> Result<Uuid, Error> {
        let mut datastore = self.datastore.write().unwrap();
        let id = datastore.uuid_generator.next();
        datastore.vertices.insert(id, t);
        Ok(id)
    }

    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<models::Vertex>, Error> {
        let vertex_values = self.datastore
            .read()
            .unwrap()
            .get_vertex_values_by_query(q)?;
        let iter = vertex_values
            .into_iter()
            .map(|(uuid, t)| models::Vertex::new(uuid, t));
        Ok(iter.collect())
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        let vertex_values = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_vertex_values_by_query(q)?
        };

        let mut datastore = self.datastore.write().unwrap();

        for (uuid, _) in vertex_values {
            datastore.vertices.remove(&uuid);
        }

        Ok(())
    }

    fn create_edge(&self, key: models::EdgeKey) -> Result<(), Error> {
        {
            let datastore = self.datastore.read().unwrap();
            let value = datastore.vertices.get(&key.outbound_id);

            if (value.is_some() && !datastore.vertices.contains_key(&key.inbound_id))
                || value.is_none()
            {
                return Err(Error::VertexNotFound);
            }
        }

        let mut datastore = self.datastore.write().unwrap();
        datastore.edges.insert(key, Utc::now());
        Ok(())
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<models::Edge>, Error> {
        let edge_values = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_edge_values_by_query(q)?
        };

        let iter = edge_values
            .into_iter()
            .map(|(key, update_datetime)| models::Edge::new(key, update_datetime));
        Ok(iter.collect())
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        let deletable_edges = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_edge_values_by_query(q)?
        };

        let mut datastore = self.datastore.write().unwrap();

        for (key, _) in deletable_edges {
            datastore.edges.remove(&key);
        }

        Ok(())
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        let edge_values = self.datastore.read().unwrap().get_edge_values_by_query(q)?;
        Ok(edge_values.len() as u64)
    }

    fn get_global_metadata(&self, name: String) -> Result<JsonValue, Error> {
        let datastore = self.datastore.read().unwrap();
        let value = datastore.global_metadata.get(&name);

        if let Some(value) = value {
            Ok(value.clone())
        } else {
            Err(Error::MetadataNotFound)
        }
    }

    fn set_global_metadata(&self, name: String, value: JsonValue) -> Result<(), Error> {
        let mut datastore = self.datastore.write().unwrap();
        datastore.global_metadata.insert(name, value);
        Ok(())
    }

    fn delete_global_metadata(&self, name: String) -> Result<(), Error> {
        let mut datastore = self.datastore.write().unwrap();
        let value = datastore.global_metadata.remove(&name);

        if value.is_some() {
            Ok(())
        } else {
            Err(Error::MetadataNotFound)
        }
    }

    fn get_vertex_metadata(
        &self,
        q: VertexQuery,
        name: String,
    ) -> Result<HashMap<Uuid, JsonValue>, Error> {
        let mut result = HashMap::new();
        let datastore = self.datastore.read().unwrap();
        let vertex_values = datastore.get_vertex_values_by_query(q)?;

        for (id, _) in vertex_values {
            let metadata_value = datastore.vertex_metadata.get(&(id, name.clone()));

            if let Some(metadata_value) = metadata_value {
                result.insert(id, metadata_value.clone());
            }
        }

        Ok(result)
    }

    fn set_vertex_metadata(
        &self,
        q: VertexQuery,
        name: String,
        value: JsonValue,
    ) -> Result<(), Error> {
        let vertex_values = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_vertex_values_by_query(q)?
        };

        let mut datastore = self.datastore.write().unwrap();

        for (id, _) in vertex_values {
            datastore
                .vertex_metadata
                .insert((id, name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_vertex_metadata(&self, q: VertexQuery, name: String) -> Result<(), Error> {
        let vertex_values = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_vertex_values_by_query(q)?
        };

        let mut datastore = self.datastore.write().unwrap();

        for (id, _) in vertex_values {
            datastore.vertex_metadata.remove(&(id, name.clone()));
        }

        Ok(())
    }

    fn get_edge_metadata(
        &self,
        q: EdgeQuery,
        name: String,
    ) -> Result<HashMap<models::EdgeKey, JsonValue>, Error> {
        let mut result = HashMap::new();
        let datastore = self.datastore.read().unwrap();
        let edge_values = datastore.get_edge_values_by_query(q)?;

        for (key, _) in edge_values {
            let metadata_value = datastore.edge_metadata.get(&(key.clone(), name.clone()));

            if let Some(metadata_value) = metadata_value {
                result.insert(key, metadata_value.clone());
            }
        }

        Ok(result)
    }

    fn set_edge_metadata(&self, q: EdgeQuery, name: String, value: JsonValue) -> Result<(), Error> {
        let edge_values = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_edge_values_by_query(q)?
        };

        let mut datastore = self.datastore.write().unwrap();

        for (key, _) in edge_values {
            datastore
                .edge_metadata
                .insert((key, name.clone()), value.clone());
        }

        Ok(())
    }

    fn delete_edge_metadata(&self, q: EdgeQuery, name: String) -> Result<(), Error> {
        let edge_values = {
            let datastore = self.datastore.read().unwrap();
            datastore.get_edge_values_by_query(q)?
        };

        let mut datastore = self.datastore.write().unwrap();

        for (key, _) in edge_values {
            datastore.edge_metadata.remove(&(key, name.clone()));
        }

        Ok(())
    }

    fn commit(self) -> Result<(), Error> {
        Ok(())
    }

    fn rollback(self) -> Result<(), Error> {
        unimplemented!()
    }
}
