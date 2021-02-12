use clap::{value_t, App, Arg, SubCommand};
use indradb::SledConfig;
use std::ffi::OsString;

pub struct CliArgs {
    pub addr: String,
    pub datastore_args: CliDatastoreArgs,
}

pub enum CliDatastoreArgs {
    Memory { path: Option<OsString> },
    Rocksdb { path: OsString, max_open_files: i32 },
    Sled { path: OsString, sled_config: SledConfig },
}

const ADDRESS: &str = "ADDRESS";
const DATABASE_PATH: &str = "DATABASE_PATH";
const ROCKSDB_MAX_OPEN_FILES: &str = "ROCKSDB_MAX_OPEN_FILES";
const SLED_COMPRESSION: &str = "SLED_COMPRESSION";
const MEMORY_PERSIST_PATH: &str = "MEMORY_PERSIST_PATH";

pub fn parse_cli_args() -> CliArgs {
    let database_path_argument = Arg::with_name(DATABASE_PATH)
        .help("Database url")
        .required(true)
        .index(1);

    let addr = Arg::with_name(ADDRESS)
        .short("a")
        .long("address")
        .value_name(ADDRESS)
        .help("The address to listen on, defaults to 127.0.0.1:27615")
        .takes_value(true)
        .default_value("127.0.0.1:27615");

    let memory_subcommand = SubCommand::with_name("memory")
        .about("Start an indradb instance backed by memory. This is the default, so including this subcommand is only useful if you want to set options.")
        .arg(
            Arg::with_name(MEMORY_PERSIST_PATH)
                .long("persist-path")
                .value_name(MEMORY_PERSIST_PATH)
                .help("Sets the path to persist images. If unspecified, the datastore will not be persisted.")
                .takes_value(true)
        );

    let rocksdb_subcommand = SubCommand::with_name("rocksdb")
        .about("Start an indradb instance backed by rocksdb")
        .arg(&database_path_argument)
        .arg(
            Arg::with_name(ROCKSDB_MAX_OPEN_FILES)
                .long("max-open-files")
                .value_name(ROCKSDB_MAX_OPEN_FILES)
                .help("Sets the number of maximum open files to have open in RocksDB.")
                .takes_value(true)
                .default_value("512"),
        );

    let sled_subcommand = SubCommand::with_name("sled")
        .about("Start an indradb instance backed by sled")
        .arg(&database_path_argument)
        .arg(Arg::with_name(SLED_COMPRESSION)
            .long("compression")
            .value_name(SLED_COMPRESSION)
            .help("If set to true, compression will be enabled at the default zstd factor of 5. If set to an integer, compression will be enabled at the zstd specified factor.")
            .takes_value(true)
            .default_value("false"));

    let matches = App::new("indradb-server")
        .arg(&addr)
        .subcommand(memory_subcommand)
        .subcommand(rocksdb_subcommand)
        .subcommand(sled_subcommand)
        .get_matches();

    CliArgs {
        addr: matches.value_of(ADDRESS).unwrap().to_string(),
        datastore_args: if let Some(matches) = matches.subcommand_matches("memory") {
            if let Some(path) = matches.value_of_os(MEMORY_PERSIST_PATH) {
                CliDatastoreArgs::Memory {
                    path: Some(path.to_os_string()),
                }
            } else {
                CliDatastoreArgs::Memory { path: None }
            }
        } else if let Some(matches) = matches.subcommand_matches("rocksdb") {
            CliDatastoreArgs::Rocksdb {
                path: matches.value_of_os(DATABASE_PATH).unwrap().to_os_string(),
                max_open_files: value_t!(matches, ROCKSDB_MAX_OPEN_FILES, i32).unwrap_or_else(|e| e.exit()),
            }
        } else if let Some(matches) = matches.subcommand_matches("sled") {
            let sled_compression = matches.value_of(SLED_COMPRESSION).unwrap();
            CliDatastoreArgs::Sled {
                path: matches.value_of_os(DATABASE_PATH).unwrap().to_os_string(),
                sled_config: match sled_compression {
                    "true" => indradb::SledConfig::with_compression(None),
                    "false" => indradb::SledConfig::default(),
                    _ => indradb::SledConfig::with_compression(Some(
                        sled_compression
                            .parse::<i32>()
                            .expect("Could not parse argument `sled_compression`: must be a bool or i32"),
                    )),
                },
            }
        } else {
            CliDatastoreArgs::Memory { path: None }
        },
    }
}
