use chrono::UTC;
use chrono::naive::datetime::NaiveDateTime;
use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use traits::Id;
use models;
use std::thread::sleep;
use std::time::Duration;

pub fn create_edge_from<T: Transaction<I>, I: Id>(trans: &T, outbound_id: I) -> I {
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(inbound_vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(1.0).unwrap();
    trans.set_edge(models::Edge::new(outbound_id, edge_t, inbound_id, weight)).unwrap();
    inbound_id
}

pub fn create_edges<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) -> (I, [I; 5]) {
    let trans = sandbox.transaction();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();
    let inbound_ids = [
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id)
    ];

    trans.commit().unwrap();
    
    (outbound_id, inbound_ids)
}

pub fn create_time_range_queryable_edges<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) -> (I, NaiveDateTime, NaiveDateTime, [I; 5]) {
    let trans = sandbox.transaction();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();

    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);

    sleep(Duration::new(2, 0));
    let start_time = NaiveDateTime::from_timestamp(UTC::now().timestamp() - 1, 0);
    let inbound_ids = [
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id)
    ];
    let end_time = NaiveDateTime::from_timestamp(UTC::now().timestamp() + 1, 0);
    sleep(Duration::new(2, 0));

    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);
    create_edge_from(&trans, outbound_id);

    trans.commit().unwrap();
    
    (outbound_id, start_time, end_time, inbound_ids)
}

pub fn create_edge_to<T: Transaction<I>, I: Id>(trans: &T, inbound_id: I) -> I {
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(1.0).unwrap();
    trans.set_edge(models::Edge::new(outbound_id, edge_t, inbound_id, weight)).unwrap();
    inbound_id
}

pub fn create_reversed_edges<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) -> (I, [I; 5]) {
    let trans = sandbox.transaction();
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(inbound_vertex_t).unwrap();
    let outbound_ids = [
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id)
    ];

    trans.commit().unwrap();
    
    (inbound_id, outbound_ids)
}

pub fn create_time_range_queryable_reversed_edges<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) -> (I, NaiveDateTime, NaiveDateTime, [I; 5]) {
    let trans = sandbox.transaction();
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(inbound_vertex_t).unwrap();

    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);

    sleep(Duration::new(2, 0));
    let start_time = UTC::now().naive_utc();
    let outbound_ids = [
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id),
        create_edge_to(&trans, inbound_id)
    ];

    // We add some padding to the end time because some implementations only
    // have second-level accuracy, in which case we'd get errors
    let end_time = NaiveDateTime::from_timestamp(UTC::now().timestamp() + 1, 0);
    sleep(Duration::new(2, 0));

    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);
    create_edge_to(&trans, inbound_id);

    trans.commit().unwrap();
     
    (inbound_id, start_time, end_time, outbound_ids)
}
