use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use errors::Error;
use models;
use uuid::Uuid;
use chrono::UTC;
use chrono::Timelike;

pub fn get_a_valid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();

    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();

    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let e = models::Edge::new_with_current_datetime(
        outbound_id,
        edge_t.clone(),
        inbound_id,
        models::Weight::new(0.5).unwrap()
    );

    // Record the start and end time. Round off the the nanoseconds off the
    // start time, since some implementations may not have that level of
    // accuracy.
    let start_time = UTC::now().with_nanosecond(0).unwrap();
    trans.set_edge(e).unwrap();
    let end_time = UTC::now();

    let e = trans.get_edge(outbound_id, edge_t.clone(), inbound_id).unwrap();
    assert_eq!(e.outbound_id, outbound_id);
    assert_eq!(e.t, edge_t);
    assert_eq!(e.inbound_id, inbound_id);
    assert!(e.update_datetime >= start_time);
    assert!(e.update_datetime <= end_time);
}

pub fn not_get_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();

    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();

    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let result = trans.get_edge(outbound_id, edge_t.clone(), Uuid::default());
    assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
    let result = trans.get_edge(Uuid::default(), edge_t, inbound_id);
    assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn update_a_valid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let trans = sandbox.transaction();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t.clone()).unwrap();

    // Edge should not exist yet
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let result = trans.get_edge(outbound_id, edge_t.clone(), inbound_id);
    assert_eq!(result.unwrap_err(), Error::EdgeNotFound);

    // Set the edge and check
    let e1 = models::Edge::new_with_current_datetime(
        outbound_id,
        edge_t.clone(),
        inbound_id,
        models::Weight::new(0.5).unwrap()
    );
    trans.set_edge(e1.clone()).unwrap();
    let e = trans.get_edge(outbound_id, edge_t.clone(), inbound_id).unwrap();
    assert_eq!(e1, e);

    // Update the edge and check
    let e2 = models::Edge::new_with_current_datetime(
        outbound_id,
        edge_t.clone(),
        inbound_id,
        models::Weight::new(-0.5).unwrap()
    );
    trans.set_edge(e2.clone()).unwrap();
    let e = trans.get_edge(outbound_id, edge_t, inbound_id).unwrap();
    assert_eq!(e2, e);
}

pub fn not_update_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let result = trans.set_edge(models::Edge::new_with_current_datetime(
        outbound_id,
        edge_t.clone(),
        Uuid::default(),
        models::Weight::new(0.5).unwrap()
    ));
    assert_eq!(result.unwrap_err(), Error::VertexNotFound);
    let result = trans.set_edge(models::Edge::new_with_current_datetime(
        Uuid::default(),
        edge_t,
        inbound_id,
        models::Weight::new(0.5).unwrap()
    ));
    assert_eq!(result.unwrap_err(), Error::VertexNotFound);
}

pub fn not_set_an_edge_with_bad_permissions<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    trans.commit().unwrap();

    let email = sandbox.generate_unique_string("isolated");
    let (id, _) = sandbox.register_account(&email[..]);
    let trans = sandbox.datastore.transaction(id).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let result = trans.set_edge(models::Edge::new_with_current_datetime(
        outbound_id,
        edge_t,
        inbound_id,
        models::Weight::new(0.5).unwrap()
    ));
    assert_eq!(result.unwrap_err(), Error::Unauthorized);
}

pub fn delete_a_valid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
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
    trans.get_edge(outbound_id, edge_t.clone(), inbound_id).unwrap();
    trans.delete_edge(outbound_id, edge_t.clone(), inbound_id).unwrap();
    let result = trans.get_edge(outbound_id, edge_t, inbound_id);
    assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn not_delete_an_invalid_edge<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let result = trans.delete_edge(outbound_id, edge_t, Uuid::default());
    assert_eq!(result.unwrap_err(), Error::EdgeNotFound);
}

pub fn not_delete_an_edge_with_bad_permissions<D, T>(mut sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
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
    trans.commit().unwrap();

    let email = sandbox.generate_unique_string("isolated");
    let (id, _) = sandbox.register_account(&email[..]);
    let trans = sandbox.datastore.transaction(id).unwrap();
    let result = trans.delete_edge(outbound_id, edge_t, inbound_id);
    assert_eq!(result.unwrap_err(), Error::Unauthorized);
}
