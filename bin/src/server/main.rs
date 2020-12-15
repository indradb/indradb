#[macro_use]
extern crate failure;
extern crate clap;

mod errors;

use std::net::ToSocketAddrs;

use async_std::net::TcpListener;
use clap::{App, Arg};
use futures::executor::LocalPool;

const DEFAULT_PORT: u16 = 27615;

const PORT: &str = "PORT";
const DATABASE_URL: &str = "DATABASE_URL";
const ROCKSDB_MAX_OPEN_FILES: &str = "ROCKSDB_MAX_OPEN_FILES";
const ROCKSDB_BULK_LOAD_OPTIMIZED: &str = "ROCKSDB_BULK_LOAD_OPTIMIZED";
const SLED_COMPRESSION: &str = "SLED_COMPRESSION";

fn main() -> Result<(), errors::Error> {
    let mut exec = LocalPool::new();

    let matches = App::new("Indra DB")
                        .version("1.2.0")
                        .about("Indra DB server")
                        .arg(Arg::with_name(PORT)
                               .short("p")
                               .long("port")
                               .value_name("PORT")
                               .help("The port to run the server on. Defaults to 27615")
                               .takes_value(true)
                               .default_value("27615"))
                        .arg(Arg::with_name(DATABASE_URL)
                               .short("d")
                               .long("database")
                               .value_name("DATABASE_URL")
                               .help("The connection string to the underlying database. Defaults to memory://")
                               .takes_value(true)
                               .default_value("memory://"))
                        .arg(Arg::with_name(ROCKSDB_MAX_OPEN_FILES)
                            .long("max_open_files")
                            .value_name("ROCKSDB_MAX_OPEN_FILES")
                            .help("Sets the number of maximum open files to have open in RocksDB.")
                            .takes_value(true)
                            .default_value("512"))
                        .arg(Arg::with_name(ROCKSDB_BULK_LOAD_OPTIMIZED)
                            .long("bulk_load_opt")
                            .value_name("ROCKSDB_BULK_LOAD_OPTIMIZED")
                            .help("If set to true, RocksDB will be configured to optimize for bulk loading of data, likely at the detriment of any other kind of workload.")
                            .takes_value(true)
                            .default_value(""))
                        .arg(Arg::with_name(SLED_COMPRESSION)
                            .long("sled_compression")
                            .value_name("SLED_COMPRESSION")
                            .help("If set to true, compression will be enabled at the default zstd factor of 5. If set to an integer, compression will be enabled at the zstd specified factor.")
                            .takes_value(true)
                            .default_value(""))
                          .get_matches();

    let port = match matches.value_of(PORT) {
        Some(value) => value.parse::<u16>().expect("Could not parse argument `port`"),
        None => DEFAULT_PORT,
    };

    let addr = format!("127.0.0.1:{}", port)
        .to_socket_addrs()?
        .next()
        .ok_or(errors::Error::CouldNotParseBinding)?;
    let listener = exec.run_until(async { TcpListener::bind(&addr).await })?;
    println!("{}", listener.local_addr()?);

    let connection_string = matches.value_of(DATABASE_URL).unwrap();
    if connection_string.starts_with("rocksdb://") {
        let path = &connection_string[10..connection_string.len()];

        let max_open_files_str = matches.value_of(ROCKSDB_MAX_OPEN_FILES).unwrap();
        let max_open_files = max_open_files_str.parse::<i32>().expect(
            "Could not parse argument `max_open_files`: must be an \
             i32",
        );

        let bulk_load_optimized = matches.value_of(ROCKSDB_BULK_LOAD_OPTIMIZED).unwrap() == "true";

        let datastore = indradb::RocksdbDatastore::new(path, Some(max_open_files), bulk_load_optimized)
            .expect("Expected to be able to create the RocksDB datastore");

        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else if connection_string.starts_with("sled://") {
        let path = &connection_string[7..connection_string.len()];

        let sled_compression_str = matches.value_of(SLED_COMPRESSION).unwrap();
        let sled_config = match &sled_compression_str[..] {
            "true" => indradb::SledConfig::with_compression(None),
            "false" | "" => indradb::SledConfig::default(),
            _ => {
                let sled_compression = sled_compression_str
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
    } else if connection_string == "memory://" {
        let datastore = indradb::MemoryDatastore::default();
        exec.run_until(common::server::run(listener, datastore, exec.spawner()))?;
        Ok(())
    } else {
        Err(errors::Error::CouldNotParseDatabaseURL)
    }
}
