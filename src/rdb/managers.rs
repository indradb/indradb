use models;
use uuid::Uuid;
use errors::Error;
use util::{generate_random_secret, get_salted_hash};
use serde_json::Value as JsonValue;
use chrono::naive::datetime::NaiveDateTime;
use rocksdb::{DB, IteratorMode, Direction, WriteBatch, DBIterator};
use super::models::{AccountValue, EdgeValue, VertexValue};
use bincode::SizeLimit;
use bincode::serde as bincode_serde;
use std::sync::Arc;
use core::iter::Map;
use std::u8;
use serde_json;
use super::keys::*;
use librocksdb_sys::rocksdb_column_family_handle_t;
use std::i64;
use std::io::Cursor;
use std::i32;
use serde::{Serialize, Deserialize};

fn bincode_serialize_value<T: Serialize>(value: &T) -> Result<Box<[u8]>, Error> {
    let result = try!(bincode_serde::serialize(value, SizeLimit::Infinite));
    Ok(result.into_boxed_slice())
}

fn bincode_deserialize_value<T: Deserialize>(value: &[u8]) -> Result<T, Error> {
    let result = try!(bincode_serde::deserialize(value));
    Ok(result)
}

fn json_serialize_value(value: &JsonValue) -> Result<Box<[u8]>, Error> {
    let result = try!(serde_json::to_vec(value));
    Ok(result.into_boxed_slice())
}

fn json_deserialize_value(value: &[u8]) -> Result<JsonValue, Error> {
    let result = try!(serde_json::from_slice(value));
    Ok(result)
}

fn exists(db: &DB, cf: ColumnFamily, key: Box<[u8]>) -> Result<bool, Error> {
    match try!(db.get_cf(cf, &key)) {
        Some(_) => Ok(true),
        None => Ok(false)
    }
}

fn get_bincode<T: Deserialize>(db: &DB, cf: ColumnFamily, key: Box<[u8]>) -> Result<Option<T>, Error> {
    match try!(db.get_cf(cf, &key)) {
        Some(value_bytes) => Ok(Some(try!(bincode_deserialize_value(&value_bytes)))),
        None => Ok(None)
    }
}

fn set_bincode<T: Serialize>(db: &DB, cf: ColumnFamily, key: Box<[u8]>, value: &T) -> Result<(), Error> {
    try!(db.put_cf(cf, &key, &try!(bincode_serialize_value(value))));
    Ok(())
}

fn get_json(db: &DB, cf: ColumnFamily, key: Box<[u8]>) -> Result<Option<JsonValue>, Error> {
    match try!(db.get_cf(cf, &key)) {
        Some(value_bytes) => Ok(Some(try!(json_deserialize_value(&value_bytes)))),
        None => Ok(None)
    }
}

fn set_json(db: &DB, cf: ColumnFamily, key: Box<[u8]>, value: &JsonValue) -> Result<(), Error> {
    try!(db.put_cf(cf, &key, &try!(json_serialize_value(value))));
    Ok(())
}

fn take_while_prefixed<'a>(iterator: DBIterator, prefix: Box<[u8]>) -> Box<Iterator<Item=(Box<[u8]>, Box<[u8]>)> + 'a> {
    let filtered = iterator.take_while(move |item| -> bool {
        let (ref k, _) = *item;
        k.starts_with(&prefix)
    });

    Box::new(filtered)
}

fn iterate_metadata_for_owner<'a>(db: &DB, cf: ColumnFamily, id: Uuid) -> Result<Box<Iterator<Item=Result<((Uuid, String), JsonValue), Error>> + 'a>, Error> {
    let prefix = build_key(vec![ KeyComponent::Uuid(id) ]);
    let iterator = try!(db.iterator_cf(cf, IteratorMode::From(&prefix, Direction::Forward)));
    let filtered = take_while_prefixed(iterator, prefix);

    let mapped = filtered.map(move |item| -> Result<((Uuid, String), JsonValue), Error> {
        let (k, v) = item;
        let mut cursor = Cursor::new(k);
        let owner_id = read_uuid(&mut cursor);
        debug_assert_eq!(id, owner_id);
        let name = read_unsized_string(&mut cursor);
        let value = try!(json_deserialize_value(&v.to_owned()[..]));
        Ok(((owner_id, name), value))
    });

    Ok(Box::new(mapped))
}

