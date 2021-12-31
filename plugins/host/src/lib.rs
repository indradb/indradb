pub mod util;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub fn indradb_version_info() -> VersionInfo {
    VersionInfo {
        rustc: env!("RUSTC_VERSION").to_string(),
        indradb_interface: 0,
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct VersionInfo {
    pub rustc: String,
    pub indradb_interface: u8,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rustc={}, indradb_interface={}", self.rustc, self.indradb_interface)
    }
}

pub trait Plugin: Send + Sync + 'static {
    fn call(
        &self,
        trans: Box<dyn indradb::Transaction + Send + Sync + 'static>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn Error>>;
}

pub struct PluginDeclaration {
    pub version_info: VersionInfo,
    pub entries: HashMap<String, Box<dyn Plugin>>,
}

#[macro_export]
macro_rules! register_plugins {
    ( $indradb_interface_version:expr, $( $name:expr, $t:expr ),* ) => {
        use indradb_plugin_host::PluginDeclaration;
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn register() -> indradb_plugin_host::PluginDeclaration {
            use std::collections::HashMap;
            let mut entries = HashMap::new();
            $(
                {
                    let t: Box<dyn indradb_plugin_host::Plugin> = $t;
                    entries.insert($name.to_string(), t);
                }
            )*
            PluginDeclaration {
                version_info: indradb_plugin_host::VersionInfo {
                    // TODO: ensure env! executes at macro expansion time
                    rustc: env!("RUSTC_VERSION").to_string(),
                    indradb_interface: $indradb_interface_version,
                },
                entries,
            }
        }
    };
}
