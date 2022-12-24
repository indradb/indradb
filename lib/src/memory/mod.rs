//! The in-memory datastore implementation. This is the simplest and generally
//! fastest implementation, but there's no support for graphs larger than what
//! can fit in-memory, and data is only persisted to disk when explicitly
//! requested.

mod datastore;

pub use self::datastore::MemoryDatastore;

#[cfg(feature = "bench-suite")]
full_bench_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
full_test_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
#[test]
fn should_serialize() {
    use super::MemoryDatastore;
    use crate::compat::DatastoreV3CompatExt;
    use crate::{Datastore, Identifier, SpecificVertexQuery};
    use tempfile::NamedTempFile;

    let path = NamedTempFile::new().unwrap();

    let id = {
        let datastore = MemoryDatastore::create(path.path()).unwrap();
        let id = datastore.create_vertex_from_type(Identifier::default()).unwrap();
        datastore.sync().unwrap();
        id
    };

    let datastore = MemoryDatastore::read(path.path()).unwrap();
    assert_eq!(datastore.get_vertex_count().unwrap(), 1);
    let vertices = datastore
        .get_vertices(SpecificVertexQuery::new(vec![id]).into())
        .unwrap();
    assert_eq!(vertices.len(), 1);
    assert_eq!(vertices[0].id, id);
    assert_eq!(vertices[0].t, Identifier::default());
}