type ColumnFamily = *mut rocksdb_column_family_handle_t;

pub struct AccountManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl AccountManager {
	pub fn new(db: Arc<DB>) -> Self {
        AccountManager {
            cf: *db.cf_handle("accounts:v1").unwrap(),
            db: db
        }
	}

	pub fn key(&self, id: Uuid) -> Box<[u8]> {
		build_key(vec![KeyComponent::Uuid(id)])
	}

	pub fn exists(&self, id: Uuid) -> Result<bool, Error> {
        exists(&self.db, self.cf, self.key(id))
	}

    pub fn get(&self, id: Uuid) -> Result<Option<AccountValue>, Error> {
        get_bincode(&self.db, self.cf, self.key(id))
    }

	pub fn create(&self, email: String) -> Result<(Uuid, String), Error> {
		let id = Uuid::new_v4();
		let salt = generate_random_secret();
		let secret = generate_random_secret();
		let hash = get_salted_hash(&salt[..], None, &secret[..]);
		let value = AccountValue::new(email, salt, hash);
        try!(set_bincode(&self.db, self.cf, self.key(id), &value));
		Ok((id, secret))
	}

	pub fn delete(&self, mut batch: &mut WriteBatch, id: Uuid) -> Result<(), Error> {
		try!(batch.delete_cf(self.cf, &self.key(id)));

		// NOTE: This currently does a sequential scan through all keys to
		// find which vertices to delete. This could be more efficient.
        let vertex_manager = VertexManager::new(self.db.clone());
        for item in try!(vertex_manager.iterate()) {
            let (vertex_id, vertex_value) = try!(item);

            if vertex_value.owner_id == id {
                try!(vertex_manager.delete(&mut batch, vertex_id));
            }
        }

        let account_metadata_manager = AccountMetadataManager::new(self.db.clone());

        for item in try!(account_metadata_manager.iterate_for_owner(id)) {
            let ((account_metadata_owner_id, account_metadata_name), _) = try!(item);
            try!(account_metadata_manager.delete(&mut batch, account_metadata_owner_id, account_metadata_name));
        }

		Ok(())
	}
}

pub struct VertexManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl VertexManager {
	pub fn new(db: Arc<DB>) -> Self {
        VertexManager {
            cf: *db.cf_handle("vertices:v1").unwrap(),
            db: db
        }
	}

	fn key(&self, id: Uuid) -> Box<[u8]> {
		build_key(vec![KeyComponent::Uuid(id)])
	}

    pub fn exists(&self, id: Uuid) -> Result<bool, Error> {
		exists(&self.db, self.cf, self.key(id))
	}

    pub fn get(&self, id: Uuid) -> Result<Option<VertexValue>, Error> {
        get_bincode(&self.db, self.cf, self.key(id))
    }

    pub fn iterate<'a>(&'a self) -> Result<Box<Iterator<Item=Result<(Uuid, VertexValue), Error>> + 'a>, Error> {
        let iterator = try!(self.db.iterator_cf(self.cf, IteratorMode::From(b"", Direction::Forward)));

        let mapped = iterator.map(|item| -> Result<(Uuid, VertexValue), Error> {
            let (k, v) = item;
            let id = parse_uuid_key(k);
            let value = try!(bincode_deserialize_value::<VertexValue>(&v.to_owned()[..]));
            Ok((id, value))
        });

        Ok(Box::new(mapped))
    }

    pub fn create(&self, t: models::Type, account_id: Uuid) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
		let value = VertexValue::new(account_id, t);
        try!(set_bincode(&self.db, self.cf, self.key(id), &value));
		Ok(id)
    }

    pub fn update(&self, id: Uuid, value: &VertexValue) -> Result<(), Error> {
        try!(set_bincode(&self.db, self.cf, self.key(id), value));
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, id: Uuid) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(id)));

        let vertex_metadata_manager = VertexMetadataManager::new(self.db.clone());
        for item in try!(vertex_metadata_manager.iterate_for_owner(id)) {
            let ((vertex_metadata_owner_id, vertex_metadata_name), _) = try!(item);
            try!(vertex_metadata_manager.delete(&mut batch, vertex_metadata_owner_id, vertex_metadata_name));
        }

        let edge_manager = EdgeManager::new(self.db.clone());

        {
            let edge_range_manager = EdgeRangeManager::new(self.db.clone());
            for item in try!(edge_range_manager.iterate_for_owner(id)) {
                let ((edge_range_outbound_id, edge_range_t, edge_range_update_datetime, edge_range_inbound_id), _) = try!(item);
                debug_assert_eq!(edge_range_outbound_id, id);
                try!(edge_manager.delete(&mut batch, edge_range_outbound_id, edge_range_t, edge_range_inbound_id, edge_range_update_datetime));
            }
        }

        {
            let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
            for item in try!(reversed_edge_range_manager.iterate_for_owner(id)) {
                let ((reversed_edge_range_inbound_id, reversed_edge_range_t, reversed_edge_range_update_datetime, reversed_edge_range_outbound_id), _) = try!(item);
                debug_assert_eq!(reversed_edge_range_inbound_id, id);
                try!(edge_manager.delete(&mut batch, reversed_edge_range_outbound_id, reversed_edge_range_t, reversed_edge_range_inbound_id, reversed_edge_range_update_datetime));
            }
        }

        Ok(())
    }
}

