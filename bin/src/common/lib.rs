//! Server logic. This is exposed in a common crate so that benches can use
//! it.

#![feature(custom_attribute)]
#![feature(transpose_result)]

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate chrono;
extern crate core;
#[macro_use]
extern crate crossbeam_channel;
extern crate futures;
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate num_cpus;
extern crate ordermap;
extern crate rand;
extern crate regex;
extern crate rlua;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate uuid;

#[cfg(feature = "test-suite")]
#[macro_use]
extern crate indradb;

#[cfg(not(feature = "test-suite"))]
extern crate indradb;

mod graphql;
mod http;
mod mapreduce;
mod script;
mod proxy_datastore;
mod statics;

pub use graphql::{RootMutation, RootQuery, Schema, Context};
pub use http::start_server;
#[cfg(feature = "test-suite")]
pub use graphql::tests::ClientDatastore;
