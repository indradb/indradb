use std::collections::HashSet;
use std::io::Cursor;
use std::ops::Deref;
use std::result::Result as StdResult;

use crate::errors::Result;
use crate::models;
use crate::util;

use rocksdb::{ColumnFamilyRef, DBIterator, Direction, IteratorMode, WriteBatch, DB};
use uuid::Uuid;

pub type OwnedPropertyItem = (Uuid, models::Identifier, models::Json);
pub type EdgePropertyItem = (models::Edge, models::Identifier, models::Json);
pub type VertexPropertyValueKey = (models::Identifier, u64, Uuid);
pub type EdgePropertyValueKey = (models::Identifier, u64, models::Edge);
type RocksReadResult = StdResult<(Box<[u8]>, Box<[u8]>), rocksdb::Error>;

fn take_with_prefix(iterator: DBIterator<'_>, prefix: Vec<u8>) -> impl Iterator<Item = RocksReadResult> + '_ {
    iterator.take_while(move |item| -> bool {
        if let Ok((ref k, _)) = *item {
            k.starts_with(&prefix)
        } else {
            true
        }
    })
}

pub(crate) struct VertexManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> VertexManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        VertexManager {
            db,
            cf: db.cf_handle("vertices:v2").unwrap(),
        }
    }

    fn key(&self, id: Uuid) -> Vec<u8> {
        util::build(&[util::Component::Uuid(id)])
    }

    pub fn exists(&self, id: Uuid) -> Result<bool> {
        Ok(self.db.get_cf(&self.cf, self.key(id))?.is_some())
    }

    pub fn get(&self, id: Uuid) -> Result<Option<models::Identifier>> {
        match self.db.get_cf(&self.cf, self.key(id))? {
            Some(value_bytes) => {
                let mut cursor = Cursor::new(value_bytes.deref());
                unsafe { Ok(Some(util::read_identifier(&mut cursor)?)) }
            }
            None => Ok(None),
        }
    }

    pub fn iterate_for_range(&'a self, id: Uuid) -> impl Iterator<Item = Result<models::Vertex>> + 'a {
        let low_key = util::build(&[util::Component::Uuid(id)]);
        let iter = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&low_key, Direction::Forward));
        iter.map(|item| -> Result<models::Vertex> {
            let (k, v) = item?;

            let id = {
                debug_assert_eq!(k.len(), 16);
                let mut cursor = Cursor::new(k);
                util::read_uuid(&mut cursor)?
            };

            let mut cursor = Cursor::new(v);
            let t = unsafe { util::read_identifier(&mut cursor)? };
            Ok(models::Vertex::with_id(id, t))
        })
    }

    pub fn create(&self, batch: &mut WriteBatch, vertex: &models::Vertex) -> Result<()> {
        let key = self.key(vertex.id);
        batch.put_cf(&self.cf, &key, util::build(&[util::Component::Identifier(vertex.t)]));
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<models::Identifier>,
        id: Uuid,
    ) -> Result<()> {
        batch.delete_cf(&self.cf, self.key(id));

        let vertex_property_manager = VertexPropertyManager::new(self.db);
        for item in vertex_property_manager.iterate_for_owner(id)? {
            let (vertex_property_owner_id, vertex_property_name, _) = item?;
            vertex_property_manager.delete(
                batch,
                indexed_properties,
                vertex_property_owner_id,
                vertex_property_name,
            )?;
        }

        let edge_manager = EdgeManager::new(self.db);

        {
            let edge_range_manager = EdgeRangeManager::new(self.db);
            for item in edge_range_manager.iterate_for_root(id, None)? {
                let edge = item?;
                debug_assert_eq!(edge.outbound_id, id);
                edge_manager.delete(batch, indexed_properties, &edge)?;
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
            for item in reversed_edge_range_manager.iterate_for_root(id, None)? {
                let edge = item?;
                debug_assert_eq!(edge.outbound_id, id);
                edge_manager.delete(batch, indexed_properties, &edge)?;
            }
        }

        Ok(())
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct EdgeManager<'a> {
    db: &'a DB,
}

impl<'a> EdgeManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgeManager { db }
    }

    pub fn set(&self, batch: &mut WriteBatch, edge: &models::Edge) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db);
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
        let reversed_edge = edge.reversed();

        if edge_range_manager.contains(edge)? {
            edge_range_manager.delete(batch, edge)?;
            reversed_edge_range_manager.delete(batch, &reversed_edge)?;
        }

        edge_range_manager.set(batch, edge)?;
        reversed_edge_range_manager.set(batch, &reversed_edge)?;
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<models::Identifier>,
        edge: &models::Edge,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db);
        edge_range_manager.delete(batch, edge)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
        reversed_edge_range_manager.delete(batch, &edge.reversed())?;

        let edge_property_manager = EdgePropertyManager::new(self.db);
        for item in edge_property_manager.iterate_for_owner(edge)? {
            let (edge_property_edge, edge_property_name, _) = item?;
            edge_property_manager.delete(batch, indexed_properties, &edge_property_edge, edge_property_name)?;
        }

        Ok(())
    }
}