pub struct EdgeManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl EdgeManager {
	pub fn new(db: Arc<DB>) -> Self {
        EdgeManager {
            cf: *db.cf_handle("edges:v1").unwrap(),
            db: db
        }
	}

	fn key(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::ShortSizedString(t.0),
            KeyComponent::Uuid(inbound_id)
        ])
	}

    pub fn exists(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<bool, Error> {
        exists(&self.db, self.cf, self.key(outbound_id, t, inbound_id))
    }

    pub fn get(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<Option<EdgeValue>, Error> {
        get_bincode(&self.db, self.cf, self.key(outbound_id, t, inbound_id))
    }

    pub fn set(&self, mut batch: &mut WriteBatch, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, new_update_datetime: NaiveDateTime, weight: models::Weight) -> Result<(), Error> {
        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());

        if let Some(existing_edge_value) = try!(self.get(outbound_id, t.clone(), inbound_id)) {
            let old_update_datetime = NaiveDateTime::from_timestamp(existing_edge_value.update_timestamp, 0);
            try!(edge_range_manager.delete(&mut batch, outbound_id, t.clone(), old_update_datetime, inbound_id));
            try!(reversed_edge_range_manager.delete(&mut batch, outbound_id, t.clone(), old_update_datetime, inbound_id));
        }

        let new_edge_value = EdgeValue::new(new_update_datetime.timestamp(), weight);
        try!(set_bincode(&self.db, self.cf, self.key(outbound_id, t.clone(), inbound_id), &new_edge_value));
        try!(edge_range_manager.set(&mut batch, outbound_id, t.clone(), new_update_datetime, inbound_id, weight));
        try!(reversed_edge_range_manager.set(&mut batch, inbound_id, t, new_update_datetime, outbound_id, weight));
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, update_datetime: NaiveDateTime) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(outbound_id, t.clone(), inbound_id)));

        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        try!(edge_range_manager.delete(&mut batch, outbound_id, t.clone(), update_datetime, inbound_id));

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        try!(reversed_edge_range_manager.delete(&mut batch, inbound_id, t.clone(), update_datetime, inbound_id));

        let edge_metadata_manager = EdgeMetadataManager::new(self.db.clone());
        for item in try!(edge_metadata_manager.iterate_for_owner(outbound_id, t, inbound_id)) {
            let ((edge_metadata_outbound_id, edge_metadata_t, edge_metadata_inbound_id, edge_metadata_name), _) = try!(item);
            try!(edge_metadata_manager.delete(&mut batch, edge_metadata_outbound_id, edge_metadata_t, edge_metadata_inbound_id, edge_metadata_name));
        }

        Ok(())
    }
}

pub struct EdgeRangeManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl EdgeRangeManager {
	pub fn new(db: Arc<DB>) -> Self {
        EdgeRangeManager {
            cf: *db.cf_handle("edge_ranges:v1").unwrap(),
            db: db
        }
	}

