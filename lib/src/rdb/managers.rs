use super::keys::*;
use bincode;
use chrono::DateTime;
use chrono::offset::Utc;
use errors::Result;
use models;
use rocksdb::{ColumnFamily, DBIterator, Direction, IteratorMode, WriteBatch, DB};
use serde::Serialize;
use serde_json;
use serde_json::Value as JsonValue;
use std::io::Cursor;
use std::sync::Arc;
use std::u8;
use uuid::Uuid;

pub type DBIteratorItem = (Box<[u8]>, Box<[u8]>);
pub type OwnedMetadataItem = Result<((Uuid, String), JsonValue)>;
pub type VertexItem = Result<(Uuid, models::Type)>;
pub type EdgeRangeItem = Result<(Uuid, models::Type, DateTime<Utc>, Uuid)>;
pub type EdgeMetadataItem = Result<((Uuid, models::Type, Uuid, String), JsonValue)>;

fn bincode_serialize_value<T: Serialize>(value: &T) -> Result<Box<[u8]>> {
    let result = bincode::serialize(value, bincode::Infinite)?;
    Ok(result.into_boxed_slice())
}

fn json_serialize_value(value: &JsonValue) -> Result<Box<[u8]>> {
    let result = serde_json::to_vec(value)?;
    Ok(result.into_boxed_slice())
}

fn json_deserialize_value(value: &[u8]) -> Result<JsonValue> {
    let result = serde_json::from_slice(value)?;
    Ok(result)
}

fn exists(db: &DB, cf: ColumnFamily, key: Box<[u8]>) -> Result<bool> {
    match db.get_cf(cf, &key)? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

fn set_bincode<T: Serialize>(db: &DB, cf: ColumnFamily, key: Box<[u8]>, value: &T) -> Result<()> {
    db.put_cf(cf, &key, &bincode_serialize_value(value)?)?;
    Ok(())
}

fn get_json(db: &DB, cf: ColumnFamily, key: Box<[u8]>) -> Result<Option<JsonValue>> {
    match db.get_cf(cf, &key)? {
        Some(value_bytes) => Ok(Some(json_deserialize_value(&value_bytes)?)),
        None => Ok(None),
    }
}

fn take_while_prefixed<'a>(iterator: DBIterator, prefix: Box<[u8]>) -> Box<Iterator<Item = DBIteratorItem> + 'a> {
    let filtered = iterator.take_while(move |item| -> bool {
        let (ref k, _) = *item;
        k.starts_with(&prefix)
    });

    Box::new(filtered)
}

fn iterate_metadata_for_owner<'a>(
    db: &DB,
    cf: ColumnFamily,
    id: Uuid,
) -> Result<Box<Iterator<Item = OwnedMetadataItem> + 'a>> {
    let prefix = build_key(vec![KeyComponent::Uuid(id)]);
    let iterator = db.iterator_cf(cf, IteratorMode::From(&prefix, Direction::Forward))?;
    let filtered = take_while_prefixed(iterator, prefix);

    let mapped = filtered.map(move |item| -> Result<((Uuid, String), JsonValue)> {
        let (k, v) = item;
        let mut cursor = Cursor::new(k);
        let owner_id = read_uuid(&mut cursor);
        debug_assert_eq!(id, owner_id);
        let name = read_unsized_string(&mut cursor);
        let value = json_deserialize_value(&v.to_owned()[..])?;
        Ok(((owner_id, name), value))
    });

    Ok(Box::new(mapped))
}

pub struct VertexManager {
    pub db: Arc<DB>,
    pub cf: ColumnFamily,
}

impl VertexManager {
    pub fn new(db: Arc<DB>) -> Self {
        VertexManager {
            cf: db.cf_handle("vertices:v1").unwrap(),
            db: db,
        }
    }

    fn key(&self, id: Uuid) -> Box<[u8]> {
        build_key(vec![KeyComponent::Uuid(id)])
    }

