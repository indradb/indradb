use std::io::Cursor;
use std::ops::Deref;
use std::u8;

use super::super::bytes::*;
use crate::errors::Result;
use crate::models;

use chrono::offset::Utc;
use chrono::DateTime;
use rocksdb::{ColumnFamily, DBIterator, Direction, IteratorMode, WriteBatch, DB};
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub type OwnedPropertyItem = ((Uuid, String), JsonValue);
pub type VertexItem = (Uuid, models::Type);
pub type EdgeRangeItem = (Uuid, models::Type, DateTime<Utc>, Uuid);
pub type EdgePropertyItem = ((Uuid, models::Type, Uuid, String), JsonValue);

pub struct VertexManager<'a> {
    pub db: &'a DB,
    pub cf: &'a ColumnFamily,
}

impl<'a> VertexManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        VertexManager {
            cf: db.cf_handle("vertices:v1").unwrap(),
            db,
        }
    }

    fn key(&self, id: Uuid) -> Vec<u8> {
        build(&[Component::Uuid(id)])
    }

    pub fn exists(&self, id: Uuid) -> Result<bool> {
        Ok(self.db.get_cf(self.cf, &self.key(id))?.is_some())
    }

    pub fn get(&self, id: Uuid) -> Result<Option<models::Type>> {
        match self.db.get_cf(self.cf, &self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_type(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    fn iterate(&'a self, iterator: DBIterator<'a>) -> impl Iterator<Item = Result<VertexItem>> + 'a {
        iterator.map(|item| -> Result<VertexItem> {
            let (k, v) = item;

            let id = {
                debug_assert_eq!(k.len(), 16);
                let mut cursor = Cursor::new(k);
                read_uuid(&mut cursor)
            };

            let mut cursor = Cursor::new(v);
            let t = read_type(&mut cursor);
            Ok((id, t))
        })
    }

    pub fn iterate_for_range(&'a self, id: Uuid) -> impl Iterator<Item = Result<VertexItem>> + 'a {
        let low_key = build(&[Component::Uuid(id)]);
        let iter = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward));
        self.iterate(iter)
    }

    pub fn create(&self, batch: &mut WriteBatch, vertex: &models::Vertex) -> Result<()> {
        let key = self.key(vertex.id);
        batch.put_cf(self.cf, &key, &build(&[Component::Type(&vertex.t)]));
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, id: Uuid) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(id));

        let vertex_property_manager = VertexPropertyManager::new(self.db);
        for item in vertex_property_manager.iterate_for_owner(id)? {
            let ((vertex_property_owner_id, vertex_property_name), _) = item?;
            vertex_property_manager.delete(&mut batch, vertex_property_owner_id, &vertex_property_name[..])?;
        }

        let edge_manager = EdgeManager::new(self.db);

        {
            let edge_range_manager = EdgeRangeManager::new(self.db);
            for item in edge_range_manager.iterate_for_owner(id) {
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
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
            for item in reversed_edge_range_manager.iterate_for_owner(id) {
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
        }

        Ok(())
    }
}

pub struct EdgeManager<'a> {
    pub db: &'a DB,
    pub cf: &'a ColumnFamily,
}

