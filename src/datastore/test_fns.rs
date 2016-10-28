use super::{Datastore, Transaction};
use super::test_sandbox::{DatastoreTestSandbox, insert_sample_data};
use super::test_util::*;
use util::Error;
use models;
use traits::Id;
use std::collections::HashSet;
use serde_json::Value as JsonValue;

pub fn should_fail_auth_with_a_bad_username<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(I::default(), "foobar".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn should_fail_auth_with_a_bad_password<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id, "bad_token".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn should_successfully_auth_with_good_credentials<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id, sandbox.owner_secret.clone());
	assert!(auth.is_ok());
	assert!(auth.unwrap());
}

pub fn should_lookup_valid_accounts<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(sandbox.owner_id);
	assert!(results.is_ok());
	assert!(results.unwrap());
}

pub fn should_fail_to_lookup_invalid_accounts<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(I::default());
	assert!(results.is_ok());
	assert!(!results.unwrap());
}

pub fn should_fail_when_attempting_to_delete_invalid_accounts<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let result = sandbox.datastore.delete_account(I::default());
	assert_eq!(result.unwrap_err(), Error::AccountNotFound);
}

pub fn should_get_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let jill_id = sandbox.search_id("user", "Jill");
	let trans = sandbox.transaction();
	let v = trans.get_vertex(jill_id).unwrap();
	assert_eq!(v.id, jill_id);
	assert_eq!(v.t, "user".to_string());
	let expected_properties = create_test_properties("Jill");
	assert_eq!(v.properties, expected_properties);
}

pub fn should_not_get_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn should_create_a_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let props = create_test_properties("Jill 2.0");
	let trans = sandbox.transaction();
	trans.create_vertex("user".to_string(), props).unwrap();
}

pub fn should_update_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	// First create a vertex
	let created_id = sandbox.create_test_vertex("movie", None);

	// Now update the vertex & double-check the results
	let props = create_test_properties("Trainspotting 2");
	let trans = sandbox.transaction();
	trans.set_vertex(models::Vertex::new_with_properties(created_id, "movie".to_string(), props.clone())).unwrap();
	let v = trans.get_vertex(created_id).unwrap();
	assert_eq!(v.id, created_id);
	assert_eq!(v.properties, props);
}

pub fn should_not_update_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.set_vertex(models::Vertex::new(I::default(), "movie".to_string()));
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn should_delete_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let jill_id = sandbox.search_id("user", "Jill");
	let christopher_id = sandbox.search_id("user", "Christopher");

	// First create a vertex
	let id = sandbox.create_test_vertex("movie", None);

	// Create some edges, then delete the vertex and make sure the edges were cleared
	let trans = sandbox.transaction();
	trans.set_edge(models::Edge::new(id, "follows".to_string(), jill_id, 1.0)).unwrap();
	trans.set_edge(models::Edge::new(id, "review".to_string(), sandbox.search_id("movie", "Memento"), 1.0)).unwrap();
	trans.set_edge(models::Edge::new(christopher_id, "follows".to_string(), id, 1.0)).unwrap();
	trans.delete_vertex(id).unwrap();
	let result = trans.get_vertex(id);
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
	let count = trans.get_edge_count(id, "follows".to_string()).unwrap();
	assert_eq!(count, 0);
	let count = trans.get_edge_count(id, "review".to_string()).unwrap();
	assert_eq!(count, 0);
	let count = trans.get_edge_count(christopher_id, "follows".to_string()).unwrap();
	assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.delete_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn should_not_delete_an_unowned_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).unwrap();
	let result = trans.delete_vertex(sandbox.search_id("user", "Jill"));
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn should_get_a_valid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let jill_id = sandbox.search_id("user", "Jill");
	let inception_id = sandbox.search_id("movie", "Inception");
	let trans = sandbox.transaction();
	let e = trans.get_edge(jill_id, "review".to_string(), inception_id).unwrap();
	assert_eq!(e.outbound_id, jill_id);
	assert_eq!(e.t, "review".to_string());
	assert_eq!(e.inbound_id, inception_id);
}

pub fn should_not_get_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let result = trans.get_edge(sandbox.search_id("user", "Jill"), "review".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn should_update_a_valid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let jill_id = sandbox.search_id("user", "Jill");
	let christopher_id = sandbox.search_id("user", "Christopher");

	// This also tests adding a new type that didn't previously exist
	let e1 = models::Edge::new(jill_id, "blocks".to_string(), christopher_id, 0.5);
	let e2 = models::Edge::new(jill_id, "blocks".to_string(), christopher_id, -0.5);

	let trans = sandbox.transaction();
	let result = trans.get_edge(jill_id, "blocks".to_string(), christopher_id);
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
	trans.set_edge(e1.clone()).unwrap();
	let e = trans.get_edge(jill_id, "blocks".to_string(), christopher_id).unwrap();
	assert_eq!(e1, e);
	trans.set_edge(e2.clone()).unwrap();
	let e = trans.get_edge(jill_id, "blocks".to_string(), christopher_id).unwrap();
	assert_eq!(e2, e);
}

