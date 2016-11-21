// This module a proxy datastore and transaction that in turn call actual
// atastore/transaction implementations. Ideally this would not be necessary,
// and we'd rely on something like trait objects to get the job done. However,
// rust is not flexible enough (yet) to support that. 

use std::env;
use nutrino::{Datastore, Transaction, RocksdbDatastore, PostgresDatastore, Error, Vertex, Edge, PostgresTransaction, RocksdbTransaction, Type};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use chrono::NaiveDateTime;

macro_rules! proxy_datastore {
    ($this: expr, $name:ident, $($arg:tt)*) => (
        {
            match *$this {
                ProxyDatastore::Postgres(ref pg) => pg.$name($($arg)*),
                ProxyDatastore::Rocksdb(ref r) => r.$name($($arg)*)
            }
        }
    )
}

pub enum ProxyDatastore {
    Postgres(PostgresDatastore),
    Rocksdb(RocksdbDatastore)
}

impl Datastore<ProxyTransaction, Uuid> for ProxyDatastore {
    fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
		proxy_datastore!(self, has_account, account_id)
	}

	fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
		proxy_datastore!(self, create_account, email)
	}

	fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
		proxy_datastore!(self, delete_account, account_id)
	}

	fn auth(&self, account_id: Uuid, secret: String) -> Result<bool, Error> {
		proxy_datastore!(self, auth, account_id, secret)
	}

	fn transaction(&self, account_id: Uuid) -> Result<ProxyTransaction, Error> {
        match *self {
            ProxyDatastore::Postgres(ref pg) => {
                let transaction = try!(pg.transaction(account_id));
                Ok(ProxyTransaction::Postgres(transaction))
            },
            ProxyDatastore::Rocksdb(ref r) => {
                let transaction = try!(r.transaction(account_id));
                Ok(ProxyTransaction::Rocksdb(transaction))
            }
        }
	}
}

macro_rules! proxy_transaction {
    ($this: expr, $name:ident, $($arg:tt)*) => (
        {
            match *$this {
                ProxyTransaction::Postgres(ref pg) => pg.$name($($arg)*),
                ProxyTransaction::Rocksdb(ref r) => r.$name($($arg)*)
            }
        }
    )
}

pub enum ProxyTransaction {
    Postgres(PostgresTransaction),
    Rocksdb(RocksdbTransaction),
}

impl Transaction<Uuid> for ProxyTransaction {
    fn get_vertex(&self, id: Uuid) -> Result<Vertex<Uuid>, Error> {
		proxy_transaction!(self, get_vertex, id)
	}

	fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
		proxy_transaction!(self, create_vertex, t)
	}

	fn set_vertex(&self, vertex: Vertex<Uuid>) -> Result<(), Error> {
		proxy_transaction!(self, set_vertex, vertex)
	}

	fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
		proxy_transaction!(self, delete_vertex, id)
	}

	fn get_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<Edge<Uuid>, Error> {
		proxy_transaction!(self, get_edge, outbound_id, t, inbound_id)
	}

	fn set_edge(&self, edge: Edge<Uuid>) -> Result<(), Error> {
		proxy_transaction!(self, set_edge, edge)
	}

	fn delete_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<(), Error> {
		proxy_transaction!(self, delete_edge, outbound_id, t, inbound_id)
	}

	fn get_edge_count(&self, outbound_id: Uuid, t: Type) -> Result<u64, Error> {
		proxy_transaction!(self, get_edge_count, outbound_id, t)
	}

	fn get_edge_range(&self, outbound_id: Uuid, t: Type, offset: u64, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
		proxy_transaction!(self, get_edge_range, outbound_id, t, offset, limit)
	}

	fn get_edge_time_range(&self, outbound_id: Uuid, t: Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
		proxy_transaction!(self, get_edge_time_range, outbound_id, t, high, low, limit)
	}

	fn get_reversed_edge_count(&self, inbound_id: Uuid, t: Type) -> Result<u64, Error> {
		proxy_transaction!(self, get_reversed_edge_count, inbound_id, t)
	}

	fn get_reversed_edge_range(&self, inbound_id: Uuid, t: Type, offset: u64, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
		proxy_transaction!(self, get_reversed_edge_range, inbound_id, t, offset, limit)
	}

	fn get_reversed_edge_time_range(&self, inbound_id: Uuid, t: Type, high: Option<NaiveDateTime>, low: Option<NaiveDateTime>, limit: u16) -> Result<Vec<Edge<Uuid>>, Error> {
		proxy_transaction!(self, get_reversed_edge_time_range, inbound_id, t, high, low, limit)
	}

	fn get_global_metadata(&self, key: String) -> Result<JsonValue, Error> {
		proxy_transaction!(self, get_global_metadata, key)
	}

	fn set_global_metadata(&self, key: String, value: JsonValue) -> Result<(), Error> {
		proxy_transaction!(self, set_global_metadata, key, value)
	}

	fn delete_global_metadata(&self, key: String) -> Result<(), Error> {
		proxy_transaction!(self, delete_global_metadata, key)
	}

	fn get_account_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		proxy_transaction!(self, get_account_metadata, owner_id, key)
	}

	fn set_account_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		proxy_transaction!(self, set_account_metadata, owner_id, key, value)
	}

	fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		proxy_transaction!(self, delete_account_metadata, owner_id, key)
	}

	fn get_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<JsonValue, Error> {
		proxy_transaction!(self, get_vertex_metadata, owner_id, key)
	}

	fn set_vertex_metadata(&self, owner_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		proxy_transaction!(self, set_vertex_metadata, owner_id, key, value)
	}

	fn delete_vertex_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
		proxy_transaction!(self, delete_vertex_metadata, owner_id, key)
	}

	fn get_edge_metadata(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid, key: String) -> Result<JsonValue, Error> {
		proxy_transaction!(self, get_edge_metadata, outbound_id, t, inbound_id, key)
	}

	fn set_edge_metadata(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid, key: String, value: JsonValue) -> Result<(), Error> {
		proxy_transaction!(self, set_edge_metadata, outbound_id, t, inbound_id, key, value)
	}

	fn delete_edge_metadata(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid, key: String) -> Result<(), Error> {
		proxy_transaction!(self, delete_edge_metadata, outbound_id, t, inbound_id, key)
	}

	fn commit(self) -> Result<(), Error> {
		match self {
            ProxyTransaction::Postgres(pg) => pg.commit(),
            ProxyTransaction::Rocksdb(r) => r.commit()
        }
	}

	fn rollback(self) -> Result<(), Error> {
		match self {
            ProxyTransaction::Postgres(pg) => pg.rollback(),
            ProxyTransaction::Rocksdb(r) => r.rollback()
        }
	}
}


pub fn datastore() -> ProxyDatastore {
    let connection_string = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => "rocksdb://.rdb".to_string()
    };

    if connection_string.starts_with("rocksdb://") {
        let path = &connection_string[10..connection_string.len()];

		let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or("512".to_string());
		let max_open_files = max_open_files_str.parse::<i32>().expect("Could not parse environment variable `ROCKSDB_MAX_OPEN_FILES`: should be an i32");

        let datastore = match RocksdbDatastore::new(path.to_string(), Some(max_open_files)) {
            Ok(datastore) => datastore,
            Err(err) => panic!("Could not instantiate a rocksdb datastore: {:?}", err)
        };

        ProxyDatastore::Rocksdb(datastore)
    } else if connection_string.starts_with("postgres://") {
        let pool_size = match env::var("DATABASE_POOL_SIZE") {
            Ok(str_val) => Some(str_val.parse().expect("Invalid DATABASE_POOL_SIZE: Must be an integer")),
            Err(_) => None
        };

        let secret = env::var("SECRET").unwrap_or("".to_string());
        let datastore = PostgresDatastore::new(pool_size, connection_string, secret);
        ProxyDatastore::Postgres(datastore)
    } else {
        panic!("Cannot parse environment variable `DATABASE_URL`");
    }
}
