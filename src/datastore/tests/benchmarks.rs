use datastore::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use super::util::*;
use models;
use traits::Id;
use test::Bencher;

pub fn bench_get_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();
    let t = models::Type::new("test_name".to_string()).unwrap();
    let id = trans.create_vertex(t).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.get_vertex(id).unwrap();
    });
}

pub fn bench_create_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    b.iter(|| {
        let trans = sandbox.transaction();
        let t = models::Type::new("user".to_string()).unwrap();
    	trans.create_vertex(t).unwrap();
    });
}

pub fn bench_set_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();
    let t = models::Type::new("test_name".to_string()).unwrap();
    let id = trans.create_vertex(t).unwrap();
    trans.commit().unwrap();
    
    b.iter(|| {
        let trans = sandbox.transaction();
        let t = models::Type::new("test_vertex".to_string()).unwrap();
        let v = models::Vertex::new(id, t);
        trans.set_vertex(v).unwrap();
    });
}

pub fn bench_get_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();

    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let e = models::Edge::new(outbound_id, edge_t, inbound_id, models::Weight::new(0.5).unwrap());
	trans.set_edge(e).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        let t = models::Type::new("test_edge_type".to_string()).unwrap();
        trans.get_edge(outbound_id, t, inbound_id).unwrap();
    });
}

pub fn bench_set_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        let t = models::Type::new("test_edge_type".to_string()).unwrap();
        let e = models::Edge::new(outbound_id, t, inbound_id, models::Weight::new(1.0).unwrap());
        trans.set_edge(e).unwrap();
    });
}

pub fn bench_get_edge_count<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let (outbound_id, _) = create_edges(&mut sandbox);

    b.iter(|| {
        let trans = sandbox.transaction();
        let t = models::Type::new("test_edge_type".to_string()).unwrap();
    	trans.get_edge_count(outbound_id, t).unwrap();
    });
}

pub fn bench_get_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let (outbound_id, _) = create_edges(&mut sandbox);

    b.iter(|| {
        let trans = sandbox.transaction();
        let t = models::Type::new("test_edge_type".to_string()).unwrap();
        trans.get_edge_range(outbound_id, t, 0, 5).unwrap();
    });
}

pub fn bench_get_edge_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(&mut sandbox);

    b.iter(|| {
        let trans = sandbox.transaction();
        let t = models::Type::new("test_edge_type".to_string()).unwrap();
    	trans.get_edge_time_range(outbound_id, t, Some(end_time), Some(start_time), 10).unwrap();
    });
}
