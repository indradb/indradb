#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

indradb_full_bench_impl!(common::ClientDatastore::default());
