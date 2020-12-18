use clap::{App, Arg, SubCommand};
use indradb::SledConfig;

pub struct CliArgs {
    pub port: u16,
    pub datastore_args: CliDatastoreArgs,
}

pub enum CliDatastoreArgs {
    Memory,
    Rocksdb {
        path: String,
        max_open_files: i32,
        bulk_load_optimized: bool,
    },
    Sled {
        path: String,
        sled_config: SledConfig,
    },
}

const PORT: &str = "PORT";
const DATABASE_PATH: &str = "DATABASE_PATH";
const ROCKSDB_MAX_OPEN_FILES: &str = "ROCKSDB_MAX_OPEN_FILES";
const ROCKSDB_BULK_LOAD_OPTIMIZED: &str = "ROCKSDB_BULK_LOAD_OPTIMIZED";
const SLED_COMPRESSION: &str = "SLED_COMPRESSION";

const DEFAULT_PORT: u16 = 27615;

pub fn parse_cli_args() -> CliArgs {
    let database_path_argument = Arg::with_name(DATABASE_PATH)
        .help("Database url")
        .required(true)
        .index(1);
    let port = Arg::with_name(PORT)
        .short("p")
        .long("port")
        .value_name(PORT)
        .help("The port to run the server on. Defaults to 27615")
        .takes_value(true)
        .default_value("27615");
    let rocksdb_subcommand = SubCommand::with_name("rocksdb")
        .about("Start an indradb instance backed by rocksdb")
        .arg(&database_path_argument)
        .arg(Arg::with_name(ROCKSDB_MAX_OPEN_FILES)
            .long("max_open_files")
            .value_name(ROCKSDB_MAX_OPEN_FILES)
            .help("Sets the number of maximum open files to have open in RocksDB.")
            .takes_value(true)
            .default_value("512"))
        .arg(Arg::with_name(ROCKSDB_BULK_LOAD_OPTIMIZED)
            .long("bulk_load_opt")
            .value_name(ROCKSDB_BULK_LOAD_OPTIMIZED)
            .help("If set to true, RocksDB will be configured to optimize for bulk loading of data, likely at the detriment of any other kind of workload.")
            .takes_value(true)
            .default_value("false"));

    let sled_subcommand = SubCommand::with_name("sled")
        .about("Start an indradb instance backed by sled")
        .arg(&database_path_argument)
        .arg(Arg::with_name(SLED_COMPRESSION)
            .long("compression")
            .value_name(SLED_COMPRESSION)
            .help("If set to true, compression will be enabled at the default zstd factor of 5. If set to an integer, compression will be enabled at the zstd specified factor.")
            .takes_value(true)
            .default_value("false"));

    let matches = App::new("Indra DB")
        .version("1.2.0")
        .about("Indra DB server")
        .subcommand(rocksdb_subcommand)
        .subcommand(sled_subcommand)
        .arg(&port)
        .get_matches();

    let mut args = CliArgs {
        port: match matches.value_of(PORT) {
            Some(value) => value.parse::<u16>().expect("Could not parse argument `port`"),
            None => DEFAULT_PORT,
        },
        datastore_args: CliDatastoreArgs::Memory,
    };

    if let Some(matches) = matches.subcommand_matches("rocksdb") {
        args.datastore_args = CliDatastoreArgs::Rocksdb {
            path: String::from(matches.value_of(DATABASE_PATH).unwrap()),
            max_open_files: matches.value_of(ROCKSDB_MAX_OPEN_FILES).unwrap().parse::<i32>().expect(
                "Could not parse argument `max_open_files`: must be an \
                 i32",
            ),
            bulk_load_optimized: matches.value_of(ROCKSDB_BULK_LOAD_OPTIMIZED).unwrap() == "true",
        }
    }

    if let Some(matches) = matches.subcommand_matches("sled") {
        let sled_compression = matches.value_of(SLED_COMPRESSION).unwrap();
        args.datastore_args = CliDatastoreArgs::Sled {
            path: String::from(matches.value_of(DATABASE_PATH).unwrap()),
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
    }

    args
}