    pub fn exists(&self, id: Uuid) -> Result<bool> {
        exists(&self.db, self.cf, self.key(id))
    }

    pub fn get(&self, id: Uuid) -> Result<Option<models::Type>> {
        match self.db.get_cf(self.cf, &self.key(id))? {
            Some(value_bytes) => Ok(Some(bincode::deserialize(&value_bytes)?)),
            None => Ok(None),
        }
    }

    fn iterate<'a>(&self, iterator: DBIterator) -> Result<Box<Iterator<Item = VertexItem> + 'a>> {
        let mapped = iterator.map(|item| -> VertexItem {
            let (k, v) = item;
            let id = parse_uuid_key(k);
            let value: models::Type = bincode::deserialize(&v.to_owned()[..])?;
            Ok((id, value))
        });

        Ok(Box::new(mapped))
    }

    pub fn iterate_for_range<'a>(&self, id: Uuid) -> Result<Box<Iterator<Item = VertexItem> + 'a>> {
        let low_key = build_key(vec![KeyComponent::Uuid(id)]);
        let iterator = self.db
            .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward))?;
        self.iterate(iterator)
    }

    pub fn create(&self, vertex: &models::Vertex) -> Result<()> {
        set_bincode(&self.db, self.cf, self.key(vertex.id), &vertex.t)?;
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, id: Uuid) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(id))?;

        let vertex_metadata_manager = VertexMetadataManager::new(self.db.clone());
        for item in vertex_metadata_manager.iterate_for_owner(id)? {
            let ((vertex_metadata_owner_id, vertex_metadata_name), _) = item?;
            vertex_metadata_manager.delete(
                &mut batch,
                vertex_metadata_owner_id,
                &vertex_metadata_name[..],
            )?;
        }

        let edge_manager = EdgeManager::new(self.db.clone());

        {
            let edge_range_manager = EdgeRangeManager::new(self.db.clone());
            for item in edge_range_manager.iterate_for_owner(id)? {
                let (edge_range_outbound_id, edge_range_t, edge_range_update_datetime, edge_range_inbound_id) = item?;
                debug_assert_eq!(edge_range_outbound_id, id);
                edge_manager.delete(
                    &mut batch,
                    edge_range_outbound_id,
                    &edge_range_t,
                    edge_range_inbound_id,
                    edge_range_update_datetime,
                )?;
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
            for item in reversed_edge_range_manager.iterate_for_owner(id)? {
                let (
                    reversed_edge_range_inbound_id,
                    reversed_edge_range_t,
                    reversed_edge_range_update_datetime,
                    reversed_edge_range_outbound_id,
                ) = item?;
                debug_assert_eq!(reversed_edge_range_inbound_id, id);
                edge_manager.delete(
                    &mut batch,
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

pub struct EdgeManager {
    pub db: Arc<DB>,
    pub cf: ColumnFamily,
}

impl EdgeManager {
    pub fn new(db: Arc<DB>) -> Self {
        EdgeManager {
            cf: db.cf_handle("edges:v1").unwrap(),
            db: db,
        }
    }

    fn key(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid) -> Box<[u8]> {
        build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::Type(t),
            KeyComponent::Uuid(inbound_id),
        ])
    }

    pub fn get(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid) -> Result<Option<DateTime<Utc>>> {
        match self.db
            .get_cf(self.cf, &self.key(outbound_id, t, inbound_id))?
        {
            Some(value_bytes) => Ok(Some(bincode::deserialize(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        mut batch: &mut WriteBatch,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        new_update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());

        if let Some(update_datetime) = self.get(outbound_id, t, inbound_id)? {
            edge_range_manager.delete(&mut batch, outbound_id, t, update_datetime, inbound_id)?;
            reversed_edge_range_manager.delete(&mut batch, inbound_id, t, update_datetime, outbound_id)?;
        }

        set_bincode(
            &self.db,
            self.cf,
            self.key(outbound_id, t, inbound_id),
            &new_update_datetime,
        )?;
        edge_range_manager.set(&mut batch, outbound_id, t, new_update_datetime, inbound_id)?;
        reversed_edge_range_manager.set(&mut batch, inbound_id, t, new_update_datetime, outbound_id)?;
        Ok(())
    }

    pub fn delete(
        &self,
        mut batch: &mut WriteBatch,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(outbound_id, t, inbound_id))?;

        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        edge_range_manager.delete(&mut batch, outbound_id, t, update_datetime, inbound_id)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        reversed_edge_range_manager.delete(&mut batch, inbound_id, t, update_datetime, outbound_id)?;

        let edge_metadata_manager = EdgeMetadataManager::new(self.db.clone());
        for item in edge_metadata_manager.iterate_for_owner(outbound_id, t, inbound_id)? {
            let ((edge_metadata_outbound_id, edge_metadata_t, edge_metadata_inbound_id, edge_metadata_name), _) = item?;
            edge_metadata_manager.delete(
                &mut batch,
                edge_metadata_outbound_id,
                &edge_metadata_t,
                edge_metadata_inbound_id,
                &edge_metadata_name[..],
            )?;
        }

        Ok(())
    }
}

pub struct EdgeRangeManager {
    pub db: Arc<DB>,
    pub cf: ColumnFamily,
}

impl EdgeRangeManager {
    pub fn new(db: Arc<DB>) -> Self {
        EdgeRangeManager {
            cf: db.cf_handle("edge_ranges:v1").unwrap(),
            db: db,
        }
    }

    pub fn new_reversed(db: Arc<DB>) -> Self {
        EdgeRangeManager {
            cf: db.cf_handle("reversed_edge_ranges:v1").unwrap(),
            db: db,
        }
    }

    fn key(&self, first_id: Uuid, t: &models::Type, update_datetime: DateTime<Utc>, second_id: Uuid) -> Box<[u8]> {
        build_key(vec![
            KeyComponent::Uuid(first_id),
            KeyComponent::Type(t),
            KeyComponent::DateTime(update_datetime),
            KeyComponent::Uuid(second_id),
        ])
    }

    fn iterate<'a>(&self, iterator: DBIterator, prefix: Box<[u8]>) -> Result<Box<Iterator<Item = EdgeRangeItem> + 'a>> {
        let filtered = take_while_prefixed(iterator, prefix);

        let mapped = filtered.map(move |item| -> EdgeRangeItem {
            let (k, _) = item;
            let mut cursor = Cursor::new(k);
            let first_id = read_uuid(&mut cursor);
            let t = read_type(&mut cursor);
            let update_datetime = read_datetime(&mut cursor);
            let second_id = read_uuid(&mut cursor);
            Ok((first_id, t, update_datetime, second_id))
        });

        Ok(Box::new(mapped))
    }

    pub fn iterate_for_range<'a>(
        &self,
        id: Uuid,
        t: Option<&models::Type>,
        high: Option<DateTime<Utc>>,
    ) -> Result<Box<Iterator<Item = EdgeRangeItem> + 'a>> {
        match t {
            Some(t) => {
                let high = high.unwrap_or_else(|| *MAX_DATETIME);
                let prefix = build_key(vec![KeyComponent::Uuid(id), KeyComponent::Type(t)]);
                let low_key = build_key(vec![
                    KeyComponent::Uuid(id),
                    KeyComponent::Type(t),
                    KeyComponent::DateTime(high),
                ]);
                let iterator = self.db
                    .iterator_cf(self.cf, IteratorMode::From(&low_key, Direction::Forward))?;
                self.iterate(iterator, prefix)
            }
            None => {
                let prefix = build_key(vec![KeyComponent::Uuid(id)]);
                let iterator = self.db
                    .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward))?;
                let mapped = self.iterate(iterator, prefix)?;

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
                    Ok(mapped)
                }
            }
        }
    }

    pub fn iterate_for_owner<'a>(&self, id: Uuid) -> Result<Box<Iterator<Item = EdgeRangeItem> + 'a>> {
        let prefix = build_key(vec![KeyComponent::Uuid(id)]);
        let iterator = self.db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward))?;
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
        let value = bincode_serialize_value(&())?;
        batch.put_cf(self.cf, &key, &value)?;
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
        batch.delete_cf(self.cf, &self.key(first_id, t, update_datetime, second_id))?;
        Ok(())
    }
}

