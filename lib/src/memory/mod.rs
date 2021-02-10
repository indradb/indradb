//! The in-memory datastore implementation. This is the simplest and generally
//! fastest implementation, but there's no support for graphs larger than what
//! can fit in-memory, and data is only persisted to disk when explicitly
//! requested.

mod datastore;

pub use self::datastore::{MemoryDatastore, MemoryTransaction};

#[cfg(feature = "bench-suite")]
full_bench_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
full_test_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
#[test]
fn should_serialize() {
    use super::MemoryDatastore;
    use crate::{Datastore, SpecificVertexQuery, Transaction, Type};
    use tempfile::NamedTempFile;

    let path = NamedTempFile::new().unwrap();

    let id = {
        let datastore = MemoryDatastore::create(path.path()).unwrap();
        let trans = datastore.transaction().unwrap();
        let id = trans.create_vertex_from_type(Type::default()).unwrap();
        datastore.sync().unwrap();
        id
    };

    let datastore = MemoryDatastore::read(path.path()).unwrap();
    let trans = datastore.transaction().unwrap();
    assert_eq!(trans.get_vertex_count().unwrap(), 1);
    let vertices = trans.get_vertices(SpecificVertexQuery::new(vec![id])).unwrap();
    assert_eq!(vertices.len(), 1);
    assert_eq!(vertices[0].id, id);
    assert_eq!(vertices[0].t, Type::default());
}

#[cfg(feature = "test-suite")]
#[test]
fn should_volatile_sync() {
    use super::MemoryDatastore;
    use crate::Datastore;
    let datastore = MemoryDatastore::default();
    datastore.sync().unwrap(); // should be a no-op
}
