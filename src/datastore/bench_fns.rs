use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use models;
use traits::Id;
use test::Bencher;

pub fn bench_get_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();
    let id = trans.create_vertex("test_name".to_string()).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.get_vertex(id).unwrap();
    });
}

pub fn bench_create_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    b.iter(|| {
        let trans = sandbox.transaction();
    	trans.create_vertex("user".to_string()).unwrap();
    });
}

pub fn bench_set_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();
    let id = trans.create_vertex("test_name".to_string()).unwrap();
    trans.commit().unwrap();
    
    b.iter(|| {
        let trans = sandbox.transaction();
        let v = models::Vertex::new(id, "test_vertex".to_string());
        trans.set_vertex(v).unwrap();
    });
}

pub fn bench_get_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();
    let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let e = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 0.5);
	trans.set_edge(e).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.get_edge(outbound_id, "test_edge_type".to_string(), inbound_id).unwrap();
    });
}

pub fn bench_set_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let trans = sandbox.transaction();
    let outbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex("test_vertex_type".to_string()).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        let e = models::Edge::new(outbound_id, "test_edge_type".to_string(), inbound_id, 1.0);
        trans.set_edge(e).unwrap();
    });
}

pub fn bench_get_edge_count<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let (outbound_id, _) = create_edges(&mut sandbox);

    b.iter(|| {
        let trans = sandbox.transaction();
    	trans.get_edge_count(outbound_id, "test_edge_type".to_string()).unwrap();
    });
}

pub fn bench_get_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let (outbound_id, _) = create_edges(&mut sandbox);

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.get_edge_range(outbound_id, "test_edge_type".to_string(), 0, 5).unwrap();
    });
}

pub fn bench_get_edge_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    let (outbound_id, _) = create_edges(&mut sandbox);
    let after = get_after();
    let before = get_before();

    b.iter(|| {
        let trans = sandbox.transaction();
    	trans.get_edge_time_range(outbound_id, "test_edge_type".to_string(), after.clone(), before.clone(), 10).unwrap();
    });
}