    pub fn new_reversed(db: Arc<DB>) -> Self {
        EdgeRangeManager {
            cf: *db.cf_handle("reversed_edge_ranges:v1").unwrap(),
            db: db
        }
    }

	fn key(&self, first_id: Uuid, t: models::Type, update_datetime: NaiveDateTime, second_id: Uuid) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(first_id),
            KeyComponent::ShortSizedString(t.0),
            KeyComponent::NaiveDateTime(update_datetime),
            KeyComponent::Uuid(second_id)
        ])
	}

    fn iterate<'a>(&self, iterator: DBIterator, prefix: Box<[u8]>) -> Result<Box<Iterator<Item=Result<((Uuid, models::Type, NaiveDateTime, Uuid), models::Weight), Error>> + 'a>, Error> {
        let filtered = take_while_prefixed(iterator, prefix);

        let mapped = filtered.map(move |item| -> Result<((Uuid, models::Type, NaiveDateTime, Uuid), models::Weight), Error> {
            let (k, v) = item;
            let mut cursor = Cursor::new(k);
            let first_id = read_uuid(&mut cursor);
            let t = read_type(&mut cursor);
            let update_datetime = read_datetime(&mut cursor);
            let second_id = read_uuid(&mut cursor);
            let weight = try!(bincode_deserialize_value::<models::Weight>(&v.to_owned()[..]));
            Ok(((first_id, t, update_datetime, second_id), weight))
        });

        Ok(Box::new(mapped))
    }

    pub fn iterate_for_range<'a>(&self, id: Uuid, t: models::Type, high: Option<NaiveDateTime>) -> Result<Box<Iterator<Item=Result<((Uuid, models::Type, NaiveDateTime, Uuid), models::Weight), Error>> + 'a>, Error> {
        let high = high.unwrap_or(max_datetime());        
        let prefix = build_key(vec![ KeyComponent::Uuid(id), KeyComponent::ShortSizedString(t.0), KeyComponent::NaiveDateTime(high) ]);
        let iterator = try!(self.db.iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward)));
        self.iterate(iterator, prefix)
    }

    pub fn iterate_for_owner<'a>(&self, id: Uuid) -> Result<Box<Iterator<Item=Result<((Uuid, models::Type, NaiveDateTime, Uuid), models::Weight), Error>> + 'a>, Error> {
        let prefix = build_key(vec![ KeyComponent::Uuid(id) ]);
        let iterator = try!(self.db.iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward)));
        self.iterate(iterator, prefix)
    }

    pub fn set(&self, mut batch: &mut WriteBatch, first_id: Uuid, t: models::Type, update_datetime: NaiveDateTime, second_id: Uuid, weight: models::Weight) -> Result<(), Error> {
        let key = self.key(first_id, t, update_datetime, second_id);
        let value = try!(bincode_serialize_value(&weight));
        try!(batch.put_cf(self.cf, &key, &value));
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, first_id: Uuid, t: models::Type, update_datetime: NaiveDateTime, second_id: Uuid) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(first_id, t, update_datetime, second_id)));
        Ok(())
    }
}

pub struct GlobalMetadataManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl GlobalMetadataManager {
	pub fn new(db: Arc<DB>) -> Self {
        GlobalMetadataManager {
            cf: *db.cf_handle("global_metadata:v1").unwrap(),
            db: db
        }
	}

	fn key(&self, name: String) -> Box<[u8]> {
		build_key(vec![ KeyComponent::UnsizedString(name) ])
	}

    pub fn get(&self, name: String) -> Result<Option<JsonValue>, Error> {
        get_json(&self.db, self.cf, self.key(name))
    }

    pub fn set(&self, name: String, value: &JsonValue) -> Result<(), Error> {
        set_json(&self.db, self.cf, self.key(name), value)
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, name: String) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(name)));
        Ok(())
    }
}

pub struct AccountMetadataManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl AccountMetadataManager {
	pub fn new(db: Arc<DB>) -> Self {
        AccountMetadataManager {
            cf: *db.cf_handle("account_metadata:v1").unwrap(),
            db: db
        }
	}