pub fn should_not_update_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);

	let trans = sandbox.transaction();
	let result = trans.set_edge(models::Edge::new(sandbox.search_id("user", "Jill"), "blocks".to_string(), I::default(), 0.5));
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn should_not_set_an_edge_with_a_bad_weight<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let jill_id = sandbox.search_id("user", "Jill");
	let bob_id = sandbox.search_id("user", "Bob");

	let trans = sandbox.transaction();
	let result = trans.set_edge(models::Edge::new(jill_id, "blocks".to_string(), bob_id, 1.01));
	assert_eq!(result.unwrap_err(), Error::WeightOutOfRange);
	let result = trans.set_edge(models::Edge::new(jill_id, "blocks".to_string(), bob_id, -1.01));
	assert_eq!(result.unwrap_err(), Error::WeightOutOfRange);
}

pub fn should_not_set_an_edge_with_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	let result = trans.set_edge(models::Edge::new(sandbox.search_id("user", "Jill"), "blocks".to_string(), sandbox.search_id("user", "Christopher"), 0.5));
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn should_delete_a_valid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let bob_id = sandbox.search_id("user", "Bob");
	let christopher_id = sandbox.search_id("user", "Christopher");

	let trans = sandbox.transaction();
	let e = models::Edge::new(bob_id, "blocks".to_string(), christopher_id, 0.5);
	trans.set_edge(e).unwrap();
	trans.get_edge(bob_id, "blocks".to_string(), christopher_id).unwrap();
	trans.delete_edge(bob_id, "blocks".to_string(), christopher_id).unwrap();
	let result = trans.get_edge(bob_id, "blocks".to_string(), christopher_id);
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn should_not_delete_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let result = trans.delete_edge(sandbox.search_id("user", "Jill"), "blocks".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn should_not_delete_an_edge_with_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	let result = trans.delete_edge(sandbox.search_id("user", "Jill"), "blocks".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn should_get_an_edge_count<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let count = trans.get_edge_count(sandbox.search_id("user", "Christopher"), "purchased".to_string()).unwrap();
	assert_eq!(count, 10);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let count = trans.get_edge_count(I::default(), "purchased".to_string()).unwrap();
	assert_eq!(count, 0);
}

pub fn should_get_an_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let christopher_id = sandbox.search_id("user", "Christopher");

	let check_range = |range: Vec<models::Edge<I>>, count: usize| {
		assert_eq!(range.len(), count);
		let mut covered_ids: HashSet<I> = HashSet::new();

		for edge in range.iter() {
			assert_eq!(edge.outbound_id, christopher_id);
			assert_eq!(edge.t, "purchased".to_string());
			assert_eq!(edge.weight, 1.0);
			assert_eq!(edge.properties.len(), 0);
			assert!(!covered_ids.contains(&edge.inbound_id));
			covered_ids.insert(edge.inbound_id);
		}
	};

	let trans = sandbox.transaction();
	let range = trans.get_edge_range(christopher_id, "purchased".to_string(), 0, 5).unwrap();
	check_range(range, 5);
	let range = trans.get_edge_range(christopher_id, "purchased".to_string(), 5, 0).unwrap();
	check_range(range, 0);
	let range = trans.get_edge_range(christopher_id, "purchased".to_string(), 5, 5).unwrap();
	check_range(range, 5);
}

pub fn should_get_an_empty_edge_range_for_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_range(sandbox.search_id("user", "Christopher"), "foo".to_string(), 0, 10).unwrap();
	assert_eq!(range.len(), 0);
}

pub fn should_not_get_an_edge_range_with_an_invalid_offset<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let result = trans.get_edge_range(sandbox.search_id("user", "Christopher"), "foo".to_string(), -1, 10);
	assert_eq!(result.unwrap_err(), Error::OffsetOutOfRange);
}

pub fn should_not_get_an_edge_range_with_an_invalid_limit<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let result = trans.get_edge_range(sandbox.search_id("user", "Christopher"), "foo".to_string(), 0, -1);
	assert_eq!(result.unwrap_err(), Error::LimitOutOfRange);
}

pub fn should_get_edges_by_a_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.search_id("user", "Christopher"), "review".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn should_get_no_edges_for_an_invalid_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.search_id("user", "Christopher"), "foo".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_time_range(sandbox, range, 0);
}

