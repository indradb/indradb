extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate chrono;
extern crate core;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate uuid;

#[cfg(test)]
#[macro_use]
extern crate indradb;
#[cfg(not(test))]
extern crate indradb;

pub mod autogen;
pub mod converters;
pub mod errors;
pub mod client_datastore;
pub mod proxy_datastore;
pub mod server;

#[cfg(test)]
mod tests;

pub use client_datastore::ClientDatastore;