impl<'a> EdgeManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgeManager {
            cf: db.cf_handle("edges:v1").unwrap(),
            db,
        }
    }

    fn key(&self, out_id: Uuid, t: &models::Type, in_id: Uuid) -> Vec<u8> {
        build(&[Component::Uuid(out_id), Component::Type(t), Component::Uuid(in_id)])
    }

    pub fn get(&self, out_id: Uuid, t: &models::Type, in_id: Uuid) -> Result<Option<DateTime<Utc>>> {
        match self.db.get_cf(self.cf, &self.key(out_id, t, in_id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_datetime(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        mut batch: &mut WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        new_update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db);
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);

        if let Some(update_datetime) = self.get(out_id, t, in_id)? {
            edge_range_manager.delete(&mut batch, out_id, t, update_datetime, in_id)?;
            reversed_edge_range_manager.delete(&mut batch, in_id, t, update_datetime, out_id)?;
        }

        let key = self.key(out_id, t, in_id);
        batch.put_cf(self.cf, &key, &build(&[Component::DateTime(new_update_datetime)]));
        edge_range_manager.set(&mut batch, out_id, t, new_update_datetime, in_id)?;
        reversed_edge_range_manager.set(&mut batch, in_id, t, new_update_datetime, out_id)?;
        Ok(())
    }

    pub fn delete(
        &self,
        mut batch: &mut WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(out_id, t, in_id));

        let edge_range_manager = EdgeRangeManager::new(self.db);
        edge_range_manager.delete(&mut batch, out_id, t, update_datetime, in_id)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
        reversed_edge_range_manager.delete(&mut batch, in_id, t, update_datetime, out_id)?;

        let edge_property_manager = EdgePropertyManager::new(self.db);
        for item in edge_property_manager.iterate_for_owner(out_id, t, in_id)? {
            let ((edge_property_out_id, edge_property_t, edge_property_in_id, edge_property_name), _) = item?;
            edge_property_manager.delete(
                &mut batch,
                edge_property_out_id,
                &edge_property_t,
                edge_property_in_id,
                &edge_property_name[..],
            )?;
        }

        Ok(())
    }
}

pub struct EdgeRangeManager<'a> {
    pub db: &'a DB,
    pub cf: &'a ColumnFamily,
}

impl<'a> EdgeRangeManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgeRangeManager {
            cf: db.cf_handle("edge_ranges:v1").unwrap(),
            db,
        }
    }

    pub fn new_reversed(db: &'a DB) -> Self {
        EdgeRangeManager {
            cf: db.cf_handle("reversed_edge_ranges:v1").unwrap(),
            db,
        }
    }

    fn key(&self, first_id: Uuid, t: &models::Type, update_datetime: DateTime<Utc>, second_id: Uuid) -> Vec<u8> {
        build(&[
            Component::Uuid(first_id),
            Component::Type(t),
            Component::DateTime(update_datetime),
            Component::Uuid(second_id),
        ])
    }

    fn iterate(
        &'a self,
        iterator: DBIterator<'a>,
        prefix: Vec<u8>,
    ) -> impl Iterator<Item = Result<EdgeRangeItem>> + 'a {
        let filtered = iterator.take_while(move |item| -> bool {
            let (ref k, _) = *item;
            k.starts_with(&prefix)
        });

        filtered.map(move |item| -> Result<EdgeRangeItem> {
            let (k, _) = item;
            let mut cursor = Cursor::new(k);
            let first_id = read_uuid(&mut cursor);
            let t = read_type(&mut cursor);
            let update_datetime = read_datetime(&mut cursor);
            let second_id = read_uuid(&mut cursor);
            Ok((first_id, t, update_datetime, second_id))
        })
    }

    pub fn iterate_for_range(
        &'a self,
        id: Uuid,
        t: Option<&models::Type>,
        high: Option<DateTime<Utc>>,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>> + 'a>> {
        match t {
            Some(t) => {
                let high = high.unwrap_or_else(|| *MAX_DATETIME);
                let prefix = build(&[Component::Uuid(id), Component::Type(t)]);
                let low_key = build(&[Component::Uuid(id), Component::Type(t), Component::DateTime(high)]);
                let iterator = self
                    .db
                    .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward));
                Ok(Box::new(self.iterate(iterator, prefix)))
            }
            None => {
                let prefix = build(&[Component::Uuid(id)]);
                let iterator = self
                    .db
                    .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
                let mapped = self.iterate(iterator, prefix);

                if let Some(high) = high {
                    // We can filter out `update_datetime`s greater than
                    // `high` via key prefix filtering, so instead we handle
                    // it here - after the key has been deserialized.
                    let filtered = mapped.filter(move |item| {
                        if let Ok((_, _, update_datetime, _)) = *item {
                            update_datetime <= high
                        } else {
                            true
                        }
                    });

                    Ok(Box::new(filtered))
                } else {
                    Ok(Box::new(mapped))
                }
            }
        }
    }

    pub fn iterate_for_owner(&'a self, id: Uuid) -> impl Iterator<Item = Result<EdgeRangeItem>> + 'a {
        let prefix = build(&[Component::Uuid(id)]);
        let iterator = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iterator, prefix)
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) -> Result<()> {
        let key = self.key(first_id, t, update_datetime, second_id);
        batch.put_cf(self.cf, &key, &[]);
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(first_id, t, update_datetime, second_id));
        Ok(())
    }
}

