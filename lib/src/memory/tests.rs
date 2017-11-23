#![cfg(test)]

pub use super::datastore::MemoryDatastore;
pub use super::super::tests;

full_test_impl!({
    MemoryDatastore::new(false)
});
