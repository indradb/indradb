use common::{ProxyDatastore, datastore};
use std::env;
use std::path::Path;

lazy_static! {
    pub static ref DATASTORE: ProxyDatastore = datastore();

    pub static ref SCRIPT_ROOT: String = match env::var("NUTRINO_SCRIPT_ROOT") {
		Ok(s) => s,
		Err(_) => Path::new(".").join("scripts").to_str().unwrap().to_string()
	};
}
