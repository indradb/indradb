#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate crypto;
extern crate chrono;
extern crate core;
extern crate serde;
extern crate serde_json;
extern crate libc;
extern crate rand;

#[macro_use] mod datastore;
mod models;
mod requests;
mod responses;
mod util;
mod traits;

pub use datastore::{Datastore, Transaction};
pub use models::{Vertex, Edge};
pub use requests::Request;
pub use responses::{Response, ErrorResponse};
pub use util::SimpleError;
pub use traits::Id;

#[cfg(feature="postgres-datastore")] mod pg;
#[cfg(feature="postgres-datastore")] pub use pg::{PostgresDatastore, PostgresTransaction};
