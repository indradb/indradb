use crate::models::{
    AllEdgeQuery, BulkInsertItem, CountQueryExt, Edge, Identifier, Query, SpecificEdgeQuery, SpecificVertexQuery,
    Vertex,
};
use crate::{Database, Datastore};

use test::Bencher;

pub fn bench_create_vertex<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) {
    let t = Identifier::new("bench_create_vertex").unwrap();

    b.iter(|| {
        db.create_vertex_from_type(t).unwrap();
    });
}

pub fn bench_get_vertices<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) {
    let id = {
        let t = Identifier::new("bench_get_vertices").unwrap();
        db.create_vertex_from_type(t).unwrap()
    };

    let q: Query = SpecificVertexQuery::single(id).into();

    b.iter(|| {
        db.get(q.clone()).unwrap();
    });
}

pub fn bench_create_edge<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) {
    let t = Identifier::new("bench_create_edge").unwrap();

    let (outbound_id, inbound_id) = {
        let outbound_id = db.create_vertex_from_type(t).unwrap();
        let inbound_id = db.create_vertex_from_type(t).unwrap();
        (outbound_id, inbound_id)
    };

    let edge = Edge::new(outbound_id, t, inbound_id);

    b.iter(|| {
        db.create_edge(edge).unwrap();
    });
}

pub fn bench_get_edges<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) {
    let t = Identifier::new("bench_get_edges").unwrap();

    let edge = {
        let outbound_id = db.create_vertex_from_type(t).unwrap();
        let inbound_id = db.create_vertex_from_type(t).unwrap();
        let edge = Edge::new(outbound_id, t, inbound_id);
        db.create_edge(edge).unwrap();
        edge
    };

    let q: Query = SpecificEdgeQuery::single(edge).into();

    b.iter(|| {
        db.get(q.clone()).unwrap();
    });
}

pub fn bench_get_edge_count<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) {
    let t = Identifier::new("bench_get_edge_count").unwrap();

    let outbound_id = db.create_vertex_from_type(t).unwrap();
    let inbound_id = db.create_vertex_from_type(t).unwrap();
    let edge = Edge::new(outbound_id, t, inbound_id);
    db.create_edge(edge).unwrap();

    let q: Query = AllEdgeQuery.count().unwrap().into();

    b.iter(|| {
        db.get(q.clone()).unwrap();
    });
}

const BULK_INSERT_COUNT: usize = 100;

pub fn bench_bulk_insert<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) {
    let t = Identifier::new("bench_bulk_insert").unwrap();

    let mut vertex_ids = Vec::with_capacity(BULK_INSERT_COUNT);
    for _ in 0..BULK_INSERT_COUNT {
        vertex_ids.push(db.create_vertex_from_type(t).unwrap());
    }

    let mut edges = Vec::with_capacity(BULK_INSERT_COUNT * BULK_INSERT_COUNT);
    for i in 0..BULK_INSERT_COUNT {
        for j in 0..BULK_INSERT_COUNT {
            edges.push(Edge::new(vertex_ids[i], t, vertex_ids[j]));
        }
    }

    let mut items = Vec::with_capacity(2 * vertex_ids.len() + 2 * edges.len());
    let t = Identifier::new("is_benchmark").unwrap();
    for vertex_id in vertex_ids.into_iter() {
        items.push(BulkInsertItem::Vertex(Vertex::new(vertex_id, t)));
        items.push(BulkInsertItem::VertexProperty(
            vertex_id,
            t,
            serde_json::Value::Bool(true),
        ));
    }
    for edge in edges.into_iter() {
        items.push(BulkInsertItem::Edge(edge));
        items.push(BulkInsertItem::EdgeProperty(edge, t, serde_json::Value::Bool(true)));
    }

    b.iter(|| {
        db.bulk_insert(items.clone()).unwrap();
    });
}
