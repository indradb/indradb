#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_lifetimes))]
include!(concat!(env!("OUT_DIR"), "/indradb_capnp.rs"));

pub mod util;