pub struct VertexMetadataManager {
    pub db: Arc<DB>,
    pub cf: ColumnFamily,
}

impl VertexMetadataManager {
    pub fn new(db: Arc<DB>) -> Self {
        VertexMetadataManager {
            cf: db.cf_handle("vertex_metadata:v1").unwrap(),
            db: db,
        }
    }

    fn key(&self, vertex_id: Uuid, name: &str) -> Box<[u8]> {
        build_key(vec![
            KeyComponent::Uuid(vertex_id),
            KeyComponent::UnsizedString(name),
        ])
    }

    pub fn iterate_for_owner(&self, vertex_id: Uuid) -> Result<Box<Iterator<Item = OwnedMetadataItem>>> {
        iterate_metadata_for_owner(&self.db, self.cf, vertex_id)
    }

    pub fn get(&self, vertex_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        get_json(&self.db, self.cf, self.key(vertex_id, name))
    }

    pub fn set(&self, batch: &mut WriteBatch, vertex_id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let key = self.key(vertex_id, name);
        let value_json = json_serialize_value(value)?;
        batch.put_cf(self.cf, &key, &value_json)?;
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, vertex_id: Uuid, name: &str) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(vertex_id, name))?;
        Ok(())
    }
}

pub struct EdgeMetadataManager {
    pub db: Arc<DB>,
    pub cf: ColumnFamily,
}

