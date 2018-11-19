use ::serializer::*;
use chrono::offset::Utc;
use chrono::DateTime;
use errors::Result;
use models;
use serde_json;
use serde_json::Value as JsonValue;
use std::io::Cursor;
use std::ops::Deref;
use std::u8;
use uuid::Uuid;
use std::path::{Path, PathBuf};
use sled::{Iter, PinnedValue, Tree};

pub type OwnedPropertyItem = ((Uuid, String), JsonValue);
pub type VertexItem = (Uuid, models::Type);
pub type EdgeRangeItem = (Uuid, models::Type, DateTime<Utc>, Uuid);
pub type EdgePropertyItem = ((Uuid, models::Type, Uuid, String), JsonValue);

fn take_while_prefixed<'a>(iterator: Iter<'a>, prefix: Vec<u8>) -> Vec<(Vec<u8>, PinnedValue)> {
    iterator
        .map(move |item| item.unwrap())
        .take_while(move |item| -> bool {
            let (ref k, _) = item;
            k.starts_with(&prefix)
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct Database {
    pub vertices: Tree,
    pub edges: Tree,
    pub edge_ranges: Tree,
    pub reversed_edge_ranges: Tree,
    pub vertex_properties: Tree,
    pub edge_properties: Tree
}

impl Database {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path: PathBuf = path.into();

        Ok(Self {
            vertices: Tree::start_default(path.join("vertices"))?,
            edges: Tree::start_default(path.join("edges"))?,
            edge_ranges: Tree::start_default(path.join("edge_ranges"))?,
            reversed_edge_ranges: Tree::start_default(path.join("reversed_edge_ranges"))?,
            vertex_properties: Tree::start_default(path.join("vertex_properties"))?,
            edge_properties: Tree::start_default(path.join("edge_properties"))?,
        })
    }
}

pub struct VertexManager {
    db: Database
}

impl VertexManager {
    pub fn new(db: Database) -> Self {
        VertexManager { db }
    }

    fn key(&self, id: Uuid) -> Vec<u8> {
        build(&[Component::Uuid(id)])
    }

    pub fn exists(&self, id: Uuid) -> Result<bool> {
        Ok(self.db.vertices.get(&self.key(id))?.is_some())
    }

    pub fn get(&self, id: Uuid) -> Result<Option<models::Type>> {
        match self.db.vertices.get(&self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_type(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    fn iterate<'a>(&self, iterator: Iter<'a>) -> Vec<Result<VertexItem>> {
        iterator
            .map(|item| -> Result<VertexItem> {
                let (k, v) = item?;

                let id = {
                    debug_assert_eq!(k.len(), 16);
                    let mut cursor = Cursor::new(k);
                    read_uuid(&mut cursor)
                };

                let mut cursor = Cursor::new(v.deref());
                let t = read_type(&mut cursor);
                Ok((id, t))
            })
            .collect()
    }

    pub fn iterate_for_range(&self, id: Uuid) -> Vec<Result<VertexItem>> {
        let low_key = build(&[Component::Uuid(id)]);
        let iter = self.db.vertices.scan(&low_key);
        self.iterate(iter)
    }

    pub fn create(&self, vertex: &models::Vertex) -> Result<()> {
        let key = self.key(vertex.id);
        self.db.vertices.set(key, build(&[Component::Type(&vertex.t)]))?;
        Ok(())
    }

    pub fn delete(&self, id: Uuid) -> Result<()> {
        self.db.vertices.del(&self.key(id))?;

        let vertex_property_manager = VertexPropertyManager::new(self.db.clone());
        for item in vertex_property_manager.iterate_for_owner(id) {
            let ((vertex_property_owner_id, vertex_property_name), _) = item?;
            vertex_property_manager.delete(vertex_property_owner_id, &vertex_property_name[..])?;
        }

        let edge_manager = EdgeManager::new(self.db.clone());

        {
            let edge_range_manager = EdgeRangeManager::new(self.db.clone());
            for item in edge_range_manager.iterate_for_owner(id) {
                let (edge_range_outbound_id, edge_range_t, edge_range_update_datetime, edge_range_inbound_id) = item?;
                debug_assert_eq!(edge_range_outbound_id, id);
                edge_manager.delete(
                    edge_range_outbound_id,
                    &edge_range_t,
                    edge_range_inbound_id,
                    edge_range_update_datetime,
                )?;
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
            for item in reversed_edge_range_manager.iterate_for_owner(id) {
                let (
                    reversed_edge_range_inbound_id,
                    reversed_edge_range_t,
                    reversed_edge_range_update_datetime,
                    reversed_edge_range_outbound_id,
                ) = item?;
                debug_assert_eq!(reversed_edge_range_inbound_id, id);
                edge_manager.delete(
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
    db: Database
}

impl EdgeManager {
    pub fn new(db: Database) -> Self {
        EdgeManager { db }
    }

    fn key(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid) -> Vec<u8> {
        build(&[
            Component::Uuid(outbound_id),
            Component::Type(t),
            Component::Uuid(inbound_id),
        ])
    }

    pub fn get(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid) -> Result<Option<DateTime<Utc>>> {
        match self.db.edges.get(&self.key(outbound_id, t, inbound_id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_datetime(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        new_update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());

        if let Some(update_datetime) = self.get(outbound_id, t, inbound_id)? {
            edge_range_manager.delete(outbound_id, t, update_datetime, inbound_id)?;
            reversed_edge_range_manager.delete(inbound_id, t, update_datetime, outbound_id)?;
        }

        let key = self.key(outbound_id, t, inbound_id);
        self.db.edges.set(key, build(&[Component::DateTime(new_update_datetime)]))?;
        edge_range_manager.set(outbound_id, t, new_update_datetime, inbound_id)?;
        reversed_edge_range_manager.set(inbound_id, t, new_update_datetime, outbound_id)?;
        Ok(())
    }

    pub fn delete(
        &self,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        update_datetime: DateTime<Utc>,
    ) -> Result<()> {
        self.db.edges.del(&self.key(outbound_id, t, inbound_id))?;

        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        edge_range_manager.delete(outbound_id, t, update_datetime, inbound_id)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        reversed_edge_range_manager.delete(inbound_id, t, update_datetime, outbound_id)?;

        let edge_property_manager = EdgePropertyManager::new(self.db.clone());
        for item in edge_property_manager.iterate_for_owner(outbound_id, t, inbound_id) {
            let ((edge_property_outbound_id, edge_property_t, edge_property_inbound_id, edge_property_name), _) = item?;
            edge_property_manager.delete(
                edge_property_outbound_id,
                &edge_property_t,
                edge_property_inbound_id,
                &edge_property_name[..],
            )?;
        }

        Ok(())
    }
}

pub struct EdgeRangeManager {
    db: Database,
    is_reversed: bool
}

impl EdgeRangeManager {
    pub fn new(db: Database) -> Self {
        EdgeRangeManager { db, is_reversed: false }
    }

    pub fn new_reversed(db: Database) -> Self {
        EdgeRangeManager { db, is_reversed: true }
    }

    fn tree(&self) -> &Tree {
        if self.is_reversed {
            &self.db.reversed_edge_ranges
        } else {
            &self.db.edge_ranges
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

    fn iterate(&self, iterator: Iter, prefix: Vec<u8>) -> impl Iterator<Item = Result<EdgeRangeItem>> {
        let filtered = take_while_prefixed(iterator, prefix);

        filtered.into_iter().map(move |(k, _)| -> Result<EdgeRangeItem> {
            let mut cursor = Cursor::new(k);
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
    ) -> Box<dyn Iterator<Item = Result<EdgeRangeItem>>> {
        match t {
            Some(t) => {
                let high = high.unwrap_or_else(|| *MAX_DATETIME);
                let prefix = build(&[Component::Uuid(id), Component::Type(t)]);
                let low_key = build(&[Component::Uuid(id), Component::Type(t), Component::DateTime(high)]);
                let iterator = self.tree().scan(&low_key);
                Box::new(self.iterate(iterator, prefix))
            }
            None => {
                let prefix = build(&[Component::Uuid(id)]);
                let iterator = self.tree().scan(&prefix);
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

    pub fn iterate_for_owner(&self, id: Uuid) -> impl Iterator<Item = Result<EdgeRangeItem>> {
        let prefix = build(&[Component::Uuid(id)]);
        let iterator = self.tree().scan(&prefix);
        self.iterate(iterator, prefix)
    }

    pub fn set(
        &self,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) -> Result<()> {
        let key = self.key(first_id, t, update_datetime, second_id);
        self.tree().set(key, Vec::new())?;
        Ok(())
    }

    pub fn delete(
        &self,
        first_id: Uuid,
        t: &models::Type,
        update_datetime: DateTime<Utc>,
        second_id: Uuid,
    ) -> Result<()> {
        self.tree().del(&self.key(first_id, t, update_datetime, second_id))?;
        Ok(())
    }
}

pub struct VertexPropertyManager {
    db: Database
}

impl VertexPropertyManager {
    pub fn new(db: Database) -> Self {
        VertexPropertyManager { db }
    }

    fn key(&self, vertex_id: Uuid, name: &str) -> Vec<u8> {
        build(&[Component::Uuid(vertex_id), Component::UnsizedString(name)])
    }

    pub fn iterate_for_owner(&self, vertex_id: Uuid) -> impl Iterator<Item = Result<OwnedPropertyItem>> {
        let prefix = build(&[Component::Uuid(vertex_id)]);
        let iterator = self.db.vertex_properties.scan(&prefix);
        let filtered = take_while_prefixed(iterator, prefix);

        filtered.into_iter().map(move |(k, v)| -> Result<OwnedPropertyItem> {
            let mut cursor = Cursor::new(k);
            let owner_id = read_uuid(&mut cursor);
            debug_assert_eq!(vertex_id, owner_id);
            let name = read_unsized_string(&mut cursor);
            let value = serde_json::from_slice(&v)?;
            Ok(((owner_id, name), value))
        })
    }

    pub fn get(&self, vertex_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(vertex_id, name);

        match self.db.vertex_properties.get(&key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(&self, vertex_id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let key = self.key(vertex_id, name);
        let value_json = serde_json::to_vec(value)?;
        self.db.vertex_properties.set(key, value_json)?;
        Ok(())
    }

    pub fn delete(&self, vertex_id: Uuid, name: &str) -> Result<()> {
        self.db.vertex_properties.del(&self.key(vertex_id, name))?;
        Ok(())
    }
}

pub struct EdgePropertyManager {
    db: Database
}

impl EdgePropertyManager {
    pub fn new(db: Database) -> Self {
        EdgePropertyManager { db }
    }

    fn key(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> Vec<u8> {
        build(&[
            Component::Uuid(outbound_id),
            Component::Type(t),
            Component::Uuid(inbound_id),
            Component::UnsizedString(name),
        ])
    }

    pub fn iterate_for_owner<'a>(
        &self,
        outbound_id: Uuid,
        t: &'a models::Type,
        inbound_id: Uuid,
    ) -> Box<dyn Iterator<Item = Result<EdgePropertyItem>> + 'a> {
        let prefix = build(&[
            Component::Uuid(outbound_id),
            Component::Type(t),
            Component::Uuid(inbound_id),
        ]);

        let iterator = self.db.edge_properties.scan(&prefix);
        let filtered = take_while_prefixed(iterator, prefix);

        let mapped = filtered.into_iter().map(move |(k, v)| -> Result<EdgePropertyItem> {
            let mut cursor = Cursor::new(k);

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
        });

        Box::new(mapped)
    }

    pub fn get(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(outbound_id, t, inbound_id, name);

        match self.db.edge_properties.get(&key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
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
        self.db.edge_properties.set(key, value_json)?;
        Ok(())
    }

    pub fn delete(
        &self,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
        name: &str,
    ) -> Result<()> {
        self.db.edge_properties.del(&self.key(outbound_id, t, inbound_id, name))?;
        Ok(())
    }
}
