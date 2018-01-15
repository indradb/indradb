#![feature(test)]

extern crate indradb;
extern crate test;

#[macro_use]
mod common;

pub use indradb::MemoryDatastore;
pub use indradb::tests;
pub use test::Bencher;

bench_transaction_impl!(MemoryDatastore::new(false));
