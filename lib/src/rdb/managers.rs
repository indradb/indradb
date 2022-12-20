use std::collections::HashSet;
use std::io::Cursor;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::u8;

use crate::errors::Result;
use crate::models;
use crate::util;

use chrono::offset::Utc;
use chrono::DateTime;
use rocksdb::{ColumnFamily, DBIterator, Direction, IteratorMode, WriteBatch, DB};

pub type OwnedPropertyItem = ((u64, models::Identifier), models::Json);
pub type VertexItem = (u64, models::Identifier);
pub type EdgeRangeItem = (u64, models::Identifier, DateTime<Utc>, u64);
pub type EdgePropertyItem = ((u64, models::Identifier, u64, models::Identifier), models::Json);
pub type VertexPropertyValueKey = (models::Identifier, u64, u64);
pub type EdgePropertyValueKey = (models::Identifier, u64, (u64, models::Identifier, u64));

fn take_with_prefix(
    iterator: DBIterator<'_>,
    prefix: Vec<u8>,
) -> impl Iterator<Item = StdResult<(Box<[u8]>, Box<[u8]>), rocksdb::Error>> + '_ {
    iterator.take_while(move |item| -> bool {
        if let Ok((ref k, _)) = *item {
            k.starts_with(&prefix)
        } else {
            true
        }
    })
}

#[derive(Copy, Clone)]
pub(crate) struct DBRef<'a> {
    pub db: &'a DB,
    pub indexed_properties: &'a HashSet<models::Identifier>,
}

impl<'a> DBRef<'a> {
    pub(crate) fn new(db: &'a DB, indexed_properties: &'a HashSet<models::Identifier>) -> Self {
        DBRef { db, indexed_properties }
    }
}

pub(crate) struct VertexManager<'a> {
    db_ref: DBRef<'a>,
    cf: &'a ColumnFamily,
}

impl<'a> VertexManager<'a> {
    pub fn new(db_ref: DBRef<'a>) -> Self {
        VertexManager {
            db_ref,
            cf: db_ref.db.cf_handle("vertices:v1").unwrap(),
        }
    }

    fn key(&self, id: u64) -> Vec<u8> {
        util::build(&[util::Component::U64(id)])
    }

    pub fn exists(&self, id: u64) -> Result<bool> {
        Ok(self.db_ref.db.get_cf(self.cf, &self.key(id))?.is_some())
    }

