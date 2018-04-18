extern crate chan_signal;
extern crate chrono;
extern crate core;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate grpcio;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate protobuf;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[cfg(test)]
#[macro_use]
extern crate indradb;
#[cfg(not(test))]
extern crate indradb;

pub mod autogen;
pub mod converters;
pub mod errors;
pub mod grpc_client_datastore;
pub mod grpc_server;
pub mod proxy_datastore;

#[cfg(test)]
mod tests;

pub use grpc_client_datastore::GrpcClientDatastore;
pub use grpc_server::IndraDbService;
