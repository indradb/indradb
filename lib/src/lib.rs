//! `IndraDB` - a graph datastore.
//!
//! `IndraDB` is broken up into a library and an application. This is the
//! library, which you would use if you want to create new datastore
//! implementations, or plug into the low-level details of `IndraDB`. For most
//! use cases, you can use the application, which exposes an API and scripting
//! layer.

// Used for error-chain, which can recurse deeply
#![recursion_limit = "1024"]

extern crate byteorder;
extern crate chrono;
extern crate core;
extern crate crypto;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

#[cfg(feature = "rocksdb-datastore")]
extern crate bincode;
#[cfg(feature = "postgres-datastore")]
extern crate num_cpus;
#[cfg(feature = "postgres-datastore")]
extern crate postgres;
#[cfg(feature = "postgres-datastore")]
extern crate r2d2;
#[cfg(feature = "postgres-datastore")]
extern crate r2d2_postgres;
#[cfg(feature = "rocksdb-datastore")]
extern crate rocksdb;

#[macro_use]
pub mod tests;
mod errors;
mod memory;
mod models;
mod traits;
pub mod util;

pub use errors::*;
pub use memory::{MemoryDatastore, MemoryTransaction};
pub use models::*;
pub use traits::*;

#[cfg(feature = "postgres-datastore")]
mod pg;
#[cfg(feature = "postgres-datastore")]
pub use pg::{PostgresDatastore, PostgresTransaction};

#[cfg(feature = "rocksdb-datastore")]
mod rdb;
#[cfg(feature = "rocksdb-datastore")]
pub use rdb::{RocksdbDatastore, RocksdbTransaction};
