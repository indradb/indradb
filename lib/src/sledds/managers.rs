use std::io::Cursor;
use std::ops::Deref;
use std::u8;

use super::super::bytes::*;
use crate::errors::Result;
use crate::models;

use chrono::offset::Utc;
use chrono::DateTime;
use serde_json::Value as JsonValue;
use sled::Result as SledResult;
use sled::{Batch, Db, IVec, Iter as DbIterator};
use uuid::Uuid;

pub type OwnedPropertyItem = ((Uuid, String), JsonValue);
pub type VertexItem = (Uuid, models::Type);
pub type EdgeRangeItem = (Uuid, models::Type, DateTime<Utc>, Uuid);
pub type EdgePropertyItem = ((Uuid, models::Type, Uuid, String), JsonValue);

fn take_while_prefixed(iterator: DbIterator, prefix: Vec<u8>) -> impl Iterator<Item = SledResult<(IVec, IVec)>> {
    iterator.take_while(move |item| -> bool {
        match item {
            Ok((k, _)) => k.starts_with(&prefix),
            Err(_) => false,
        }
    })
}

#[derive(Copy, Clone, Default, Debug)]
pub struct SledConfig {
    pub(crate) use_compression: bool,
    pub(crate) compression_factor: Option<i32>,
}

impl SledConfig {
    /// Creates a new sled config with zstd compression enabled.
    ///
    /// # Arguments
    /// * `factor` - The zstd compression factor to use. If unspecified, this
    ///   will default to 5.
    pub fn with_compression(factor: Option<i32>) -> SledConfig {
        Self {
            use_compression: true,
            compression_factor: factor,
        }
    }
}

pub(crate) struct VertexManager<'db> {
    pub db: &'db Db,
}

impl<'db> VertexManager<'db> {
    pub fn new(db: &'db Db) -> Self {
        VertexManager { db }
    }

    fn key(&self, id: Uuid) -> IVec {
        build(&[Component::AsciiChar('v'), Component::Uuid(id)]).into()
    }

    pub fn exists(&self, id: Uuid) -> Result<bool> {
        Ok(self.db.contains_key(&self.key(id))?)
    }

