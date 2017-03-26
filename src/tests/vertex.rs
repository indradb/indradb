use super::super::{Datastore, Transaction, VertexQuery};
use super::sandbox::DatastoreTestSandbox;
use super::util::*;
use uuid::Uuid;
use models;
use std::u32;
use std::collections::HashSet;

pub fn should_get_all_vertices<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
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
    let range = trans.get_vertices(VertexQuery::All(Uuid::default(), u32::MAX)).unwrap();
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

pub fn should_get_all_vertices_with_zero_limit<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
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
    let range = trans.get_vertices(VertexQuery::All(Uuid::default(), 0)).unwrap();
    trans.commit().unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_all_vertices_out_of_range<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
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
    let range = trans.get_vertices(VertexQuery::All(Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap(), u32::MAX)).unwrap();
    trans.commit().unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_single_vertices<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let inserted_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let range = trans.get_vertices(VertexQuery::Vertex(inserted_id)).unwrap();
    trans.commit().unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);
    assert_eq!(range[0].t.0, "test_vertex_type");
}

pub fn should_get_single_vertices_nonexisting<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    trans.create_vertex(vertex_t.clone()).unwrap();
    let range = trans.get_vertices(VertexQuery::Vertex(Uuid::default())).unwrap();
    trans.commit().unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_vertices<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let mut inserted_ids = vec![
        trans.create_vertex(vertex_t.clone()).unwrap(),
        trans.create_vertex(vertex_t.clone()).unwrap(),
        trans.create_vertex(vertex_t.clone()).unwrap()
    ];

    inserted_ids.sort();
    let range = trans.get_vertices(VertexQuery::Vertices(vec![inserted_ids[0], inserted_ids[1], inserted_ids[2], Uuid::default()])).unwrap();
    trans.commit().unwrap();

    assert!(range.len() == 3);

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

pub fn should_get_vertices_piped<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();

    let inserted_id_1 = trans.create_vertex(vertex_t.clone()).unwrap();
    let inserted_id_2 = create_edge_from::<D, T>(&trans, inserted_id_1);
    let inserted_id_3 = create_edge_from::<D, T>(&trans, inserted_id_2);
    let inserted_id_4 = create_edge_from::<D, T>(&trans, inserted_id_3);
    let inserted_id_5 = create_edge_from::<D, T>(&trans, inserted_id_4);

    let query = VertexQuery::Vertex(inserted_id_1)
        .outbound_edges(Some(models::Type::new("test_edge_type".to_string()).unwrap()), None, None, 1).inbound_vertices(1)
        .outbound_edges(None, None, None, 1).inbound_vertices(1)
        .outbound_edges(None, None, None, 1).inbound_vertices(1)
        .outbound_edges(None, None, None, 1).inbound_vertices(1);

    let range = trans.get_vertices(query).unwrap();
    trans.commit().unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id_5);
}

pub fn should_update_a_valid_vertex<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let original_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let id = trans.create_vertex(original_t).unwrap();
    let updated_t = models::Type::new("test_vertex_type_2".to_string()).unwrap();
    trans.set_vertices(VertexQuery::Vertex(id), updated_t.clone()).unwrap();
    let v = trans.get_vertices(VertexQuery::Vertex(id)).unwrap();
    assert_eq!(v[0].id, id);
    assert_eq!(v[0].t, updated_t);
}

pub fn should_not_update_an_invalid_vertex<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_vertex_type".to_string()).unwrap();
    trans.set_vertices(VertexQuery::Vertex(Uuid::default()), t).unwrap();
}

pub fn should_delete_a_valid_vertex<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let q = VertexQuery::Vertex(outbound_id);
    trans.delete_vertices(q.clone()).unwrap();
    let v = trans.get_vertices(q.clone()).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans.get_edge_count(VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), None, None, 10)).unwrap();
    assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction,
{
    let trans = sandbox.transaction();
    trans.delete_vertices(VertexQuery::Vertex(Uuid::default())).unwrap();
}

pub fn should_not_delete_an_unowned_vertex<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let vertex_id = trans.create_vertex(t).unwrap();
    trans.commit().unwrap();

    let email = sandbox.generate_unique_string("isolated");
    let (account_id, _) = sandbox.register_account(&email[..]);
    let trans = sandbox.datastore.transaction(account_id).unwrap();
    let q = VertexQuery::Vertex(vertex_id);
    trans.delete_vertices(q.clone()).unwrap();
    let result = trans.get_vertices(q).unwrap();
    assert_eq!(result.len(), 1);
}
