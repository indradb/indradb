use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use errors::Error;
use models;
use traits::Id;
use serde_json::Value as JsonValue;
use std::collections::HashSet;

pub fn should_get_an_edge_count<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let count = trans.get_edge_count(outbound_id, "test_edge_type".to_string()).unwrap();
	assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let count = trans.get_edge_count(I::default(), "test_edge_type".to_string()).unwrap();
	assert_eq!(count, 0);
}

pub fn should_get_an_empty_edge_range_with_zero_limit<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_range(outbound_id, "test_edge_type".to_string(), 5, 0).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_empty_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_range(outbound_id, "test_edge_type".to_string(), 5, 5).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_range(outbound_id, "test_edge_type".to_string(), 0, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_an_empty_edge_range_for_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let range = trans.get_edge_range(outbound_id, "test_edge_type".to_string(), 0, 10).unwrap();
	assert_eq!(range.len(), 0);
}

pub fn should_get_edges_by_a_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "foo".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_edges_by_a_time_range_with_no_high<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), Option::None, get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_by_a_time_range_with_no_low<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), get_after(), Option::None, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_by_a_time_range_with_no_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), Option::None, Option::None, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_a_reversed_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), get_before(), get_after(), 10).unwrap();
	check_edge_range(range, outbound_id, 0);
}

fn check_edge_range<I: Id>(range: Vec<models::Edge<I>>, expected_outbound_id: I, expected_length: usize) {
	assert_eq!(range.len(), expected_length);
	let mut covered_ids: HashSet<I> = HashSet::new();

	for edge in range.iter() {
		assert_eq!(edge.outbound_id, expected_outbound_id);
		assert_eq!(edge.t, "test_edge_type".to_string());
		assert_eq!(edge.weight, 1.0);
		assert!(!covered_ids.contains(&edge.inbound_id));
		covered_ids.insert(edge.inbound_id);
	}
}