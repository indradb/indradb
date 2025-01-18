//! Unit tests for datastore implementations.
//!
//! These are exported so that datastore implementations outside of the
//! `indradb` crate can reuse them. Generally you can use the convenience macro
//! `full_test_impl`.

mod bulk_insert;
mod edge;
mod include_query;
mod indexing;
#[macro_use]
mod macros;
mod properties;
mod sync;
mod util;
mod vertex;

pub use self::bulk_insert::*;
pub use self::edge::*;
pub use self::include_query::*;
pub use self::indexing::*;
pub use self::properties::*;
pub use self::sync::*;
pub use self::vertex::*;
