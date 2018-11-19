use super::super::{
    Datastore, EdgeDirection, EdgePropertyQuery, EdgeQuery, Transaction, VertexPropertyQuery, VertexQuery,
};
use super::managers::*;
use chrono::offset::Utc;
use errors::Result;
use models;
use serde_json::Value as JsonValue;
use std::u64;
use std::usize;
use util::{remove_nones_from_iterator, next_uuid};
use uuid::Uuid;
use std::path::{Path, PathBuf};
use sled::Tree;

/// A datastore that is backed by sled.
#[derive(Debug)]
pub struct SledDatastore {
    db: Database
}

impl SledDatastore {
    /// Creates a new sled datastore.
    ///
    /// # Arguments
    /// * `path` - The file path to the sled database.
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        Ok(Self { db: Database::new(path)? })
    }
}

impl Datastore for SledDatastore {
    type Trans = SledTransaction;

    fn transaction(&self) -> Result<Self::Trans> {
        Ok(SledTransaction::new(self.db.clone()))
    }
}

/// A transaction that is backed by sled.
#[derive(Debug)]
pub struct SledTransaction {
    db: Database
}

impl SledTransaction {
    fn new(db: Database) -> Self {
        Self { db }
    }

    fn vertex_query_to_iterator(&self, q: VertexQuery) -> Result<Box<dyn Iterator<Item = Result<VertexItem>>>> {
        match q {
            VertexQuery::Range(q) => {
                let vertex_manager = VertexManager::new(self.db.clone());

                let next_uuid = match q.start_id {
                    Some(start_id) => {
                        match next_uuid(start_id) {
                            Ok(next_uuid) => next_uuid,
                            // If we get an error back, it's because
                            // `start_id` is the maximum possible value. We
                            // know that no vertices exist whose ID is greater
                            // than the maximum possible value, so just return
                            // an empty list.
                            Err(_) => return Ok(Box::new(vec![].into_iter())),
                        }
                    }
                    None => Uuid::default(),
                };

                let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> =
                    Box::new(vertex_manager.iterate_for_range(next_uuid).into_iter());

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |item| match item {
                        Ok((_, v)) => v == t,
                        Err(_) => true,
                    }));
                }

                let results: Vec<Result<VertexItem>> = iter.take(q.limit as usize).collect();
                Ok(Box::new(results.into_iter()))
            }
            VertexQuery::Specific(q) => {
                let vertex_manager = VertexManager::new(self.db.clone());

                let iter = q.ids.into_iter().map(move |id| match vertex_manager.get(id)? {
                    Some(value) => Ok(Some((id, value))),
                    None => Ok(None),
                });

                Ok(Box::new(remove_nones_from_iterator(iter)))
            }
            VertexQuery::Pipe(q) => {
                let vertex_manager = VertexManager::new(self.db.clone());
                let edge_iterator = self.edge_query_to_iterator(*q.inner)?;
                let direction = q.direction;

                let iter = edge_iterator.map(move |item| {
                    let (outbound_id, _, _, inbound_id) = item?;

                    let id = match direction {
                        EdgeDirection::Outbound => outbound_id,
                        EdgeDirection::Inbound => inbound_id,
                    };

                    match vertex_manager.get(id)? {
                        Some(value) => Ok(Some((id, value))),
                        None => Ok(None),
                    }
                });

                let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> = Box::new(remove_nones_from_iterator(iter));

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |item| match item {
                        Ok((_, v)) => v == t,
                        Err(_) => true,
                    }));
                }

                let results: Vec<Result<VertexItem>> = iter.take(q.limit as usize).collect();
                Ok(Box::new(results.into_iter()))
            }
        }
    }

    fn edge_query_to_iterator(&self, q: EdgeQuery) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>>>> {
        match q {
            EdgeQuery::Specific(q) => {
                let edge_manager = EdgeManager::new(self.db.clone());

                let edges = q.keys.into_iter().map(move |key| {
                    match edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                        Some(update_datetime) => {
                            Ok(Some((key.outbound_id, key.t.clone(), update_datetime, key.inbound_id)))
                        }
                        None => Ok(None),
                    }
                });

                let iterator = remove_nones_from_iterator(edges);
                Ok(Box::new(iterator))
            }
            EdgeQuery::Pipe(q) => {
                let vertex_iterator = self.vertex_query_to_iterator(*q.inner)?;

                let edge_range_manager = match q.direction {
                    EdgeDirection::Outbound => EdgeRangeManager::new(self.db.clone()),
                    EdgeDirection::Inbound => EdgeRangeManager::new_reversed(self.db.clone()),
                };

                // Ideally we'd use iterators all the way down, but things
                // start breaking apart due to conditional expressions not
                // returning the same type signature, issues with `Result`s
                // and some of the iterators, etc. So at this point, we'll
                // just resort to building a vector.
                let mut edges: Vec<Result<EdgeRangeItem>> = Vec::new();

                for item in vertex_iterator {
                    let (id, _) = item?;
                    let edge_iterator = edge_range_manager.iterate_for_range(id, q.t.as_ref(), q.high);

                    for item in edge_iterator {
                        match item {
                            Ok((
                                edge_range_first_id,
                                edge_range_t,
                                edge_range_update_datetime,
                                edge_range_second_id,
                            )) => {
                                if let Some(low) = q.low {
                                    if edge_range_update_datetime < low {
                                        break;
                                    }
                                }

                                edges.push(match q.direction {
                                    EdgeDirection::Outbound => Ok((
                                        edge_range_first_id,
                                        edge_range_t,
                                        edge_range_update_datetime,
                                        edge_range_second_id,
                                    )),
                                    EdgeDirection::Inbound => Ok((
                                        edge_range_second_id,
                                        edge_range_t,
                                        edge_range_update_datetime,
                                        edge_range_first_id,
                                    )),
                                })
                            }
                            Err(_) => edges.push(item),
                        }

                        if edges.len() == q.limit as usize {
                            break;
                        }
                    }
                }

                Ok(Box::new(edges.into_iter()))
            }
        }
    }
}

