#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

full_test_impl!(common::GrpcDatastore::default());
