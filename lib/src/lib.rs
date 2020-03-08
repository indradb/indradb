//! `IndraDB` - a graph datastore.
//!
//! `IndraDB` is broken up into a library and an application. This is the
//! library, which you would use if you want to create new datastore
//! implementations, or plug into the low-level details of `IndraDB`. For most
//! use cases, you can use the application, which exposes an API and scripting
//! layer.

#![cfg_attr(feature = "bench-suite", feature(test))]

#[cfg(feature = "bench-suite")]
extern crate test;

extern crate chrono;
extern crate core;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;
extern crate serde_json;
extern crate uuid;

#[cfg(feature = "rocksdb-datastore")]
extern crate byteorder;
#[cfg(feature = "rocksdb-datastore")]
extern crate rocksdb;

#[cfg(feature = "sled-datastore")]
extern crate sled;

#[cfg(feature = "test-suite")]
#[macro_use]
pub mod tests;

#[cfg(feature = "bench-suite")]
#[macro_use]
pub mod benches;

mod errors;
mod memory;
mod models;
mod traits;
pub mod util;

pub use crate::errors::*;
pub use crate::memory::{MemoryDatastore, MemoryTransaction};
pub use crate::models::*;
pub use crate::traits::*;

#[cfg(any(feature = "rocksdb-datastore", feature = "sled-datastore"))]
mod bytes;
#[cfg(feature = "rocksdb-datastore")]
mod rdb;
#[cfg(feature = "sled-datastore")]
mod sledds;

#[cfg(feature = "rocksdb-datastore")]
pub use crate::rdb::{RocksdbDatastore, RocksdbTransaction};

#[cfg(feature = "sled-datastore")]
pub use crate::sledds::{SledDatastore, SledTransaction};
