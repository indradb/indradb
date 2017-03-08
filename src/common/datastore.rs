/// This module exposes a proxy datastore and transaction that in turn call
/// actual datastore/transaction implementations. Ideally this would not be
/// necessary, and we'd rely on something like trait objects to get the job
/// done. However, rust is not flexible enough (yet) to support that.

use std::env;
use braid::{Datastore, Transaction, RocksdbDatastore, PostgresDatastore, Error, Vertex, Edge,
              PostgresTransaction, RocksdbTransaction, Type};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use chrono::{DateTime, UTC};

/// This macro is used to proxy most methods.
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

#[derive(Debug)]
pub enum ProxyDatastore {
    Postgres(PostgresDatastore),
    Rocksdb(RocksdbDatastore),
}

impl Datastore<ProxyTransaction> for ProxyDatastore {
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
                let transaction = pg.transaction(account_id)?;
                Ok(ProxyTransaction::Postgres(transaction))
            }
            ProxyDatastore::Rocksdb(ref r) => {
                let transaction = r.transaction(account_id)?;
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

#[derive(Debug)]
pub enum ProxyTransaction {
    Postgres(PostgresTransaction),
    Rocksdb(RocksdbTransaction),
}

impl Transaction for ProxyTransaction {
    fn get_vertex_range(&self, start_id: Uuid, limit: u16) -> Result<Vec<Vertex>, Error> {
        proxy_transaction!(self, get_vertex_range, start_id, limit)
    }

    fn get_vertex(&self, id: Uuid) -> Result<Vertex, Error> {
        proxy_transaction!(self, get_vertex, id)
    }

    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        proxy_transaction!(self, create_vertex, t)
    }

    fn set_vertex(&self, vertex: Vertex) -> Result<(), Error> {
        proxy_transaction!(self, set_vertex, vertex)
    }

    fn delete_vertex(&self, id: Uuid) -> Result<(), Error> {
        proxy_transaction!(self, delete_vertex, id)
    }

    fn get_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<Edge, Error> {
        proxy_transaction!(self, get_edge, outbound_id, t, inbound_id)
    }

    fn set_edge(&self, edge: Edge) -> Result<(), Error> {
        proxy_transaction!(self, set_edge, edge)
    }

    fn delete_edge(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid) -> Result<(), Error> {
        proxy_transaction!(self, delete_edge, outbound_id, t, inbound_id)
    }

    fn get_edge_count(&self, outbound_id: Uuid, t: Option<Type>) -> Result<u64, Error> {
        proxy_transaction!(self, get_edge_count, outbound_id, t)
    }

    fn get_edge_range(&self, outbound_id: Uuid, t: Option<Type>, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u16) -> Result<Vec<Edge>, Error> {
        proxy_transaction!(self, get_edge_range, outbound_id, t, high, low, limit)
    }

    fn get_reversed_edge_count(&self, inbound_id: Uuid, t: Option<Type>) -> Result<u64, Error> {
        proxy_transaction!(self, get_reversed_edge_count, inbound_id, t)
    }

    fn get_reversed_edge_range(&self, inbound_id: Uuid, t: Option<Type>, high: Option<DateTime<UTC>>, low: Option<DateTime<UTC>>, limit: u16) -> Result<Vec<Edge>, Error> {
        proxy_transaction!(
            self,
            get_reversed_edge_range,
            inbound_id,
            t,
            high,
            low,
            limit
        )
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
        proxy_transaction!(
            self,
            set_edge_metadata,
            outbound_id,
            t,
            inbound_id,
            key,
            value
        )
    }

    fn delete_edge_metadata(&self, outbound_id: Uuid, t: Type, inbound_id: Uuid, key: String) -> Result<(), Error> {
        proxy_transaction!(self, delete_edge_metadata, outbound_id, t, inbound_id, key)
    }

    fn commit(self) -> Result<(), Error> {
        match self {
            ProxyTransaction::Postgres(pg) => pg.commit(),
            ProxyTransaction::Rocksdb(r) => r.commit(),
        }
    }

    fn rollback(self) -> Result<(), Error> {
        match self {
            ProxyTransaction::Postgres(pg) => pg.rollback(),
            ProxyTransaction::Rocksdb(r) => r.rollback(),
        }
    }
}

/// Creates a new datastore.
///
/// This looks at the `DATABASE_URL` environment variable to figure out which
/// datastore to use. If it starts with `rocksdb://`, the rocksdb
/// implementation is used. If it starts with `postgres://`, the postgres
/// implementation is used.
///
/// # Errors
/// Returns an error if we are unable to figure out what kind of datastore to
/// use.
pub fn datastore() -> ProxyDatastore {
    let connection_string = env::var("DATABASE_URL").unwrap_or("rocksdb://.rdb".to_string());

    if connection_string.starts_with("rocksdb://") {
        let path = &connection_string[10..connection_string.len()];

        let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or("512".to_string());
        let max_open_files = max_open_files_str.parse::<i32>()
            .expect("Could not parse environment variable `ROCKSDB_MAX_OPEN_FILES`: must be an \
                     i32");

        let datastore = match RocksdbDatastore::new(path, Some(max_open_files)) {
            Ok(datastore) => datastore,
            Err(err) => panic!("Could not instantiate a rocksdb datastore: {:?}", err),
        };

        ProxyDatastore::Rocksdb(datastore)
    } else if connection_string.starts_with("postgres://") {
        let pool_size = match env::var("DATABASE_POOL_SIZE") {
            Ok(str_val) => {
                Some(str_val.parse()
                    .expect("Could not parse environment variable `DATABASE_POOL_SIZE`: must be \
                             a u32"))
            }
            Err(_) => None,
        };

        let secret = env::var("SECRET").unwrap_or("".to_string());
        let datastore = PostgresDatastore::new(pool_size, connection_string, secret);
        ProxyDatastore::Postgres(datastore)
    } else {
        panic!("Cannot parse environment variable `DATABASE_URL`");
    }
}
