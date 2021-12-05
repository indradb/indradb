//! IndraDB: a graph datastore.
//!
//! IndraDB is broken up into a library and an application. This is the
//! library, which you would use if you want to create new datastore
//! implementations, or plug into the low-level details of IndraDB. For most
//! use cases, you can use the application, which exposes an API and scripting
//! layer.

#![cfg_attr(feature = "bench-suite", feature(test))]

#[cfg(feature = "bench-suite")]
extern crate test;

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

#[cfg(all(feature = "plugin", feature = "rocksdb-datastore"))]
pub mod plugin;

#[cfg(feature = "rocksdb-datastore")]
mod rdb;

#[cfg(feature = "rocksdb-datastore")]
pub use crate::rdb::{RocksdbDatastore, RocksdbTransaction};
