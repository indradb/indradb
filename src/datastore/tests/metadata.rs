use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use errors::Error;
use models;
use traits::Id;
use serde_json::Value as JsonValue;

pub fn should_handle_global_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let key = sandbox.generate_unique_string("global-metadata");
    let trans = sandbox.transaction();

    // Check to make sure there's no initial value
    let result = trans.get_global_metadata(key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    // Set and get the value as true
    trans.set_global_metadata(key.clone(), JsonValue::Bool(true)).unwrap();

    let result = trans.get_global_metadata(key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(true));

    // Set and get the value as false
    trans.set_global_metadata(key.clone(), JsonValue::Bool(false)).unwrap();

    let result = trans.get_global_metadata(key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_global_metadata(key.clone()).unwrap();

    let result = trans.get_global_metadata(key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_account_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let key = sandbox.generate_unique_string("account-metadata");
    let trans = sandbox.transaction();

    // Check to make sure there's no initial value
    let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    // Set and get the value as true
    trans.set_account_metadata(sandbox.owner_id, key.clone(), JsonValue::Bool(true)).unwrap();

    let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(true));

    // Set and get the value as false
    trans.set_account_metadata(sandbox.owner_id, key.clone(), JsonValue::Bool(false)).unwrap();

    let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_account_metadata(sandbox.owner_id, key.clone()).unwrap();

    let result = trans.get_account_metadata(sandbox.owner_id, key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_not_set_invalid_account_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let result = trans.set_account_metadata(I::default(), "foo".to_string(), JsonValue::Null);
    assert_eq!(result.unwrap_err(), Error::AccountNotFound);
}

pub fn should_not_delete_invalid_account_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let result = trans.delete_account_metadata(I::default(), "foo".to_string());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
    let result = trans.delete_account_metadata(sandbox.owner_id, "foo".to_string());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_vertex_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let owner_id = trans.create_vertex(t).unwrap();
    let key = sandbox.generate_unique_string("vertex-metadata");

    // Check to make sure there's no initial value
    let result = trans.get_vertex_metadata(owner_id, key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    // Set and get the value as true
    trans.set_vertex_metadata(owner_id, key.clone(), JsonValue::Bool(true)).unwrap();

    let result = trans.get_vertex_metadata(owner_id, key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(true));

    // Set and get the value as false
    trans.set_vertex_metadata(owner_id, key.clone(), JsonValue::Bool(false)).unwrap();

    let result = trans.get_vertex_metadata(owner_id, key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_vertex_metadata(owner_id, key.clone()).unwrap();

    let result = trans.get_vertex_metadata(owner_id, key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_not_set_invalid_vertex_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let result = trans.set_vertex_metadata(I::default(), "foo".to_string(), JsonValue::Null);
    assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn should_not_delete_invalid_vertex_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let result = trans.delete_vertex_metadata(I::default(), "foo".to_string());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    let vertex_id = trans.create_vertex(models::Type::new("foo".to_string()).unwrap()).unwrap();
    let result = trans.delete_vertex_metadata(vertex_id, "foo".to_string());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_handle_edge_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let e = models::Edge::new_with_current_datetime(
        outbound_id,
        edge_t.clone(),
        inbound_id,
        models::Weight::new(0.5).unwrap()
    );
    trans.set_edge(e).unwrap();

    let key = sandbox.generate_unique_string("edge-metadata");

    // Check to make sure there's no initial value
    let result = trans.get_edge_metadata(outbound_id, edge_t.clone(), inbound_id, key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    // Set and get the value as true
    trans.set_edge_metadata(outbound_id,
                            edge_t.clone(),
                            inbound_id,
                            key.clone(),
                            JsonValue::Bool(true)).unwrap();

    let result = trans.get_edge_metadata(outbound_id, edge_t.clone(), inbound_id, key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(true));

    // Set and get the value as false
    trans.set_edge_metadata(outbound_id,
                            edge_t.clone(),
                            inbound_id,
                            key.clone(),
                            JsonValue::Bool(false)).unwrap();

    let result = trans.get_edge_metadata(outbound_id, edge_t.clone(), inbound_id, key.clone());
    assert_eq!(result.unwrap(), JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_edge_metadata(outbound_id, edge_t.clone(), inbound_id, key.clone()).unwrap();

    let result = trans.get_edge_metadata(outbound_id, edge_t, inbound_id, key.clone());
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}

pub fn should_not_set_invalid_edge_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let result = trans.set_edge_metadata(
        I::default(),
        models::Type::new("foo".to_string()).unwrap(),
        I::default(),
        "bar".to_string(),
        JsonValue::Null
    );
    assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn should_not_delete_invalid_edge_metadata<D, T, I>(sandbox: &mut DatastoreTestSandbox<D, T, I>)
    where D: Datastore<T, I>,
          T: Transaction<I>,
          I: Id
{
    let trans = sandbox.transaction();
    let result = trans.delete_edge_metadata(
        I::default(),
        models::Type::new("foo".to_string()).unwrap(),
        I::default(),
        "bar".to_string()
    );
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);

    let outbound_id = trans.create_vertex(models::Type::new("foo".to_string()).unwrap()).unwrap();
    let inbound_id = trans.create_vertex(models::Type::new("bar".to_string()).unwrap()).unwrap();
    let edge = models::Edge::new_with_current_datetime(
        outbound_id,
        models::Type::new("baz".to_string()).unwrap(),
        inbound_id,
        models::Weight::new(1.0).unwrap()
    );
    trans.set_edge(edge).unwrap();
    
    let result = trans.delete_edge_metadata(
        outbound_id,
        models::Type::new("baz".to_string()).unwrap(),
        inbound_id,
        "bleh".to_string()
    );
    
    assert_eq!(result.unwrap_err(), Error::MetadataNotFound);
}
