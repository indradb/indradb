use super::super::{
    Datastore, EdgeDirection, EdgePropertyQuery, EdgeQuery, Transaction, VertexPropertyQuery, VertexQuery,
};
use super::managers::*;
use crate::errors::Result;
use crate::models;
use crate::models::*;
use crate::util::remove_nones_from_iterator;
use serde_json::Value as JsonValue;
use sled::{Db, Tree};
use std::sync::Arc;
use std::u64;
use std::usize;

/// The meat of a Sled datastore
pub struct SledHolder {
    pub(crate) db: Arc<Db>, // Derefs to Tree, holds the vertices
    pub(crate) edge_ranges: Tree,
    pub(crate) reversed_edge_ranges: Tree,
    pub(crate) vertex_properties: Tree,
    pub(crate) edge_properties: Tree,
}

impl<'ds> SledHolder {
    /// The meat of a Sled datastore.
    ///
    /// # Arguments
    /// * `path` - The file path to the Sled database.
    pub fn new(path: &str) -> Result<SledHolder> {
        let db = sled::open(path)?;
        Ok(SledHolder {
            edge_ranges: db.open_tree("edge_ranges")?,
            reversed_edge_ranges: db.open_tree("reversed_edge_ranges")?,
            vertex_properties: db.open_tree("vertex_properties")?,
            edge_properties: db.open_tree("edge_properties")?,
            db: Arc::new(db),
        })
    }
}

/// A datastore that is backed by Sled.
pub struct SledDatastore {
    pub(crate) holder: Arc<SledHolder>,
}

impl<'ds> SledDatastore {
    /// Creates a new Sled datastore.
    ///
    /// # Arguments
    /// * `path` - The file path to the Sled database.
    pub fn new(path: &str) -> Result<SledDatastore> {
        Ok(SledDatastore {
            holder: Arc::new(SledHolder::new(path)?),
        })
    }
}

impl Datastore for SledDatastore {
    type Trans = SledTransaction;

    fn transaction(&self) -> Result<Self::Trans> {
        SledTransaction::new(self.holder.clone())
    }

    fn bulk_insert<I>(&self, items: I) -> Result<models::BulkInsertResult>
    where
        I: Iterator<Item = models::BulkInsertItem>,
    {
        let vertex_manager = VertexManager::new(&self.holder);
        let edge_manager = EdgeManager::new(&self.holder);
        let vertex_property_manager = VertexPropertyManager::new(&self.holder.vertex_properties);
        let edge_property_manager = EdgePropertyManager::new(&self.holder.edge_properties);
        let mut id_range = None;

        for item in items {
            match item {
                models::BulkInsertItem::Vertex(ref t) => {
                    let id = {
                        let db = self.holder.db.clone();
                        db.generate_id()?
                    };
                    let vertex = models::Vertex::new(id, t.clone());
                    vertex_manager.create(&vertex)?;
                    id_range = match id_range {
                        Some((start_id, _)) => Some((start_id, id)),
                        None => Some((id, id)),
                    };
                }
                models::BulkInsertItem::Edge(ref key) => {
                    edge_manager.set(key.outbound_id, &key.t, key.inbound_id)?;
                }
                models::BulkInsertItem::VertexProperty(id, ref name, ref value) => {
                    vertex_property_manager.set(id, name, value)?;
                }
                models::BulkInsertItem::EdgeProperty(ref key, ref name, ref value) => {
                    edge_property_manager.set(key.outbound_id, &key.t, key.inbound_id, name, value)?;
                }
            }
        }

        self.holder.db.flush()?;
        Ok(BulkInsertResult { id_range })
    }
}

/// A transaction that is backed by Sled.
pub struct SledTransaction {
    holder: Arc<SledHolder>,
}

impl SledTransaction {
    fn new(holder: Arc<SledHolder>) -> Result<Self> {
        Ok(SledTransaction { holder })
    }

