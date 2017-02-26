use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use super::util::*;
use models;
use uuid::Uuid;
use std::collections::HashSet;
use std::f32;

pub fn should_get_an_edge_count<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans.get_edge_count(outbound_id, Some(t)).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let count = trans.get_edge_count(outbound_id, None).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans.get_edge_count(Uuid::default(), Some(t)).unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_empty_edge_range_with_zero_limit<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_range(outbound_id, Some(t), Uuid::default(), 0).unwrap();
    check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_empty_edge_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, inbound_ids) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_range(outbound_id, Some(t), next_uuid(max_uuid(&inbound_ids)), 5).unwrap();
    check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_edge_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_range(outbound_id, Some(t), Uuid::default(), 10).unwrap();
    check_edge_range(range, outbound_id, 5);
}

pub fn should_get_an_edge_range_with_no_type<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let range = trans.get_edge_range(outbound_id, None, Uuid::default(), 10).unwrap();
    check_edge_range(range, outbound_id, 5);
}

pub fn should_get_a_partial_edge_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, inbound_ids) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_range(outbound_id, Some(t), next_uuid(min_uuid(&inbound_ids)), 3).unwrap();
    check_edge_range(range, outbound_id, 3);
}

pub fn should_get_an_empty_edge_range_for_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_range(outbound_id, Some(edge_t), Uuid::default(), 10).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_edges_by_a_time_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_time_range(outbound_id, Some(t), Some(end_time), Some(start_time), 10)
        .unwrap();
    check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_by_a_time_range_with_no_type<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let range = trans.get_edge_time_range(outbound_id, None, Some(end_time), Some(start_time), 10)
        .unwrap();
    check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_time_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("foo".to_string()).unwrap();
    let range = trans.get_edge_time_range(outbound_id, Some(t), Some(end_time), Some(start_time), 10)
        .unwrap();
    check_edge_range(range, outbound_id, 0);
}

pub fn should_get_edges_by_a_time_range_with_no_high<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, _, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_time_range(outbound_id, Some(t), Option::None, Some(start_time), 15)
        .unwrap();
    check_edge_range(range, outbound_id, 10);
}

pub fn should_get_edges_by_a_time_range_with_no_low<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_time_range(outbound_id, Some(t), Some(end_time), None, 15).unwrap();
    check_edge_range(range, outbound_id, 10);
}

pub fn should_get_edges_by_a_time_range_with_no_time<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _, _, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_time_range(outbound_id, Some(t), None, None, 20).unwrap();
    check_edge_range(range, outbound_id, 15);
}

pub fn should_get_no_edges_for_a_reversed_time_range<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_edge_time_range(outbound_id, Some(t), Some(start_time), Some(end_time), 10)
        .unwrap();
    check_edge_range(range, outbound_id, 0);
}

fn check_edge_range(range: Vec<models::Edge>, expected_outbound_id: Uuid, expected_length: usize) {
    assert_eq!(range.len(), expected_length);
    let mut covered_ids: HashSet<Uuid> = HashSet::new();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();

    for edge in &range {
        assert_eq!(edge.outbound_id, expected_outbound_id);
        assert_eq!(edge.t, t);
        assert!(edge.weight.0 <= 1.0 + f32::EPSILON && edge.weight.0 >= 1.0 - f32::EPSILON);
        assert!(!covered_ids.contains(&edge.inbound_id));
        covered_ids.insert(edge.inbound_id);
    }
}
