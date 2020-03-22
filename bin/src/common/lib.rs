#[macro_use]
extern crate capnp_rpc;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate indradb;
#[cfg(not(test))]
extern crate indradb;

pub mod autogen;
#[macro_use]
pub mod converters;
pub mod client_datastore;
pub mod server;

#[cfg(test)]
mod tests;

pub use crate::client_datastore::ClientDatastore;
