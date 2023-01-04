//! The in-memory datastore implementation. This is the simplest and generally
//! fastest implementation, but there's no support for graphs larger than what
//! can fit in-memory, and data is only persisted to disk when explicitly
//! requested.

mod datastore;

pub use datastore::{default, create_msgpack, read_msgpack, MemoryTransactionBuilder};

#[cfg(feature = "bench-suite")]
full_bench_impl!(MemoryDatastore::default());

#[cfg(feature = "test-suite")]
#[cfg(test)]
mod tests {
    use super::{read_msgpack, create_msgpack, default, MemoryTransactionBuilder};
    use crate::{Datastore, Identifier, SpecificVertexQuery};
    use tempfile::NamedTempFile;
    use uuid::Uuid;

    full_test_impl!(default());

    #[allow(deprecated)]
    fn create_vertex_with_property(datastore: &Datastore<MemoryTransactionBuilder>) -> Uuid {
        let id = datastore.create_vertex_from_type(Identifier::default()).unwrap();
        datastore
            .set_properties(
                SpecificVertexQuery::single(id).into(),
                Identifier::default(),
                serde_json::Value::Bool(true),
            )
            .unwrap();
        id
    }

    #[allow(deprecated)]
    fn expect_vertex(datastore: &Datastore<MemoryTransactionBuilder>, id: Uuid) {
        assert_eq!(datastore.get_vertex_count().unwrap(), 1);
        let vertices = datastore
            .get_vertices(SpecificVertexQuery::new(vec![id]).into())
            .unwrap();
        assert_eq!(vertices.len(), 1);
        assert_eq!(vertices[0].id, id);
        assert_eq!(vertices[0].t, Identifier::default());
    }

    #[test]
    fn should_serialize_msgpack() {
        let path = NamedTempFile::new().unwrap();
        let datastore = create_msgpack(path.path()).unwrap();
        let id = create_vertex_with_property(&datastore);
        datastore.sync().unwrap();
        let datastore = read_msgpack(path.path()).unwrap();
        expect_vertex(&datastore, id);
    }
}
