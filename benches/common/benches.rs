use braid::{Datastore, Transaction, Type, Weight, EdgeQuery, VertexQuery, EdgeKey};
use braid::tests::DatastoreTestSandbox;
use test::Bencher;

pub fn bench_create_vertex<D, T>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    b.iter(|| {
        let trans = sandbox.transaction();
        let t = Type::new("user".to_string()).unwrap();
        trans.create_vertex(t).unwrap();
    });
}

pub fn bench_get_vertices<D, T>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let t = Type::new("test_name".to_string()).unwrap();
    let id = trans.create_vertex(t).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        let q = VertexQuery::Vertex(id);
        trans.get_vertices(q).unwrap();
    });
}

pub fn bench_create_edge<D, T>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        let k = EdgeKey::new(outbound_id.clone(), edge_t, inbound_id.clone());
        let w = Weight::new(0.5).unwrap();
        trans.create_edge(k, w).unwrap();
    });
}

pub fn bench_get_edges<D, T>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
    let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    trans.create_edge(EdgeKey::new(outbound_id, edge_t, inbound_id), Weight::new(0.5).unwrap()).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        let q = EdgeQuery::Edge(EdgeKey::new(outbound_id.clone(), edge_t, inbound_id));
        trans.get_edges(q).unwrap();
    });
}

pub fn bench_get_edge_count<D, T>(b: &mut Bencher, sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
    let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    trans.create_edge(EdgeKey::new(outbound_id, edge_t, inbound_id), Weight::new(0.5).unwrap()).unwrap();
    trans.commit().unwrap();

    b.iter(|| {
        let trans = sandbox.transaction();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        let q = EdgeQuery::Edge(EdgeKey::new(outbound_id.clone(), edge_t, inbound_id));
        trans.get_edge_count(q).unwrap();
    });
}