pub struct VertexPropertyManager<'a> {
    pub db: &'a DB,
    pub cf: &'a ColumnFamily,
}

impl<'a> VertexPropertyManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        VertexPropertyManager {
            cf: db.cf_handle("vertex_properties:v1").unwrap(),
            db,
        }
    }

    fn key(&self, vertex_id: Uuid, name: &str) -> Vec<u8> {
        build(&[Component::Uuid(vertex_id), Component::UnsizedString(name)])
    }

    pub fn iterate_for_owner(
        &'a self,
        vertex_id: Uuid,
    ) -> Result<impl Iterator<Item = Result<OwnedPropertyItem>> + 'a> {
        let prefix = build(&[Component::Uuid(vertex_id)]);

        let iterator = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = iterator.take_while(move |item| -> bool {
            let (ref k, _) = *item;
            k.starts_with(&prefix)
        });

        Ok(filtered.map(move |item| -> Result<OwnedPropertyItem> {
            let (k, v) = item;
            let mut cursor = Cursor::new(k);
            let owner_id = read_uuid(&mut cursor);
            debug_assert_eq!(vertex_id, owner_id);
            let name = read_unsized_string(&mut cursor);
            let value = serde_json::from_slice(&v)?;
            Ok(((owner_id, name), value))
        }))
    }

    pub fn get(&self, vertex_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(vertex_id, name);

        match self.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(&self, batch: &mut WriteBatch, vertex_id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let key = self.key(vertex_id, name);
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(self.cf, &key, &value_json);
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, vertex_id: Uuid, name: &str) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(vertex_id, name));
        Ok(())
    }
}

pub struct EdgePropertyManager<'a> {
    pub db: &'a DB,
    pub cf: &'a ColumnFamily,
}

impl<'a> EdgePropertyManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgePropertyManager {
            cf: db.cf_handle("edge_properties:v1").unwrap(),
            db,
        }
    }

    fn key(&self, out_id: Uuid, t: &models::Type, in_id: Uuid, name: &str) -> Vec<u8> {
        build(&[
            Component::Uuid(out_id),
            Component::Type(t),
            Component::Uuid(in_id),
            Component::UnsizedString(name),
        ])
    }

    pub fn iterate_for_owner(
        &'a self,
        out_id: Uuid,
        t: &'a models::Type,
        in_id: Uuid,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgePropertyItem>> + 'a>> {
        let prefix = build(&[Component::Uuid(out_id), Component::Type(t), Component::Uuid(in_id)]);

        let iterator = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = iterator.take_while(move |item| -> bool {
            let (ref k, _) = *item;
            k.starts_with(&prefix)
        });

        let mapped = filtered.map(move |item| -> Result<EdgePropertyItem> {
            let (k, v) = item;
            let mut cursor = Cursor::new(k);

            let edge_property_out_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_property_out_id, out_id);

            let edge_property_t = read_type(&mut cursor);
            debug_assert_eq!(&edge_property_t, t);

            let edge_property_in_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_property_in_id, in_id);

            let edge_property_name = read_unsized_string(&mut cursor);

            let value = serde_json::from_slice(&v)?;
            Ok((
                (
                    edge_property_out_id,
                    edge_property_t,
                    edge_property_in_id,
                    edge_property_name,
                ),
                value,
            ))
        });

        Ok(Box::new(mapped))
    }

    pub fn get(&self, out_id: Uuid, t: &models::Type, in_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(out_id, t, in_id, name);

        match self.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        name: &str,
        value: &JsonValue,
    ) -> Result<()> {
        let key = self.key(out_id, t, in_id, name);
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(self.cf, &key, &value_json);
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        name: &str,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(out_id, t, in_id, name));
        Ok(())
    }
}
