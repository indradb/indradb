use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use util::Error;
use models;
use traits::Id;
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

pub fn should_get_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let id = trans.create_vertex("test_vertex".to_string()).unwrap();
	let v = trans.get_vertex(id).unwrap();
	assert_eq!(v.id, id);
	assert_eq!(v.t, "test_vertex".to_string());
}

pub fn should_not_get_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_update_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let id = trans.create_vertex("test_vertex".to_string()).unwrap();
	trans.set_vertex(models::Vertex::new(id, "test_vertex_2".to_string())).unwrap();
	let v = trans.get_vertex(id).unwrap();
	assert_eq!(v.id, id);
	assert_eq!(v.t, "test_vertex_2".to_string());
}

pub fn should_not_update_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.set_vertex(models::Vertex::new(I::default(), "movie".to_string()));
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_delete_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	trans.delete_vertex(outbound_id).unwrap();
	let result = trans.get_vertex(outbound_id);
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
	let count = trans.get_edge_count(outbound_id, "test_edge_type".to_string()).unwrap();
	assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.delete_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_not_delete_an_unowned_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let vertex_id = trans.create_vertex("test_vertex".to_string()).unwrap();
	trans.commit().unwrap();

	let email = sandbox.generate_unique_string("isolated");
	let (account_id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(account_id).unwrap();
	let result = trans.delete_vertex(vertex_id);
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_get_a_valid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let e = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 0.5);
	trans.set_edge(e).unwrap();

	let e = trans.get_edge(outbound_id, "test_edge_type".to_string(), inbound_id).unwrap();
	assert_eq!(e.outbound_id, outbound_id);
	assert_eq!(e.t, "test_edge_type".to_string());
	assert_eq!(e.inbound_id, inbound_id);
}

pub fn should_not_get_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let result = trans.get_edge(outbound_id, "test_edge_type".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
	let result = trans.get_edge(I::default(), "test_edge_type".to_string(), inbound_id);
	assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn should_update_a_valid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();

	// Edge should not exist yet
	let result = trans.get_edge(outbound_id, "test_edge_type".to_string(), inbound_id);
	assert_eq!(result.unwrap_err(), Error::EdgeNotFound);

	// Set the edge and check
	let e1 = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 0.5);
	trans.set_edge(e1.clone()).unwrap();
	let e = trans.get_edge(outbound_id, "test_edge_type".to_string(), inbound_id).unwrap();
	assert_eq!(e1, e);

	// Update the edge and check
	let e2 = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, -0.5);
	trans.set_edge(e2.clone()).unwrap();
	let e = trans.get_edge(outbound_id, "test_edge_type".to_string(), inbound_id).unwrap();
	assert_eq!(e2, e);
}

pub fn should_not_update_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let result = trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), I::default(), 0.5));
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
	let result = trans.set_edge(models::Edge::new(I::default(), "test_edge_type".to_string(), inbound_id, 0.5));
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_not_set_an_edge_with_a_bad_weight<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let result = trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 1.01));
	assert_eq!(result.unwrap_err(), Error::OutOfRange("weight".to_string()));
	let result = trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, -1.01));
	assert_eq!(result.unwrap_err(), Error::OutOfRange("weight".to_string()));
}

pub fn should_not_set_an_edge_with_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	trans.commit().unwrap();

	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).unwrap();
	let result = trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 0.5));
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_delete_a_valid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let e = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 0.5);
	trans.set_edge(e).unwrap();
	trans.get_edge(outbound_id, "test_edge_type".to_string(), inbound_id).unwrap();
	trans.delete_edge(outbound_id, "test_edge_type".to_string(), inbound_id).unwrap();
	let result = trans.get_edge(outbound_id, "test_edge_type".to_string(), inbound_id);
	assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn should_not_delete_an_invalid_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let result = trans.delete_edge(outbound_id, "test_edge_type".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn should_not_delete_an_edge_with_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let e = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 0.5);
	trans.set_edge(e).unwrap();
	trans.commit().unwrap();

	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).unwrap();
	let result = trans.delete_edge(outbound_id, "test_edge_type".to_string(), inbound_id);
	assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn should_get_an_edge_count<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
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
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_range(outbound_id, "test_edge_type".to_string(), 5, 0).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_empty_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_range(outbound_id, "test_edge_type".to_string(), 5, 5).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_an_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
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
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "foo".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_get_edges_by_a_time_range_with_no_high<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), Option::None, get_before(), 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_by_a_time_range_with_no_low<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), get_after(), Option::None, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_edges_by_a_time_range_with_no_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), Option::None, Option::None, 10).unwrap();
	check_edge_range(range, outbound_id, 5);
}

pub fn should_get_no_edges_for_a_reversed_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let outbound_id = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), get_before(), get_after(), 10).unwrap();
	check_edge_range(range, outbound_id, 0);
}

pub fn should_handle_global_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let key = sandbox.generate_unique_string("global-metadata");
	let trans = sandbox.transaction();

	// Check to make sure there's no initial value
	let result = trans.get_global_metadata(key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

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
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_account_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let key = sandbox.generate_unique_string("account-metadata");
	let trans = sandbox.transaction();

	// Check to make sure there's no initial value
	let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

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
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_vertex_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let owner_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let key = sandbox.generate_unique_string("vertex-metadata");

	// Check to make sure there's no initial value
	let result = trans.get_vertex_metadata(owner_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

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
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_edge_metadata<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
	let e = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 0.5);
	trans.set_edge(e).unwrap();

	let key = sandbox.generate_unique_string("edge-metadata");

	// Check to make sure there's no initial value
	let result = trans.get_edge_metadata(outbound_id, "test_edge_type".to_string(), inbound_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

	// Set and get the value as true
	let result = trans.set_edge_metadata(outbound_id, "test_edge_type".to_string(), inbound_id, key.clone(), JsonValue::Bool(true));
	assert!(result.is_ok());

	let result = trans.get_edge_metadata(outbound_id, "test_edge_type".to_string(), inbound_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(true));

	// Set and get the value as false
	let result = trans.set_edge_metadata(outbound_id, "test_edge_type".to_string(), inbound_id, key.clone(), JsonValue::Bool(false));
	assert!(result.is_ok());

	let result = trans.get_edge_metadata(outbound_id, "test_edge_type".to_string(), inbound_id, key.clone());
	assert_eq!(result.unwrap(), JsonValue::Bool(false));

	// Delete & check that it's deleted
	let result = trans.delete_edge_metadata(outbound_id, "test_edge_type".to_string(), inbound_id, key.clone());
	assert!(result.is_ok());

	let result = trans.get_edge_metadata(outbound_id, "test_edge_type".to_string(), inbound_id, key.clone());
	assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}