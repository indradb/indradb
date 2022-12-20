use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use crate::errors::Error;

/// Represents the rustc compiler version and the plugin interface version.
/// When a plugin is loaded, the `VersionInfo` of the server is
/// cross-referenced against the `VersionInfo` exported by the plugin. If they
/// don't match, the plugin fails.
///
/// The rustc version must match exactly because rust doesn't expose a stable
/// ABI, and there is a risk of it changing in any version. The plugin
/// interface version is also exposed to allow for changes to the interface in
/// the future. If the interface is changed, this value should be incremented.
#[derive(Debug, Eq, PartialEq)]
pub struct VersionInfo {
    /// The version of rustc used to compile. This must match because rust
    /// doesn't expose a stable ABI, and there is a risk of it changing in any
    /// version.
    pub rustc: String,
    /// The plugin interface version is also exposed to allow for changes to
    /// the interface in the future.
    pub plugin_interface: u8,
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            rustc: env!("RUSTC_VERSION").to_string(),
            // If the interface is changed, this value should be incremented.
            plugin_interface: 0,
        }
    }
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rustc={}, plugin_interface={}", self.rustc, self.plugin_interface)
    }
}

/// Plugins should implement this trait.
pub trait Plugin: Send + Sync + 'static {
    /// Executes the plugin. Returns JSON that will be sent back to the
    /// calling client.
    ///
    /// # Arguments
    /// * `datastore`: The datastore.
    /// * `arg`: The argument from the calling client.
    fn call(
        &self,
        datastore: Arc<dyn indradb::Datastore + Send + Sync + 'static>,
        arg: serde_json::Value,
    ) -> Result<serde_json::Value, Error>;
}

/// A declaration of a plugin.
pub struct PluginDeclaration {
    pub version_info: VersionInfo,
    pub entries: HashMap<String, Box<dyn Plugin>>,
}

/// Libraries use this macro to register their plugins.
#[macro_export]
macro_rules! register_plugins {
    ( $indradb_interface_version:expr, $( $name:expr, $t:expr ),* ) => {
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn register() -> $crate::PluginDeclaration {
            use std::collections::HashMap;
            let mut entries = HashMap::new();
            $(
                {
                    let t: Box<dyn $crate::Plugin> = $t;
                    entries.insert($name.to_string(), t);
                }
            )*
            $crate::PluginDeclaration {
                version_info: $crate::VersionInfo::default(),
                entries,
            }
        }
    };
}
