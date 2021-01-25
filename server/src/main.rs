extern crate clap;

mod cli;

use std::error::Error;
use std::net::ToSocketAddrs;

use crate::cli::CliDatastoreArgs;

use indradb_proto as proto;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse_cli_args();

    let addr = args.addr.to_socket_addrs()?.next().unwrap();
    let listener = TcpListener::bind(addr).await?;
    let binding = listener.local_addr()?;
    println!("grpc://{}", binding);

    match args.datastore_args {
        CliDatastoreArgs::Rocksdb { path, max_open_files } => {
            let datastore = indradb::RocksdbDatastore::new(&path, Some(max_open_files))
                .expect("Expected to be able to create the RocksDB datastore");

            proto::run_server(datastore, listener).await?;
            Ok(())
        }
        CliDatastoreArgs::Sled { path, sled_config } => {
            let datastore = sled_config
                .open(&path)
                .expect("Expected to be able to create the Sled datastore");

            proto::run_server(datastore, listener).await?;
            Ok(())
        }
        CliDatastoreArgs::Memory => {
            let datastore = indradb::MemoryDatastore::default();
            proto::run_server(datastore, listener).await?;
            Ok(())
        }
    }
}
