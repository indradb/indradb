use super::{Datastore, Transaction};
use super::test_sandbox::{DatastoreTestSandbox, insert_sample_data};
use super::test_util::*;
use models;
use traits::Id;
use test::Bencher;

pub fn bench_get_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    insert_sample_data(&mut sandbox);
    let jill_id = sandbox.search_id("user", "Jill");

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.get_vertex(jill_id).unwrap();
    });
}

pub fn bench_create_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    b.iter(|| {
        let trans = sandbox.transaction();
    	trans.create_vertex("user".to_string()).unwrap();
    });
}

pub fn bench_set_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let id = sandbox.create_test_vertex("movie", "Some New Movie");
    let v = models::Vertex::new(id, "movie".to_string());

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.set_vertex(v.clone()).unwrap();
    });
}

pub fn bench_get_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    insert_sample_data(&mut sandbox);
    let jill_id = sandbox.search_id("user", "Jill");
	let inception_id = sandbox.search_id("movie", "Inception");

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.get_edge(jill_id, "review".to_string(), inception_id).unwrap();
    });
}

pub fn bench_set_edge<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    insert_sample_data(&mut sandbox);
	let christopher_id = sandbox.search_id("user", "Christopher");
    let inception_id = sandbox.search_id("movie", "Inception");
    let e = models::Edge::new(christopher_id, "purchased".to_string(), inception_id, 1.0);

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.set_edge(e.clone()).unwrap();
    });
}

pub fn bench_get_edge_count<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    insert_sample_data(&mut sandbox);

    b.iter(|| {
        let trans = sandbox.transaction();
    	trans.get_edge_count(sandbox.search_id("user", "Christopher"), "purchased".to_string()).unwrap();
    });
}

pub fn bench_get_edge_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    insert_sample_data(&mut sandbox);
	let christopher_id = sandbox.search_id("user", "Christopher");

    b.iter(|| {
        let trans = sandbox.transaction();
        trans.get_edge_range(christopher_id, "purchased".to_string(), 0, 5).unwrap();
    });
}

pub fn bench_get_edge_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(b: &mut Bencher, mut sandbox: &mut DatastoreTestSandbox<D, T, I>) {
    insert_sample_data(&mut sandbox);
    let christopher_id = sandbox.search_id("user", "Christopher");
    let after = get_after();
    let before = get_before();

    b.iter(|| {
        let trans = sandbox.transaction();
    	trans.get_edge_time_range(christopher_id, "review".to_string(), after.clone(), before.clone(), 10).unwrap();
    });
}
