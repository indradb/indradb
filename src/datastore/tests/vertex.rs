use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use super::util::*;
use errors::Error;
use models;
use traits::Id;

pub fn should_get_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let t = models::Type::new("test_vertex_type".to_string()).unwrap();
	let id = trans.create_vertex(t.clone()).unwrap();
	let v = trans.get_vertex(id).unwrap();
	assert_eq!(v.id, id);
	assert_eq!(v.t, t);
}

pub fn should_not_get_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.get_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_update_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let original_t = models::Type::new("test_vertex_type".to_string()).unwrap();
	let id = trans.create_vertex(original_t).unwrap();
	let updated_t = models::Type::new("test_vertex_type_2".to_string()).unwrap();
	trans.set_vertex(models::Vertex::new(id, updated_t.clone())).unwrap();
	let v = trans.get_vertex(id).unwrap();
	assert_eq!(v.id, id);
	assert_eq!(v.t, updated_t);
}

pub fn should_not_update_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let t = models::Type::new("test_vertex_type".to_string()).unwrap();
	let result = trans.set_vertex(models::Vertex::new(I::default(), t));
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_delete_a_valid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (outbound_id, _) = create_edges(&mut sandbox);
	let trans = sandbox.transaction();
	trans.delete_vertex(outbound_id).unwrap();
	let result = trans.get_vertex(outbound_id);
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
	let t = models::Type::new("test_edge_type".to_string()).unwrap();
	let count = trans.get_edge_count(outbound_id, t).unwrap();
	assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let result = trans.delete_vertex(I::default());
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_not_delete_an_unowned_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let trans = sandbox.transaction();
	let t = models::Type::new("test_vertex_type".to_string()).unwrap();
	let vertex_id = trans.create_vertex(t).unwrap();
	trans.commit().unwrap();

	let email = sandbox.generate_unique_string("isolated");
	let (account_id, _) = sandbox.register_account(&email[..]);
	let trans = sandbox.datastore.transaction(account_id).unwrap();
	let result = trans.delete_vertex(vertex_id);
	assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}
