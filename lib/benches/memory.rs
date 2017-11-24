#![feature(test)]

extern crate braid;
extern crate test;

#[macro_use]
mod common;

pub use braid::MemoryDatastore;
pub use braid::tests;
pub use test::Bencher;

bench_transaction_impl!(MemoryDatastore::new(false));
