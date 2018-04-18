#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

full_bench_impl!(common::GrpcClientDatastore::default());