pub fn should_get_edges_by_a_time_range_with_no_high<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.search_id("user", "Christopher"), "review".to_string(), Option::None, get_before(), 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn should_get_edges_by_a_time_range_with_no_low<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.search_id("user", "Christopher"), "review".to_string(), get_after(), Option::None, 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn should_get_edges_by_a_time_range_with_no_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.search_id("user", "Christopher"), "review".to_string(), Option::None, Option::None, 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn should_get_no_edges_for_a_reversed_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.search_id("user", "Christopher"), "review".to_string(), get_after(), get_after(), 10).unwrap();
	check_edge_time_range(sandbox, range, 0);
}

fn check_edge_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>, range: Vec<models::Edge<I>>, expected_length: usize) {
	assert_eq!(range.len(), expected_length);
	let mut covered_ids: HashSet<I> = HashSet::new();

	for edge in range.iter() {
		assert_eq!(edge.outbound_id, sandbox.search_id("user", "Christopher"));
		assert_eq!(edge.t, "review".to_string());
		assert_eq!(edge.weight, 1.0);
		assert_eq!(edge.properties.len(), 0);
		assert!(!covered_ids.contains(&edge.inbound_id));
		covered_ids.insert(edge.inbound_id);
	}
}

pub fn should_handle_global_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let key = sandbox.generate_unique_string("global-metadata");
	let trans = sandbox.transaction();

	// Check to make sure there's no initial value
	let result = trans.get_global_metadata(key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);

	// Set and get the value as true
	let result = trans.set_global_metadata(key.clone(), JsonValue::Bool(true));
	assert!(result.is_ok());

	let result = trans.get_global_metadata(key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(true));

	// Set and get the value as false
	let result = trans.set_global_metadata(key.clone(), JsonValue::Bool(false));
	assert!(result.is_ok());

	let result = trans.get_global_metadata(key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(false));

	// Delete & check that it's deleted
	let result = trans.delete_global_metadata(key.clone());
	assert!(result.is_ok());

	let result = trans.get_global_metadata(key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn should_handle_account_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let key = sandbox.generate_unique_string("account-metadata");
	let trans = sandbox.transaction();

	// Check to make sure there's no initial value
	let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);

	// Set and get the value as true
	let result = trans.set_account_metadata(sandbox.owner_id, key.clone(), JsonValue::Bool(true));
	assert!(result.is_ok());

	let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(true));

	// Set and get the value as false
	let result = trans.set_account_metadata(sandbox.owner_id, key.clone(), JsonValue::Bool(false));
	assert!(result.is_ok());

	let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(false));

	// Delete & check that it's deleted
	let result = trans.delete_account_metadata(sandbox.owner_id, key.clone());
	assert!(result.is_ok());

	let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn should_handle_vertex_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let key = sandbox.generate_unique_string("vertex-metadata");
	let trans = sandbox.transaction();
	let owner_id = sandbox.search_id("user", "Jill");

	// Check to make sure there's no initial value
	let result = trans.get_vertex_metadata(owner_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);

	// Set and get the value as true
	let result = trans.set_vertex_metadata(owner_id, key.clone(), JsonValue::Bool(true));
	assert!(result.is_ok());

	let result = trans.get_vertex_metadata(owner_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(true));

	// Set and get the value as false
	let result = trans.set_vertex_metadata(owner_id, key.clone(), JsonValue::Bool(false));
	assert!(result.is_ok());

	let result = trans.get_vertex_metadata(owner_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(false));

	// Delete & check that it's deleted
	let result = trans.delete_vertex_metadata(owner_id, key.clone());
	assert!(result.is_ok());

	let result = trans.get_vertex_metadata(owner_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn should_handle_edge_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	insert_sample_data(&mut sandbox);
	let key = sandbox.generate_unique_string("edge-metadata");
	let trans = sandbox.transaction();
	let christopher_id = sandbox.search_id("user", "Christopher");
	let interstellar_id = sandbox.search_id("movie", "Interstellar");

	// Check to make sure there's no initial value
	let result = trans.get_edge_metadata(christopher_id, "purchased".to_string(), interstellar_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);

	// Set and get the value as true
	let result = trans.set_edge_metadata(christopher_id, "purchased".to_string(), interstellar_id, key.clone(), JsonValue::Bool(true));
	assert!(result.is_ok());

	let result = trans.get_edge_metadata(christopher_id, "purchased".to_string(), interstellar_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(true));

	// Set and get the value as false
	let result = trans.set_edge_metadata(christopher_id, "purchased".to_string(), interstellar_id, key.clone(), JsonValue::Bool(false));
	assert!(result.is_ok());

	let result = trans.get_edge_metadata(christopher_id, "purchased".to_string(), interstellar_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(false));

	// Delete & check that it's deleted
	let result = trans.delete_edge_metadata(christopher_id, "purchased".to_string(), interstellar_id, key.clone());
	assert!(result.is_ok());

	let result = trans.get_edge_metadata(christopher_id, "purchased".to_string(), interstellar_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}