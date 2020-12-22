#[macro_use]
extern crate failure;
extern crate clap;

mod cli;
mod errors;

use async_std::net::TcpListener;
use cli::CliDatastoreArgs;
use futures::executor::LocalPool;
use std::net::ToSocketAddrs;

fn main() -> Result<(), errors::Error> {
    let mut exec = LocalPool::new();
    let args = cli::parse_cli_args();
    let port = args.port;

    let addr = format!("127.0.0.1:{}", port)
        .to_socket_addrs()?
        .next()
        .ok_or(errors::Error::CouldNotParseBinding)?;
    let listener = exec.run_until(async { TcpListener::bind(&addr).await })?;
    println!("{}", listener.local_addr()?);

    match args.datastore_args {
        CliDatastoreArgs::Rocksdb {
            path,
            max_open_files,
        } => {
            let datastore = indradb::RocksdbDatastore::new(&path, Some(max_open_files))
                .expect("Expected to be able to create the RocksDB datastore");

            exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
            Ok(())
        }
        CliDatastoreArgs::Sled { path, sled_config } => {
            let datastore = sled_config
                .open(&path)
                .expect("Expected to be able to create the Sled datastore");

            exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
            Ok(())
        }
        CliDatastoreArgs::Memory => {
            let datastore = indradb::MemoryDatastore::default();
            exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
            Ok(())
        }
    }
}
