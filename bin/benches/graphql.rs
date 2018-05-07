//! Benchmarks for the graphql frontend. Note that these benchmarks aren't
//! very realistic:
//! 1) They avoid network overhead.
//! 2) But they introduce their own overhead in running through a
//!    not-performance-oriented faux datastore.
//! What these benchmarks do provide is /some/ idea of how performance might
//! be relatively affected by changes.

#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

indradb_full_bench_impl!(common::ClientDatastore::default());
