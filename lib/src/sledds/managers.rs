use std::io::Cursor;
use std::ops::Deref;
use std::u8;

use super::super::bytes::*;
use crate::errors::Result;
use crate::models;
use crate::sledds::datastore::SledHolder;

use serde_json::Value as JsonValue;
use sled::Result as SledResult;
use sled::{IVec, Iter as DbIterator, Tree};
use uuid::Uuid;

pub type OwnedPropertyItem = ((Uuid, String), JsonValue);
pub type VertexItem = (Uuid, models::Type);
pub type EdgeRangeItem = (Uuid, models::Type, Uuid);
pub type EdgePropertyItem = ((Uuid, models::Type, Uuid, String), JsonValue);

fn take_while_prefixed(iterator: DbIterator, prefix: Vec<u8>) -> impl Iterator<Item = SledResult<(IVec, IVec)>> {
    iterator.take_while(move |item| -> bool {
        match item {
            Ok((k, _)) => k.starts_with(&prefix),
            Err(_) => false,
        }
    })
}

pub struct VertexManager<'db: 'tree, 'tree> {
    pub holder: &'db SledHolder,
    pub tree: &'tree Tree,
}

impl<'db: 'tree, 'tree> VertexManager<'db, 'tree> {
    pub fn new(ds: &'db SledHolder) -> Self {
        VertexManager {
            holder: ds,
            tree: &ds.db.deref(),
        }
    }

    fn key(&self, id: Uuid) -> Vec<u8> {
        build(&[Component::Uuid(id)])
    }

    pub fn exists(&self, id: Uuid) -> Result<bool> {
        Ok(self.tree.get(&self.key(id))?.is_some())
    }

    pub fn get(&self, id: Uuid) -> Result<Option<models::Type>> {
        match self.tree.get(&self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                Ok(Some(read_type(&mut cursor)))
            }
            None => Ok(None),
        }
    }

    fn iterate(&self, iterator: DbIterator) -> impl Iterator<Item = Result<VertexItem>> + '_ {
        iterator.map(move |item| -> Result<VertexItem> {
            let (k, v) = item?;

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

    pub fn iterate_for_range<'a>(&'a self, id: Uuid) -> Result<impl Iterator<Item = Result<VertexItem>> + 'a> {
        let from = build(&[Component::Uuid(id)]);
        let iter = self.tree.range(from..);
        Ok(self.iterate(iter))
    }

    pub fn create(&self, vertex: &models::Vertex) -> Result<()> {
        let key = self.key(vertex.id);
        self.tree.insert(&key, build(&[Component::Type(&vertex.t)]))?;
        Ok(())
    }

    pub fn delete(&self, id: Uuid) -> Result<()> {
        self.tree.remove(&self.key(id))?;

        let vertex_property_manager = VertexPropertyManager::new(&self.holder.vertex_properties);
        for item in vertex_property_manager.iterate_for_owner(id)? {
            let ((vertex_property_owner_id, vertex_property_name), _) = item?;
            vertex_property_manager.delete(vertex_property_owner_id, &vertex_property_name[..])?;
        }

        let edge_manager = EdgeManager::new(&self.holder);

        {
            let edge_range_manager = EdgeRangeManager::new(&self.holder);
            for item in edge_range_manager.iterate_for_owner(id)? {
                let (edge_range_outbound_id, edge_range_t, edge_range_inbound_id) = item?;
                debug_assert_eq!(edge_range_outbound_id, id);
                edge_manager.delete(
                    edge_range_outbound_id,
                    &edge_range_t,
                    edge_range_inbound_id,
                )?;
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(&self.holder);
            for item in reversed_edge_range_manager.iterate_for_owner(id)? {
                let (
                    reversed_edge_range_inbound_id,
                    reversed_edge_range_t,
                    reversed_edge_range_outbound_id,
                ) = item?;
                debug_assert_eq!(reversed_edge_range_inbound_id, id);
                edge_manager.delete(
                    reversed_edge_range_outbound_id,
                    &reversed_edge_range_t,
                    reversed_edge_range_inbound_id,
                )?;
            }
        }
        Ok(())
    }
}

pub struct EdgeManager<'db> {
    pub holder: &'db SledHolder,
}

impl<'db> EdgeManager<'db> {
    pub fn new(ds: &'db SledHolder) -> Self {
        EdgeManager {
            holder: ds,
        }
    }

    pub fn set(
        &self,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(&self.holder);
        edge_range_manager.set(outbound_id, t, inbound_id)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(&self.holder);
        reversed_edge_range_manager.set(inbound_id, t, outbound_id)?;

        Ok(())
    }

    pub fn delete(
        &self,
        outbound_id: Uuid,
        t: &models::Type,
        inbound_id: Uuid,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(&self.holder);
        edge_range_manager.delete(outbound_id, t, inbound_id)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(&self.holder);
        reversed_edge_range_manager.delete(inbound_id, t, outbound_id)?;

        let edge_property_manager = EdgePropertyManager::new(&self.holder.edge_properties);
        for item in edge_property_manager.iterate_for_owner(outbound_id, t, inbound_id)? {
            let ((edge_property_outbound_id, edge_property_t, edge_property_inbound_id, edge_property_name), _) = item?;
            edge_property_manager.delete(
                edge_property_outbound_id,
                &edge_property_t,
                edge_property_inbound_id,
                &edge_property_name,
            )?;
        }
        Ok(())
    }
}

pub struct EdgeRangeManager<'tree> {
    pub tree: &'tree Tree,
}

impl<'tree> EdgeRangeManager<'tree> {
    pub fn new<'db: 'tree>(ds: &'db SledHolder) -> Self {
        EdgeRangeManager { tree: &ds.edge_ranges }
    }

