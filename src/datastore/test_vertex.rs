use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use errors::Error;
use models;
use traits::Id;
use serde_json::Value as JsonValue;

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
	let (outbound_id, _) = create_edges(&mut sandbox);
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
