extern crate uuid;
extern crate rocksdb;

mod datastore;
mod tests;

pub use self::datastore::{RocksdbDatastore, RocksdbTransaction};