    pub fn get(&self, id: Uuid) -> Result<Option<models::Type>> {
        match self.db.get(&self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_type(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    fn iterate(&self, iterator: DbIterator) -> impl Iterator<Item = Result<VertexItem>> {
        iterator.map(move |item| -> Result<VertexItem> {
            let (k, v) = item?;

            let id = {
                debug_assert_eq!(k.len(), 17);
                let mut cursor = Cursor::new(k);
                read_expected_char(&mut cursor, 'v');
                read_uuid(&mut cursor)
            };

            let mut cursor = Cursor::new(v);
            let t = read_type(&mut cursor);
            Ok((id, t))
        })
    }

    pub fn iterate_for_range(&self, id: Uuid) -> impl Iterator<Item = Result<VertexItem>> {
        let low_key = build(&[Component::AsciiChar('v'), Component::Uuid(id)]);
        let low_key_bytes: &[u8] = low_key.as_ref();
        let iter = self.db.range(low_key_bytes..);
        self.iterate(iter)
    }

    pub fn create(&self, vertex: &models::Vertex) -> Result<()> {
        let key = self.key(vertex.id);
        self.db.insert(&key, build(&[Component::Type(&vertex.t)]))?;
        Ok(())
    }

    pub fn delete(&self, batch: &mut Batch, id: Uuid) -> Result<()> {
        batch.remove(&self.key(id));

        let vertex_property_manager = VertexPropertyManager::new(&self.db);
        for item in vertex_property_manager.iterate_for_owner(id) {
            let ((vertex_property_owner_id, vertex_property_name), _) = item?;
            vertex_property_manager.delete(batch, vertex_property_owner_id, &vertex_property_name[..]);
        }

        let edge_manager = EdgeManager::new(&self.db);

        {
            let edge_range_manager = EdgeRangeManager::new(&self.db);
            for item in edge_range_manager.iterate_for_owner(id) {
                let (edge_range_outbound_id, edge_range_t, edge_range_update_datetime, edge_range_inbound_id) = item?;
                debug_assert_eq!(edge_range_outbound_id, id);
                edge_manager.delete(
                    batch,
                    edge_range_outbound_id,
                    &edge_range_t,
                    edge_range_inbound_id,
                    edge_range_update_datetime,
                )?;
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(&self.db);
            for item in reversed_edge_range_manager.iterate_for_owner(id) {
                let (
                    reversed_edge_range_inbound_id,
                    reversed_edge_range_t,
                    reversed_edge_range_update_datetime,
                    reversed_edge_range_outbound_id,
                ) = item?;
                debug_assert_eq!(reversed_edge_range_inbound_id, id);
                edge_manager.delete(
                    batch,
                    reversed_edge_range_outbound_id,
                    &reversed_edge_range_t,
                    reversed_edge_range_inbound_id,
                    reversed_edge_range_update_datetime,
                )?;
            }
        }
        Ok(())
    }
}

pub(crate) struct EdgeManager<'db> {
    pub db: &'db Db,
}

impl<'db> EdgeManager<'db> {
    pub fn new(db: &'db Db) -> Self {
        EdgeManager { db }
    }

    fn key(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid) -> IVec {
        build(&[
            Component::AsciiChar('e'),
            Component::Uuid(outbound_id),
            Component::Type(t),
            Component::Uuid(inbound_id),
        ])
        .into()
    }

    pub fn get(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid) -> Result<Option<DateTime<Utc>>> {
        match self.db.get(self.key(outbound_id, t, inbound_id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_datetime(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut Batch,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        new_update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(&self.db);
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(&self.db);

        if let Some(update_datetime) = self.get(outbound_id, t, inbound_id)? {
            edge_range_manager.delete(batch, outbound_id, t, update_datetime, inbound_id);
            reversed_edge_range_manager.delete(batch, inbound_id, t, update_datetime, outbound_id);
        }

        let key = self.key(outbound_id, t, inbound_id);
        batch.insert(key, build(&[Component::DateTime(new_update_datetime)]));
        edge_range_manager.set(batch, outbound_id, t, new_update_datetime, inbound_id);
        reversed_edge_range_manager.set(batch, inbound_id, t, new_update_datetime, outbound_id);
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut Batch,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        batch.remove(&self.key(outbound_id, t, inbound_id));

        let edge_range_manager = EdgeRangeManager::new(&self.db);
        edge_range_manager.delete(batch, outbound_id, t, update_datetime, inbound_id);

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(&self.db);
        reversed_edge_range_manager.delete(batch, inbound_id, t, update_datetime, outbound_id);

        let edge_property_manager = EdgePropertyManager::new(&self.db);
        for item in edge_property_manager.iterate_for_owner(outbound_id, t, inbound_id) {
            let ((edge_property_outbound_id, edge_property_t, edge_property_inbound_id, edge_property_name), _) = item?;
            edge_property_manager.delete(
                batch,
                edge_property_outbound_id,
                &edge_property_t,
                edge_property_inbound_id,
                &edge_property_name[..],
            );
        }
        Ok(())
    }
}

pub(crate) struct EdgeRangeManager<'db> {
    pub db: &'db Db,
    reversed: bool,
}

impl<'db> EdgeRangeManager<'db> {
    pub fn new(db: &'db Db) -> Self {
        EdgeRangeManager { db, reversed: false }
    }

    pub fn new_reversed(db: &'db Db) -> Self {
        EdgeRangeManager { db, reversed: true }
    }

    fn prefix(&self) -> char {
        if self.reversed {
            '<'
        } else {
            '>'
        }
    }

    fn key(&self, first_id: Uuid, t: &models::Type, update_datetime: DateTime<Utc>, second_id: Uuid) -> IVec {
        build(&[
            Component::AsciiChar(self.prefix()),
            Component::Uuid(first_id),
            Component::Type(t),
            Component::DateTime(update_datetime),
            Component::Uuid(second_id),
        ])
        .into()
    }

    fn iterate(&self, iterator: DbIterator, prefix: Vec<u8>) -> impl Iterator<Item = Result<EdgeRangeItem>> + '_ {
        let filtered = take_while_prefixed(iterator, prefix);
        filtered.map(move |item| -> Result<EdgeRangeItem> {
            let (k, _) = item?;
            let mut cursor = Cursor::new(k);
            read_expected_char(&mut cursor, self.prefix());
            let first_id = read_uuid(&mut cursor);
            let t = read_type(&mut cursor);
            let update_datetime = read_datetime(&mut cursor);
            let second_id = read_uuid(&mut cursor);
            Ok((first_id, t, update_datetime, second_id))
        })
    }

    pub fn iterate_for_range(
        &self,
        id: Uuid,
        t: Option<&models::Type>,
        high: Option<DateTime<Utc>>,
    ) -> Box<dyn Iterator<Item = Result<EdgeRangeItem>> + '_> {
        match t {
            Some(t) => {
                let high = high.unwrap_or_else(|| *MAX_DATETIME);
                let prefix = build(&[
                    Component::AsciiChar(self.prefix()),
                    Component::Uuid(id),
                    Component::Type(t),
                ]);
                let low_key = build(&[
                    Component::AsciiChar(self.prefix()),
                    Component::Uuid(id),
                    Component::Type(t),
                    Component::DateTime(high),
                ]);
                let low_key_bytes: &[u8] = low_key.as_ref();
                let iterator = self.db.range(low_key_bytes..);
                Box::new(self.iterate(iterator, prefix))
            }
            None => {
                let prefix = build(&[Component::AsciiChar(self.prefix()), Component::Uuid(id)]);
                let prefix_bytes: &[u8] = prefix.as_ref();
                let iterator = self.db.range(prefix_bytes..);
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

                    Box::new(filtered)
                } else {
                    Box::new(mapped)
                }
            }
        }
    }

    pub fn iterate_for_owner(&self, id: Uuid) -> impl Iterator<Item = Result<EdgeRangeItem>> + '_ {
        let prefix: Vec<u8> = build(&[Component::AsciiChar(self.prefix()), Component::Uuid(id)]);
        let iterator = self.db.scan_prefix(&prefix);
        self.iterate(iterator, prefix)
    }

    pub fn set(
        &self,
        batch: &mut Batch,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) {
        let key = self.key(first_id, t, update_datetime, second_id);
        batch.insert(&key, &[]);
    }

    pub fn delete(
        &self,
        batch: &mut Batch,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) {
        batch.remove(&self.key(first_id, t, update_datetime, second_id));
    }
}

pub(crate) struct VertexPropertyManager<'db> {
    pub db: &'db Db,
}