    pub fn get(&self, id: u64) -> Result<Option<models::Identifier>> {
        match self.db_ref.db.get_cf(self.cf, &self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(util::read_identifier(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    pub fn iterate_for_range(&'a self, id: u64) -> impl Iterator<Item = Result<VertexItem>> + 'a {
        let low_key = util::build(&[util::Component::U64(id)]);
        let iter = self
            .db_ref
            .db
            .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward));
        iter.map(|item| -> Result<VertexItem> {
            let (k, v) = item?;

            let id = {
                debug_assert_eq!(k.len(), 8);
                let mut cursor = Cursor::new(k);
                util::read_u64(&mut cursor)
            };

            let mut cursor = Cursor::new(v);
            let t = util::read_identifier(&mut cursor);
            Ok((id, t))
        })
    }

    pub fn create(&self, batch: &mut WriteBatch, vertex: &models::Vertex) -> Result<()> {
        let key = self.key(vertex.id);
        batch.put_cf(self.cf, &key, &util::build(&[util::Component::Identifier(&vertex.t)]));
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, id: u64) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(id));

        let vertex_property_manager = VertexPropertyManager::new(self.db_ref);
        for item in vertex_property_manager.iterate_for_owner(id)? {
            let ((vertex_property_owner_id, vertex_property_name), _) = item?;
            vertex_property_manager.delete(batch, vertex_property_owner_id, &vertex_property_name)?;
        }

        let edge_manager = EdgeManager::new(self.db_ref);

        {
            let edge_range_manager = EdgeRangeManager::new(self.db_ref);
            for item in edge_range_manager.iterate_for_range(id, None, None)? {
                let (edge_range_out_id, edge_range_t, edge_range_update_datetime, edge_range_in_id) = item?;
                debug_assert_eq!(edge_range_out_id, id);
                edge_manager.delete(
                    batch,
                    edge_range_out_id,
                    &edge_range_t,
                    edge_range_in_id,
                    edge_range_update_datetime,
                )?;
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db_ref);
            for item in reversed_edge_range_manager.iterate_for_range(id, None, None)? {
                let (
                    reversed_edge_range_in_id,
                    reversed_edge_range_t,
                    reversed_edge_range_update_datetime,
                    reversed_edge_range_out_id,
                ) = item?;
                debug_assert_eq!(reversed_edge_range_in_id, id);
                edge_manager.delete(
                    batch,
                    reversed_edge_range_out_id,
                    &reversed_edge_range_t,
                    reversed_edge_range_in_id,
                    reversed_edge_range_update_datetime,
                )?;
            }
        }

        Ok(())
    }

    pub fn compact(&self) {
        self.db_ref
            .db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct EdgeManager<'a> {
    db_ref: DBRef<'a>,
    cf: &'a ColumnFamily,
}

impl<'a> EdgeManager<'a> {
    pub fn new(db_ref: DBRef<'a>) -> Self {
        EdgeManager {
            db_ref,
            cf: db_ref.db.cf_handle("edges:v1").unwrap(),
        }
    }

    fn key(&self, out_id: u64, t: &models::Identifier, in_id: u64) -> Vec<u8> {
        util::build(&[
            util::Component::U64(out_id),
            util::Component::Identifier(t),
            util::Component::U64(in_id),
        ])
    }

    pub fn get(&self, out_id: u64, t: &models::Identifier, in_id: u64) -> Result<Option<DateTime<Utc>>> {
        match self.db_ref.db.get_cf(self.cf, &self.key(out_id, t, in_id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(util::read_datetime(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
        new_update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db_ref);
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db_ref);

        if let Some(update_datetime) = self.get(out_id, t, in_id)? {
            edge_range_manager.delete(batch, out_id, t, update_datetime, in_id)?;
            reversed_edge_range_manager.delete(batch, in_id, t, update_datetime, out_id)?;
        }

        let key = self.key(out_id, t, in_id);
        batch.put_cf(
            self.cf,
            &key,
            &util::build(&[util::Component::DateTime(new_update_datetime)]),
        );
        edge_range_manager.set(batch, out_id, t, new_update_datetime, in_id)?;
        reversed_edge_range_manager.set(batch, in_id, t, new_update_datetime, out_id)?;
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
        update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(out_id, t, in_id));

        let edge_range_manager = EdgeRangeManager::new(self.db_ref);
        edge_range_manager.delete(batch, out_id, t, update_datetime, in_id)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db_ref);
        reversed_edge_range_manager.delete(batch, in_id, t, update_datetime, out_id)?;

        let edge_property_manager = EdgePropertyManager::new(self.db_ref);
        for item in edge_property_manager.iterate_for_owner(out_id, t, in_id)? {
            let ((edge_property_out_id, edge_property_t, edge_property_in_id, edge_property_name), _) = item?;
            edge_property_manager.delete(
                batch,
                edge_property_out_id,
                &edge_property_t,
                edge_property_in_id,
                &edge_property_name,
            )?;
        }

        Ok(())
    }

    pub fn compact(&self) {
        self.db_ref
            .db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct EdgeRangeManager<'a> {
    db_ref: DBRef<'a>,
    cf: &'a ColumnFamily,
}

impl<'a> EdgeRangeManager<'a> {
    pub fn new(db_ref: DBRef<'a>) -> Self {
        EdgeRangeManager {
            db_ref,
            cf: db_ref.db.cf_handle("edge_ranges:v1").unwrap(),
        }
    }

    pub fn new_reversed(db_ref: DBRef<'a>) -> Self {
        EdgeRangeManager {
            db_ref,
            cf: db_ref.db.cf_handle("reversed_edge_ranges:v1").unwrap(),
        }
    }

    fn key(&self, first_id: u64, t: &models::Identifier, update_datetime: DateTime<Utc>, second_id: u64) -> Vec<u8> {
        util::build(&[
            util::Component::U64(first_id),
            util::Component::Identifier(t),
            util::Component::DateTime(update_datetime),
            util::Component::U64(second_id),
        ])
    }

    fn iterate<I>(&'a self, iterator: I) -> impl Iterator<Item = Result<EdgeRangeItem>> + 'a
    where
        I: Iterator<Item = StdResult<(Box<[u8]>, Box<[u8]>), rocksdb::Error>> + 'a,
    {
        iterator.map(move |item| -> Result<EdgeRangeItem> {
            let (k, _) = item?;
            let mut cursor = Cursor::new(k);
            let first_id = util::read_u64(&mut cursor);
            let t = util::read_identifier(&mut cursor);
            let update_datetime = util::read_datetime(&mut cursor);
            let second_id = util::read_u64(&mut cursor);
            Ok((first_id, t, update_datetime, second_id))
        })
    }

    pub fn iterate_for_range(
        &'a self,
        id: u64,
        t: Option<&models::Identifier>,
        high: Option<DateTime<Utc>>,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>> + 'a>> {
        match t {
            Some(t) => {
                let high = high.unwrap_or(*util::MAX_DATETIME);
                let prefix = util::build(&[util::Component::U64(id), util::Component::Identifier(t)]);
                let low_key = util::build(&[
                    util::Component::U64(id),
                    util::Component::Identifier(t),
                    util::Component::DateTime(high),
                ]);
                let iterator = self
                    .db_ref
                    .db
                    .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward));
                let iterator = take_with_prefix(iterator, prefix);
                Ok(Box::new(self.iterate(iterator)))
            }
            None => {
                let prefix = util::build(&[util::Component::U64(id)]);
                let iterator = self
                    .db_ref
                    .db
                    .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
                let iterator = take_with_prefix(iterator, prefix);
                let mapped = self.iterate(iterator);

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

    pub fn iterate_for_all(&'a self) -> impl Iterator<Item = Result<EdgeRangeItem>> + 'a {
        let iterator = self.db_ref.db.iterator_cf(self.cf, IteratorMode::Start);
        self.iterate(iterator)
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        first_id: u64,
        t: &models::Identifier,
        update_datetime: DateTime<Utc>,
        second_id: u64,
    ) -> Result<()> {
        let key = self.key(first_id, t, update_datetime, second_id);
        batch.put_cf(self.cf, &key, &[]);
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        first_id: u64,
        t: &models::Identifier,
        update_datetime: DateTime<Utc>,
        second_id: u64,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(first_id, t, update_datetime, second_id));
        Ok(())
    }

    pub fn compact(&self) {
        self.db_ref
            .db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct VertexPropertyManager<'a> {
    db_ref: DBRef<'a>,
    cf: &'a ColumnFamily,
}

impl<'a> VertexPropertyManager<'a> {
    pub fn new(db_ref: DBRef<'a>) -> Self {
        VertexPropertyManager {
            db_ref,
            cf: db_ref.db.cf_handle("vertex_properties:v1").unwrap(),
        }
    }

    fn key(&self, vertex_id: u64, name: &models::Identifier) -> Vec<u8> {
        util::build(&[
            util::Component::U64(vertex_id),
            util::Component::FixedLengthString(&name.0),
        ])
    }

    pub fn iterate_for_owner(&'a self, vertex_id: u64) -> Result<impl Iterator<Item = Result<OwnedPropertyItem>> + 'a> {
        let prefix = util::build(&[util::Component::U64(vertex_id)]);

        let iterator = self
            .db_ref
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = take_with_prefix(iterator, prefix);

        Ok(filtered.map(move |item| -> Result<OwnedPropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);
            let owner_id = util::read_u64(&mut cursor);
            debug_assert_eq!(vertex_id, owner_id);
            let name_str = util::read_fixed_length_string(&mut cursor);
            let name = unsafe { models::Identifier::new_unchecked(name_str) };
            let value = serde_json::from_slice(&v)?;
            Ok(((owner_id, name), value))
        }))
    }

    pub fn get(&self, vertex_id: u64, name: &models::Identifier) -> Result<Option<models::Json>> {
        let key = self.key(vertex_id, name);

        match self.db_ref.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        vertex_id: u64,
        name: &models::Identifier,
        value: &models::Json,
    ) -> Result<()> {
        let is_indexed = self.db_ref.indexed_properties.contains(name);
        let key = self.key(vertex_id, name);
        if is_indexed {
            self.delete(batch, vertex_id, name)?;
        }
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(self.cf, &key, &value_json);
        if is_indexed {
            let vertex_property_value_manager = VertexPropertyValueManager::new(self.db_ref);
            vertex_property_value_manager.set(batch, vertex_id, name, value);
        }
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, vertex_id: u64, name: &models::Identifier) -> Result<()> {
        if self.db_ref.indexed_properties.contains(name) {
            if let Some(value) = self.get(vertex_id, name)? {
                let vertex_property_value_manager = VertexPropertyValueManager::new(self.db_ref);
                vertex_property_value_manager.delete(batch, vertex_id, name, &value);
            }
        }
        batch.delete_cf(self.cf, &self.key(vertex_id, name));
        Ok(())
    }

    pub fn compact(&self) {
        self.db_ref
            .db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct EdgePropertyManager<'a> {
    db_ref: DBRef<'a>,
    cf: &'a ColumnFamily,
}

impl<'a> EdgePropertyManager<'a> {
    pub fn new(db_ref: DBRef<'a>) -> Self {
        EdgePropertyManager {
            db_ref,
            cf: db_ref.db.cf_handle("edge_properties:v1").unwrap(),
        }
    }

    fn key(&self, out_id: u64, t: &models::Identifier, in_id: u64, name: &models::Identifier) -> Vec<u8> {
        util::build(&[
            util::Component::U64(out_id),
            util::Component::Identifier(t),
            util::Component::U64(in_id),
            util::Component::FixedLengthString(&name.0),
        ])
    }

    pub fn iterate_for_owner(
        &'a self,
        out_id: u64,
        t: &'a models::Identifier,
        in_id: u64,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgePropertyItem>> + 'a>> {
        let prefix = util::build(&[
            util::Component::U64(out_id),
            util::Component::Identifier(t),
            util::Component::U64(in_id),
        ]);

        let iterator = self
            .db_ref
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = take_with_prefix(iterator, prefix);

        let mapped = filtered.map(move |item| -> Result<EdgePropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);

            let edge_property_out_id = util::read_u64(&mut cursor);
            debug_assert_eq!(edge_property_out_id, out_id);

            let edge_property_t = util::read_identifier(&mut cursor);
            debug_assert_eq!(&edge_property_t, t);

            let edge_property_in_id = util::read_u64(&mut cursor);
            debug_assert_eq!(edge_property_in_id, in_id);

            let edge_property_name_str = util::read_fixed_length_string(&mut cursor);
            let edge_property_name = unsafe { models::Identifier::new_unchecked(edge_property_name_str) };

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

    pub fn get(
        &self,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
        name: &models::Identifier,
    ) -> Result<Option<models::Json>> {
        let key = self.key(out_id, t, in_id, name);

        match self.db_ref.db.get_cf(self.cf, &key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
        name: &models::Identifier,
        value: &models::Json,
    ) -> Result<()> {
        let is_indexed = self.db_ref.indexed_properties.contains(name);
        let key = self.key(out_id, t, in_id, name);
        if is_indexed {
            self.delete(batch, out_id, t, in_id, name)?;
        }
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(self.cf, &key, &value_json);
        if is_indexed {
            let edge_property_value_manager = EdgePropertyValueManager::new(self.db_ref);
            edge_property_value_manager.set(batch, out_id, t, in_id, name, value);
        }
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
        name: &models::Identifier,
    ) -> Result<()> {
        if self.db_ref.indexed_properties.contains(name) {
            if let Some(value) = self.get(out_id, t, in_id, name)? {
                let edge_property_value_manager = EdgePropertyValueManager::new(self.db_ref);
                edge_property_value_manager.delete(batch, out_id, t, in_id, name, &value);
            }
        }
        batch.delete_cf(self.cf, &self.key(out_id, t, in_id, name));
        Ok(())
    }

    pub fn compact(&self) {
        self.db_ref
            .db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct VertexPropertyValueManager<'a> {
    db_ref: DBRef<'a>,
    cf: &'a ColumnFamily,
}

impl<'a> VertexPropertyValueManager<'a> {
    pub fn new(db_ref: DBRef<'a>) -> Self {
        VertexPropertyValueManager {
            db_ref,
            cf: db_ref.db.cf_handle("vertex_property_values:v1").unwrap(),
        }
    }

    fn key(&self, property_name: &models::Identifier, property_value: &models::Json, vertex_id: u64) -> Vec<u8> {
        util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
            util::Component::U64(vertex_id),
        ])
    }

    fn iterate(
        &'a self,
        iterator: DBIterator<'a>,
        prefix: Vec<u8>,
    ) -> impl Iterator<Item = Result<VertexPropertyValueKey>> + 'a {
        let filtered = take_with_prefix(iterator, prefix);

        filtered.map(move |item| -> Result<VertexPropertyValueKey> {
            let (k, _) = item?;
            let mut cursor = Cursor::new(k);
            let name = util::read_identifier(&mut cursor);
            let value_hash = util::read_u64(&mut cursor);
            let vertex_id = util::read_u64(&mut cursor);
            Ok((name, value_hash, vertex_id))
        })
    }

    pub fn iterate_for_name(
        &'a self,
        property_name: &models::Identifier,
    ) -> impl Iterator<Item = Result<VertexPropertyValueKey>> + 'a {
        let prefix = util::build(&[util::Component::Identifier(property_name)]);
        let iter = self
            .db_ref
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn iterate_for_value(
        &'a self,
        property_name: &models::Identifier,
        property_value: &models::Json,
    ) -> impl Iterator<Item = Result<VertexPropertyValueKey>> + 'a {
        let prefix = util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
        ]);
        let iter = self
            .db_ref
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        vertex_id: u64,
        property_name: &models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, vertex_id);
        batch.put_cf(self.cf, key, &[]);
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        vertex_id: u64,
        property_name: &models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, vertex_id);
        batch.delete_cf(self.cf, key);
    }

    pub fn compact(&self) {
        self.db_ref
            .db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct EdgePropertyValueManager<'a> {
    db_ref: DBRef<'a>,
    cf: &'a ColumnFamily,
}

impl<'a> EdgePropertyValueManager<'a> {
    pub fn new(db_ref: DBRef<'a>) -> Self {
        EdgePropertyValueManager {
            db_ref,
            cf: db_ref.db.cf_handle("edge_property_values:v1").unwrap(),
        }
    }

    fn key(
        &self,
        property_name: &models::Identifier,
        property_value: &models::Json,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
    ) -> Vec<u8> {
        util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
            util::Component::U64(out_id),
            util::Component::Identifier(t),
            util::Component::U64(in_id),
        ])
    }

