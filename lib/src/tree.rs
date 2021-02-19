//! The in-memory, sled, and rocksdb datastores were fairly similar - under
//! the hood, they all operated on a few tree-like data structures. This
//! module abstracts out the common datastore and transaction logic for
//! operating on those kinds of systems.

use crate::errors::Result;
use crate::models;

use serde_json::Value as JsonValue;
use uuid::Uuid;
use chrono::DateTime;
use chrono::offset::Utc;
use crate::util::next_uuid;

// TODO: rework comment above when design settles down

pub type OwnedPropertyItem = ((Uuid, String), JsonValue);
pub type VertexItem = (Uuid, models::Type);
pub type EdgeRangeItem = (Uuid, models::Type, DateTime<Utc>, Uuid);
pub type EdgePropertyItem = ((Uuid, models::Type, Uuid, String), JsonValue);

pub trait WriteBatch {}

pub trait VertexManager {
    type WriteBatch: WriteBatch;
    fn exists(&self, id: Uuid) -> Result<bool>;
    fn get(&self, id: Uuid) -> Result<Option<models::Type>>;
    fn iterate_for_range(&self, id: Uuid) -> Result<Box<dyn Iterator<Item = Result<VertexItem>>>>;
    fn create(&self, batch: &mut Self::WriteBatch, vertex: &models::Vertex) -> Result<()>;
    fn delete(&self, batch: &mut Self::WriteBatch, id: Uuid) -> Result<()>;
}

pub trait EdgeManager {
    type WriteBatch: WriteBatch;

    fn get(&self, out_id: Uuid, t: &models::Type, in_id: Uuid) -> Result<Option<DateTime<Utc>>>;

    fn set(
        &self,
        batch: &mut Self::WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        new_update_datetime: DateTime<Utc>,
    ) -> Result<()>;

    fn delete(
        &self,
        batch: &mut Self::WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        update_datetime: DateTime<Utc>,
    ) -> Result<()>;
}

pub trait EdgeRangeManager {
    type WriteBatch: WriteBatch;

    fn iterate_for_range(
        &self,
        id: Uuid,
        t: Option<&models::Type>,
        high: Option<DateTime<Utc>>,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>>>>;

    fn iterate_for_owner(&self, id: Uuid) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>>>>;

    fn set(
        &self,
        batch: &mut Self::WriteBatch,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) -> Result<()>;

    fn delete(
        &self,
        batch: &mut Self::WriteBatch,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) -> Result<()>;
}

pub trait VertexPropertyManager {
    type WriteBatch: WriteBatch;

    fn iterate_for_owner(&self, vertex_id: Uuid) -> Result<Box<dyn Iterator<Item = Result<OwnedPropertyItem>>>>;

    fn get(&self, vertex_id: Uuid, name: &str) -> Result<Option<JsonValue>>;

    fn set(&self, batch: &mut Self::WriteBatch, vertex_id: Uuid, name: &str, value: &JsonValue) -> Result<()>;

    fn delete(&self, batch: &mut Self::WriteBatch, vertex_id: Uuid, name: &str) -> Result<()>;
}

pub trait EdgePropertyManager {
    type WriteBatch: WriteBatch;

    fn iterate_for_owner(
        &self,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgePropertyItem>>>>;

    fn get(&self, out_id: Uuid, t: &models::Type, in_id: Uuid, name: &str) -> Result<Option<JsonValue>>;

    fn set(
        &self,
        batch: &mut Self::WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        name: &str,
        value: &JsonValue,
    ) -> Result<()>;

    fn delete(
        &self,
        batch: &mut Self::WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        name: &str,
    ) -> Result<()>;
}

// TODO: better name
pub trait TreeLikeDatastore {
    type VertexManager: VertexManager<WriteBatch=Self::WriteBatch>;
    fn vertex_manager(&self) -> Self::VertexManager;

    type EdgeManager: EdgeManager<WriteBatch=Self::WriteBatch>;
    fn edge_manager(&self) -> Self::EdgeManager;

    type EdgeRangeManager: EdgeRangeManager<WriteBatch=Self::WriteBatch>;
    fn edge_range_manager(&self) -> Self::EdgeRangeManager;
    fn reversed_edge_range_manager(&self) -> Self::EdgeRangeManager;

    type VertexPropertyManager: VertexPropertyManager<WriteBatch=Self::WriteBatch>;
    fn vertex_property_manager(&self) -> Self::VertexPropertyManager;

