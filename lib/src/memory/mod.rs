//! The in-memory datastore implementation. This is the simplest and generally
//! fastest implementation, but there's no support for graphs larger than what
//! can fit in-memory, and data is only persisted to disk when explicitly
//! requested.

mod datastore;

pub use self::datastore::MemoryDatastore;

#[cfg(feature = "bench-suite")]
full_bench_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
#[cfg(test)]
mod tests {
    use super::MemoryDatastore;
    use crate::{Datastore, Error, Identifier, SpecificVertexQuery, VertexPropertyQuery};
    use tempfile::NamedTempFile;
    use uuid::Uuid;

    full_test_impl!(MemoryDatastore::default());

    fn create_vertex(datastore: &MemoryDatastore, with_property: bool) -> Uuid {
        let id = datastore.create_vertex_from_type(Identifier::default()).unwrap();
        if with_property {
            datastore
                .set_vertex_properties(
                    VertexPropertyQuery {
                        inner: SpecificVertexQuery::single(id).into(),
                        name: Identifier::default(),
                    },
                    serde_json::Value::Bool(true),
                )
                .unwrap();
        }
        id
    }

    fn expect_vertex(datastore: &MemoryDatastore, id: Uuid) {
        assert_eq!(datastore.get_vertex_count().unwrap(), 1);
        let vertices = datastore
            .get_vertices(SpecificVertexQuery::new(vec![id]).into())
            .unwrap();
        assert_eq!(vertices.len(), 1);
        assert_eq!(vertices[0].id, id);
        assert_eq!(vertices[0].t, Identifier::default());
    }

    #[test]
    #[allow(deprecated)]
    fn should_serialize_bincode() {
        let path = NamedTempFile::new().unwrap();
        let datastore = MemoryDatastore::create(path.path()).unwrap();
        let id = create_vertex(&datastore, false);
        datastore.sync().unwrap();
        let datastore = MemoryDatastore::read(path.path()).unwrap();
        expect_vertex(&datastore, id);
    }

    #[test]
    #[allow(deprecated)]
    fn should_not_serialize_bincode_properties() {
        let path = NamedTempFile::new().unwrap();
        let datastore = MemoryDatastore::create(path.path()).unwrap();
        create_vertex(&datastore, true);
        let result = datastore.sync();
        match result {
            Err(Error::Unsupported) => (),
            _ => assert!(false, "unexpected result: {:?}", result),
        }
    }

    #[test]
    fn should_serialize_msgpack() {
        let path = NamedTempFile::new().unwrap();
        let datastore = MemoryDatastore::create_msgpack(path.path()).unwrap();
        let id = create_vertex(&datastore, true);
        datastore.sync().unwrap();
        let datastore = MemoryDatastore::read_msgpack(path.path()).unwrap();
        expect_vertex(&datastore, id);
    }
}