pub(crate) struct EdgeRangeManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> EdgeRangeManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgeRangeManager {
            db,
            cf: db.cf_handle("edge_ranges:v2").unwrap(),
        }
    }

    pub fn new_reversed(db: &'a DB) -> Self {
        EdgeRangeManager {
            db,
            cf: db.cf_handle("reversed_edge_ranges:v2").unwrap(),
        }
    }

    fn key(&self, edge: &models::Edge) -> Vec<u8> {
        util::build(&[
            util::Component::Uuid(edge.outbound_id),
            util::Component::Identifier(edge.t),
            util::Component::Uuid(edge.inbound_id),
        ])
    }

    fn iterate<I>(&'a self, iterator: I) -> impl Iterator<Item = Result<models::Edge>> + 'a
    where
        I: Iterator<Item = RocksReadResult> + 'a,
    {
        iterator.map(move |item| -> Result<models::Edge> {
            let (k, _) = item?;
            let mut cursor = Cursor::new(k);
            let first_id = util::read_uuid(&mut cursor)?;
            let t = unsafe { util::read_identifier(&mut cursor)? };
            let second_id = util::read_uuid(&mut cursor)?;
            Ok(models::Edge::new(first_id, t, second_id))
        })
    }

    pub fn contains(&self, edge: &models::Edge) -> Result<bool> {
        Ok(self.db.get_cf(&self.cf, self.key(edge))?.is_some())
    }

    pub fn iterate_for_root(
        &'a self,
        id: Uuid,
        t: Option<models::Identifier>,
    ) -> Result<Box<dyn Iterator<Item = Result<models::Edge>> + 'a>> {
        let (prefix, iter) = match t {
            Some(t) => {
                let prefix = util::build(&[util::Component::Uuid(id), util::Component::Identifier(t)]);
                let low_key = util::build(&[util::Component::Uuid(id), util::Component::Identifier(t)]);
                let iter = self
                    .db
                    .iterator_cf(&self.cf, IteratorMode::From(&low_key, Direction::Forward));
                (prefix, iter)
            }
            None => {
                let prefix = util::build(&[util::Component::Uuid(id)]);
                let iter = self
                    .db
                    .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));
                (prefix, iter)
            }
        };

        let iter = take_with_prefix(iter, prefix);
        Ok(Box::new(self.iterate(iter)))
    }

    pub fn iterate_for_range(
        &'a self,
        first_id: Uuid,
        t: models::Identifier,
        second_id: Uuid,
    ) -> Result<Box<dyn Iterator<Item = Result<models::Edge>> + 'a>> {
        let low_key = util::build(&[
            util::Component::Uuid(first_id),
            util::Component::Identifier(t),
            util::Component::Uuid(second_id),
        ]);
        let iter = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&low_key, Direction::Forward));
        Ok(Box::new(self.iterate(iter)))
    }

    pub fn iterate_for_all(&'a self) -> impl Iterator<Item = Result<models::Edge>> + 'a {
        let iterator = self.db.iterator_cf(&self.cf, IteratorMode::Start);
        self.iterate(iterator)
    }

    pub fn set(&self, batch: &mut WriteBatch, edge: &models::Edge) -> Result<()> {
        let key = self.key(edge);
        batch.put_cf(&self.cf, &key, []);
        Ok(())
    }

    pub fn delete(&self, batch: &mut WriteBatch, edge: &models::Edge) -> Result<()> {
        batch.delete_cf(&self.cf, self.key(edge));
        Ok(())
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct VertexPropertyManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> VertexPropertyManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        VertexPropertyManager {
            db,
            cf: db.cf_handle("vertex_properties:v2").unwrap(),
        }
    }

    fn key(&self, vertex_id: Uuid, name: models::Identifier) -> Vec<u8> {
        util::build(&[
            util::Component::Uuid(vertex_id),
            util::Component::FixedLengthString(&name.0),
        ])
    }

    pub fn iterate_for_owner(
        &'a self,
        vertex_id: Uuid,
    ) -> Result<impl Iterator<Item = Result<OwnedPropertyItem>> + 'a> {
        let prefix = util::build(&[util::Component::Uuid(vertex_id)]);

        let iterator = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = take_with_prefix(iterator, prefix);

        Ok(filtered.map(move |item| -> Result<OwnedPropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);
            let owner_id = util::read_uuid(&mut cursor)?;
            debug_assert_eq!(vertex_id, owner_id);
            let name_str = util::read_fixed_length_string(&mut cursor)?;
            let name = unsafe { models::Identifier::new_unchecked(name_str) };
            let value = serde_json::from_slice(&v)?;
            Ok((owner_id, name, value))
        }))
    }

    pub fn get(&self, vertex_id: Uuid, name: models::Identifier) -> Result<Option<models::Json>> {
        match self.db.get_cf(&self.cf, self.key(vertex_id, name))? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<models::Identifier>,
        vertex_id: Uuid,
        name: models::Identifier,
        value: &models::Json,
    ) -> Result<()> {
        let is_indexed = indexed_properties.contains(&name);
        let key = self.key(vertex_id, name);
        if is_indexed {
            self.delete(batch, indexed_properties, vertex_id, name)?;
        }
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(&self.cf, &key, &value_json);
        if is_indexed {
            let vertex_property_value_manager = VertexPropertyValueManager::new(self.db);
            vertex_property_value_manager.set(batch, vertex_id, name, value);
        }
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<models::Identifier>,
        vertex_id: Uuid,
        name: models::Identifier,
    ) -> Result<()> {
        if indexed_properties.contains(&name) {
            if let Some(value) = self.get(vertex_id, name)? {
                let vertex_property_value_manager = VertexPropertyValueManager::new(self.db);
                vertex_property_value_manager.delete(batch, vertex_id, name, &value);
            }
        }
        batch.delete_cf(&self.cf, self.key(vertex_id, name));
        Ok(())
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct EdgePropertyManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> EdgePropertyManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgePropertyManager {
            db,
            cf: db.cf_handle("edge_properties:v2").unwrap(),
        }
    }

    fn key(&self, edge: &models::Edge, name: models::Identifier) -> Vec<u8> {
        util::build(&[
            util::Component::Uuid(edge.outbound_id),
            util::Component::Identifier(edge.t),
            util::Component::Uuid(edge.inbound_id),
            util::Component::FixedLengthString(&name.0),
        ])
    }

    pub fn iterate_for_owner(
        &'a self,
        edge: &'a models::Edge,
    ) -> Result<Box<dyn Iterator<Item = Result<EdgePropertyItem>> + 'a>> {
        let prefix = util::build(&[
            util::Component::Uuid(edge.outbound_id),
            util::Component::Identifier(edge.t),
            util::Component::Uuid(edge.inbound_id),
        ]);

        let iterator = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = take_with_prefix(iterator, prefix);

        let mapped = filtered.map(move |item| -> Result<EdgePropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);

            let edge_property_out_id = util::read_uuid(&mut cursor)?;
            debug_assert_eq!(edge_property_out_id, edge.outbound_id);

            let edge_property_t = unsafe { util::read_identifier(&mut cursor)? };
            debug_assert_eq!(edge_property_t, edge.t);

            let edge_property_in_id = util::read_uuid(&mut cursor)?;
            debug_assert_eq!(edge_property_in_id, edge.inbound_id);

            let edge_property_name_str = util::read_fixed_length_string(&mut cursor)?;
            let edge_property_name = unsafe { models::Identifier::new_unchecked(edge_property_name_str) };

            let value = serde_json::from_slice(&v)?;
            let edge_property_edge = models::Edge::new(edge_property_out_id, edge_property_t, edge_property_in_id);
            Ok((edge_property_edge, edge_property_name, value))
        });

        Ok(Box::new(mapped))
    }

    pub fn get(&self, edge: &models::Edge, name: models::Identifier) -> Result<Option<models::Json>> {
        match self.db.get_cf(&self.cf, self.key(edge, name))? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<models::Identifier>,
        edge: &models::Edge,
        name: models::Identifier,
        value: &models::Json,
    ) -> Result<()> {
        let is_indexed = indexed_properties.contains(&name);
        let key = self.key(edge, name);
        if is_indexed {
            self.delete(batch, indexed_properties, edge, name)?;
        }
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(&self.cf, &key, &value_json);
        if is_indexed {
            let edge_property_value_manager = EdgePropertyValueManager::new(self.db);
            edge_property_value_manager.set(batch, edge, name, value);
        }
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<models::Identifier>,
        edge: &models::Edge,
        name: models::Identifier,
    ) -> Result<()> {
        if indexed_properties.contains(&name) {
            if let Some(value) = self.get(edge, name)? {
                let edge_property_value_manager = EdgePropertyValueManager::new(self.db);
                edge_property_value_manager.delete(batch, edge, name, &value);
            }
        }
        batch.delete_cf(&self.cf, self.key(edge, name));
        Ok(())
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct VertexPropertyValueManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> VertexPropertyValueManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        VertexPropertyValueManager {
            db,
            cf: db.cf_handle("vertex_property_values:v2").unwrap(),
        }
    }

    fn key(&self, property_name: models::Identifier, property_value: &models::Json, vertex_id: Uuid) -> Vec<u8> {
        util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
            util::Component::Uuid(vertex_id),
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
            let name = unsafe { util::read_identifier(&mut cursor)? };
            let value_hash = util::read_u64(&mut cursor)?;
            let vertex_id = util::read_uuid(&mut cursor)?;
            Ok((name, value_hash, vertex_id))
        })
    }

    pub fn iterate_for_name(
        &'a self,
        property_name: models::Identifier,
    ) -> impl Iterator<Item = Result<VertexPropertyValueKey>> + 'a {
        let prefix = util::build(&[util::Component::Identifier(property_name)]);
        let iter = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn iterate_for_value(
        &'a self,
        property_name: models::Identifier,
        property_value: &models::Json,
    ) -> impl Iterator<Item = Result<VertexPropertyValueKey>> + 'a {
        let prefix = util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
        ]);
        let iter = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        vertex_id: Uuid,
        property_name: models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, vertex_id);
        batch.put_cf(&self.cf, key, []);
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        vertex_id: Uuid,
        property_name: models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, vertex_id);
        batch.delete_cf(&self.cf, key);
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct EdgePropertyValueManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> EdgePropertyValueManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgePropertyValueManager {
            db,
            cf: db.cf_handle("edge_property_values:v2").unwrap(),
        }
    }

    fn key(&self, property_name: models::Identifier, property_value: &models::Json, edge: &models::Edge) -> Vec<u8> {
        util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
            util::Component::Uuid(edge.outbound_id),
            util::Component::Identifier(edge.t),
            util::Component::Uuid(edge.inbound_id),
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
            let name = unsafe { util::read_identifier(&mut cursor)? };
            let value_hash = util::read_u64(&mut cursor)?;
            let out_id = util::read_uuid(&mut cursor)?;
            let t = unsafe { util::read_identifier(&mut cursor)? };
            let in_id = util::read_uuid(&mut cursor)?;
            Ok((name, value_hash, models::Edge::new(out_id, t, in_id)))
        })
    }

    pub fn iterate_for_name(
        &'a self,
        property_name: models::Identifier,
    ) -> impl Iterator<Item = Result<EdgePropertyValueKey>> + 'a {
        let prefix = util::build(&[util::Component::Identifier(property_name)]);
        let iter = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn iterate_for_value(
        &'a self,
        property_name: models::Identifier,
        property_value: &models::Json,
    ) -> impl Iterator<Item = Result<EdgePropertyValueKey>> + 'a {
        let prefix = util::build(&[
            util::Component::Identifier(property_name),
            util::Component::Json(property_value),
        ]);
        let iter = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));
        self.iterate(iter, prefix)
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        edge: &models::Edge,
        property_name: models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, edge);
        batch.put_cf(&self.cf, key, []);
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        edge: &models::Edge,
        property_name: models::Identifier,
        property_value: &models::Json,
    ) {
        let key = self.key(property_name, property_value, edge);
        batch.delete_cf(&self.cf, key);
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}

pub(crate) struct MetadataManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> MetadataManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        MetadataManager {
            db,
            cf: db.cf_handle("metadata:v2").unwrap(),
        }
    }

    pub fn get_indexed_properties(&self) -> Result<HashSet<models::Identifier>> {
        match self.db.get_cf(&self.cf, "indexed_properties")? {
            Some(value_bytes) => Ok(bincode::deserialize(&value_bytes)?),
            None => Ok(HashSet::default()),
        }
    }

    pub fn set_indexed_properties(&self, batch: &mut WriteBatch, indices: &HashSet<models::Identifier>) -> Result<()> {
        let value_bytes = bincode::serialize(&indices)?;
        batch.put_cf(&self.cf, "indexed_properties", &value_bytes);
        Ok(())
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}
