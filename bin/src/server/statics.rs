use datastore::{datastore, ProxyDatastore};
use num_cpus;
use std::env;
use std::path::Path;

lazy_static! {
    /// The underlying datastore
    pub static ref DATASTORE: ProxyDatastore = datastore();

    /// The path to the script root directory
    pub static ref SCRIPT_ROOT: String = env::var("INDRADB_SCRIPT_ROOT").unwrap_or_else(|_| {
        Path::new(".").join("scripts").to_str().unwrap().to_string()
    });

    /// Limits how many vertices are pulled at a time in mapreduce.
    pub static ref MAP_REDUCE_QUERY_LIMIT: u32 = match env::var("INDRADB_MAP_REDUCE_QUERY_LIMIT") {
        Ok(s) => {
            let value = s.parse::<u32>().expect("The `INDRADB_MAP_REDUCE_QUERY_LIMIT` environment variable is not a valid `u32`.");
            if value == 0 {
                panic!("The `INDRADB_MAP_REDUCE_QUERY_LIMIT` environment variable must be greater than 0.");
            }

            value
        },
        Err(_) => 10_000
    };

    /// The size of the mapreduce worker pool. "u16 ought to be enough for
    /// anybody..."
    pub static ref MAP_REDUCE_WORKER_POOL_SIZE: u16 = match env::var("MAP_REDUCE_WORKER_POOL_SIZE") {
        Ok(s) => {
            let value = s.parse::<u16>().expect("The `MAP_REDUCE_WORKER_POOL_SIZE` environment variable is not a valid `u16`.");
            if value < 1 {
                panic!("The `MAP_REDUCE_WORKER_POOL_SIZE` environment variable must be greater than or equal to 1.");
            }
            value
        },
        Err(_) => num_cpus::get() as u16
    };
}
