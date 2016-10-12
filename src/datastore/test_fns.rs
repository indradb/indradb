use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use util::Error;
use models;
use traits::Id;
use std::collections::HashSet;
use serde_json::Value as JsonValue;

pub fn auth_bad_username<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(I::default(), "foobar".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn auth_bad_password<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id, "bad_token".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn auth_good<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id, sandbox.owner_secret.clone());
	assert!(auth.is_ok());
	assert!(auth.unwrap());
}

pub fn has_account_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(sandbox.owner_id);
	assert!(results.is_ok());
	assert!(results.unwrap());
}

pub fn has_account_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(I::default());
	assert!(results.is_ok());
	assert!(!results.unwrap());
}

pub fn delete_account_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let result = sandbox.datastore.delete_account(I::default());
	assert_eq!(result.unwrap_err(), Error::AccountNotFound);
}

pub fn get_vertex_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let v = trans.get_vertex(sandbox.jill_id()).unwrap();
	assert_eq!(v.id, sandbox.jill_id());
	assert_eq!(v.t, "user".to_string());
	let expected_properties = create_test_properties("Jill");
	assert_eq!(v.properties, expected_properties);
}

pub fn get_vertex_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn create_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let props = create_test_properties("Jill 2.0");
	let trans = sandbox.transaction();
	trans.create_vertex("user".to_string(), props).unwrap();
}

pub fn set_vertex_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn set_vertex_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.set_vertex(models::Vertex::new(I::default(), "movie".to_string()));
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn delete_vertex_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	// First create a vertex
	let id = sandbox.create_test_vertex("movie", None);

	// Create some edges, then delete the vertex and make sure the edges were cleared
	let trans = sandbox.transaction();
	trans.set_edge(models::Edge::new(id, "follows".to_string(), sandbox.jill_id(), 1.0)).unwrap();
	trans.set_edge(models::Edge::new(id, "review".to_string(), sandbox.memento_id(), 1.0)).unwrap();
	trans.set_edge(models::Edge::new(sandbox.christopher_id(), "follows".to_string(), id, 1.0)).unwrap();
	trans.delete_vertex(id).unwrap();
	let result = trans.get_vertex(id);
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
	let count = trans.get_edge_count(id, "follows".to_string()).unwrap();
	assert_eq!(count, 0);
	let count = trans.get_edge_count(id, "review".to_string()).unwrap();
	assert_eq!(count, 0);
	let count = trans.get_edge_count(sandbox.christopher_id(), "follows".to_string()).unwrap();
	assert_eq!(count, 0);
}

pub fn delete_vertex_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.delete_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn delete_vertex_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).unwrap();
	let result = trans.delete_vertex(sandbox.jill_id());
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn get_edge_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let e = trans.get_edge(sandbox.jill_id(), "review".to_string(), sandbox.inception_id()).unwrap();
	assert_eq!(e.outbound_id, sandbox.jill_id());
	assert_eq!(e.t, "review".to_string());
	assert_eq!(e.inbound_id, sandbox.inception_id());
}

pub fn get_edge_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_edge(sandbox.jill_id(), "review".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn set_edge_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	// This also tests adding a new type that didn't previously exist
	let e1 = models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id(), 0.5);
	let e2 = models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id(), -0.5);

	let trans = sandbox.transaction();
	let result = trans.get_edge(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id());
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
	trans.set_edge(e1.clone()).unwrap();
	let e = trans.get_edge(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id()).unwrap();
	assert_eq!(e1, e);
	trans.set_edge(e2.clone()).unwrap();
	let e = trans.get_edge(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id()).unwrap();
	assert_eq!(e2, e);
}

pub fn set_edge_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.set_edge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), I::default(), 0.5));
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn set_edge_bad_weight<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.set_edge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.bob_id(), 1.01));
	assert_eq!(result.unwrap_err(), Error::WeightOutOfRange);
	let result = trans.set_edge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.bob_id(), -1.01));
	assert_eq!(result.unwrap_err(), Error::WeightOutOfRange);
}

pub fn set_edge_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	let result = trans.set_edge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id(), 0.5));
	assert_eq!(result.unwrap_err(), Error::VertexDoesNotExist);
}

pub fn delete_edge_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let bob_id = sandbox.bob_id();
	let christopher_id = sandbox.christopher_id();

	let trans = sandbox.transaction();
	let e = models::Edge::new(bob_id, "blocks".to_string(), christopher_id, 0.5);
	trans.set_edge(e).unwrap();
	trans.get_edge(bob_id, "blocks".to_string(), christopher_id).unwrap();
	trans.delete_edge(bob_id, "blocks".to_string(), christopher_id).unwrap();
	let result = trans.get_edge(bob_id, "blocks".to_string(), christopher_id);
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn delete_edge_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.delete_edge(sandbox.jill_id(), "blocks".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn delete_edge_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let email = sandbox.generate_unique_string("isolated");
	let (id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	let result = trans.delete_edge(sandbox.jill_id(), "blocks".to_string(), I::default());
	assert_eq!(result.unwrap_err(), Error::EdgeDoesNotExist);
}

pub fn get_edge_count_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let count = trans.get_edge_count(sandbox.christopher_id(), "purchased".to_string()).unwrap();
	assert_eq!(count, 10);
}

pub fn get_edge_count_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let count = trans.get_edge_count(I::default(), "purchased".to_string()).unwrap();
	assert_eq!(count, 0);
}