	fn key(&self, account_id: Uuid, name: String) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(account_id),
            KeyComponent::UnsizedString(name)
        ])
	}

    pub fn iterate_for_owner<'a>(&'a self, account_id: Uuid) -> Result<Box<Iterator<Item=Result<((Uuid, String), JsonValue), Error>> + 'a>, Error> {
        iterate_metadata_for_owner(&self.db, self.cf, account_id)
    }

    pub fn get(&self, account_id: Uuid, name: String) -> Result<Option<JsonValue>, Error> {
        get_json(&self.db, self.cf, self.key(account_id, name))
    }

    pub fn set(&self, account_id: Uuid, name: String, value: &JsonValue) -> Result<(), Error> {
        set_json(&self.db, self.cf, self.key(account_id, name), value)
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, account_id: Uuid, name: String) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(account_id, name)));
        Ok(())
    }
}

pub struct VertexMetadataManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl VertexMetadataManager {
	pub fn new(db: Arc<DB>) -> Self {
        VertexMetadataManager {
            cf: *db.cf_handle("vertex_metadata:v1").unwrap(),
            db: db
        }
	}

	fn key(&self, vertex_id: Uuid, name: String) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(vertex_id),
            KeyComponent::UnsizedString(name)
        ])
	}

    pub fn iterate_for_owner<'a>(&'a self, vertex_id: Uuid) -> Result<Box<Iterator<Item=Result<((Uuid, String), JsonValue), Error>> + 'a>, Error> {
        iterate_metadata_for_owner(&self.db, self.cf, vertex_id)
    }

    pub fn get(&self, vertex_id: Uuid, name: String) -> Result<Option<JsonValue>, Error> {
        get_json(&self.db, self.cf, self.key(vertex_id, name))
    }

    pub fn set(&self, vertex_id: Uuid, name: String, value: &JsonValue) -> Result<(), Error> {
        set_json(&self.db, self.cf, self.key(vertex_id, name), value)
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, vertex_id: Uuid, name: String) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(vertex_id, name)));
        Ok(())
    }
}

pub struct EdgeMetadataManager {
	pub db: Arc<DB>,
    pub cf: ColumnFamily
}

impl EdgeMetadataManager {
	pub fn new(db: Arc<DB>) -> Self {
        EdgeMetadataManager {
            cf: *db.cf_handle("edge_metadata:v1").unwrap(),
            db: db
        }
	}

	fn key(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, name: String) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::ShortSizedString(t.0),
            KeyComponent::Uuid(inbound_id),
            KeyComponent::UnsizedString(name)
        ])
	}

    pub fn iterate_for_owner<'a>(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<Box<Iterator<Item=Result<((Uuid, models::Type, Uuid, String), JsonValue), Error>> + 'a>, Error> {
        let prefix = build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::ShortSizedString(t.0.clone()),
            KeyComponent::Uuid(inbound_id)
        ]);

        let iterator = try!(self.db.iterator_cf(self.cf, IteratorMode::From(&prefix, Direction::Forward)));
        let filtered = take_while_prefixed(iterator, prefix);

        let mapped = filtered.map(move |item| -> Result<((Uuid, models::Type, Uuid, String), JsonValue), Error> {
            let (k, v) = item;
            let mut cursor = Cursor::new(k);

            let edge_metadata_outbound_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_metadata_outbound_id, outbound_id);
            
            let edge_metadata_t = read_type(&mut cursor);
            debug_assert_eq!(edge_metadata_t, t);

            let edge_metadata_inbound_id = read_uuid(&mut cursor);
            debug_assert_eq!(edge_metadata_inbound_id, inbound_id);

            let edge_metadata_name = read_unsized_string(&mut cursor);

            let value = try!(json_deserialize_value(&v.to_owned()[..]));
            Ok(((edge_metadata_outbound_id, edge_metadata_t, edge_metadata_inbound_id, edge_metadata_name), value))
        });

        Ok(Box::new(mapped))
    }

    pub fn get(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, name: String) -> Result<Option<JsonValue>, Error> {
        get_json(&self.db, self.cf, self.key(outbound_id, t, inbound_id, name))
    }

    pub fn set(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, name: String, value: &JsonValue) -> Result<(), Error> {
        set_json(&self.db, self.cf, self.key(outbound_id, t, inbound_id, name), value)
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, name: String) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(outbound_id, t, inbound_id, name)));
        Ok(())
    }
}
