#![recursion_limit = "1024"]

extern crate common;
extern crate indradb;
extern crate num_cpus;
extern crate futures;
extern crate tokio_core;
extern crate tokio_signal;

use futures::{Future, Stream};
use std::env;
use std::io::Error as IoError;
use std::net::ToSocketAddrs;

const DEFAULT_PORT: u16 = 27615;

#[cfg(unix)]
fn interrupt_stream() -> Box<dyn Stream<Item=(), Error=IoError>> {
    use tokio_signal::unix::{Signal, SIGINT, SIGTERM};
    let sigint = Signal::new(SIGINT).flatten_stream();
    let sigterm = Signal::new(SIGTERM).flatten_stream();
    Box::new(sigint.select(sigterm).map(|_| ()))
}

#[cfg(not(unix))]
fn interrupt_stream() -> Box<dyn Stream<Item=(), Error=IoError>> {
    Box::new(empty())
}

fn main() {
    let port = match env::var("PORT") {
        Ok(value) => value
            .parse::<u16>()
            .expect("Could not parse environment variable `PORT`"),
        Err(_) => DEFAULT_PORT,
    };

    let binding = format!("127.0.0.1:{}", port);

    let addr = binding
        .to_socket_addrs()
        .expect("Could not parse binding")
        .next()
        .expect("Could not parse binding");

    let worker_count = match env::var("WORKER_COUNT") {
        Ok(value) => value
            .parse::<usize>()
            .expect("Could not parse environment variable `WORKER_COUNT`"),
        Err(_) => num_cpus::get() * 2,
    };

    let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| "memory://".to_string());

    if connection_string.starts_with("rocksdb://") {
        let path = &connection_string[10..connection_string.len()];

        let max_open_files_str = env::var("ROCKSDB_MAX_OPEN_FILES").unwrap_or_else(|_| "512".to_string());
        let max_open_files = max_open_files_str.parse::<i32>().expect(
            "Could not parse environment variable `ROCKSDB_MAX_OPEN_FILES`: must be an \
             i32",
        );

        let bulk_load_optimized = env::var("ROCKSDB_BULK_LOAD_OPTIMIZED").unwrap_or_else(|_| "".to_string()) == "true";
        let datastore = indradb::RocksdbDatastore::new(path, Some(max_open_files), bulk_load_optimized).expect("Could not create RocksDB datastore");
        common::executor::run(addr, worker_count, datastore, interrupt_stream()).expect("Server failed to run");
    } else if connection_string == "memory://" {
        let datastore = indradb::MemoryDatastore::default();
        common::executor::run(addr, worker_count, datastore, interrupt_stream()).expect("Server failed to run");
    } else {
        panic!("Cannot parse environment variable `DATABASE_URL`");
    };
}
