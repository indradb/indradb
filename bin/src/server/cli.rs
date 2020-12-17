use clap::{App, Arg, SubCommand};

pub struct CliArgs {
    pub port: u16,
    pub database_url: String,
    pub max_open_files: Option<i32>,
    pub bulk_load_opt: Option<bool>,
    pub sled_compression: Option<String>,
}

const PORT: &str = "PORT";
const DATABASE_URL: &str = "DATABASE_URL";
const ROCKSDB_MAX_OPEN_FILES: &str = "ROCKSDB_MAX_OPEN_FILES";
const ROCKSDB_BULK_LOAD_OPTIMIZED: &str = "ROCKSDB_BULK_LOAD_OPTIMIZED";
const SLED_COMPRESSION: &str = "SLED_COMPRESSION";

const DEFAULT_PORT: u16 = 27615;

pub fn parse_cli_args() -> CliArgs {
    let database_url_argument = Arg::with_name(DATABASE_URL)
        .help("Database url")
        .required(true)
        .default_value("memory://")
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
        .arg(&database_url_argument)
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
            .default_value(""));

    let sled_subcommand = SubCommand::with_name("sled")
        .about("Start an indradb instance backed by sled")
        .arg(&database_url_argument)
        .arg(Arg::with_name(SLED_COMPRESSION)
            .long("sled_compression")
            .value_name(SLED_COMPRESSION)
            .help("If set to true, compression will be enabled at the default zstd factor of 5. If set to an integer, compression will be enabled at the zstd specified factor.")
            .takes_value(true)
            .default_value(""));

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
        database_url: String::from("memory://"),
        max_open_files: None,
        bulk_load_opt: None,
        sled_compression: None,
    };

    if let Some(matches) = matches.subcommand_matches("rocksdb") {
        if let Some(url) = matches.value_of(DATABASE_URL) {
            args.database_url = String::from(url)
        }

        args.max_open_files = match matches.value_of(ROCKSDB_MAX_OPEN_FILES) {
            Some(value) => Some(value.parse::<i32>().expect(
                "Could not parse argument `max_open_files`: must be an \
                 i32",
            )),
            None => None,
        };

        args.bulk_load_opt = match matches.value_of(ROCKSDB_BULK_LOAD_OPTIMIZED) {
            Some(value) => Some(value == "true"),
            None => None,
        };
    }

    if let Some(matches) = matches.subcommand_matches("sled") {
        if let Some(url) = matches.value_of(DATABASE_URL) {
            args.database_url = String::from(url)
        }
        args.sled_compression = match matches.value_of(SLED_COMPRESSION) {
            Some(value) => Some(String::from(value)),
            None => None,
        };
    }

    args
}