    pub fn new_reversed<'db: 'tree>(ds: &'db SledHolder) -> Self {
        EdgeRangeManager {
            tree: &ds.reversed_edge_ranges,
        }
    }

    fn key(&self, first_id: Uuid, t: &models::Type, second_id: Uuid) -> Vec<u8> {
        build(&[
            Component::Uuid(first_id),
            Component::Type(t),
            Component::Uuid(second_id),
        ])
    }

    fn iterate<'it>(
        &self,
        iterator: DbIterator,
        prefix: Vec<u8>,
    ) -> Result<impl Iterator<Item = Result<EdgeRangeItem>> + 'it> {
        let filtered = take_while_prefixed(iterator, prefix);
        Ok(filtered.map(move |item| -> Result<EdgeRangeItem> {
            let (k, _) = item?;
            let mut cursor = Cursor::new(k);
            let first_id = read_uuid(&mut cursor);
            let t = read_type(&mut cursor);
            let second_id = read_uuid(&mut cursor);
            Ok((first_id, t, second_id))
        }))
    }

    pub fn iterate_for_range<'iter, 'trans: 'iter>(
        &'trans self,
        id: Uuid,
        t: Option<&models::Type>,
        offset: u64
    ) -> Result<Box<dyn Iterator<Item = Result<EdgeRangeItem>> + 'iter>> {
        match t {
            Some(t) => {
                let prefix = build(&[Component::Uuid(id), Component::Type(t)]);
                let prefix_bytes: &[u8] = prefix.as_ref();
                let iterator = self.tree.range(prefix_bytes..);
                Ok(Box::new(self.iterate(iterator, prefix)?.skip(offset as usize)))
            }
            None => {
                let prefix = build(&[Component::Uuid(id)]);
                let prefix_bytes: &[u8] = prefix.as_ref();
                let iterator = self.tree.range(prefix_bytes..);
                Ok(Box::new(self.iterate(iterator, prefix)?.skip(offset as usize)))
            }
        }
    }

    pub fn iterate_for_owner<'iter, 'trans: 'iter>(
        &'trans self,
        id: Uuid,
    ) -> Result<impl Iterator<Item = Result<EdgeRangeItem>> + 'iter> {
        let prefix: Vec<u8> = build(&[Component::Uuid(id)]);
        let iterator = self.tree.scan_prefix(&prefix);
        self.iterate(iterator, prefix)
    }

    pub fn exists(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid) -> Result<bool> {
        Ok(self.tree.get(self.key(outbound_id, t, inbound_id))?.is_some())
    }

    pub fn set(&self, first_id: Uuid, t: &models::Type, second_id: Uuid) -> Result<()> {
        let key = self.key(first_id, t, second_id);
        self.tree.insert(&key, &[])?;
        Ok(())
    }

    pub fn delete(
        &self,
        first_id: Uuid,
        t: &models::Type,
        second_id: Uuid,
    ) -> Result<()> {
        self.tree.remove(&self.key(first_id, t, second_id))?;
        Ok(())
    }
}

pub struct VertexPropertyManager<'tree> {
    pub tree: &'tree Tree,
}

impl<'tree> VertexPropertyManager<'tree> {
    pub fn new(tree: &'tree Tree) -> Self {
        VertexPropertyManager { tree }
    }

    fn key(&self, vertex_id: Uuid, name: &str) -> Vec<u8> {
        build(&[Component::Uuid(vertex_id), Component::UnsizedString(name)])
    }

    pub fn iterate_for_owner(&self, vertex_id: Uuid) -> Result<impl Iterator<Item = Result<OwnedPropertyItem>> + '_> {
        let prefix = build(&[Component::Uuid(vertex_id)]);
        let iterator = self.tree.scan_prefix(&prefix);

        Ok(iterator.map(move |item| -> Result<OwnedPropertyItem> {
            let (k, v) = item?;
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

        match self.tree.get(&key)? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(&self, vertex_id: Uuid, name: &str, value: &JsonValue) -> Result<()> {
        let key = self.key(vertex_id, name);
        let value_json = serde_json::to_vec(value)?;
        self.tree.insert(key.as_slice(), value_json.as_slice())?;
        Ok(())
    }

    pub fn delete(&self, vertex_id: Uuid, name: &str) -> Result<()> {
        self.tree.remove(&self.key(vertex_id, name))?;
        Ok(())
    }
}

pub struct EdgePropertyManager<'tree> {
    pub tree: &'tree Tree,
}

impl<'tree> EdgePropertyManager<'tree> {
    pub fn new(tree: &'tree Tree) -> Self {
        EdgePropertyManager { tree }
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
        &'a self,
        outbound_id: Uuid,
        t: &'a models::Type,
        inbound_id: Uuid,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgePropertyItem>> + 'a>> {
        let prefix = build(&[
            Component::Uuid(outbound_id),
            Component::Type(t),
            Component::Uuid(inbound_id),
        ]);

        let iterator = self.tree.scan_prefix(&prefix);

        let mapped = iterator.map(move |item| -> Result<EdgePropertyItem> {
            let (k, v) = item?;
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

        Ok(Box::new(mapped))
    }

    pub fn get(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> Result<Option<JsonValue>> {
        let key = self.key(outbound_id, t, inbound_id, name);

        match self.tree.get(&key)? {
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
        self.tree.insert(key.as_slice(), value_json.as_slice())?;
        Ok(())
    }

    pub fn delete(&self, outbound_id: Uuid, t: &models::Type, inbound_id: Uuid, name: &str) -> Result<()> {
        self.tree.remove(&self.key(outbound_id, t, inbound_id, name))?;
        Ok(())
    }
}
