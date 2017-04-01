use super::super::{Datastore, Transaction, EdgeQuery, VertexQuery, EdgeKey};
use super::sandbox::DatastoreTestSandbox;
use errors::Error;
use models;
use uuid::Uuid;
use chrono::UTC;
use chrono::Timelike;
use super::util::{create_edges, create_time_range_queryable_edges};
use std::collections::HashSet;
use std::f32;
use std::u32;

pub fn should_get_a_valid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();

    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(0.5).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);

    // Record the start and end time. Round off the the nanoseconds off the
    // start time, since some implementations may not have that level of
    // accuracy.
    let start_time = UTC::now().with_nanosecond(0).unwrap();
    trans.create_edge(key, weight).unwrap();
    let end_time = UTC::now();

    let e = trans.get_edges(EdgeQuery::Edge(EdgeKey::new(outbound_id, edge_t.clone(), inbound_id))).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].key.outbound_id, outbound_id);
    assert_eq!(e[0].key.t, edge_t);
    assert_eq!(e[0].key.inbound_id, inbound_id);
    assert!(e[0].weight.0 > 0.0);
    assert!(e[0].update_datetime >= start_time);
    assert!(e[0].update_datetime <= end_time);
}

pub fn should_not_get_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();

    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();

    let e = trans.get_edges(EdgeQuery::Edge(EdgeKey::new(outbound_id, edge_t.clone(), Uuid::default()))).unwrap();
    assert_eq!(e.len(), 0);
    let e = trans.get_edges(EdgeQuery::Edge(EdgeKey::new(Uuid::default(), edge_t, inbound_id))).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_create_a_valid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let trans = sandbox.transaction();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t.clone()).unwrap();

    // Set the edge and check
    let weight = models::Weight::new(0.5).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    trans.create_edge(key.clone(), weight).unwrap();
    let e = trans.get_edges(EdgeQuery::Edge(key.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);
    assert!(e[0].weight.0 > 0.0);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    let weight = models::Weight::new(-0.5).unwrap();
    trans.create_edge(key.clone(), weight).unwrap();
    let e = trans.get_edges(EdgeQuery::Edge(key.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);
    assert!(e[0].weight.0 < 0.0);
}

pub fn should_update_a_valid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let trans = sandbox.transaction();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t.clone()).unwrap();

    // Set the edge and check
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    let weight = models::Weight::new(0.5).unwrap();
    trans.create_edge(key.clone(), weight).unwrap();
    let e = trans.get_edges(EdgeQuery::Edge(key.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);
    assert!(e[0].weight.0 > 0.0);

    // Update the edge
    let weight = models::Weight::new(-0.5).unwrap();
    trans.set_edges(EdgeQuery::Edge(key.clone()), weight).unwrap();
    let e = trans.get_edges(EdgeQuery::Edge(key)).unwrap();
    assert_eq!(e.len(), 1);
    assert!(e[0].weight.0 < 0.0);
}

pub fn should_not_create_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(0.5).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), Uuid::default());
    let result = trans.create_edge(key, weight);
    assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_not_create_an_edge_with_bad_permissions<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    trans.commit().unwrap();

    let email = sandbox.generate_unique_string("isolated");
    let (id, _) = sandbox.register_account(&email[..]);
    let trans = sandbox.datastore.transaction(id).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t, inbound_id);
    let weight = models::Weight::new(0.5).unwrap();
    let result = trans.create_edge(key, weight);
    assert_eq!(result.unwrap_err(), Error::Unauthorized);
}

pub fn should_not_update_an_edge_with_bad_permissions<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    let weight = models::Weight::new(0.5).unwrap();
    trans.create_edge(key.clone(), weight).unwrap();
    trans.commit().unwrap();

    let email = sandbox.generate_unique_string("isolated");
    let (id, _) = sandbox.register_account(&email[..]);
    let trans = sandbox.datastore.transaction(id).unwrap();
    trans.set_edges(EdgeQuery::Edge(key.clone()), models::Weight::new(-0.5).unwrap()).unwrap();
    let e = trans.get_edges(EdgeQuery::Edge(key)).unwrap();
    assert_eq!(e.len(), 1);
    assert!(e[0].weight.0 > 0.0);
}

pub fn should_delete_a_valid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();

    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(0.5).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    trans.create_edge(key.clone(), weight).unwrap();
    trans.delete_edges(EdgeQuery::Edge(key.clone())).unwrap();
    let e = trans.get_edges(EdgeQuery::Edge(key)).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_not_delete_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    trans.delete_edges(EdgeQuery::Edge(EdgeKey::new(outbound_id, edge_t, Uuid::default()))).unwrap();
}

pub fn should_not_delete_an_edge_with_bad_permissions<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();

    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(0.5).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    trans.create_edge(key, weight).unwrap();
    trans.commit().unwrap();

    let email = sandbox.generate_unique_string("isolated");
    let (id, _) = sandbox.register_account(&email[..]);
    let trans = sandbox.datastore.transaction(id).unwrap();
    trans.delete_edges(EdgeQuery::Edge(EdgeKey::new(outbound_id, edge_t.clone(), inbound_id))).unwrap();
    let e = trans.get_edges(EdgeQuery::Edge(EdgeKey::new(outbound_id, edge_t, inbound_id))).unwrap();
    assert_eq!(e.len(), 1);
}

pub fn should_get_an_edge_count<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), None, None, u32::MAX);
    let count = trans.get_edge_count(q).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(None, None, None, u32::MAX);
    let count = trans.get_edge_count(q).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertex(Uuid::default()).outbound_edges(Some(t), None, None, u32::MAX);
    let count = trans.get_edge_count(q).unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_edge_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), Some(end_time), Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_with_no_type<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(None, Some(end_time), Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("foo".to_string()).unwrap();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), Some(end_time), Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 0);
}

pub fn should_get_edges_with_no_high<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, _, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), None, Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 10);
}

pub fn should_get_edges_with_no_low<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), Some(end_time), None, 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 10);
}

pub fn should_get_edges_with_no_time<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _, _, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), None, None, 100);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 15);
}

pub fn should_get_no_edges_for_reversed_time<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertex(outbound_id).outbound_edges(Some(t), Some(start_time), Some(end_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 0);
}

pub fn should_get_edges<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _, _, inbound_ids) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = EdgeQuery::Edges(vec![
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[0]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[1]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[2]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[3]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[4]),
    ]);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(range, outbound_id, 5);
}

fn check_edge_range(range: Vec<models::Edge>, expected_outbound_id: Uuid, expected_length: usize) {
    assert_eq!(range.len(), expected_length);
    let mut covered_ids: HashSet<Uuid> = HashSet::new();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();

    for edge in &range {
        assert_eq!(edge.key.outbound_id, expected_outbound_id);
        assert_eq!(edge.key.t, t);
        assert!(edge.weight.0 <= 1.0 + f32::EPSILON && edge.weight.0 >= 1.0 - f32::EPSILON);
        assert!(!covered_ids.contains(&edge.key.inbound_id));
        covered_ids.insert(edge.key.inbound_id);
    }
}