    fn vertex_query_to_iterator<'iter, 'trans: 'iter>(
        &'trans self,
        q: VertexQuery,
    ) -> Result<Box<dyn Iterator<Item = Result<VertexItem>> + 'iter>> {
        match q {
            VertexQuery::Range(q) => {
                let vertex_manager = VertexManager::new(&self.holder);

                let next_id = match q.start_id {
                    Some(start_id) => {
                        match start_id.checked_add(1) {
                            Some(id) => id,
                            // If we get an error back, it's because
                            // `start_id` is the maximum possible value. We
                            // know that no vertices exist whose ID is greater
                            // than the maximum possible value, so just return
                            // an empty list.
                            None => return Ok(Box::new(vec![].into_iter())),
                        }
                    }
                    None => 0,
                };

                let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> =
                    Box::new(vertex_manager.iterate_for_range(next_id)?);

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
                let vertex_manager = VertexManager::new(&self.holder);

                let iter = q.ids.into_iter().map(move |id| match vertex_manager.get(id)? {
                    Some(value) => Ok(Some((id, value))),
                    None => Ok(None),
                });

                Ok(Box::new(remove_nones_from_iterator(iter)))
            }
            VertexQuery::Pipe(q) => {
                let vertex_manager = VertexManager::new(&self.holder);
                let edge_iterator = self.edge_query_to_iterator(*q.inner)?;
                let direction = q.direction;

                let iter = edge_iterator.map(move |item| {
                    let (outbound_id, _, inbound_id) = item?;

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

    fn edge_query_to_iterator<'iter, 'trans: 'iter>(
        &'trans self,
        q: EdgeQuery,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>> + 'iter>> {
        match q {
            EdgeQuery::Specific(q) => {
                let edge_range_manager = EdgeRangeManager::new(&self.holder);

                let edges = q.edges.into_iter().map(move |key| {
                    if edge_range_manager.exists(key.outbound_id, &key.t, key.inbound_id)? {
                        Ok(Some((key.outbound_id, key.t.clone(), key.inbound_id)))
                    } else {
                        Ok(None)
                    }
                });

                let iterator = remove_nones_from_iterator(edges);
                Ok(Box::new(iterator))
            }
            EdgeQuery::Pipe(q) => {
                let vertex_iterator = self.vertex_query_to_iterator(*q.inner)?;

                let edge_range_manager = match q.direction {
                    EdgeDirection::Outbound => EdgeRangeManager::new(&self.holder),
                    EdgeDirection::Inbound => EdgeRangeManager::new_reversed(&self.holder),
                };

                // Ideally we'd use iterators all the way down, but things
                // start breaking apart due to conditional expressions not
                // returning the same type signature, issues with `Result`s
                // and some of the iterators, etc. So at this point, we'll
                // just resort to building a vector.
                let mut edges: Vec<Result<EdgeRangeItem>> = Vec::new();

                for item in vertex_iterator {
                    let (id, _) = item?;
                    let edge_iterator = edge_range_manager.iterate_for_range(id, q.t.as_ref())?;

                    for item in edge_iterator {
                        match item {
                            Ok((
                                edge_range_first_id,
                                edge_range_t,
                                edge_range_second_id,
                            )) => {
                                edges.push(match q.direction {
                                    EdgeDirection::Outbound => Ok((
                                        edge_range_first_id,
                                        edge_range_t,
                                        edge_range_second_id,
                                    )),
                                    EdgeDirection::Inbound => Ok((
                                        edge_range_second_id,
                                        edge_range_t,
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
    fn create_vertex(&self, t: &Type) -> Result<u64> {
        let id = {
            let db = self.holder.db.clone();
            db.generate_id()?
        };

        let vertex = Vertex::new(id, t.clone());
        let vertex_manager = VertexManager::new(&self.holder);
        vertex_manager.create(&vertex)?;
        Ok(id)
    }

    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>> {
        let iterator = self.vertex_query_to_iterator(q.into())?;

        let mapped = iterator.map(move |item| {
            let (id, t) = item?;
            let vertex = models::Vertex::new(id, t);
            Ok(vertex)
        });

        mapped.collect()
    }

    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<()> {
        let iterator = self.vertex_query_to_iterator(q.into())?;
        let vertex_manager = VertexManager::new(&self.holder);

        for item in iterator {
            let (id, _) = item?;
            vertex_manager.delete(id)?;
        }

        Ok(())
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let vertex_manager = VertexManager::new(&self.holder);
        let iterator = vertex_manager.iterate_for_range(0)?;
        Ok(iterator.count() as u64)
    }

    fn create_edge(&self, edge: &models::Edge) -> Result<bool> {
        let vertex_manager = VertexManager::new(&self.holder);

        if !vertex_manager.exists(edge.outbound_id)? || !vertex_manager.exists(edge.inbound_id)? {
            Ok(false)
        } else {
            let edge_manager = EdgeManager::new(&self.holder);
            edge_manager.set(edge.outbound_id, &edge.t, edge.inbound_id)?;
            Ok(true)
        }
    }

    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>> {
        let iterator = self.edge_query_to_iterator(q.into())?;

        let mapped = iterator.map(move |item: Result<EdgeRangeItem>| {
            let (outbound_id, t, inbound_id) = item?;
            let edge = models::Edge::new(outbound_id, t, inbound_id);
            Ok(edge)
        });

        mapped.collect()
    }

    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<()> {
        let edge_manager = EdgeManager::new(&self.holder);
        let vertex_manager = VertexManager::new(&self.holder);
        let iterator = self.edge_query_to_iterator(q.into())?;

        for item in iterator {
            let (outbound_id, t, inbound_id) = item?;

            if vertex_manager.get(outbound_id)?.is_some() {
                edge_manager.delete(outbound_id, &t, inbound_id)?;
            };
        }
        Ok(())
    }

    fn get_edge_count(&self, id: u64, t: Option<&models::Type>, direction: models::EdgeDirection) -> Result<u64> {
        let edge_range_manager = match direction {
            EdgeDirection::Outbound => EdgeRangeManager::new(&self.holder),
            EdgeDirection::Inbound => EdgeRangeManager::new_reversed(&self.holder),
        };

        let iter = edge_range_manager.iterate_for_range(id, t)?;
        let count = iter.count();

        Ok(count as u64)
    }

    fn get_vertex_properties(&self, q: VertexPropertyQuery) -> Result<Vec<models::VertexProperty>> {
        let manager = VertexPropertyManager::new(&self.holder.vertex_properties);
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

    fn get_all_vertex_properties<Q: Into<VertexQuery>>(&self, q: Q) -> Result<Vec<models::VertexProperties>> {
        let manager = VertexPropertyManager::new(&self.holder.vertex_properties);
        let iterator = self.vertex_query_to_iterator(q.into())?;

        let iter = iterator.map(move |item| {
            let (id, t) = item?;
            let vertex = models::Vertex::new(id, t);

            let it = manager.iterate_for_owner(id)?;
            let props: Result<Vec<_>> = it.map(|r| r).collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, name), value)| models::NamedProperty::new(name, value))
                .collect();

            Ok(models::VertexProperties::new(vertex, props))
        });

        iter.collect()
    }

    fn set_vertex_properties(&self, q: VertexPropertyQuery, value: &JsonValue) -> Result<()> {
        let manager = VertexPropertyManager::new(&self.holder.vertex_properties);

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            manager.set(id, &q.name, value)?;
        }
        Ok(())
    }

    fn delete_vertex_properties(&self, q: VertexPropertyQuery) -> Result<()> {
        let manager = VertexPropertyManager::new(&self.holder.vertex_properties);

        for item in self.vertex_query_to_iterator(q.inner)? {
            let (id, _) = item?;
            manager.delete(id, &q.name)?;
        }
        Ok(())
    }

    fn get_edge_properties(&self, q: EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        let manager = EdgePropertyManager::new(&self.holder.edge_properties);
        let mut properties = Vec::new();

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, inbound_id) = item?;
            let value = manager.get(outbound_id, &t, inbound_id, &q.name)?;

            if let Some(value) = value {
                let key = models::Edge::new(outbound_id, t, inbound_id);
                properties.push(models::EdgeProperty::new(key, value));
            }
        }

        Ok(properties)
    }

    fn get_all_edge_properties<Q: Into<EdgeQuery>>(&self, q: Q) -> Result<Vec<EdgeProperties>> {
        let manager = EdgePropertyManager::new(&self.holder.edge_properties);
        let iterator = self.edge_query_to_iterator(q.into())?;

        let iter = iterator.map(move |item| {
            let (out_id, t, in_id) = item?;
            let edge = Edge::new(out_id, t.clone(), in_id);
            let it = manager.iterate_for_owner(out_id, &t, in_id)?;
            let props: Result<Vec<_>> = it.map(|r| r).collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, _, _, name), value)| NamedProperty::new(name, value))
                .collect();

            Ok(EdgeProperties::new(edge, props))
        });

        iter.collect()
    }

    fn set_edge_properties(&self, q: EdgePropertyQuery, value: &JsonValue) -> Result<()> {
        let manager = EdgePropertyManager::new(&self.holder.edge_properties);

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, inbound_id) = item?;
            manager.set(outbound_id, &t, inbound_id, &q.name, value)?;
        }
        Ok(())
    }

    fn delete_edge_properties(&self, q: EdgePropertyQuery) -> Result<()> {
        let manager = EdgePropertyManager::new(&self.holder.edge_properties);

        for item in self.edge_query_to_iterator(q.inner)? {
            let (outbound_id, t, inbound_id) = item?;
            manager.delete(outbound_id, &t, inbound_id, &q.name)?;
        }
        Ok(())
    }
}
