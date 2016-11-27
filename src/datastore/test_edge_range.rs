use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use models;
use traits::Id;
use std::collections::HashSet;

pub fn should_get_an_edge_count<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let count = trans.get_edge_count(outbound_id, t).unwrap();
	assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let count = trans.get_edge_count(I::default(), t).unwrap();
	assert_eq!(count, 0);
}

pub fn should_get_an_empty_edge_range_with_zero_limit<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_range(outbound_id, t, 5, 0).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_empty_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_range(outbound_id, t, 5, 5).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_range(outbound_id, t, 0, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_an_empty_edge_range_for_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
	let outbound_id = trans.create_vertex(vertex_t).unwrap();
	let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_range(outbound_id, edge_t, 0, 10).unwrap();
	assert_eq!(range.len(), 0);
}

pub fn should_get_edges_by_a_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_time_range(outbound_id, t, get_after(), get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("foo".to_string()).unwrap();
	let range = trans.get_edge_time_range(outbound_id, t, get_after(), get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_edges_by_a_time_range_with_no_high<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_time_range(outbound_id, t, Option::None, get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_by_a_time_range_with_no_low<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_time_range(outbound_id, t, get_after(), Option::None, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_by_a_time_range_with_no_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_time_range(outbound_id, t, Option::None, Option::None, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_a_reversed_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let range = trans.get_edge_time_range(outbound_id, t, get_before(), get_after(), 10).unwrap();
	check_edge_range(range, outbound_id, 0);
}

fn check_edge_range<I: Id>(range: Vec<models::Edge<I>>, expected_outbound_id: I, expected_length: usize) {
	assert_eq!(range.len(), expected_length);
	let mut covered_ids: HashSet<I> = HashSet::new();
	let t = models::Type::new("test_edge_type".to_string()).unwrap();

	for edge in range.iter() {
		assert_eq!(edge.outbound_id, expected_outbound_id);
		assert_eq!(edge.t, t);
		assert_eq!(edge.weight.0, 1.0);
		assert!(!covered_ids.contains(&edge.inbound_id));
		covered_ids.insert(edge.inbound_id);
	}
}