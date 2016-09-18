#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate crypto;
extern crate chrono;
extern crate core;
extern crate serde;
extern crate serde_json;
extern crate libc;
#[cfg(test)] extern crate rand;

mod datastore;
mod models;
mod requests;
mod responses;
mod util;
#[cfg(test)] mod datastore_test;

pub use datastore::{Datastore, Transaction};
pub use models::{Vertex, Type, Edge};
pub use requests::Request;
pub use responses::{Response, ErrorResponse};
pub use util::SimpleError;

#[cfg(feature="postgres-datastore")] mod pg;
#[cfg(feature="postgres-datastore")] pub use pg::{PostgresDatastore, PostgresTransaction};
