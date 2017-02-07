use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use super::util::*;
use models;
use traits::Id;
use std::collections::HashSet;

pub fn should_get_a_reversed_edge_count<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, _) = create_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans.get_reversed_edge_count(inbound_id, Some(t)).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_a_reversed_edge_count_without_type<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, _) = create_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let count = trans.get_reversed_edge_count(inbound_id, None).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_a_reversed_edge_count_for_an_invalid_edge<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans.get_reversed_edge_count(I::default(), Some(t)).unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_empty_reversed_edge_range_with_zero_limit<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let (inbound_id, _) = create_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_reversed_edge_range(inbound_id, Some(t), 5, 0).unwrap();
    check_edge_range(range, inbound_id, 0);
}

pub fn should_get_an_empty_reversed_edge_range<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, _) = create_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_reversed_edge_range(inbound_id, Some(t), 5, 5).unwrap();
    check_edge_range(range, inbound_id, 0);
}

pub fn should_get_a_reversed_edge_range<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, _) = create_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_reversed_edge_range(inbound_id, Some(t), 0, 10).unwrap();
    check_edge_range(range, inbound_id, 5);
}

pub fn should_get_a_reversed_edge_range_without_type<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, _) = create_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let range = trans.get_reversed_edge_range(inbound_id, None, 0, 10).unwrap();
    check_edge_range(range, inbound_id, 5);
}

pub fn should_get_a_partial_reversed_edge_range<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, _) = create_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_reversed_edge_range(inbound_id, Some(t), 1, 3).unwrap();
    check_edge_range(range, inbound_id, 3);
}

pub fn should_get_an_empty_reversed_edge_range_for_an_invalid_edge<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_reversed_edge_range(inbound_id, Some(edge_t), 0, 10).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_reversed_edges_by_a_time_range<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, start_time, end_time, _) =
        create_time_range_queryable_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range =
        trans.get_reversed_edge_time_range(inbound_id, Some(t), Some(end_time), Some(start_time), 10)
            .unwrap();
    check_edge_range(range, inbound_id, 5);
}

pub fn should_get_reversed_edges_by_a_time_range_without_type<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let (inbound_id, start_time, end_time, _) =
        create_time_range_queryable_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let range =
        trans.get_reversed_edge_time_range(inbound_id, None, Some(end_time), Some(start_time), 10)
            .unwrap();
    check_edge_range(range, inbound_id, 5);
}

pub fn should_get_no_reversed_edges_for_an_invalid_time_range<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let (inbound_id, start_time, end_time, _) =
        create_time_range_queryable_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("foo".to_string()).unwrap();
    let range =
        trans.get_reversed_edge_time_range(inbound_id, Some(t), Some(end_time), Some(start_time), 10)
            .unwrap();
    check_edge_range(range, inbound_id, 0);
}

pub fn should_get_reversed_edges_by_a_time_range_with_no_high<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let (inbound_id, start_time, _, _) = create_time_range_queryable_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range =
        trans.get_reversed_edge_time_range(inbound_id, Some(t), Option::None, Some(start_time), 15)
            .unwrap();
    check_edge_range(range, inbound_id, 10);
}

pub fn should_get_reversed_edges_by_a_time_range_with_no_low<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let (inbound_id, _, end_time, _) = create_time_range_queryable_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_reversed_edge_time_range(inbound_id, Some(t), Some(end_time), None, 15)
        .unwrap();
    check_edge_range(range, inbound_id, 10);
}

pub fn should_get_reversed_edges_by_a_time_range_with_no_time<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let (inbound_id, _, _, _) = create_time_range_queryable_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range = trans.get_reversed_edge_time_range(inbound_id, Some(t), None, None, 20).unwrap();
    check_edge_range(range, inbound_id, 15);
}

pub fn should_get_no_reversed_edges_for_a_reversed_time_range<D, T, I>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>)
	where D: Datastore<T, I>,
		  T: Transaction<I>,
		  I: Id {
    let (inbound_id, start_time, end_time, _) =
        create_time_range_queryable_reversed_edges(&mut sandbox);
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let range =
        trans.get_reversed_edge_time_range(inbound_id, Some(t), Some(start_time), Some(end_time), 10)
            .unwrap();
    check_edge_range(range, inbound_id, 0);
}

fn check_edge_range<I: Id>(range: Vec<models::Edge<I>>, expected_inbound_id: I, expected_length: usize) {
    assert_eq!(range.len(), expected_length);
    let mut covered_ids: HashSet<I> = HashSet::new();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();

    for edge in range.iter() {
        assert_eq!(edge.inbound_id, expected_inbound_id);
        assert_eq!(edge.t, t);
        assert_eq!(edge.weight.0, 1.0);
        assert!(!covered_ids.contains(&edge.outbound_id));
        covered_ids.insert(edge.outbound_id);
    }
}
