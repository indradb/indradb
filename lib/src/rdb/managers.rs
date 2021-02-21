use std::io::Cursor;
use std::ops::Deref;
use std::u8;

use super::super::bytes::*;
use crate::errors::Result;
use crate::models;
use crate::tree;

use chrono::offset::Utc;
use chrono::DateTime;
use rocksdb::{ColumnFamily, DBIterator, Direction, IteratorMode, DB};
use serde_json::Value as JsonValue;
use uuid::Uuid;

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

    fn iterate(&'a self, iterator: DBIterator<'a>) -> impl Iterator<Item = Result<tree::VertexItem>> + 'a {
        iterator.map(|item| -> Result<tree::VertexItem> {
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

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}

impl<'a> tree::VertexManager<'a> for VertexManager<'a> {
    type WriteBatch = rocksdb::WriteBatch;

    fn exists(&self, id: Uuid) -> Result<bool> {
        Ok(self.db.get_cf(self.cf, &self.key(id))?.is_some())
    }

    fn get(&self, id: Uuid) -> Result<Option<models::Type>> {
        match self.db.get_cf(self.cf, &self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_type(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    fn iterate_for_range(&'a self, id: Uuid) -> Result<Box<dyn Iterator<Item = Result<tree::VertexItem>> + 'a>> {
        let low_key = build(&[Component::Uuid(id)]);
        let iter = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward));
        Ok(Box::new(self.iterate(iter)))
    }

    fn create(&self, batch: &mut rocksdb::WriteBatch, vertex: &models::Vertex) -> Result<()> {
        let key = self.key(vertex.id);
        batch.put_cf(self.cf, &key, &build(&[Component::Type(&vertex.t)]));
        Ok(())
    }

    fn delete(&self, mut batch: &mut rocksdb::WriteBatch, id: Uuid) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(id));
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

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}

impl<'a> tree::EdgeManager<'a> for EdgeManager<'a> {
    type WriteBatch = rocksdb::WriteBatch;
    
    fn get(&self, out_id: Uuid, t: &models::Type, in_id: Uuid) -> Result<Option<DateTime<Utc>>> {
        match self.db.get_cf(self.cf, &self.key(out_id, t, in_id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_datetime(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    fn set(
        &self,
        mut batch: &mut rocksdb::WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        new_update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        let key = self.key(out_id, t, in_id);
        batch.put_cf(self.cf, &key, &build(&[Component::DateTime(new_update_datetime)]));
        Ok(())
    }

    fn delete(
        &self,
        mut batch: &mut rocksdb::WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(out_id, t, in_id));
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
    ) -> impl Iterator<Item = Result<tree::EdgeRangeItem>> + 'a {
        let filtered = iterator.take_while(move |item| -> bool {
            let (ref k, _) = *item;
            k.starts_with(&prefix)
        });

        filtered.map(move |item| -> Result<tree::EdgeRangeItem> {
            let (k, _) = item;
            let mut cursor = Cursor::new(k);
            let first_id = read_uuid(&mut cursor);
            let t = read_type(&mut cursor);
            let update_datetime = read_datetime(&mut cursor);
            let second_id = read_uuid(&mut cursor);
            Ok((first_id, t, update_datetime, second_id))
        })
    }

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}

impl<'a> tree::EdgeRangeManager<'a> for EdgeRangeManager<'a> {
    type WriteBatch = rocksdb::WriteBatch;

    fn iterate_for_range(
        &'a self,
        id: Uuid,
        t: Option<&models::Type>,
        high: Option<DateTime<Utc>>,
    ) -> Result<Box<dyn Iterator<Item = Result<tree::EdgeRangeItem>> + 'a>> {
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

    fn iterate_for_owner(&'a self, id: Uuid) -> Result<Box<dyn Iterator<Item = Result<tree::EdgeRangeItem>> + 'a>> {
        let prefix = build(&[Component::Uuid(id)]);
        let iterator = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
        Ok(Box::new(self.iterate(iterator, prefix)))
    }

    fn set(
        &self,
        batch: &mut rocksdb::WriteBatch,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) -> Result<()> {
        let key = self.key(first_id, t, update_datetime, second_id);
        batch.put_cf(self.cf, &key, &[]);
        Ok(())
    }

    fn delete(
        &self,
        batch: &mut rocksdb::WriteBatch,
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

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}

impl<'a> tree::VertexPropertyManager<'a> for VertexPropertyManager<'a> {
    type WriteBatch = rocksdb::WriteBatch;

    fn iterate_for_owner(
        &'a self,
        vertex_id: Uuid,
    ) -> Result<Box<dyn Iterator<Item = Result<tree::OwnedPropertyItem>> + 'a>> {
        let prefix = build(&[Component::Uuid(vertex_id)]);

        let iterator = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = iterator.take_while(move |item| -> bool {
            let (ref k, _) = *item;
            k.starts_with(&prefix)
        });

        Ok(Box::new(filtered.map(move |item| -> Result<tree::OwnedPropertyItem> {
            let (k, v) = item;
            let mut cursor = Cursor::new(k);
            let owner_id = read_uuid(&mut cursor);
            debug_assert_eq!(vertex_id, owner_id);
            let name = read_unsized_string(&mut cursor);
            let value = serde_json::from_slice(&v)?;
            Ok(((owner_id, name), value))
        })))
    }

    fn get(&self, vertex_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(vertex_id, name);

        match self.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    fn set(&self, batch: &mut rocksdb::WriteBatch, vertex_id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let key = self.key(vertex_id, name);
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(self.cf, &key, &value_json);
        Ok(())
    }

    fn delete(&self, batch: &mut rocksdb::WriteBatch, vertex_id: Uuid, name: &str) -> Result<()> {
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

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}

impl<'a> tree::EdgePropertyManager<'a> for EdgePropertyManager<'a> {
    type WriteBatch = rocksdb::WriteBatch;

    fn iterate_for_owner(
        &'a self,
        out_id: Uuid,
        t: &'a models::Type,
        in_id: Uuid,
    ) -> Result<Box<dyn Iterator<Item = Result<tree::EdgePropertyItem>> + 'a>> {
        let prefix = build(&[Component::Uuid(out_id), Component::Type(t), Component::Uuid(in_id)]);

        let iterator = self
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = iterator.take_while(move |item| -> bool {
            let (ref k, _) = *item;
            k.starts_with(&prefix)
        });

        let mapped = filtered.map(move |item| -> Result<tree::EdgePropertyItem> {
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

    fn get(&self, out_id: Uuid, t: &models::Type, in_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(out_id, t, in_id, name);

        match self.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    fn set(
        &self,
        batch: &mut rocksdb::WriteBatch,
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

    fn delete(
        &self,
        batch: &mut rocksdb::WriteBatch,
        out_id: Uuid,
        t: &models::Type,
        in_id: Uuid,
        name: &str,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(out_id, t, in_id, name));
        Ok(())
    }
}
