use models;
use uuid::Uuid;
use errors::Error;
use util::{generate_random_secret, get_salted_hash};
use serde_json::Value as JsonValue;
use chrono::naive::datetime::NaiveDateTime;
use rocksdb::{DB, IteratorMode, Direction, WriteBatch};
use super::models::{AccountValue, EdgeValue, VertexValue, EdgeRangeValue};
use bincode::SizeLimit;
use bincode::serde as bincode_serde;
use std::sync::Arc;
use std::u8;
use serde_json;
use super::keys::*;
use librocksdb_sys::rocksdb_column_family_handle_t;
use std::i64;
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
		prefix_iterate!(vertex_manager, b"", key, value, {
            let vertex_value = try!(vertex_manager.deserialize_value(&value.to_owned()[..]));

			if vertex_value.owner_id == id {
				let vertex_id = parse_uuid_key(&key);
                try!(vertex_manager.delete(&mut batch, vertex_id));
			}
		});

        let account_metadata_manager = AccountMetadataManager::new(self.db.clone());
        let account_metadata_prefix_key = account_metadata_manager.prefix_key(id);
        prefix_iterate!(account_metadata_manager, &account_metadata_prefix_key, key, value, {
            try!(account_metadata_manager.delete_from_key(&mut batch, key));
		});

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

	pub fn key(&self, id: Uuid) -> Box<[u8]> {
		build_key(vec![KeyComponent::Uuid(id)])
	}

    pub fn deserialize_value(&self, value: &[u8]) -> Result<VertexValue, Error> {
        bincode_deserialize_value(value)
    }

    pub fn exists(&self, id: Uuid) -> Result<bool, Error> {
		exists(&self.db, self.cf, self.key(id))
	}

    pub fn get(&self, id: Uuid) -> Result<Option<VertexValue>, Error> {
        get_bincode(&self.db, self.cf, self.key(id))
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
        let vertex_metadata_prefix_key = vertex_metadata_manager.prefix_key(id);
        prefix_iterate!(vertex_metadata_manager, &vertex_metadata_prefix_key, key, value, {
            try!(vertex_metadata_manager.delete_from_key(&mut batch, key));
        });

        let edge_manager = EdgeManager::new(self.db.clone());

        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        let edge_range_prefix_key = edge_range_manager.prefix_key_no_type(id);
        prefix_iterate!(edge_range_manager, &edge_range_prefix_key, key, value, {
            let (outbound_id, t, update_datetime) = parse_edge_range_key(&key);
            assert!(outbound_id == id);
            let edge_value = try!(edge_range_manager.deserialize_value(&value));
            try!(edge_manager.delete(&mut batch, outbound_id, t.clone(), edge_value.other_id, update_datetime));
        });

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        let reversed_edge_range_prefix_key = reversed_edge_range_manager.prefix_key_no_type(id);
        prefix_iterate!(reversed_edge_range_manager, &reversed_edge_range_prefix_key, key, value, {
            let (inbound_id, t, update_datetime) = parse_edge_range_key(&key);
            assert!(inbound_id == id);
            let edge_value = try!(reversed_edge_range_manager.deserialize_value(&value));
            try!(edge_manager.delete(&mut batch, edge_value.other_id, t, inbound_id, update_datetime));
        });

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

	pub fn key(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Box<[u8]> {
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

    pub fn set(&self, mut batch: &mut WriteBatch, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, old_update_datetime: Option<NaiveDateTime>, new_update_datetime: NaiveDateTime, weight: models::Weight) -> Result<(), Error> {
        let edge_value = EdgeValue::new(new_update_datetime.timestamp(), weight);
        try!(set_bincode(&self.db, self.cf, self.key(outbound_id, t.clone(), inbound_id), &edge_value));
        
        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        try!(edge_range_manager.update(&mut batch, outbound_id, t.clone(), inbound_id, old_update_datetime, new_update_datetime, weight));

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        try!(reversed_edge_range_manager.update(&mut batch, inbound_id, t, outbound_id, old_update_datetime, new_update_datetime, weight));
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, update_datetime: NaiveDateTime) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(outbound_id, t.clone(), inbound_id)));

        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        try!(edge_range_manager.delete(&mut batch, outbound_id, t.clone(), update_datetime));

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        try!(reversed_edge_range_manager.delete(&mut batch, inbound_id, t.clone(), update_datetime));

        let edge_metadata_manager = EdgeMetadataManager::new(self.db.clone());
        let edge_metadata_prefix_key = edge_metadata_manager.prefix_key(outbound_id, t, inbound_id);
        prefix_iterate!(edge_metadata_manager, &edge_metadata_prefix_key, key, value, {
            try!(edge_metadata_manager.delete_from_key(&mut batch, key));
        });

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

	pub fn key(&self, first_id: Uuid, t: models::Type, update_datetime: NaiveDateTime) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(first_id),
            KeyComponent::ShortSizedString(t.0),
            KeyComponent::NaiveDateTime(update_datetime)
        ])
	}

    pub fn prefix_key(&self, first_id: Uuid, t: models::Type) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(first_id),
            KeyComponent::ShortSizedString(t.0)
        ])
	}

    pub fn prefix_key_no_type(&self, first_id: Uuid) -> Box<[u8]> {
        build_key(vec![ KeyComponent::Uuid(first_id) ])
    }

    pub fn max_key_in_range(&self, first_id: Uuid, t: models::Type) -> Box<[u8]> {
        build_key(vec![
            KeyComponent::Uuid(first_id),
            KeyComponent::ShortSizedString(t.0),
            // NOTE: this suffers from the year 2038 problem, but we can't use
            // i64::MAX because chrono sees it as an invalid time
            KeyComponent::NaiveDateTime(NaiveDateTime::from_timestamp(i32::MAX as i64, 0))
        ])
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<EdgeRangeValue, Error> {
        bincode_deserialize_value(value)
    }

    pub fn update(&self, mut batch: &mut WriteBatch, first_id: Uuid, t: models::Type, second_id: Uuid, old_update_datetime: Option<NaiveDateTime>, new_update_datetime: NaiveDateTime, weight: models::Weight) -> Result<(), Error> {
        if let Some(old_update_datetime) = old_update_datetime {
            try!(self.delete(&mut batch, first_id, t.clone(), old_update_datetime));
        }

        let value = EdgeRangeValue::new(second_id, weight);
        try!(set_bincode(&self.db, self.cf, self.key(first_id, t, new_update_datetime), &value));
        Ok(())
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, first_id: Uuid, t: models::Type, update_datetime: NaiveDateTime) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &self.key(first_id, t, update_datetime)));
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

	pub fn key(&self, name: String) -> Box<[u8]> {
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

	pub fn key(&self, account_id: Uuid, name: String) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(account_id),
            KeyComponent::UnsizedString(name)
        ])
	}

    pub fn prefix_key(&self, account_id: Uuid) -> Box<[u8]> {
        build_key(vec![ KeyComponent::Uuid(account_id) ])
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

    pub fn delete_from_key(&self, mut batch: &mut WriteBatch, key: Box<[u8]>) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &key));
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

	pub fn key(&self, vertex_id: Uuid, name: String) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(vertex_id),
            KeyComponent::UnsizedString(name)
        ])
	}

    pub fn prefix_key(&self, vertex_id: Uuid) -> Box<[u8]> {
        build_key(vec![ KeyComponent::Uuid(vertex_id) ])
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

    pub fn delete_from_key(&self, mut batch: &mut WriteBatch, key: Box<[u8]>) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &key));
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

	pub fn key(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, name: String) -> Box<[u8]> {
		build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::ShortSizedString(t.0),
            KeyComponent::Uuid(inbound_id),
            KeyComponent::UnsizedString(name)
        ])
	}

    pub fn prefix_key(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Box<[u8]> {
        build_key(vec![
            KeyComponent::Uuid(outbound_id),
            KeyComponent::ShortSizedString(t.0),
            KeyComponent::Uuid(inbound_id)
        ])
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

    pub fn delete_from_key(&self, mut batch: &mut WriteBatch, key: Box<[u8]>) -> Result<(), Error> {
        try!(batch.delete_cf(self.cf, &key));
        Ok(())
    }
}
