use std::collections::HashMap;
use std::fmt;

static RUSTC_VERSION: &str = env!("RUSTC_VERSION");
static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn indradb_version_info() -> VersionInfo {
    VersionInfo {
        rustc: RUSTC_VERSION.to_string(),
        core: CORE_VERSION.to_string(),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct VersionInfo {
    rustc: String,
    core: String
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rustc={}, core={}", self.rustc, self.core)
    }
}

pub trait Plugin: 'static + Send + Sync {
    fn call(&self, datastore: Box<dyn crate::Transaction>, arg: serde_json::Value) -> crate::Result<serde_json::Value>;
}

pub struct PluginDeclaration {
    pub version_info: VersionInfo,
    pub register: unsafe extern "C" fn(&mut HashMap<String, Box<dyn Plugin>>),
}

#[macro_export]
macro_rules! plugins {
    ( $( $name:expr, $t:item ),* ) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            version_info: VersionInfo {
                rustc: $crate::RUSTC_VERSION,
                core: $crate::CORE_VERSION,
            }
            register: extern "C" fn register(entries: &mut HashMap<String, Box<dyn Plugin>>) {
                $(
                    entries.insert($name.to_string(), Box::new($t));
                )*
            },
        };
    };
}
