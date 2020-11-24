#[macro_use]
extern crate capnp_rpc;
#[cfg(test)]
#[macro_use]
extern crate indradb;
#[cfg(not(test))]
extern crate indradb;

pub mod client_datastore;
pub mod server;

#[cfg(test)]
mod tests;
