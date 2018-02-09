use common::{datastore, ProxyDatastore};
use std::env;
use std::path::Path;

lazy_static! {
    /// The underlying datastore
    pub static ref DATASTORE: ProxyDatastore = datastore();

    /// The path to the script root directory
    pub static ref SCRIPT_ROOT: String = env::var("INDRADB_SCRIPT_ROOT").unwrap_or_else(|_| {
        Path::new(".").join("scripts").to_str().unwrap().to_string()
    });
}