    fn iterate(
        &'a self,
        iterator: DBIterator<'a>,
        prefix: Vec<u8>,
    ) -> impl Iterator<Item = Result<EdgePropertyValueKey>> + 'a {
        let filtered = take_with_prefix(iterator, prefix);

        filtered.map(move |item| -> Result<EdgePropertyValueKey> {
            let (k, _) = item?;
            let mut cursor = Cursor::new(k);
            let name = util::read_identifier(&mut cursor);
            let value_hash = util::read_u64(&mut cursor);
            let out_id = util::read_u64(&mut cursor);
            let t = util::read_identifier(&mut cursor);
            let in_id = util::read_u64(&mut cursor);
            Ok((name, value_hash, (out_id, t, in_id)))
        })
    }

    pub fn iterate_for_name(
        &'a self,
        property_name: &models::Identifier,
    ) -> impl Iterator<Item = Result<EdgePropertyValueKey>> + 'a {
        let prefix = util::build(&[util::Component::Identifier(property_name)]);
        let iter = self
            .db_ref
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn iterate_for_value(
        &'a self,
        property_name: &models::Identifier,
        property_value: &models::Json,
    ) -> impl Iterator<Item = Result<EdgePropertyValueKey>> + 'a {
        let prefix = util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
        ]);
        let iter = self
            .db_ref
            .db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
        property_name: &models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, out_id, t, in_id);
        batch.put_cf(self.cf, key, &[]);
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        out_id: u64,
        t: &models::Identifier,
        in_id: u64,
        property_name: &models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, out_id, t, in_id);
        batch.delete_cf(self.cf, key);
    }

    pub fn compact(&self) {
        self.db_ref
            .db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct MetadataManager<'a> {
    db: &'a DB,
    cf: &'a ColumnFamily,
}

impl<'a> MetadataManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        MetadataManager {
            db,
            cf: db.cf_handle("metadata:v1").unwrap(),
        }
    }

    pub fn get_indexed_properties(&self) -> Result<HashSet<models::Identifier>> {
        match self.db.get_cf(self.cf, "indexed_properties")? {
            Some(value_bytes) => Ok(bincode::deserialize(&value_bytes)?),
            None => Ok(HashSet::default()),
        }
    }

    pub fn set_indexed_properties(&self, batch: &mut WriteBatch, indices: &HashSet<models::Identifier>) -> Result<()> {
        let value_bytes = bincode::serialize(&indices)?;
        batch.put_cf(self.cf, "indexed_properties", &value_bytes);
        Ok(())
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}
