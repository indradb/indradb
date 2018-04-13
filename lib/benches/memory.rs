#![feature(test)]
#![cfg(feature = "bench-suite")]

#[macro_use]
extern crate indradb;

pub use indradb::MemoryDatastore;

bench_transaction_impl!(MemoryDatastore::default());
