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

    pub fn serialize_value(&self, value: &AccountValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(bincode_serde::serialize(value, SizeLimit::Infinite)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<AccountValue, Error> {
        let result = try!(bincode_serde::deserialize(value));
        Ok(result)
    }

	pub fn exists(&self, id: Uuid) -> Result<bool, Error> {
		match try!(self.db.get_cf(self.cf, &self.key(id))) {
			Some(_) => Ok(true),
			None => Ok(false)
		}
	}

    pub fn get(&self, id: Uuid) -> Result<Option<AccountValue>, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(id))) {
            Some(value_bytes) => Ok(Some(try!(self.deserialize_value(&value_bytes)))),
            None => Ok(None)
        }
    }

	pub fn create(&self, email: String) -> Result<(Uuid, String), Error> {
		let id = Uuid::new_v4();
		let salt = generate_random_secret();
		let secret = generate_random_secret();
		let hash = get_salted_hash(&salt[..], None, &secret[..]);
		let value = AccountValue::new(email, salt, hash);
        let value_bytes = try!(self.serialize_value(&value));
		try!(self.db.put_cf(self.cf, &self.key(id), &value_bytes[..]));
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

    pub fn serialize_value(&self, value: &VertexValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(bincode_serde::serialize(value, SizeLimit::Infinite)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<VertexValue, Error> {
        let result = try!(bincode_serde::deserialize(value));
        Ok(result)
    }

    pub fn exists(&self, id: Uuid) -> Result<bool, Error> {
		match try!(self.db.get_cf(self.cf, &self.key(id))) {
			Some(_) => Ok(true),
			None => Ok(false)
		}
	}

    pub fn get(&self, id: Uuid) -> Result<Option<VertexValue>, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(id))) {
            Some(value_bytes) => Ok(Some(try!(self.deserialize_value(&value_bytes)))),
            None => Ok(None)
        }
    }

    pub fn create(&self, t: models::Type, account_id: Uuid) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
		let value = VertexValue::new(account_id, t);
		let value_bytes = try!(self.serialize_value(&value));
		try!(self.db.put_cf(self.cf, &self.key(id), &value_bytes[..]));
		Ok(id)
    }

    pub fn update(&self, id: Uuid, value: &VertexValue) -> Result<(), Error> {
        try!(self.db.put_cf(self.cf, &self.key(id), &try!(self.serialize_value(value))));
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
        let edge_prefix_key = edge_manager.prefix_key_no_type(id);
        prefix_iterate!(edge_manager, &edge_prefix_key, key, value, {
            let (outbound_id, t, inbound_id) = parse_edge_key(&key);
            let edge_value = try!(edge_manager.deserialize_value(&value));
            try!(edge_manager.delete(&mut batch, outbound_id, t, inbound_id, &edge_value));
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

    pub fn prefix_key_no_type(&self, outbound_id: Uuid) -> Box<[u8]> {
        build_key(vec![KeyComponent::Uuid(outbound_id)])
    }

    pub fn serialize_value(&self, value: &EdgeValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(bincode_serde::serialize(value, SizeLimit::Infinite)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<EdgeValue, Error> {
        let result = try!(bincode_serde::deserialize(value));
        Ok(result)
    }

    pub fn exists(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<bool, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(outbound_id, t, inbound_id))) {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }

    pub fn get(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid) -> Result<Option<EdgeValue>, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(outbound_id, t, inbound_id))) {
            Some(value_bytes) => Ok(Some(try!(self.deserialize_value(&value_bytes)))),
            None => Ok(None)
        }
    }

    pub fn set(&self, mut batch: &mut WriteBatch, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, old_update_datetime: Option<NaiveDateTime>, new_update_datetime: NaiveDateTime, weight: models::Weight) -> Result<(), Error> {
        let edge_value = EdgeValue::new(new_update_datetime.timestamp(), weight);
        try!(batch.put_cf(self.cf, &self.key(outbound_id, t.clone(), inbound_id), &try!(self.serialize_value(&edge_value))));

        let edge_range_manager = EdgeRangeManager::new(self.db.clone());
        try!(edge_range_manager.update(&mut batch, outbound_id, t.clone(), inbound_id, old_update_datetime, new_update_datetime, weight));

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db.clone());
        edge_range_manager.update(&mut batch, inbound_id, t, outbound_id, old_update_datetime, new_update_datetime, weight)
    }

    pub fn delete(&self, mut batch: &mut WriteBatch, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, value: &EdgeValue) -> Result<(), Error> {
        let update_datetime = NaiveDateTime::from_timestamp(value.update_timestamp, 0);

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

    pub fn max_key_in_range(&self, first_id: Uuid, t: models::Type) -> Box<[u8]> {
        build_key(vec![
            KeyComponent::Uuid(first_id),
            KeyComponent::ShortSizedString(t.0),
            // NOTE: this suffers from the year 2038 problem, but we can't use
            // i64::MAX because chrono sees it as an invalid time
            KeyComponent::NaiveDateTime(NaiveDateTime::from_timestamp(i32::MAX as i64, 0))
        ])
    }

    pub fn serialize_value(&self, value: &EdgeRangeValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(bincode_serde::serialize(value, SizeLimit::Infinite)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<EdgeRangeValue, Error> {
        let result = try!(bincode_serde::deserialize(value));
        Ok(result)
    }

    pub fn update(&self, mut batch: &mut WriteBatch, first_id: Uuid, t: models::Type, second_id: Uuid, old_update_datetime: Option<NaiveDateTime>, new_update_datetime: NaiveDateTime, weight: models::Weight) -> Result<(), Error> {
        if let Some(old_update_datetime) = old_update_datetime {
            try!(self.delete(&mut batch, first_id, t.clone(), old_update_datetime));
        }

        let value = EdgeRangeValue::new(second_id, weight);
        try!(batch.put_cf(self.cf, &self.key(first_id, t, new_update_datetime), &try!(self.serialize_value(&value))));
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

    pub fn serialize_value(&self, value: &JsonValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(serde_json::to_vec(value)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<JsonValue, Error> {
        let result = try!(serde_json::from_slice(value));
        Ok(result)
    }

    pub fn get(&self, name: String) -> Result<Option<JsonValue>, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(name))) {
            Some(value_bytes) => Ok(Some(try!(self.deserialize_value(&value_bytes)))),
            None => Ok(None)
        }
    }

    pub fn set(&self, name: String, value: &JsonValue) -> Result<(), Error> {
        try!(self.db.put_cf(self.cf, &self.key(name), &try!(self.serialize_value(value))));
        Ok(())
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

    pub fn serialize_value(&self, value: &JsonValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(serde_json::to_vec(value)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<JsonValue, Error> {
        let result = try!(serde_json::from_slice(value));
        Ok(result)
    }

    pub fn get(&self, account_id: Uuid, name: String) -> Result<Option<JsonValue>, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(account_id, name))) {
            Some(value_bytes) => Ok(Some(try!(self.deserialize_value(&value_bytes)))),
            None => Ok(None)
        }
    }

    pub fn set(&self, account_id: Uuid, name: String, value: &JsonValue) -> Result<(), Error> {
        try!(self.db.put_cf(self.cf, &self.key(account_id, name), &try!(self.serialize_value(value))));
        Ok(())
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

    pub fn serialize_value(&self, value: &JsonValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(serde_json::to_vec(value)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<JsonValue, Error> {
        let result = try!(serde_json::from_slice(value));
        Ok(result)
    }

    pub fn get(&self, vertex_id: Uuid, name: String) -> Result<Option<JsonValue>, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(vertex_id, name))) {
            Some(value_bytes) => Ok(Some(try!(self.deserialize_value(&value_bytes)))),
            None => Ok(None)
        }
    }

    pub fn set(&self, vertex_id: Uuid, name: String, value: &JsonValue) -> Result<(), Error> {
        try!(self.db.put_cf(self.cf, &self.key(vertex_id, name), &try!(self.serialize_value(value))));
        Ok(())
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

    pub fn serialize_value(&self, value: &JsonValue) -> Result<Box<[u8]>, Error> {
        Ok(try!(serde_json::to_vec(value)).into_boxed_slice())
    }

    pub fn deserialize_value(&self, value: &[u8]) -> Result<JsonValue, Error> {
        let result = try!(serde_json::from_slice(value));
        Ok(result)
    }

    pub fn get(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, name: String) -> Result<Option<JsonValue>, Error> {
        match try!(self.db.get_cf(self.cf, &self.key(outbound_id, t, inbound_id, name))) {
            Some(value_bytes) => Ok(Some(try!(self.deserialize_value(&value_bytes)))),
            None => Ok(None)
        }
    }

    pub fn set(&self, outbound_id: Uuid, t: models::Type, inbound_id: Uuid, name: String, value: &JsonValue) -> Result<(), Error> {
        try!(self.db.put_cf(self.cf, &self.key(outbound_id, t, inbound_id, name), &try!(self.serialize_value(value))));
        Ok(())
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
