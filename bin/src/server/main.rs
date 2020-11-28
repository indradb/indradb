#[macro_use]
extern crate failure;

mod errors;

use std::env;
use std::net::ToSocketAddrs;

use async_std::net::TcpListener;
use futures::executor::LocalPool;

const DEFAULT_PORT: u16 = 27615;

fn main() -> Result<(), errors::Error> {
    let mut exec = LocalPool::new();

    let port = match env::var("PORT") {
        Ok(value) => value
            .parse::<u16>()
            .expect("Could not parse environment variable `PORT`"),
        Err(_) => DEFAULT_PORT,
    };

    let addr = format!("127.0.0.1:{}", port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| -> errors::Error { errors::Error::CouldNotParseBinding })?;
    let listener = exec.run_until(async { TcpListener::bind(&addr).await })?;
    println!("{}", listener.local_addr()?);

    let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| "memory://".to_string());

    if connection_string.starts_with("rocksdb://") {
        let path = &connection_string[10..connection_string.len()];

        let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or_else(|_| "512".to_string());
        let max_open_files = max_open_files_str.parse::<i32>().expect(
            "Could not parse environment variable `ROCKSDB_MAX_OPEN_FILES`: must be an \
             i32",
        );

        let bulk_load_optimized = env::var("ROCKSDB_BULK_LOAD_OPTIMIZED").unwrap_or_else(|_| "".to_string()) == "true";

        let datastore = indradb::RocksdbDatastore::new(path, Some(max_open_files), bulk_load_optimized)
            .expect("Expected to be able to create the RocksDB datastore");

        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else if connection_string.starts_with("sled://") {
        let path = &connection_string[7..connection_string.len()];

        let sled_compression_str = env::var("SLEDDB_COMPRESSION").unwrap_or_else(|_| "".to_string());
        let sled_config = match &sled_compression_str[..] {
            "true" => indradb::SledConfig::with_compression(None),
            "false" | "" => indradb::SledConfig::default(),
            _ => {
                let sled_compression = sled_compression_str
                    .parse::<i32>()
                    .expect("Could not parse environment variable `SLEDDB_COMPRESSION`: must be a bool or i32");
                indradb::SledConfig::with_compression(Some(sled_compression))
            }
        };

        let datastore = indradb::SledDatastore::new_with_config(path, sled_config)
            .expect("Expected to be able to create the Sled datastore");

        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else if connection_string == "memory://" {
        let datastore = indradb::MemoryDatastore::default();
        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else {
        Err(errors::Error::CouldNotParseDatabaseURL)
    }
}
