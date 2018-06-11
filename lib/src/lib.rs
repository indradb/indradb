//! `IndraDB` - a graph datastore.
//!
//! `IndraDB` is broken up into a library and an application. This is the
//! library, which you would use if you want to create new datastore
//! implementations, or plug into the low-level details of `IndraDB`. For most
//! use cases, you can use the application, which exposes an API and scripting
//! layer.

// Used for error-chain, which can recurse deeply
#![recursion_limit = "1024"]

#![cfg_attr(feature = "bench-suite", feature(test))]

#[cfg(feature = "bench-suite")]
extern crate test;

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
#[cfg(feature = "rocksdb-datastore")]
extern crate rocksdb;

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

pub use errors::*;
pub use memory::{MemoryDatastore, MemoryTransaction};
pub use models::*;
pub use traits::*;

#[cfg(feature = "rocksdb-datastore")]
mod rdb;
#[cfg(feature = "rocksdb-datastore")]
pub use rdb::{RocksdbDatastore, RocksdbTransaction};