impl Transaction for SledTransaction {
    fn create_vertex(&self, vertex: &models::Vertex) -> Result<bool> {
        let vertex_manager = VertexManager::new(self.db.clone());

        if vertex_manager.exists(vertex.id)? {
            Ok(false)
        } else {
            vertex_manager.create(vertex)?;
            Ok(true)
        }
    }

    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>> {
        let iterator = self.vertex_query_to_iterator(q.into())?;

        let mapped = iterator.map(move |item| {
            let (id, t) = item?;
            let vertex = models::Vertex::with_id(id, t);
            Ok(vertex)
        });

        mapped.collect()
    }

    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<()> {
        let iterator = self.vertex_query_to_iterator(q.into())?;
        let vertex_manager = VertexManager::new(self.db.clone());

        for item in iterator {
            let (id, _) = item?;
            vertex_manager.delete(id)?;
        }

        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let vertex_manager = VertexManager::new(self.db.clone());
        let iterator = vertex_manager.iterate_for_range(Uuid::default());
        Ok(iterator.len() as u64)
    }

    fn create_edge(&self, key: &models::EdgeKey) -> Result<bool> {
        let vertex_manager = VertexManager::new(self.db.clone());

        if !vertex_manager.exists(key.outbound_id)? || !vertex_manager.exists(key.inbound_id)? {
            Ok(false)
        } else {
            let edge_manager = EdgeManager::new(self.db.clone());
            edge_manager.set(key.outbound_id, &key.t, key.inbound_id, Utc::now())?;
            Ok(true)
        }
    }

    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>> {
        let iterator = self.edge_query_to_iterator(q.into())?;

        let mapped = iterator.map(move |item| {
            let (outbound_id, t, update_datetime, inbound_id) = item?;
            let key = models::EdgeKey::new(outbound_id, t, inbound_id);
            let edge = models::Edge::new(key, update_datetime);
            Ok(edge)
        });

        mapped.collect()
    }

    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<()> {
        let edge_manager = EdgeManager::new(self.db.clone());
        let vertex_manager = VertexManager::new(self.db.clone());
        let iterator = self.edge_query_to_iterator(q.into())?;

        for item in iterator {
            let (outbound_id, t, update_datetime, inbound_id) = item?;

            if vertex_manager.get(outbound_id)?.is_some() {
                edge_manager.delete(outbound_id, &t, inbound_id, update_datetime)?;
            };
        }

        Ok(())
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&models::Type>, direction: models::EdgeDirection) -> Result<u64> {
        let edge_range_manager = match direction {
            EdgeDirection::Outbound => EdgeRangeManager::new(self.db.clone()),
            EdgeDirection::Inbound => EdgeRangeManager::new_reversed(self.db.clone()),
        };

        let count = edge_range_manager.iterate_for_range(id, t, None).count();

        Ok(count as u64)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<models::VertexProperty>> {
        let manager = VertexPropertyManager::new(self.db.clone());
        let mut properties = Vec::new();

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            let value = manager.get(id, &q.name)?;

            if let Some(value) = value {
                properties.push(models::VertexProperty::new(id, value));
            }
        }

        Ok(properties)
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let manager = VertexPropertyManager::new(self.db.clone());

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            manager.set(id, &q.name, value)?;
        }

        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let manager = VertexPropertyManager::new(self.db.clone());

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            manager.delete(id, &q.name)?;
        }

        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        let manager = EdgePropertyManager::new(self.db.clone());
        let mut properties = Vec::new();

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, _, inbound_id) = item?;
            let value = manager.get(outbound_id, &t, inbound_id, &q.name)?;

            if let Some(value) = value {
                let key = models::EdgeKey::new(outbound_id, t, inbound_id);
                properties.push(models::EdgeProperty::new(key, value));
            }
        }

        Ok(properties)
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let manager = EdgePropertyManager::new(self.db.clone());

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, _, inbound_id) = item?;
            manager.set(outbound_id, &t, inbound_id, &q.name, value)?;
        }

        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let manager = EdgePropertyManager::new(self.db.clone());

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, _, inbound_id) = item?;
            manager.delete(outbound_id, &t, inbound_id, &q.name)?;
        }

        Ok(())
    }
}
