#![feature(test)]

extern crate common_tests;
#[macro_use]
extern crate indradb;
extern crate test;

full_bench_impl!(common_tests::GrpcDatastore::default());
