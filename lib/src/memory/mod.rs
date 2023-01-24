//! The in-memory datastore implementation. This is the simplest and generally
//! fastest implementation, but there's no support for graphs larger than what
//! can fit in-memory, and data is only persisted to disk when explicitly
//! requested.

mod datastore;

pub use datastore::MemoryDatastore;

#[cfg(feature = "bench-suite")]
full_bench_impl!(MemoryDatastore::new_db());

#[cfg(feature = "test-suite")]
#[cfg(test)]
mod tests {
    use super::MemoryDatastore;
    use crate::util::{extract_count, extract_vertices};
    use crate::{ijson, AllVertexQuery, CountQueryExt, Database, Identifier, SpecificVertexQuery};

    use tempfile::NamedTempFile;
    use uuid::Uuid;

    full_test_impl!(MemoryDatastore::new_db());

    fn create_vertex_with_property(db: &Database<MemoryDatastore>) -> Uuid {
        let id = db.create_vertex_from_type(Identifier::default()).unwrap();
        db.set_properties(SpecificVertexQuery::single(id), Identifier::default(), &ijson!(true))
            .unwrap();
        id
    }

    fn expect_vertex(db: &Database<MemoryDatastore>, id: Uuid) {
        assert_eq!(extract_count(db.get(AllVertexQuery.count().unwrap()).unwrap()), Some(1));
        let vertices = extract_vertices(db.get(SpecificVertexQuery::new(vec![id])).unwrap()).unwrap();
        assert_eq!(vertices.len(), 1);
        assert_eq!(vertices[0].id, id);
        assert_eq!(vertices[0].t, Identifier::default());
    }

    #[test]
    fn should_serialize_msgpack() {
        let path = NamedTempFile::new().unwrap();
        let db = MemoryDatastore::create_msgpack_db(path.path());
        let id = create_vertex_with_property(&db);
        db.sync().unwrap();
        let db = MemoryDatastore::read_msgpack_db(path.path()).unwrap();
        expect_vertex(&db, id);
    }
}
