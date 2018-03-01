use indradb::{Datastore, EdgeDirection, EdgeKey, EdgeQuery, Transaction, Type, VertexQuery};
use test::Bencher;

pub fn bench_create_vertex<D, T>(b: &mut Bencher, datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let t = Type::new("user".to_string()).unwrap();
        trans.create_vertex(&t).unwrap();
    });
}

pub fn bench_get_vertices<D, T>(b: &mut Bencher, datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let id = {
        let trans = datastore.transaction().unwrap();
        let t = Type::new("test_name".to_string()).unwrap();
        trans.create_vertex(&t).unwrap()
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let q = VertexQuery::Vertices { ids: vec![id] };
        trans.get_vertices(&q).unwrap();
    });
}

pub fn bench_create_edge<D, T>(b: &mut Bencher, datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, inbound_id) = {
        let trans = datastore.transaction().unwrap();
        let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
        let outbound_id = trans.create_vertex(&vertex_t).unwrap();
        let inbound_id = trans.create_vertex(&vertex_t).unwrap();
        (outbound_id, inbound_id)
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        let k = EdgeKey::new(outbound_id, edge_t, inbound_id);
        trans.create_edge(&k).unwrap();
    });
}

pub fn bench_get_edges<D, T>(b: &mut Bencher, datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, inbound_id) = {
        let trans = datastore.transaction().unwrap();
        let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        let outbound_id = trans.create_vertex(&vertex_t).unwrap();
        let inbound_id = trans.create_vertex(&vertex_t).unwrap();
        let key = EdgeKey::new(outbound_id, edge_t, inbound_id);
        trans.create_edge(&key).unwrap();
        (outbound_id, inbound_id)
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        let q = EdgeQuery::Edges {
            keys: vec![EdgeKey::new(outbound_id, edge_t, inbound_id)],
        };
        trans.get_edges(&q).unwrap();
    });
}

pub fn bench_get_edge_count<D, T>(b: &mut Bencher, datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let outbound_id = {
        let trans = datastore.transaction().unwrap();
        let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        let outbound_id = trans.create_vertex(&vertex_t).unwrap();
        let inbound_id = trans.create_vertex(&vertex_t).unwrap();
        let key = EdgeKey::new(outbound_id, edge_t, inbound_id);
        trans.create_edge(&key).unwrap();
        outbound_id
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let edge_t = Type::new("test_vertex_type".to_string()).unwrap();
        trans
            .get_edge_count(outbound_id, Some(&edge_t), EdgeDirection::Outbound)
            .unwrap();
    });
}
