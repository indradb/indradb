#[macro_use]
extern crate failure;
extern crate clap;

mod cli;
mod errors;

use async_std::net::TcpListener;
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

    if args.database_url.starts_with("rocksdb://") {
        let path = &args.database_url[10..args.database_url.len()];
        let datastore =
            indradb::RocksdbDatastore::new(path, Some(args.max_open_files.unwrap()), args.bulk_load_opt.unwrap())
                .expect("Expected to be able to create the RocksDB datastore");

        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else if args.database_url.starts_with("sled://") {
        let path = &args.database_url[7..args.database_url.len()];

        let sled_compression = args.sled_compression.unwrap();
        let sled_config = match &sled_compression[..] {
            "true" => indradb::SledConfig::with_compression(None),
            "false" | "" => indradb::SledConfig::default(),
            _ => {
                let sled_compression = sled_compression
                    .parse::<i32>()
                    .expect("Could not parse argument `sled_compression`: must be a bool or i32");
                indradb::SledConfig::with_compression(Some(sled_compression))
            }
        };

        let datastore = sled_config
            .open(path)
            .expect("Expected to be able to create the Sled datastore");

        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else if args.database_url == "memory://" {
        let datastore = indradb::MemoryDatastore::default();
        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else {
        Err(errors::Error::CouldNotParseDatabaseURL)
    }
}
