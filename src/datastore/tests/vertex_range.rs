use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use models;
use uuid::Uuid;
use std::collections::HashSet;
use std::u16;

// NOTE: We don't have nearly as extensive of set of tests against vertex
// range operations, because the results from a query may include vertices
// created in concurrently running tests.
pub fn should_get_a_vertex_range<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();

    let mut inserted_ids = vec![
        trans.create_vertex(vertex_t.clone()).unwrap(),
        trans.create_vertex(vertex_t.clone()).unwrap(),
        trans.create_vertex(vertex_t.clone()).unwrap(),
        trans.create_vertex(vertex_t.clone()).unwrap(),
        trans.create_vertex(vertex_t.clone()).unwrap()
    ];

    inserted_ids.sort();
    let range = trans.get_vertex_range(Uuid::default(), u16::MAX).unwrap();
    trans.commit().unwrap();

    assert!(range.len() >= 5);

    let mut covered_ids: HashSet<Uuid> = HashSet::new();

    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, vertex_t.clone());
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}