pub fn get_edge_range_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let check_range = |range: Vec<models::Edge<I>>, count: usize| {
		assert_eq!(range.len(), count);
		let mut covered_ids: HashSet<I> = HashSet::new();

		for edge in range.iter() {
			assert_eq!(edge.outbound_id, sandbox.christopher_id());
			assert_eq!(edge.t, "purchased".to_string());
			assert_eq!(edge.weight, 1.0);
			assert_eq!(edge.properties.len(), 0);
			assert!(!covered_ids.contains(&edge.inbound_id));
			covered_ids.insert(edge.inbound_id);
		}
	};

	let trans = sandbox.transaction();
	let range = trans.get_edge_range(sandbox.christopher_id(), "purchased".to_string(), 0, 5).unwrap();
	check_range(range, 5);
	let range = trans.get_edge_range(sandbox.christopher_id(), "purchased".to_string(), 5, 0).unwrap();
	check_range(range, 0);
	let range = trans.get_edge_range(sandbox.christopher_id(), "purchased".to_string(), 5, 5).unwrap();
	check_range(range, 5);
}

pub fn get_edge_range_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let range = trans.get_edge_range(sandbox.christopher_id(), "foo".to_string(), 0, 10).unwrap();
	assert_eq!(range.len(), 0);
}

pub fn get_edge_range_bad_offset<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_edge_range(sandbox.christopher_id(), "foo".to_string(), -1, 10);
	assert_eq!(result.unwrap_err(), Error::OffsetOutOfRange);
}

pub fn get_edge_range_bad_limit<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_edge_range(sandbox.christopher_id(), "foo".to_string(), 0, -1);
	assert_eq!(result.unwrap_err(), Error::LimitOutOfRange);
}

pub fn get_edge_time_range_full<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.christopher_id(), "review".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn get_edge_time_range_empty<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.christopher_id(), "foo".to_string(), get_after(), get_before(), 10).unwrap();
	check_edge_time_range(sandbox, range, 0);
}

pub fn get_edge_time_range_no_high<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.christopher_id(), "review".to_string(), Option::None, get_before(), 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn get_edge_time_range_no_low<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.christopher_id(), "review".to_string(), get_after(), Option::None, 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn get_edge_time_range_no_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.christopher_id(), "review".to_string(), Option::None, Option::None, 10).unwrap();
	check_edge_time_range(sandbox, range, 6);
}

pub fn get_edge_time_range_reversed_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let range = trans.get_edge_time_range(sandbox.christopher_id(), "review".to_string(), get_after(), get_after(), 10).unwrap();
	check_edge_time_range(sandbox, range, 0);
}

fn check_edge_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>, range: Vec<models::Edge<I>>, expected_length: usize) {
	assert_eq!(range.len(), expected_length);
	let mut covered_ids: HashSet<I> = HashSet::new();

	for edge in range.iter() {
		assert_eq!(edge.outbound_id, sandbox.christopher_id());
		assert_eq!(edge.t, "review".to_string());
		assert_eq!(edge.weight, 1.0);
		assert_eq!(edge.properties.len(), 0);
		assert!(!covered_ids.contains(&edge.inbound_id));
		covered_ids.insert(edge.inbound_id);
	}
}

pub fn local_get_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let metadata = trans.get_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local")).unwrap();
	assert_eq!(metadata, JsonValue::Bool(true));
}

pub fn local_get_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_metadata(Some(sandbox.owner_id), "".to_string());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn local_set_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	trans.set_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local"), JsonValue::String("test".to_string())).unwrap();
	let metadata = trans.get_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local")).unwrap();
	assert_eq!(metadata, JsonValue::String("test".to_string()));
}

pub fn local_set_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	trans.set_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local-2"), JsonValue::String("test".to_string())).unwrap();
	let metadata = trans.get_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local-2")).unwrap();
	assert_eq!(metadata, JsonValue::String("test".to_string()));
}

pub fn local_delete_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	trans.delete_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local")).unwrap();
	let result = trans.get_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local"));
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn local_delete_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.delete_metadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local-3"));
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn global_get_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let metadata = trans.get_metadata(None, sandbox.generate_unique_string("global")).unwrap();
	assert_eq!(metadata, JsonValue::Bool(true));
}

pub fn global_get_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_metadata(None, "".to_string());
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn global_set_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	trans.set_metadata(None, sandbox.generate_unique_string("global"), JsonValue::String("test".to_string())).unwrap();
	let metadata = trans.get_metadata(None, sandbox.generate_unique_string("global")).unwrap();
	assert_eq!(metadata, JsonValue::String("test".to_string()));
}

pub fn global_set_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	trans.set_metadata(None, sandbox.generate_unique_string("global-2"), JsonValue::String("test".to_string())).unwrap();
	let metadata = trans.get_metadata(None, sandbox.generate_unique_string("global-2")).unwrap();
	assert_eq!(metadata, JsonValue::String("test".to_string()));
}

pub fn global_delete_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	trans.delete_metadata(None, sandbox.generate_unique_string("global")).unwrap();
	let result = trans.get_metadata(None, sandbox.generate_unique_string("global"));
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}

pub fn global_delete_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.delete_metadata(None, sandbox.generate_unique_string("global-3"));
	assert_eq!(result.unwrap_err(), Error::MetadataDoesNotExist);
}
