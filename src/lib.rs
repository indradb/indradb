#![feature(custom_derive, plugin)]
#![feature(test)]
#![plugin(serde_macros)]
#![cfg_attr(test, plugin(stainless))]

extern crate uuid;
extern crate crypto;
extern crate chrono;
extern crate core;
extern crate serde;
extern crate serde_json;
extern crate libc;
extern crate rand;
extern crate test;
extern crate regex;

#[cfg(feature="postgres-datastore")] extern crate postgres;
#[cfg(feature="postgres-datastore")] extern crate r2d2;
#[cfg(feature="postgres-datastore")] extern crate r2d2_postgres;
#[cfg(feature="postgres-datastore")] extern crate num_cpus;
#[cfg(feature="rocksdb-datastore")] #[macro_use] extern crate lazy_static;
#[cfg(feature="rocksdb-datastore")] extern crate rocksdb;
#[cfg(feature="rocksdb-datastore")] extern crate bincode;

#[macro_use] mod datastore;
mod errors;
mod models;
mod util;
mod traits;

pub use datastore::*;
pub use models::{Vertex, Edge, Type, Weight};
pub use errors::{Error, ValidationError};
pub use traits::Id;

#[cfg(feature="postgres-datastore")] mod pg;
#[cfg(feature="postgres-datastore")] pub use pg::{PostgresDatastore, PostgresTransaction};

#[cfg(feature="rocksdb-datastore")] mod rdb;
#[cfg(feature="rocksdb-datastore")] pub use rdb::{RocksdbDatastore, RocksdbTransaction};