impl<'db> VertexPropertyManager<'db> {
    pub fn new(db: &'db Db) -> Self {
        VertexPropertyManager { db }
    }

    fn key(&self, vertex_id: Uuid, name: &str) -> IVec {
        build(&[
            Component::AsciiChar('1'),
            Component::Uuid(vertex_id),
            Component::UnsizedString(name),
        ])
        .into()
    }

    pub fn iterate_for_owner(&self, vertex_id: Uuid) -> impl Iterator<Item = Result<OwnedPropertyItem>> {
        let prefix = build(&[Component::AsciiChar('1'), Component::Uuid(vertex_id)]);
        let iterator = self.db.scan_prefix(&prefix);

        iterator.map(move |item| -> Result<OwnedPropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);
            read_expected_char(&mut cursor, '1');
            let owner_id = read_uuid(&mut cursor);
            debug_assert_eq!(vertex_id, owner_id);
            let name = read_unsized_string(&mut cursor);
            let value = serde_json::from_slice(&v)?;
            Ok(((owner_id, name), value))
        })
    }

    pub fn get(&self, vertex_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(vertex_id, name);

        match self.db.get(&key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(&self, vertex_id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let key = self.key(vertex_id, name);
        let value_json = serde_json::to_vec(value)?;
        self.db.insert(key, value_json.as_slice())?;
        Ok(())
    }

    pub fn delete(&self, batch: &mut Batch, vertex_id: Uuid, name: &str) {
        batch.remove(&self.key(vertex_id, name));
    }
}

pub(crate) struct EdgePropertyManager<'db> {
    pub db: &'db Db,
}

impl<'db> EdgePropertyManager<'db> {
    pub fn new(db: &'db Db) -> Self {
        EdgePropertyManager { db }
    }

    fn key(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> IVec {
        build(&[
            Component::AsciiChar('2'),
            Component::Uuid(outbound_id),
            Component::Type(t),
            Component::Uuid(inbound_id),
            Component::UnsizedString(name),
        ])
        .into()
    }

    pub fn iterate_for_owner<'a>(
        &'a self,
        outbound_id: Uuid,
        t: &'a models::Type,
        inbound_id: Uuid,
    ) -> impl Iterator<Item = Result<EdgePropertyItem>> + 'a {
        let prefix = build(&[
            Component::AsciiChar('2'),
            Component::Uuid(outbound_id),
            Component::Type(t),
            Component::Uuid(inbound_id),
        ]);

        let iterator = self.db.scan_prefix(&prefix);

        iterator.map(move |item| -> Result<EdgePropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);
            read_expected_char(&mut cursor, '2');

            let edge_property_outbound_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_property_outbound_id, outbound_id);

            let edge_property_t = read_type(&mut cursor);
            debug_assert_eq!(&edge_property_t, t);

            let edge_property_inbound_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_property_inbound_id, inbound_id);

            let edge_property_name = read_unsized_string(&mut cursor);

            let value = serde_json::from_slice(&v)?;
            Ok((
                (
                    edge_property_outbound_id,
                    edge_property_t,
                    edge_property_inbound_id,
                    edge_property_name,
                ),
                value,
            ))
        })
    }

    pub fn get(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(outbound_id, t, inbound_id, name);

        match self.db.get(&key)? {
            Some(ref value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        name: &str,
        value: &JsonValue,
    ) -> Result<()> {
        let key = self.key(outbound_id, t, inbound_id, name);
        let value_json = serde_json::to_vec(value)?;
        self.db.insert(key, value_json.as_slice())?;
        Ok(())
    }

    pub fn delete(&self, batch: &mut Batch, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) {
        batch.remove(&self.key(outbound_id, t, inbound_id, name));
    }
}
