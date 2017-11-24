use common::{datastore, ProxyDatastore};
use std::env;
use std::path::Path;

lazy_static! {
    /// The underlying datastore
    pub static ref DATASTORE: ProxyDatastore = datastore();

    /// The path to the script root directory
    pub static ref SCRIPT_ROOT: String = match env::var("BRAID_SCRIPT_ROOT") {
        Ok(s) => s,
        Err(_) => Path::new(".").join("scripts").to_str().unwrap().to_string()
    };
}
