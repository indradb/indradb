use super::super::{Datastore, EdgeKey, EdgeQuery, Transaction, Type, VertexQuery};
use super::sandbox::DatastoreTestSandbox;
use errors::Error;
use uuid::Uuid;
use serde_json::Value as JsonValue;

pub fn should_handle_global_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let name = sandbox.generate_unique_string("global-metadata");
    let trans = sandbox.transaction();

    // Check to make sure there's no initial value
    let result = trans.get_global_metadata(name.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    // Set and get the value as true
    trans
        .set_global_metadata(name.clone(), JsonValue::Bool(true))
        .unwrap();

    let result = trans.get_global_metadata(name.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(true));

    // Set and get the value as false
    trans
        .set_global_metadata(name.clone(), JsonValue::Bool(false))
        .unwrap();

    let result = trans.get_global_metadata(name.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_global_metadata(name.clone()).unwrap();

    let result = trans.get_global_metadata(name.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_account_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let name = sandbox.generate_unique_string("account-metadata");
    let trans = sandbox.transaction();

    // Check to make sure there's no initial value
    let result = trans.get_account_metadata(sandbox.owner_id, name.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    // Set and get the value as true
    trans
        .set_account_metadata(sandbox.owner_id, name.clone(), JsonValue::Bool(true))
        .unwrap();

    let result = trans.get_account_metadata(sandbox.owner_id, name.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(true));

    // Set and get the value as false
    trans
        .set_account_metadata(sandbox.owner_id, name.clone(), JsonValue::Bool(false))
        .unwrap();

    let result = trans.get_account_metadata(sandbox.owner_id, name.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans
        .delete_account_metadata(sandbox.owner_id, name.clone())
        .unwrap();

    let result = trans.get_account_metadata(sandbox.owner_id, name.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_not_set_invalid_account_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let result = trans.set_account_metadata(Uuid::default(), "foo".to_string(), JsonValue::Null);
    assert_eq!(result.unwrap_err(), Error::AccountNotFound);
}

pub fn should_not_delete_invalid_account_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let result = trans.delete_account_metadata(Uuid::default(), "foo".to_string());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
    let result = trans.delete_account_metadata(sandbox.owner_id, "foo".to_string());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_vertex_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let t = Type::new("test_edge_type".to_string()).unwrap();
    let owner_id = trans.create_vertex(t).unwrap();
    let name = sandbox.generate_unique_string("vertex-metadata");
    let q = VertexQuery::Vertices {
        ids: vec![owner_id],
    };

    // Check to make sure there's no initial value
    let result = trans.get_vertex_metadata(q.clone(), name.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    trans
        .set_vertex_metadata(q.clone(), name.clone(), JsonValue::Bool(true))
        .unwrap();
    let result = trans.get_vertex_metadata(q.clone(), name.clone()).unwrap();
    assert_eq!(result[&owner_id], JsonValue::Bool(true));

    // Set and get the value as false
    trans
        .set_vertex_metadata(q.clone(), name.clone(), JsonValue::Bool(false))
        .unwrap();
    let result = trans.get_vertex_metadata(q.clone(), name.clone()).unwrap();
    assert_eq!(result[&owner_id], JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans
        .delete_vertex_metadata(q.clone(), name.clone())
        .unwrap();
    let result = trans.get_vertex_metadata(q, name.clone()).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_set_invalid_vertex_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let q = VertexQuery::Vertices {
        ids: vec![Uuid::default()],
    };
    trans
        .set_vertex_metadata(q.clone(), "foo".to_string(), JsonValue::Null)
        .unwrap();
    let result = trans.get_vertex_metadata(q, "foo".to_string()).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_vertex_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let q = VertexQuery::Vertices {
        ids: vec![Uuid::default()],
    };
    trans.delete_vertex_metadata(q, "foo".to_string()).unwrap();

    let vertex_id = trans
        .create_vertex(Type::new("foo".to_string()).unwrap())
        .unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![vertex_id],
    };
    trans.delete_vertex_metadata(q, "foo".to_string()).unwrap();
}

pub fn should_handle_edge_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let vertex_t = Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = Type::new("test_edge_type".to_string()).unwrap();
    let key = EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    let q = EdgeQuery::Edges {
        keys: vec![key.clone()],
    };
    let name = sandbox.generate_unique_string("edge-metadata");

    trans.create_edge(key.clone()).unwrap();

    // Check to make sure there's no initial value
    let result = trans.get_edge_metadata(q.clone(), name.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    trans
        .set_edge_metadata(q.clone(), name.clone(), JsonValue::Bool(true))
        .unwrap();
    let result = trans.get_edge_metadata(q.clone(), name.clone()).unwrap();
    assert_eq!(result[&key], JsonValue::Bool(true));

    // Set and get the value as false
    trans
        .set_edge_metadata(q.clone(), name.clone(), JsonValue::Bool(false))
        .unwrap();
    let result = trans.get_edge_metadata(q.clone(), name.clone()).unwrap();
    assert_eq!(result[&key], JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_edge_metadata(q.clone(), name.clone()).unwrap();
    let result = trans.get_edge_metadata(q, name.clone()).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_set_invalid_edge_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let q = EdgeQuery::Edges {
        keys: vec![
            EdgeKey::new(
                Uuid::default(),
                Type::new("foo".to_string()).unwrap(),
                Uuid::default(),
            ),
        ],
    };
    trans
        .set_edge_metadata(q.clone(), "bar".to_string(), JsonValue::Null)
        .unwrap();
    let result = trans.get_edge_metadata(q, "bar".to_string()).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_edge_metadata<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let q = EdgeQuery::Edges {
        keys: vec![
            EdgeKey::new(
                Uuid::default(),
                Type::new("foo".to_string()).unwrap(),
                Uuid::default(),
            ),
        ],
    };
    trans.delete_edge_metadata(q, "bar".to_string()).unwrap();

    let outbound_id = trans
        .create_vertex(Type::new("foo".to_string()).unwrap())
        .unwrap();
    let inbound_id = trans
        .create_vertex(Type::new("bar".to_string()).unwrap())
        .unwrap();
    let key = EdgeKey::new(
        outbound_id,
        Type::new("baz".to_string()).unwrap(),
        inbound_id,
    );
    trans.create_edge(key.clone()).unwrap();
    trans
        .delete_edge_metadata(EdgeQuery::Edges { keys: vec![key] }, "bleh".to_string())
        .unwrap();
}