    type EdgePropertyManager: EdgePropertyManager<WriteBatch=Self::WriteBatch>;
    fn edge_property_manager(&self) -> Self::EdgePropertyManager;

    type WriteBatch: WriteBatch;
    fn write_batch(&self) -> Self::WriteBatch;
    fn write(&self, batch: Self::WriteBatch) -> Result<()>;

    fn execute_vertex_query(&self, q: models::VertexQuery) -> Result<Vec<VertexItem>> {
        match q {
            models::VertexQuery::Range(q) => {
                let vertex_manager = self.vertex_manager();

                let next_uuid = match q.start_id {
                    Some(start_id) => {
                        match next_uuid(start_id) {
                            Ok(next_uuid) => next_uuid,
                            // If we get an error back, it's because
                            // `start_id` is the maximum possible value. We
                            // know that no vertices exist whose ID is greater
                            // than the maximum possible value, so just return
                            // an empty list.
                            Err(_) => return Ok(vec![]),
                        }
                    }
                    None => Uuid::default(),
                };

                let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> =
                    Box::new(vertex_manager.iterate_for_range(next_uuid)?);

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |item| match item {
                        Ok((_, v)) => v == t,
                        Err(_) => true,
                    }));
                }

                let vertices: Result<Vec<VertexItem>> = iter.take(q.limit as usize).collect();
                vertices
            }
            models::VertexQuery::Specific(q) => {
                let vertex_manager = self.vertex_manager();

                let iter = q.ids.into_iter().map(move |id| match vertex_manager.get(id)? {
                    Some(value) => Ok(Some((id, value))),
                    None => Ok(None),
                });

                let iter = iter.filter_map(|item| match item {
                    Err(err) => Some(Err(err)),
                    Ok(Some(value)) => Some(Ok(value)),
                    _ => None,
                });

                let vertices: Result<Vec<VertexItem>> = iter.collect();
                vertices
            }
            models::VertexQuery::Pipe(q) => {
                let vertex_manager = self.vertex_manager();
                let iter = self.execute_edge_query(*q.inner)?.into_iter();
                let direction = q.direction;

                let iter = iter.map(move |(out_id, _, _, in_id)| {
                    let id = match direction {
                        models::EdgeDirection::Outbound => out_id,
                        models::EdgeDirection::Inbound => in_id,
                    };

                    match vertex_manager.get(id)? {
                        Some(value) => Ok(Some((id, value))),
                        None => Ok(None),
                    }
                });

                let iter = iter.filter_map(|item| match item {
                    Err(err) => Some(Err(err)),
                    Ok(Some(value)) => Some(Ok(value)),
                    _ => None,
                });

                let mut iter: Box<dyn Iterator<Item = Result<VertexItem>>> = Box::new(iter);

                if let Some(ref t) = q.t {
                    iter = Box::new(iter.filter(move |item| match item {
                        Ok((_, v)) => v == t,
                        Err(_) => true,
                    }));
                }

                let vertices: Result<Vec<VertexItem>> = iter.take(q.limit as usize).collect();
                vertices
            }
        }
    }

    fn execute_edge_query(&self, q: models::EdgeQuery) -> Result<Vec<EdgeRangeItem>> {
        match q {
            models::EdgeQuery::Specific(q) => {
                let edge_manager = self.edge_manager();

                let iter = q.keys.into_iter().map(move |key| -> Result<Option<EdgeRangeItem>> {
                    match edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                        Some(update_datetime) => {
                            Ok(Some((key.outbound_id, key.t.clone(), update_datetime, key.inbound_id)))
                        }
                        None => Ok(None),
                    }
                });

                let iter = iter.filter_map(|item| match item {
                    Err(err) => Some(Err(err)),
                    Ok(Some(value)) => Some(Ok(value)),
                    _ => None,
                });

                let edges: Result<Vec<EdgeRangeItem>> = iter.collect();
                edges
            }
            models::EdgeQuery::Pipe(q) => {
                let vertices = self.execute_vertex_query(*q.inner)?;

                let edge_range_manager = match q.direction {
                    models::EdgeDirection::Outbound => self.edge_range_manager(),
                    models::EdgeDirection::Inbound => self.reversed_edge_range_manager(),
                };

                // Ideally we'd use iterators all the way down, but things
                // start breaking apart due to conditional expressions not
                // returning the same type signature, issues with `Result`s
                // and some of the iterators, etc. So at this point, we'll
                // just resort to building a vector.
                let mut edges: Vec<EdgeRangeItem> = Vec::new();

                for (id, _) in vertices.into_iter() {
                    let edge_iterator = edge_range_manager.iterate_for_range(id, q.t.as_ref(), q.high)?;

                    for item in edge_iterator {
                        let (edge_range_first_id, edge_range_t, edge_range_update_datetime, edge_range_second_id) = item?;

                        if let Some(low) = q.low {
                            if edge_range_update_datetime < low {
                                break;
                            }
                        }

                        edges.push(match q.direction {
                            models::EdgeDirection::Outbound => (
                                edge_range_first_id,
                                edge_range_t,
                                edge_range_update_datetime,
                                edge_range_second_id,
                            ),
                            models::EdgeDirection::Inbound => (
                                edge_range_second_id,
                                edge_range_t,
                                edge_range_update_datetime,
                                edge_range_first_id,
                            ),
                        });

                        if edges.len() == q.limit as usize {
                            break;
                        }
                    }
                }

                Ok(edges)
            }
        }
    }

    fn bulk_insert<I>(&self, items: I) -> Result<Self::WriteBatch>
    where
        I: Iterator<Item = models::BulkInsertItem>,
    {
        let vertex_manager = self.vertex_manager();
        let edge_manager = self.edge_manager();
        let edge_range_manager = self.edge_range_manager();
        let reversed_edge_range_manager = self.reversed_edge_range_manager();
        let vertex_property_manager = self.vertex_property_manager();
        let edge_property_manager = self.edge_property_manager();
        let mut batch = self.write_batch();

        for item in items {
            match item {
                models::BulkInsertItem::Vertex(ref vertex) => {
                    vertex_manager.create(&mut batch, vertex)?;
                }
                models::BulkInsertItem::Edge(ref key) => {
                    if let Some(update_datetime) = edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                        edge_range_manager.delete(&mut batch, key.outbound_id, &key.t, update_datetime, key.inbound_id)?;
                        reversed_edge_range_manager.delete(&mut batch, key.inbound_id, &key.t, update_datetime, key.outbound_id)?;
                    }
                    let dt = Utc::now();
                    edge_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, dt)?;
                    edge_range_manager.set(&mut batch, key.outbound_id, &key.t, dt, key.inbound_id)?;
                    reversed_edge_range_manager.set(&mut batch, key.outbound_id, &key.t, dt, key.inbound_id)?;
                }
                models::BulkInsertItem::VertexProperty(id, ref name, ref value) => {
                    vertex_property_manager.set(&mut batch, id, name, value)?;
                }
                models::BulkInsertItem::EdgeProperty(ref key, ref name, ref value) => {
                    edge_property_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, name, value)?;
                }
            }
        }

        Ok(batch)
    }

    fn create_vertex(&self, vertex: &models::Vertex) -> Result<Option<Self::WriteBatch>> {
        let vertex_manager = self.vertex_manager();

        if vertex_manager.exists(vertex.id)? {
            Ok(None)
        } else {
            let mut batch = self.write_batch();
            vertex_manager.create(&mut batch, vertex)?;
            Ok(Some(batch))
        }
    }

    fn get_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::Vertex>> {
        let iter = self.execute_vertex_query(q.into())?.into_iter();

        let iter = iter.map(move |(id, t)| {
            let vertex = models::Vertex::with_id(id, t);
            Ok(vertex)
        });

        iter.collect()
    }

    // TODO: delete propagation needs to be reworked
    fn delete_vertices<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Self::WriteBatch> {
        let iter = self.execute_vertex_query(q.into())?.into_iter();
        let vertex_manager = self.vertex_manager();
        let vertex_property_manager = self.vertex_property_manager();
        let edge_manager = self.edge_manager();
        let edge_range_manager = self.edge_range_manager();
        let reversed_edge_range_manager = self.reversed_edge_range_manager();
        let mut batch = self.write_batch();

        for (id, _) in iter {
            for item in vertex_property_manager.iterate_for_owner(id)? {
                let ((vertex_property_owner_id, vertex_property_name), _) = item?;
                vertex_property_manager.delete(&mut batch, vertex_property_owner_id, &vertex_property_name)?;
            }

            for item in edge_range_manager.iterate_for_owner(id)? {
                let (edge_range_out_id, edge_range_t, edge_range_update_datetime, edge_range_in_id) = item?;
                debug_assert_eq!(edge_range_out_id, id);
                edge_manager.delete(
                    &mut batch,
                    edge_range_out_id,
                    &edge_range_t,
                    edge_range_in_id,
                    edge_range_update_datetime,
                )?;
            }

            for item in reversed_edge_range_manager.iterate_for_owner(id)? {
                let (
                    reversed_edge_range_in_id,
                    reversed_edge_range_t,
                    reversed_edge_range_update_datetime,
                    reversed_edge_range_out_id,
                ) = item?;
                debug_assert_eq!(reversed_edge_range_in_id, id);
                edge_manager.delete(
                    &mut batch,
                    reversed_edge_range_out_id,
                    &reversed_edge_range_t,
                    reversed_edge_range_in_id,
                    reversed_edge_range_update_datetime,
                )?;
            }

            vertex_manager.delete(&mut batch, id)?;
        }

        Ok(batch)
    }

    fn get_vertex_count(&self) -> Result<u64> {
        let vertex_manager = self.vertex_manager();
        let iterator = vertex_manager.iterate_for_range(Uuid::default())?;
        Ok(iterator.count() as u64)
    }

    fn create_edge(&self, key: &models::EdgeKey) -> Result<Option<Self::WriteBatch>> {
        let vertex_manager = self.vertex_manager();

        if !vertex_manager.exists(key.outbound_id)? || !vertex_manager.exists(key.inbound_id)? {
            Ok(None)
        } else {
            let edge_manager = self.edge_manager();
            let edge_range_manager = self.edge_range_manager();
            let reversed_edge_range_manager = self.reversed_edge_range_manager();
            let mut batch = self.write_batch();
            if let Some(update_datetime) = edge_manager.get(key.outbound_id, &key.t, key.inbound_id)? {
                edge_range_manager.delete(&mut batch, key.outbound_id, &key.t, update_datetime, key.inbound_id)?;
                reversed_edge_range_manager.delete(&mut batch, key.inbound_id, &key.t, update_datetime, key.outbound_id)?;
            }
            let dt = Utc::now();
            edge_manager.set(&mut batch, key.outbound_id, &key.t, key.inbound_id, dt)?;
            edge_range_manager.set(&mut batch, key.outbound_id, &key.t, dt, key.inbound_id)?;
            reversed_edge_range_manager.set(&mut batch, key.outbound_id, &key.t, dt, key.inbound_id)?;
            Ok(Some(batch))
        }
    }

    fn get_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::Edge>> {
        let iter = self.execute_edge_query(q.into())?.into_iter();

        let iter = iter.map(move |(out_id, t, update_datetime, in_id)| {
            let key = models::EdgeKey::new(out_id, t, in_id);
            let edge = models::Edge::new(key, update_datetime);
            Ok(edge)
        });

        iter.collect()
    }

    fn delete_edges<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Self::WriteBatch> {
        let edge_manager = self.edge_manager();
        let edge_range_manager = self.edge_range_manager();
        let reversed_edge_range_manager = self.reversed_edge_range_manager();
        let edge_property_manager = self.edge_property_manager();
        let vertex_manager = self.vertex_manager();
        let iter = self.execute_edge_query(q.into())?;
        let mut batch = self.write_batch();

        for (out_id, t, update_datetime, in_id) in iter {
            if vertex_manager.get(out_id)?.is_some() {
                edge_range_manager.delete(&mut batch, out_id, &t, update_datetime, in_id)?;
                reversed_edge_range_manager.delete(&mut batch, in_id, &t, update_datetime, out_id)?;

                for item in edge_property_manager.iterate_for_owner(out_id, &t, in_id)? {
                    let ((edge_property_out_id, edge_property_t, edge_property_in_id, edge_property_name), _) = item?;
                    edge_property_manager.delete(
                        &mut batch,
                        edge_property_out_id,
                        &edge_property_t,
                        edge_property_in_id,
                        &edge_property_name,
                    )?;
                }

                edge_manager.delete(&mut batch, out_id, &t, in_id, update_datetime)?;
            };
        }

        Ok(batch)
    }

    fn get_edge_count(&self, id: Uuid, t: Option<&models::Type>, direction: models::EdgeDirection) -> Result<u64> {
        let edge_range_manager = match direction {
            models::EdgeDirection::Outbound => self.edge_range_manager(),
            models::EdgeDirection::Inbound => self.reversed_edge_range_manager(),
        };

        let count = edge_range_manager.iterate_for_range(id, t, None)?.count();

        Ok(count as u64)
    }

    fn get_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<Vec<models::VertexProperty>> {
        let manager = self.vertex_property_manager();
        let mut properties = Vec::new();

        for (id, _) in self.execute_vertex_query(q.inner)?.into_iter() {
            let value = manager.get(id, &q.name)?;

            if let Some(value) = value {
                properties.push(models::VertexProperty::new(id, value));
            }
        }

        Ok(properties)
    }

    fn get_all_vertex_properties<Q: Into<models::VertexQuery>>(&self, q: Q) -> Result<Vec<models::VertexProperties>> {
        let iter = self.execute_vertex_query(q.into())?.into_iter();
        let manager = self.vertex_property_manager();

        let iter = iter.map(move |(id, t)| {
            let vertex = models::Vertex::with_id(id, t);

            let it = manager.iterate_for_owner(id)?;
            let props: Result<Vec<_>> = it.collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, name), value)| models::NamedProperty::new(name, value))
                .collect();

            Ok(models::VertexProperties::new(vertex, props))
        });

        iter.collect()
    }

    fn set_vertex_properties(&self, q: models::VertexPropertyQuery, value: &JsonValue) -> Result<Self::WriteBatch> {
        let manager = self.vertex_property_manager();
        let mut batch = self.write_batch();

        for (id, _) in self.execute_vertex_query(q.inner)?.into_iter() {
            manager.set(&mut batch, id, &q.name, value)?;
        }

        Ok(batch)
    }

    fn delete_vertex_properties(&self, q: models::VertexPropertyQuery) -> Result<Self::WriteBatch> {
        let manager = self.vertex_property_manager();
        let mut batch = self.write_batch();

        for (id, _) in self.execute_vertex_query(q.inner)?.into_iter() {
            manager.delete(&mut batch, id, &q.name)?;
        }

        Ok(batch)
    }

    fn get_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<Vec<models::EdgeProperty>> {
        let manager = self.edge_property_manager();
        let mut properties = Vec::new();

        for (out_id, t, _, in_id) in self.execute_edge_query(q.inner)?.into_iter() {
            let value = manager.get(out_id, &t, in_id, &q.name)?;

            if let Some(value) = value {
                let key = models::EdgeKey::new(out_id, t, in_id);
                properties.push(models::EdgeProperty::new(key, value));
            }
        }

        Ok(properties)
    }

    fn get_all_edge_properties<Q: Into<models::EdgeQuery>>(&self, q: Q) -> Result<Vec<models::EdgeProperties>> {
        let iter = self.execute_edge_query(q.into())?.into_iter();
        let manager = self.edge_property_manager();

        let iter = iter.map(move |(out_id, t, time, in_id)| {
            let edge = models::Edge::new(models::EdgeKey::new(out_id, t.clone(), in_id), time);
            let it = manager.iterate_for_owner(out_id, &t, in_id)?;
            let props: Result<Vec<_>> = it.collect();
            let props_iter = props?.into_iter();
            let props = props_iter
                .map(|((_, _, _, name), value)| models::NamedProperty::new(name, value))
                .collect();

            Ok(models::EdgeProperties::new(edge, props))
        });

        iter.collect()
    }

    fn set_edge_properties(&self, q: models::EdgePropertyQuery, value: &JsonValue) -> Result<Self::WriteBatch> {
        let manager = self.edge_property_manager();
        let mut batch = self.write_batch();

        for (out_id, t, _, in_id) in self.execute_edge_query(q.inner)?.into_iter() {
            manager.set(&mut batch, out_id, &t, in_id, &q.name, value)?;
        }

        Ok(batch)
    }

    fn delete_edge_properties(&self, q: models::EdgePropertyQuery) -> Result<Self::WriteBatch> {
        let manager = self.edge_property_manager();
        let mut batch = self.write_batch();

        for (out_id, t, _, in_id) in self.execute_edge_query(q.inner)?.into_iter() {
            manager.delete(&mut batch, out_id, &t, in_id, &q.name)?;
        }

        Ok(batch)
    }
}
