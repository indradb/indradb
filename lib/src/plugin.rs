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

macro_rules! proxy_fn {
    ($this: expr, $name:ident, $($arg:tt)*) => (
        {
            match *$this {
                Self::Memory(ref t) => t.$name($($arg)*),
                Self::Rocksdb(ref t) => t.$name($($arg)*),
            }
        }
    )
}

pub enum ProxyDatastore {
    Memory(crate::MemoryDatastore),
    Rocksdb(crate::RocksdbDatastore),
}

impl crate::Datastore for ProxyDatastore {
    type Trans = ProxyTransaction;

    fn transaction(&self) -> crate::Result<Self::Trans> {
        match self {
            ProxyDatastore::Memory(ds) => Ok(ProxyTransaction::Memory(ds.transaction()?)),
            ProxyDatastore::Rocksdb(ds) => Ok(ProxyTransaction::Rocksdb(ds.transaction()?)),
        }
    }

    fn bulk_insert<I>(&self, items: I) -> crate::Result<()>
    where
        I: Iterator<Item = crate::BulkInsertItem>,
    {
        proxy_fn!(self, bulk_insert, items)
    }

    fn index_property<T: Into<crate::Identifier>>(&self, name: T) -> crate::Result<()> {
        proxy_fn!(self, index_property, name)
    }
}

pub enum ProxyTransaction {
    Memory(crate::MemoryTransaction),
    Rocksdb(crate::RocksdbTransaction),
}

impl crate::Transaction for ProxyTransaction {
    fn create_vertex(&self, vertex: &crate::Vertex) -> crate::Result<bool> {
        proxy_fn!(self, create_vertex, vertex)
    }

    fn create_vertex_from_type(&self, t: crate::Identifier) -> crate::Result<uuid::Uuid> {
        proxy_fn!(self, create_vertex_from_type, t)
    }

    fn get_vertices<Q: Into<crate::VertexQuery>>(&self, q: Q) -> crate::Result<Vec<crate::Vertex>> {
        proxy_fn!(self, get_vertices, q)
    }

    fn delete_vertices<Q: Into<crate::VertexQuery>>(&self, q: Q) -> crate::Result<()> {
        proxy_fn!(self, delete_vertices, q)
    }

    fn get_vertex_count(&self) -> crate::Result<u64> {
        proxy_fn!(self, get_vertex_count,)
    }

    fn create_edge(&self, key: &crate::EdgeKey) -> crate::Result<bool> {
        proxy_fn!(self, create_edge, key)
    }

    fn get_edges<Q: Into<crate::EdgeQuery>>(&self, q: Q) -> crate::Result<Vec<crate::Edge>> {
        proxy_fn!(self, get_edges, q)
    }

    fn delete_edges<Q: Into<crate::EdgeQuery>>(&self, q: Q) -> crate::Result<()> {
        proxy_fn!(self, delete_edges, q)
    }

    fn get_edge_count(&self, id: uuid::Uuid, t: Option<&crate::Identifier>, direction: crate::EdgeDirection) -> crate::Result<u64> {
        proxy_fn!(self, get_edge_count, id, t, direction)
    }

    fn get_vertex_properties(&self, q: crate::VertexPropertyQuery) -> crate::Result<Vec<crate::VertexProperty>> {
        proxy_fn!(self, get_vertex_properties, q)
    }

    fn get_all_vertex_properties<Q: Into<crate::VertexQuery>>(&self, q: Q) -> crate::Result<Vec<crate::VertexProperties>> {
        proxy_fn!(self, get_all_vertex_properties, q)
    }

    fn set_vertex_properties(&self, q: crate::VertexPropertyQuery, value: &crate::JsonValue) -> crate::Result<()> {
        proxy_fn!(self, set_vertex_properties, q, value)
    }

    fn delete_vertex_properties(&self, q: crate::VertexPropertyQuery) -> crate::Result<()> {
        proxy_fn!(self, delete_vertex_properties, q)
    }

    fn get_edge_properties(&self, q: crate::EdgePropertyQuery) -> crate::Result<Vec<crate::EdgeProperty>> {
        proxy_fn!(self, get_edge_properties, q)
    }

    fn get_all_edge_properties<Q: Into<crate::EdgeQuery>>(&self, q: Q) -> crate::Result<Vec<crate::EdgeProperties>> {
        proxy_fn!(self, get_all_edge_properties, q)
    }

    fn set_edge_properties(&self, q: crate::EdgePropertyQuery, value: &crate::JsonValue) -> crate::Result<()> {
        proxy_fn!(self, set_edge_properties, q, value)
    }

    fn delete_edge_properties(&self, q: crate::EdgePropertyQuery) -> crate::Result<()> {
        proxy_fn!(self, delete_edge_properties, q)
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
    fn call(&self, datastore: ProxyDatastore, arg: crate::JsonValue) -> crate::Result<crate::JsonValue>;
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
