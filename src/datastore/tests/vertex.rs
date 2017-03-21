use datastore::{Datastore, Transaction, VertexQuery};
use super::sandbox::DatastoreTestSandbox;
use super::util::*;
use uuid::Uuid;
use errors::Error;
use models;
use std::u32;

pub fn update_a_valid_vertex<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let original_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let id = trans.create_vertex(original_t).unwrap();
    let updated_t = models::Type::new("test_vertex_type_2".to_string()).unwrap();
    trans.set_vertex(models::Vertex::new(id, updated_t.clone())).unwrap();
    let v = trans.get_vertices(VertexQuery::Vertex(id)).unwrap();
    assert_eq!(v[0].id, id);
    assert_eq!(v[0].t, updated_t);
}

pub fn not_update_an_invalid_vertex<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let result = trans.set_vertex(models::Vertex::new(Uuid::default(), t));
    assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn delete_a_valid_vertex<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let (outbound_id, _) = create_edges(&mut sandbox);
    let trans = sandbox.transaction();
    trans.delete_vertex(outbound_id).unwrap();
    let v = trans.get_vertices(VertexQuery::Vertex(outbound_id)).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans.get_edge_count(outbound_id, Some(t)).unwrap();
    assert_eq!(count, 0);
}

pub fn not_delete_an_invalid_vertex<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction,
{
    let trans = sandbox.transaction();
    let result = trans.delete_vertex(Uuid::default());
    assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn not_delete_an_unowned_vertex<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let vertex_id = trans.create_vertex(t).unwrap();
    trans.commit().unwrap();

    let email = sandbox.generate_unique_string("isolated");
    let (account_id, _) = sandbox.register_account(&email[..]);
    let trans = sandbox.datastore.transaction(account_id).unwrap();
    let result = trans.delete_vertex(vertex_id);
    assert_eq!(result.unwrap_err(), Error::Unauthorized);
}
