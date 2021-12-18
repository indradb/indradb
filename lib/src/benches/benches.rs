use crate::models::{
    BulkInsertItem, EdgeDirection, EdgeKey, Identifier, SpecificEdgeQuery, SpecificVertexQuery, Vertex,
};
use crate::traits::{Datastore, Transaction};

use test::Bencher;

pub fn bench_create_vertex<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Identifier::new("bench_create_vertex").unwrap();

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let v = Vertex::new(t.clone());
        trans.create_vertex(&v).unwrap();
    });
}

pub fn bench_get_vertices<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let id = {
        let trans = datastore.transaction().unwrap();
        let t = Identifier::new("bench_get_vertices").unwrap();
        let v = Vertex::new(t);
        trans.create_vertex(&v).unwrap();
        v.id
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let q = SpecificVertexQuery::single(id);
        trans.get_vertices(q.into()).unwrap();
    });
}

pub fn bench_create_edge<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Identifier::new("bench_create_edge").unwrap();

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
    let t = Identifier::new("bench_get_edges").unwrap();

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
        trans.get_edges(q.into()).unwrap();
    });
}

pub fn bench_get_edge_count<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Identifier::new("bench_get_edge_count").unwrap();

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

const BULK_INSERT_COUNT: usize = 100;

pub fn bench_bulk_insert<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Identifier::new("bench_bulk_insert").unwrap();

    let mut vertices = Vec::with_capacity(BULK_INSERT_COUNT);
    for _ in 0..BULK_INSERT_COUNT {
        vertices.push(Vertex::new(t.clone()));
    }

    let mut edge_keys = Vec::with_capacity(BULK_INSERT_COUNT * BULK_INSERT_COUNT);
    for i in 0..BULK_INSERT_COUNT {
        for j in 0..BULK_INSERT_COUNT {
            edge_keys.push(EdgeKey::new(vertices[i].id, t.clone(), vertices[j].id));
        }
    }

    let mut items = Vec::with_capacity(2 * vertices.len() + 2 * edge_keys.len());
    let t = Identifier::new("is_benchmark").unwrap();
    for vertex in vertices.into_iter() {
        items.push(BulkInsertItem::Vertex(vertex.clone()));
        items.push(BulkInsertItem::VertexProperty(
            vertex.id,
            t.clone(),
            serde_json::Value::Bool(true),
        ));
    }
    for edge_key in edge_keys.into_iter() {
        items.push(BulkInsertItem::Edge(edge_key.clone()));
        items.push(BulkInsertItem::EdgeProperty(
            edge_key,
            t.clone(),
            serde_json::Value::Bool(true),
        ));
    }

    b.iter(|| {
        datastore.bulk_insert(items.clone().into_iter()).unwrap();
    });
}
