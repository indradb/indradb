extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate chrono;
extern crate core;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate futures_cpupool;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
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
#[macro_use]
pub mod converters;
pub mod client_datastore;
pub mod errors;
pub mod server;

#[cfg(test)]
mod tests;

pub use crate::client_datastore::ClientDatastore;
