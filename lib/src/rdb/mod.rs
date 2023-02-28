//! The rocksdb datastore implementation.

mod datastore;
mod managers;

pub use self::datastore::RocksdbDatastore;

#[cfg(feature = "bench-suite")]
full_bench_impl!({
    use super::RocksdbDatastore;
    use tempfile::tempdir;
    let path = tempdir().unwrap().into_path();
    RocksdbDatastore::new_db_with_options(path, &RocksdbDatastore::get_options(Some(1))).unwrap()
});

#[cfg(feature = "test-suite")]
#[cfg(test)]
mod tests {
    use crate::QueryExt;
    use tempfile::tempdir;

    #[cfg(feature = "test-suite")]
    full_test_impl!({
        use super::RocksdbDatastore;
        use tempfile::tempdir;

        let path = tempdir().unwrap().into_path();
        RocksdbDatastore::new_db_with_options(path, &RocksdbDatastore::get_options(Some(1))).unwrap()
    });

    #[test]
    fn should_repair() {
        use super::RocksdbDatastore;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();

        // // Make sure we just initialize the database
        RocksdbDatastore::new_db_with_options(dir.path(), &RocksdbDatastore::get_options(Some(1))).unwrap();

        // Now try to repair
        RocksdbDatastore::repair(dir.path(), &RocksdbDatastore::get_options(Some(1))).unwrap();
    }

    // Tests for a regression where reversed range queries were incorrect.
    // See https://github.com/indradb/indradb/issues/280
    #[test]
    fn test_repeated_reverse_fetch_operation() {
        let path = tempdir().unwrap().into_path();

        for _i in 0..2 {
            let db: crate::Database<crate::RocksdbDatastore> = crate::RocksdbDatastore::new_db(&path).unwrap();

            // Create a couple of vertices
            let out_v = crate::Vertex::new(crate::Identifier::new("person").unwrap());
            let in_v = crate::Vertex::new(crate::Identifier::new("movie").unwrap());
            db.create_vertex(&out_v).unwrap();
            db.create_vertex(&in_v).unwrap();

            // Add an edge between the vertices
            let edge = crate::Edge::new(out_v.id, crate::Identifier::new("likes").unwrap(), in_v.id);
            db.create_edge(&edge).unwrap();

            // Query for the edge, by inbound of the in_v
            let output_failed = db
                .get(crate::SpecificVertexQuery::new(vec![in_v.id]).inbound().unwrap())
                .unwrap();

            let e = crate::util::extract_edges(output_failed).unwrap();
            assert_eq!(e.len(), 1);
            assert_eq!(edge, e[0]);
            assert_eq!(edge.inbound_id, in_v.id);
            assert_eq!(edge.outbound_id, out_v.id);
        }
    }
}
