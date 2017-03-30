extern crate uuid;
extern crate crypto;
extern crate chrono;
extern crate core;
extern crate serde;
extern crate serde_json;
extern crate libc;
extern crate rand;
extern crate regex;
#[macro_use] extern crate serde_derive;

#[cfg(feature="postgres-datastore")]
extern crate postgres;
#[cfg(feature="postgres-datastore")]
extern crate r2d2;
#[cfg(feature="postgres-datastore")]
extern crate r2d2_postgres;
#[cfg(feature="postgres-datastore")]
extern crate num_cpus;
#[cfg(feature="rocksdb-datastore")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature="rocksdb-datastore")]
extern crate rocksdb;
#[cfg(feature="rocksdb-datastore")]
extern crate librocksdb_sys;
#[cfg(feature="rocksdb-datastore")]
extern crate bincode;
#[cfg(feature="rocksdb-datastore")]
extern crate byteorder;

#[macro_use]
pub mod tests;
mod errors;
mod models;
mod traits;
mod util;

pub use models::*;
pub use errors::*;
pub use traits::*;

#[cfg(feature="postgres-datastore")]
mod pg;
#[cfg(feature="postgres-datastore")]
pub use pg::{PostgresDatastore, PostgresTransaction};

#[cfg(feature="rocksdb-datastore")]
mod rdb;
#[cfg(feature="rocksdb-datastore")]
pub use rdb::{RocksdbDatastore, RocksdbTransaction};