impl EdgeMetadataManager {
    pub fn new(db: Arc<DB>) -> Self {
        EdgeMetadataManager {
            cf: db.cf_handle("edge_metadata:v1").unwrap(),
            db: db,
        }
    }

    fn key(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> Box<[u8]> {
        build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::Type(t),
            KeyComponent::Uuid(inbound_id),
            KeyComponent::UnsizedString(name),
        ])
    }

    pub fn iterate_for_owner<'a>(
        &self,
        outbound_id: Uuid,
        t: &'a models::Type,
        inbound_id: Uuid,
    ) -> Result<Box<Iterator<Item = EdgeMetadataItem> + 'a>> {
        let prefix = build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::Type(t),
            KeyComponent::Uuid(inbound_id),
        ]);

        let iterator = self.db
            .iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward))?;
        let filtered = take_while_prefixed(iterator, prefix);

        let mapped = filtered.map(move |item| -> EdgeMetadataItem {
            let (k, v) = item;
            let mut cursor = Cursor::new(k);

            let edge_metadata_outbound_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_metadata_outbound_id, outbound_id);

            let edge_metadata_t = read_type(&mut cursor);
            debug_assert_eq!(&edge_metadata_t, t);

            let edge_metadata_inbound_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_metadata_inbound_id, inbound_id);

            let edge_metadata_name = read_unsized_string(&mut cursor);

            let value = json_deserialize_value(&v.to_owned()[..])?;
            Ok((
                (
                    edge_metadata_outbound_id,
                    edge_metadata_t,
                    edge_metadata_inbound_id,
                    edge_metadata_name,
                ),
                value,
            ))
        });

        Ok(Box::new(mapped))
    }

    pub fn get(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        get_json(
            &self.db,
            self.cf,
            self.key(outbound_id, t, inbound_id, name),
        )
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        name: &str,
        value: &JsonValue,
    ) -> Result<()> {
        let key = self.key(outbound_id, t, inbound_id, name);
        let value_json = json_serialize_value(value)?;
        batch.put_cf(self.cf, &key, &value_json)?;
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        name: &str,
    ) -> Result<()> {
        batch.delete_cf(self.cf, &self.key(outbound_id, t, inbound_id, name))?;
        Ok(())
    }
}
