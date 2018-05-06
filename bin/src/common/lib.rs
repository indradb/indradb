//! Server logic. This is exposed in a common crate so that benches can use
//! it.

#![feature(custom_attribute)]
#![feature(transpose_result)]

extern crate chrono;
extern crate core;
#[macro_use]
extern crate crossbeam_channel;
extern crate hyper;
#[macro_use]
extern crate indradb;
extern crate iron;
#[macro_use]
extern crate juniper;
extern crate juniper_iron;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate num_cpus;
extern crate ordermap;
extern crate rand;
extern crate regex;
extern crate rlua;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate uuid;

mod http;
mod script;
mod proxy_datastore;
mod statics;
mod util;

pub use http::start_server;

#[cfg(feature = "test-suite")]
pub use http::tests::ClientDatastore;
