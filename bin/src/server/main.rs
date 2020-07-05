#[macro_use]
extern crate failure;

mod errors;

use std::env;
use std::net::ToSocketAddrs;

use async_std::task::block_on;
use async_std::net::TcpListener;

const DEFAULT_PORT: u16 = 27615;

fn main() -> Result<(), errors::Error> {
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
    let listener = block_on(async { TcpListener::bind(&addr).await })?;
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

        block_on(common::server::run(listener, datastore))?;
        Ok(())
    } else if connection_string == "memory://" {
        let datastore = indradb::MemoryDatastore::default();
        block_on(common::server::run(listener, datastore))?;
        Ok(())
    } else {
        Err(errors::Error::CouldNotParseDatabaseURL)
    }
}
