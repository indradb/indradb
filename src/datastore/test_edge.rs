use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use errors::Error;
use models;
use traits::Id;
use serde_json::Value as JsonValue;

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
