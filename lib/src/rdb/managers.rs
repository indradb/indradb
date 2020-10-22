use std::io::Cursor;
use std::ops::Deref;
use std::u8;

use super::super::bytes::*;
use crate::errors::Result;
use crate::models;

use rocksdb::{ColumnFamily, DBIterator, Direction, IteratorMode, WriteBatch, DB};
use serde_json::Value as JsonValue;

pub type OwnedPropertyItem = ((u64, String), JsonValue);
pub type VertexItem = (u64, models::Type);
pub type EdgeRangeItem = (u64, models::Type, u64);
pub type EdgePropertyItem = ((u64, models::Type, u64, String), JsonValue);

pub struct MetaManager<'a> {
    pub db: &'a DB,
    pub cf: &'a ColumnFamily,
}

impl<'a> MetaManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        MetaManager {
            cf: db.cf_handle("meta:v1").unwrap(),
            db,
        }
    }

    pub fn get_last_id(&self) -> Result<u64> {
        match self.db.get_cf(self.cf, "last_id")? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(read_id(&mut cursor))
            }
            None => Ok(0),
        }
    }

    pub fn set_last_id(&self, batch: &mut WriteBatch, id: u64) -> Result<()> {
        batch.put_cf(self.cf, "last_id", &build(&[Component::Id(id)]));
        Ok(())
    }

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}

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

    fn key(&self, id: u64) -> Vec<u8> {
        build(&[Component::Id(id)])
    }

    pub fn exists(&self, id: u64) -> Result<bool> {
        Ok(self.db.get_cf(self.cf, &self.key(id))?.is_some())
    }

    pub fn get(&self, id: u64) -> Result<Option<models::Type>> {
        match self.db.get_cf(self.cf, &self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_type(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    fn iterate(&'a self, iterator: DBIterator<'a>) -> Result<impl Iterator<Item = Result<VertexItem>> + 'a> {
        Ok(iterator.map(|item| -> Result<VertexItem> {
            let (k, v) = item;

            let id = {
                debug_assert_eq!(k.len(), 8);
                let mut cursor = Cursor::new(k);
                read_id(&mut cursor)
            };

            let mut cursor = Cursor::new(v);
            let t = read_type(&mut cursor);
            Ok((id, t))
        }))
    }

    pub fn iterate_for_range(&'a self, id: u64) -> Result<impl Iterator<Item = Result<VertexItem>> + 'a> {
        let low_key = build(&[Component::Id(id)]);
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

    pub fn delete(&self, mut batch: &mut WriteBatch, id: u64) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(id));

        let vertex_property_manager = VertexPropertyManager::new(self.db);
        for item in vertex_property_manager.iterate_for_owner(id)? {
            let ((vertex_property_owner_id, vertex_property_name), _) = item?;
            vertex_property_manager.delete(&mut batch, vertex_property_owner_id, &vertex_property_name[..])?;
        }

        let edge_manager = EdgeManager::new(self.db);

        {
            let edge_range_manager = EdgeRangeManager::new(self.db);
            for item in edge_range_manager.iterate_for_range(id, None)? {
                let (edge_range_out_id, edge_range_t, edge_range_in_id) = item?;
                debug_assert_eq!(edge_range_out_id, id);
                edge_manager.delete(&mut batch, edge_range_out_id, &edge_range_t, edge_range_in_id)?;
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
            for item in reversed_edge_range_manager.iterate_for_range(id, None)? {
                let (reversed_edge_range_in_id, reversed_edge_range_t, reversed_edge_range_out_id) = item?;
                debug_assert_eq!(reversed_edge_range_in_id, id);
                edge_manager.delete(
                    &mut batch,
                    reversed_edge_range_out_id,
                    &reversed_edge_range_t,
                    reversed_edge_range_in_id,
                )?;
            }
        }

        Ok(())
    }

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}

pub struct EdgeManager<'a> {
    pub db: &'a DB,
}

impl<'a> EdgeManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgeManager { db }
    }

    pub fn set(&self, mut batch: &mut WriteBatch, out_id: u64, t: &models::Type, in_id: u64) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db);
        edge_range_manager.set(&mut batch, out_id, t, in_id)?;
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
        reversed_edge_range_manager.set(&mut batch, in_id, t, out_id)?;
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, out_id: u64, t: &models::Type, in_id: u64) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db);
        edge_range_manager.delete(&mut batch, out_id, t, in_id)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
        reversed_edge_range_manager.delete(&mut batch, in_id, t, out_id)?;

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

    pub fn compact(&self) {
        EdgeRangeManager::new(self.db).compact();
        EdgeRangeManager::new_reversed(self.db).compact();
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

    fn key(&self, first_id: u64, t: &models::Type, second_id: u64) -> Vec<u8> {
        build(&[Component::Id(first_id), Component::Type(t), Component::Id(second_id)])
    }

    fn iterate(
        &'a self,
        iterator: DBIterator<'a>,
        prefix: Vec<u8>,
    ) -> Result<impl Iterator<Item = Result<EdgeRangeItem>> + 'a> {
        let filtered = iterator.take_while(move |item| -> bool {
            let (ref k, _) = *item;
            k.starts_with(&prefix)
        });

        Ok(filtered.map(move |item| -> Result<EdgeRangeItem> {
            let (k, _) = item;
            let mut cursor = Cursor::new(k);
            let first_id = read_id(&mut cursor);
            let t = read_type(&mut cursor);
            let second_id = read_id(&mut cursor);
            Ok((first_id, t, second_id))
        }))
    }

    pub fn iterate_for_range(
        &'a self,
        id: u64,
        t: Option<&models::Type>,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>> + 'a>> {
        match t {
            Some(t) => {
                let prefix = build(&[Component::Id(id), Component::Type(t)]);
                let low_key = build(&[Component::Id(id), Component::Type(t)]);
                let iterator = self
                    .db
                    .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward));
                Ok(Box::new(self.iterate(iterator, prefix)?))
            }
            None => {
                let prefix = build(&[Component::Id(id)]);
                let iterator = self
                    .db
                    .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
                let mapped = self.iterate(iterator, prefix)?;

                Ok(Box::new(mapped))
            }
        }
    }

    pub fn exists(&self, first_id: u64, t: &models::Type, second_id: u64) -> Result<bool> {
        Ok(self.db.get_cf(self.cf, &self.key(first_id, t, second_id))?.is_some())
    }

    pub fn set(&self, batch: &mut WriteBatch, first_id: u64, t: &models::Type, second_id: u64) -> Result<()> {
        let key = self.key(first_id, t, second_id);
        batch.put_cf(self.cf, &key, &[]);
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, first_id: u64, t: &models::Type, second_id: u64) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(first_id, t, second_id));
        Ok(())
    }

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
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

    fn key(&self, vertex_id: u64, name: &str) -> Vec<u8> {
        build(&[Component::Id(vertex_id), Component::UnsizedString(name)])
    }

    pub fn iterate_for_owner(&'a self, vertex_id: u64) -> Result<impl Iterator<Item = Result<OwnedPropertyItem>> + 'a> {
        let prefix = build(&[Component::Id(vertex_id)]);

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
            let owner_id = read_id(&mut cursor);
            debug_assert_eq!(vertex_id, owner_id);
            let name = read_unsized_string(&mut cursor);
            let value = serde_json::from_slice(&v)?;
            Ok(((owner_id, name), value))
        }))
    }

    pub fn get(&self, vertex_id: u64, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(vertex_id, name);

        match self.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(&self, batch: &mut WriteBatch, vertex_id: u64, name: &str, value: &JsonValue) -> Result<()> {
        let key = self.key(vertex_id, name);
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(self.cf, &key, &value_json);
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, vertex_id: u64, name: &str) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(vertex_id, name));
        Ok(())
    }

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
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

    fn key(&self, out_id: u64, t: &models::Type, in_id: u64, name: &str) -> Vec<u8> {
        build(&[
            Component::Id(out_id),
            Component::Type(t),
            Component::Id(in_id),
            Component::UnsizedString(name),
        ])
    }

    pub fn iterate_for_owner(
        &'a self,
        out_id: u64,
        t: &'a models::Type,
        in_id: u64,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgePropertyItem>> + 'a>> {
        let prefix = build(&[Component::Id(out_id), Component::Type(t), Component::Id(in_id)]);

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

            let edge_property_out_id = read_id(&mut cursor);
            debug_assert_eq!(edge_property_out_id, out_id);

            let edge_property_t = read_type(&mut cursor);
            debug_assert_eq!(&edge_property_t, t);

            let edge_property_in_id = read_id(&mut cursor);
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

    pub fn get(&self, out_id: u64, t: &models::Type, in_id: u64, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(out_id, t, in_id, name);

        match self.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        out_id: u64,
        t: &models::Type,
        in_id: u64,
        name: &str,
        value: &JsonValue,
    ) -> Result<()> {
        let key = self.key(out_id, t, in_id, name);
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(self.cf, &key, &value_json);
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, out_id: u64, t: &models::Type, in_id: u64, name: &str) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(out_id, t, in_id, name));
        Ok(())
    }

    pub fn compact(&self) {
        self.db.compact_range_cf::<&[u8], &[u8]>(self.cf, None, None);
    }
}
