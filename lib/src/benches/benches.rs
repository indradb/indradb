use crate::{
    ijson, AllEdgeQuery, BulkInsertItem, CountQueryExt, Database, Datastore, Edge, Error, Identifier, Query,
    SpecificEdgeQuery, SpecificVertexQuery, Vertex,
};

use test::Bencher;

#[cfg(test)]
fn generate_rand_ident_value(len: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

// The following two benchmarks check the construction of a new identifier that
// hasn't been previously interned. A random value needs to be generated within
// benchmark iterations to ensure the value hasn't been previously interned, so
// these benchmarks will also include the overhead of random string
// construction.
#[bench]
fn bench_ident_new(b: &mut crate::benches::Bencher) {
    b.iter(|| {
        let value = generate_rand_ident_value(255);
        Identifier::new(value).unwrap();
    });
}

#[bench]
fn bench_ident_new_unchecked(b: &mut crate::benches::Bencher) {
    b.iter(|| unsafe {
        let value = generate_rand_ident_value(255);
        Identifier::new_unchecked(value);
    });
}

// The following two benchmarks check the construction of a new identifier that
// has been previously interned.
#[bench]
fn bench_ident_renew(b: &mut crate::benches::Bencher) {
    let value = generate_rand_ident_value(255);
    Identifier::new(&value).unwrap();

    b.iter(|| {
        Identifier::new(&value).unwrap();
    });
}

#[bench]
fn bench_ident_renew_unchecked(b: &mut crate::benches::Bencher) {
    let value = generate_rand_ident_value(255);
    Identifier::new(&value).unwrap();

    b.iter(|| unsafe {
        Identifier::new_unchecked(&value);
    });
}

#[bench]
fn bench_ident_comparison(b: &mut crate::benches::Bencher) {
    let i1 = Identifier::new("foo").unwrap();
    let i2 = Identifier::new("bar").unwrap();
    let i3 = Identifier::new("baz").unwrap();

    b.iter(|| {
        assert!(i1 > i2);
        assert!(i1 > i3);
        assert!(i2 < i3);

        assert_eq!(i1, i1);
        assert_eq!(i2, i2);
        assert_eq!(i3, i3);

        assert_ne!(i1, i2);
        assert_ne!(i1, i3);
        assert_ne!(i2, i3);
    });
}

pub fn bench_create_vertex<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) -> Result<(), Error> {
    let t = Identifier::new("bench_create_vertex")?;
    b.iter(|| {
        let v = Vertex::new(t);
        db.create_vertex(&v).unwrap();
    });
    Ok(())
}

pub fn bench_get_vertices<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) -> Result<(), Error> {
    let id = {
        let t = Identifier::new("bench_get_vertices")?;
        let v = Vertex::new(t);
        db.create_vertex(&v)?;
        v.id
    };

    let q: Query = SpecificVertexQuery::single(id).into();

    b.iter(|| {
        db.get(q.clone()).unwrap();
    });

    Ok(())
}

pub fn bench_create_edge<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) -> Result<(), Error> {
    let t = Identifier::new("bench_create_edge")?;

    let (outbound_id, inbound_id) = {
        let outbound_v = Vertex::new(t);
        let inbound_v = Vertex::new(t);
        db.create_vertex(&outbound_v)?;
        db.create_vertex(&inbound_v)?;
        (outbound_v.id, inbound_v.id)
    };

    let edge = Edge::new(outbound_id, t, inbound_id);

    b.iter(|| {
        db.create_edge(&edge).unwrap();
    });

    Ok(())
}

pub fn bench_get_edges<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) -> Result<(), Error> {
    let t = Identifier::new("bench_get_edges")?;

    let edge = {
        let outbound_v = Vertex::new(t);
        let inbound_v = Vertex::new(t);
        db.create_vertex(&outbound_v)?;
        db.create_vertex(&inbound_v)?;
        let edge = Edge::new(outbound_v.id, t, inbound_v.id);
        db.create_edge(&edge)?;
        edge
    };

    let q: Query = SpecificEdgeQuery::single(edge.clone()).into();

    b.iter(|| {
        db.get(q.clone()).unwrap();
    });

    Ok(())
}

pub fn bench_get_edge_count<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) -> Result<(), Error> {
    let t = Identifier::new("bench_get_edge_count")?;

    let outbound_v = Vertex::new(t);
    let inbound_v = Vertex::new(t);
    db.create_vertex(&outbound_v)?;
    db.create_vertex(&inbound_v)?;
    let edge = Edge::new(outbound_v.id, t, inbound_v.id);
    db.create_edge(&edge)?;

    let q: Query = AllEdgeQuery.count()?.into();

    b.iter(|| {
        db.get(q.clone()).unwrap();
    });

    Ok(())
}

const BULK_INSERT_COUNT: usize = 100;

pub fn bench_bulk_insert<D: Datastore>(b: &mut Bencher, db: &mut Database<D>) -> Result<(), Error> {
    let t = Identifier::new("bench_bulk_insert")?;

    let mut vertices = Vec::with_capacity(BULK_INSERT_COUNT);
    for _ in 0..BULK_INSERT_COUNT {
        vertices.push(Vertex::new(t));
    }

    let mut edges = Vec::with_capacity(BULK_INSERT_COUNT * BULK_INSERT_COUNT);
    for i in 0..BULK_INSERT_COUNT {
        for j in 0..BULK_INSERT_COUNT {
            edges.push(Edge::new(vertices[i].id, t, vertices[j].id));
        }
    }

    let mut items = Vec::with_capacity(2 * vertices.len() + 2 * edges.len());
    let t = Identifier::new("is_benchmark")?;
    for vertex in vertices.into_iter() {
        items.push(BulkInsertItem::Vertex(vertex.clone()));
        items.push(BulkInsertItem::VertexProperty(vertex.id, t, ijson!(true)));
    }
    for edge in edges.into_iter() {
        items.push(BulkInsertItem::Edge(edge.clone()));
        items.push(BulkInsertItem::EdgeProperty(edge, t, ijson!(true)));
    }

    b.iter(|| {
        db.bulk_insert(items.clone()).unwrap();
    });

    Ok(())
}
