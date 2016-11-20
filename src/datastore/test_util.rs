use core::ops::{Add, Sub};
use chrono::duration::Duration;
use chrono::UTC;
use chrono::naive::datetime::NaiveDateTime;
use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use traits::Id;
use models;

pub fn get_before() -> Option<NaiveDateTime> {
	let time = UTC::now().sub(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}

pub fn get_after() -> Option<NaiveDateTime> {
	let time = UTC::now().add(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}

pub fn create_edges<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) -> (I, [I; 5]) {
    let trans = sandbox.transaction();
    let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let inbound_id_1 = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let inbound_id_2 = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let inbound_id_3 = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let inbound_id_4 = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let inbound_id_5 = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id_1, 1.0)).unwrap();
    trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id_2, 1.0)).unwrap();
    trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id_3, 1.0)).unwrap();
    trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id_4, 1.0)).unwrap();
    trans.set_edge(models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id_5, 1.0)).unwrap();
    trans.commit().unwrap();
    (outbound_id, [inbound_id_1, inbound_id_2, inbound_id_3, inbound_id_4, inbound_id_5])
}

