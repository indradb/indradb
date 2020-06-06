use crate::models::{BulkInsertItem, Edge, EdgeDirection, SpecificEdgeQuery, SpecificVertexQuery, Type};
use crate::traits::{Datastore, Transaction};

use test::Bencher;

pub fn bench_create_vertex<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_create_vertex").unwrap();

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        trans.create_vertex(&t).unwrap();
    });
}

pub fn bench_get_vertices<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let id = {
        let trans = datastore.transaction().unwrap();
        let t = Type::new("bench_get_vertices").unwrap();
        trans.create_vertex(&t).unwrap()
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let q = SpecificVertexQuery::single(id);
        trans.get_vertices(q).unwrap();
    });
}

pub fn bench_create_edge<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_create_edge").unwrap();

    let (out_id, in_id) = {
        let trans = datastore.transaction().unwrap();
        let out_id = trans.create_vertex(&t).unwrap();
        let in_id = trans.create_vertex(&t).unwrap();
        (out_id, in_id)
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        let e = Edge::new(out_id, t.clone(), in_id);
        trans.create_edge(&e).unwrap();
    });
}

pub fn bench_get_edges<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_get_edges").unwrap();

    let key = {
        let trans = datastore.transaction().unwrap();
        let out_id = trans.create_vertex(&t).unwrap();
        let in_id = trans.create_vertex(&t).unwrap();
        let key = Edge::new(out_id, t.clone(), in_id);
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

    let out_id = {
        let trans = datastore.transaction().unwrap();
        let out_id = trans.create_vertex(&t).unwrap();
        let in_id = trans.create_vertex(&t).unwrap();
        let key = Edge::new(out_id, t.clone(), in_id);
        trans.create_edge(&key).unwrap();
        out_id
    };

    b.iter(|| {
        let trans = datastore.transaction().unwrap();
        trans.get_edge_count(out_id, Some(&t), EdgeDirection::Outbound).unwrap();
    });
}

const BULK_INSERT_COUNT: usize = 100;

pub fn bench_bulk_insert_vertices<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_bulk_insert_vertices").unwrap();

    let mut items = Vec::with_capacity(BULK_INSERT_COUNT);
    for _ in 0..BULK_INSERT_COUNT {
        items.push(BulkInsertItem::Vertex(t.clone()));
    }

    b.iter(|| {
        datastore.bulk_insert(items.iter().cloned()).unwrap();
    });
}

pub fn bench_bulk_insert_edges<D: Datastore>(b: &mut Bencher, datastore: &mut D) {
    let t = Type::new("bench_bulk_insert").unwrap();

    let mut items = Vec::with_capacity(BULK_INSERT_COUNT);
    for _ in 0..BULK_INSERT_COUNT {
        items.push(BulkInsertItem::Vertex(t.clone()));
    }
    let (start_id, end_id) = datastore.bulk_insert(items.into_iter()).unwrap().id_range.unwrap();

    let mut items = Vec::with_capacity(BULK_INSERT_COUNT * BULK_INSERT_COUNT);
    for out_id in start_id..=end_id {
        for in_id in start_id..=end_id {
            items.push(BulkInsertItem::Edge(Edge::new(out_id, t.clone(), in_id)));
        }
    }

    b.iter(|| {
        datastore.bulk_insert(items.iter().cloned()).unwrap();
    });
}
