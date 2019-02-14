use crate::models::{EdgeDirection, EdgeKey, SpecificEdgeQuery, SpecificVertexQuery, Type, Vertex};
use test::Bencher;
use crate::traits::{Datastore, Transaction};

pub fn bench_create_vertex<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_create_vertex").unwrap();

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let v = Vertex::new(t.clone());
        trans.create_vertex(&v).unwrap();
    });
}

pub fn bench_get_vertices<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let id = {
        let trans = datastore.transaction().unwrap();
        let t = Type::new("bench_get_vertices").unwrap();
        let v = Vertex::new(t);
        trans.create_vertex(&v).unwrap();
        v.id
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let q = SpecificVertexQuery::single(id);
        trans.get_vertices(q).unwrap();
    });
}

pub fn bench_create_edge<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_create_edge").unwrap();

    let (outbound_id, inbound_id) = {
        let trans = datastore.transaction().unwrap();
        let outbound_v = Vertex::new(t.clone());
        let inbound_v = Vertex::new(t.clone());
        trans.create_vertex(&outbound_v).unwrap();
        trans.create_vertex(&inbound_v).unwrap();
        (outbound_v.id, inbound_v.id)
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let k = EdgeKey::new(outbound_id, t.clone(), inbound_id);
        trans.create_edge(&k).unwrap();
    });
}

pub fn bench_get_edges<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_get_edges").unwrap();

    let key = {
        let trans = datastore.transaction().unwrap();
        let outbound_v = Vertex::new(t.clone());
        let inbound_v = Vertex::new(t.clone());
        trans.create_vertex(&outbound_v).unwrap();
        trans.create_vertex(&inbound_v).unwrap();
        let key = EdgeKey::new(outbound_v.id, t.clone(), inbound_v.id);
        trans.create_edge(&key).unwrap();
        key
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let q = SpecificEdgeQuery::single(key.clone());
        trans.get_edges(q).unwrap();
    });
}

pub fn bench_get_edge_count<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_get_edge_count").unwrap();

    let outbound_id = {
        let trans = datastore.transaction().unwrap();
        let outbound_v = Vertex::new(t.clone());
        let inbound_v = Vertex::new(t.clone());
        trans.create_vertex(&outbound_v).unwrap();
        trans.create_vertex(&inbound_v).unwrap();
        let key = EdgeKey::new(outbound_v.id, t.clone(), inbound_v.id);
        trans.create_edge(&key).unwrap();
        outbound_v.id
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        trans
            .get_edge_count(outbound_id, Some(&t), EdgeDirection::Outbound)
            .unwrap();
    });
}
