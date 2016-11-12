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
#[cfg(feature="rocksdb-datastore")] #[macro_use] extern crate lazy_static;

#[macro_use] mod datastore;
mod models;
mod util;
mod traits;

pub use datastore::*;
pub use models::{Vertex, Edge};
pub use util::Error;
pub use traits::Id;

#[cfg(feature="postgres-datastore")] mod pg;
#[cfg(feature="postgres-datastore")] pub use pg::{PostgresDatastore, PostgresTransaction};

#[cfg(feature="rocksdb-datastore")] mod rocksdb;
#[cfg(feature="rocksdb-datastore")] pub use rocksdb::{RocksdbDatastore, RocksdbTransaction};
