/// This module exposes a proxy datastore and transaction that in turn call
/// actual datastore/transaction implementations. Ideally this would not be
/// necessary, and we'd rely on something like trait objects to get the job
/// done. However, rust is not flexible enough (yet) to support that.

use std::env;
use braid::{Datastore, Transaction, RocksdbDatastore, PostgresDatastore, Error, Vertex, Edge,
            PostgresTransaction, RocksdbTransaction, Type, VertexQuery, EdgeQuery, Weight, EdgeKey,
            MemoryDatastore, MemoryTransaction};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// This macro is used to proxy most methods.
macro_rules! proxy_datastore {
    ($this: expr, $name:ident, $($arg:tt)*) => (
        {
            match *$this {
                ProxyDatastore::Postgres(ref pg) => pg.$name($($arg)*),
                ProxyDatastore::Rocksdb(ref r) => r.$name($($arg)*),
                ProxyDatastore::Memory(ref r) => r.$name($($arg)*)
            }
        }
    )
}

#[derive(Debug)]
pub enum ProxyDatastore {
    Postgres(PostgresDatastore),
    Rocksdb(RocksdbDatastore),
    Memory(MemoryDatastore)
}

impl Datastore<ProxyTransaction> for ProxyDatastore {
    fn has_account(&self, account_id: Uuid) -> Result<bool, Error> {
        proxy_datastore!(self, has_account, account_id)
    }

    fn create_account(&self) -> Result<(Uuid, String), Error> {
        proxy_datastore!(self, create_account,)
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
            },
            ProxyDatastore::Memory(ref mem) => {
                let transaction = mem.transaction(account_id)?;
                Ok(ProxyTransaction::Memory(transaction))
            }
        }
    }
}

macro_rules! proxy_transaction {
    ($this: expr, $name:ident, $($arg:tt)*) => (
        {
            match *$this {
                ProxyTransaction::Postgres(ref pg) => pg.$name($($arg)*),
                ProxyTransaction::Rocksdb(ref r) => r.$name($($arg)*),
                ProxyTransaction::Memory(ref mem) => mem.$name($($arg)*)
            }
        }
    )
}

#[derive(Debug)]
pub enum ProxyTransaction {
    Postgres(PostgresTransaction),
    Rocksdb(RocksdbTransaction),
    Memory(MemoryTransaction)
}

impl Transaction for ProxyTransaction {
    fn get_vertices(&self, q: VertexQuery) -> Result<Vec<Vertex>, Error> {
        proxy_transaction!(self, get_vertices, q)
    }

    fn create_vertex(&self, t: Type) -> Result<Uuid, Error> {
        proxy_transaction!(self, create_vertex, t)
    }

    fn delete_vertices(&self, q: VertexQuery) -> Result<(), Error> {
        proxy_transaction!(self, delete_vertices, q)
    }

    fn create_edge(&self, key: EdgeKey, weight: Weight) -> Result<(), Error> {
        proxy_transaction!(self, create_edge, key, weight)
    }

    fn get_edges(&self, q: EdgeQuery) -> Result<Vec<Edge>, Error> {
        proxy_transaction!(self, get_edges, q)
    }

    fn delete_edges(&self, q: EdgeQuery) -> Result<(), Error> {
        proxy_transaction!(self, delete_edges, q)
    }

    fn get_edge_count(&self, q: EdgeQuery) -> Result<u64, Error> {
        proxy_transaction!(self, get_edge_count, q)
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

    fn set_account_metadata(
        &self,
        owner_id: Uuid,
        key: String,
        value: JsonValue,
    ) -> Result<(), Error> {
        proxy_transaction!(self, set_account_metadata, owner_id, key, value)
    }

    fn delete_account_metadata(&self, owner_id: Uuid, key: String) -> Result<(), Error> {
        proxy_transaction!(self, delete_account_metadata, owner_id, key)
    }

    fn get_vertex_metadata(
        &self,
        q: VertexQuery,
        key: String,
    ) -> Result<HashMap<Uuid, JsonValue>, Error> {
        proxy_transaction!(self, get_vertex_metadata, q, key)
    }

    fn set_vertex_metadata(
        &self,
        q: VertexQuery,
        key: String,
        value: JsonValue,
    ) -> Result<(), Error> {
        proxy_transaction!(self, set_vertex_metadata, q, key, value)
    }

    fn delete_vertex_metadata(&self, q: VertexQuery, key: String) -> Result<(), Error> {
        proxy_transaction!(self, delete_vertex_metadata, q, key)
    }

    fn get_edge_metadata(
        &self,
        q: EdgeQuery,
        key: String,
    ) -> Result<HashMap<EdgeKey, JsonValue>, Error> {
        proxy_transaction!(self, get_edge_metadata, q, key)
    }

    fn set_edge_metadata(&self, q: EdgeQuery, key: String, value: JsonValue) -> Result<(), Error> {
        proxy_transaction!(self, set_edge_metadata, q, key, value)
    }

    fn delete_edge_metadata(&self, q: EdgeQuery, key: String) -> Result<(), Error> {
        proxy_transaction!(self, delete_edge_metadata, q, key)
    }

    fn commit(self) -> Result<(), Error> {
        match self {
            ProxyTransaction::Postgres(pg) => pg.commit(),
            ProxyTransaction::Rocksdb(r) => r.commit(),
            ProxyTransaction::Memory(mem) => mem.commit()
        }
    }

    fn rollback(self) -> Result<(), Error> {
        match self {
            ProxyTransaction::Postgres(pg) => pg.rollback(),
            ProxyTransaction::Rocksdb(r) => r.rollback(),
            ProxyTransaction::Memory(mem) => mem.rollback()
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
    let connection_string = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "memory://".to_string());
    let secure_uuids = env::var("BRAID_SECURE_UUIDS").unwrap_or_else(|_| "false".to_string()) ==
        "true";

    if connection_string.starts_with("rocksdb://") {
        let path = &connection_string[10..connection_string.len()];

        let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES")
            .unwrap_or_else(|_| "512".to_string());
        let max_open_files = max_open_files_str.parse::<i32>().expect(
            "Could not parse environment variable `ROCKSDB_MAX_OPEN_FILES`: must be an \
             i32",
        );

        let datastore = match RocksdbDatastore::new(path, Some(max_open_files), secure_uuids) {
            Ok(datastore) => datastore,
            Err(err) => panic!("Could not instantiate a rocksdb datastore: {:?}", err),
        };

        ProxyDatastore::Rocksdb(datastore)
    } else if connection_string.starts_with("postgres://") {
        let pool_size = match env::var("DATABASE_POOL_SIZE") {
            Ok(str_val) => {
                Some(str_val.parse().expect(
                    "Could not parse environment variable `DATABASE_POOL_SIZE`: must be \
                     a u32",
                ))
            }
            Err(_) => None,
        };

        let secret = env::var("SECRET").unwrap_or_else(|_| "".to_string());
        let datastore = PostgresDatastore::new(pool_size, connection_string, secret, secure_uuids);
        ProxyDatastore::Postgres(datastore)
    } else if connection_string == "memory://" {
        let datastore = MemoryDatastore::new(true);
        ProxyDatastore::Memory(datastore)
    } else {
        panic!("Cannot parse environment variable `DATABASE_URL`");
    }
}
