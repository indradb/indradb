extern crate clap;

mod cli;

use std::error::Error;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::sync::Arc;

use crate::cli::CliDatastoreArgs;

use indradb_proto as proto;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse_cli_args();

    let addr = args.addr.to_socket_addrs()?.next().unwrap();
    let listener = TcpListener::bind(addr).await?;
    let binding = listener.local_addr()?;

    match args.datastore_args {
        CliDatastoreArgs::Rocksdb {
            path,
            max_open_files,
            repair,
        } => {
            if repair {
                indradb::RocksdbDatastore::repair(&path, Some(max_open_files))
                    .expect("Expected to be able to repair the RocksDB datastore");
                println!("repair successful");
                return Ok(());
            }

            let datastore = indradb::RocksdbDatastore::new(&path, Some(max_open_files))
                .expect("Expected to be able to create the RocksDB datastore");
            println!("grpc://{}", binding);
            proto::run_server(Arc::new(datastore), listener).await?;
            Ok(())
        }
        CliDatastoreArgs::Memory { path: None } => {
            let datastore = indradb::MemoryDatastore::default();
            println!("grpc://{}", binding);
            proto::run_server(Arc::new(datastore), listener).await?;
            Ok(())
        }
        CliDatastoreArgs::Memory { path: Some(path) } => {
            let datastore = if Path::new(path.as_os_str()).exists() {
                Arc::new(indradb::MemoryDatastore::read(path)?)
            } else {
                Arc::new(indradb::MemoryDatastore::create(path)?)
            };
            println!("grpc://{}", binding);
            proto::run_server(datastore.clone(), listener).await?;
            Ok(())
        }
    }
}
