#[cfg(feature = "test-suite")]
#[macro_use]
extern crate indradb;

tonic::include_proto!("indradb");

pub use bulk_insert_item::Item as BulkInsertItemVariant;
pub use indra_db_client::IndraDbClient as ProtoClient;
pub use query::Query as QueryVariant;
pub use query_output_value::QueryOutputValue as QueryOutputValueVariant;

mod converters;
pub use converters::*;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{Client, ClientError};

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::{run as run_server, run_with_plugins as run_server_with_plugins, Server};

#[cfg(feature = "test-suite")]
pub mod tests;
