use std::collections::HashSet;

use super::util::{create_edge_from, create_edges, TestDatabase};
use crate::{models, Datastore, Edge, EdgeDirection, QueryExt, SpecificEdgeQuery, SpecificVertexQuery};

use uuid::Uuid;

pub fn should_get_a_valid_edge<D: Datastore>(db: &TestDatabase<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t.clone(), inbound_v.id);

    db.create_edge(&edge).unwrap();

    let e = db.get_edges(SpecificEdgeQuery::single(edge).into()).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].outbound_id, outbound_v.id);
    assert_eq!(e[0].t, edge_t);
    assert_eq!(e[0].inbound_id, inbound_v.id);
}

pub fn should_not_get_an_invalid_edge<D: Datastore>(db: &TestDatabase<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    let e = db
        .get_edges(SpecificEdgeQuery::single(Edge::new(outbound_v.id, edge_t.clone(), Uuid::default())).into())
        .unwrap();
    assert_eq!(e.len(), 0);
    let e = db
        .get_edges(SpecificEdgeQuery::single(Edge::new(Uuid::default(), edge_t, inbound_v.id)).into())
        .unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_create_a_valid_edge<D: Datastore>(db: &TestDatabase<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    // Set the edge and check
    let edge = models::Edge::new(outbound_v.id, edge_t, inbound_v.id);
    db.create_edge(&edge).unwrap();
    let e = db.get_edges(SpecificEdgeQuery::single(edge.clone()).into()).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    db.create_edge(&edge).unwrap();

    // First check that getting a single edge will still...get a single edge
    let e = db.get_edges(SpecificEdgeQuery::single(edge.clone()).into()).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // REGRESSION: Second check that getting an edge range will only fetch a
    // single edge
    let e = db
        .get_edges(
            SpecificVertexQuery::single(outbound_v.id)
                .outbound()
                .unwrap()
                .limit(10)
                .into(),
        )
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);
}

pub fn should_not_create_an_invalid_edge<D: Datastore>(db: &TestDatabase<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t, Uuid::default());
    let result = db.create_edge(&edge);
    assert_eq!(result.unwrap(), false);
}

pub fn should_delete_a_valid_edge<D: Datastore>(db: &TestDatabase<D>) {
    let vertex_t = models::Identifier::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();

    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t, inbound_v.id);
    db.create_edge(&edge).unwrap();

    let q = SpecificEdgeQuery::single(edge);
    db.set_edge_properties(
        q.clone().property(models::Identifier::new("foo").unwrap()).unwrap(),
        serde_json::Value::Bool(true),
    )
    .unwrap();

    db.delete_edges(q.clone().into()).unwrap();
    let e = db.get_edges(q.into()).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_not_delete_an_invalid_edge<D: Datastore>(db: &TestDatabase<D>) {
    let vertex_t = models::Identifier::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    db.delete_edges(SpecificEdgeQuery::single(Edge::new(outbound_v.id, edge_t, Uuid::default())).into())
        .unwrap();
}

pub fn should_get_an_edge_count<D: Datastore>(db: &TestDatabase<D>) {
    let (outbound_id, _) = create_edges(db);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = db
        .get_edge_count(outbound_id, Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D: Datastore>(db: &TestDatabase<D>) {
    let (outbound_id, _) = create_edges(db);
    let count = db.get_edge_count(outbound_id, None, EdgeDirection::Outbound).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore>(db: &TestDatabase<D>) {
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = db
        .get_edge_count(Uuid::default(), Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_inbound_edge_count<D: Datastore>(db: &TestDatabase<D>) {
    let (_, inbound_ids) = create_edges(db);
    let count = db.get_edge_count(inbound_ids[0], None, EdgeDirection::Inbound).unwrap();
    assert_eq!(count, 1);
}

pub fn should_get_edges_with_no_type<D: Datastore>(db: &TestDatabase<D>) {
    let (outbound_id, _) = create_edges(db);
    let range = db
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .unwrap()
                .limit(10)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edge_range<D: Datastore>(db: &TestDatabase<D>) {
    let (outbound_id, _) = create_edges(db);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = db
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .unwrap()
                .limit(100)
                .t(t)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges<D: Datastore>(db: &TestDatabase<D>) {
    let (outbound_id, inbound_ids) = create_edges(db);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let q = SpecificEdgeQuery::new(vec![
        Edge::new(outbound_id, t.clone(), inbound_ids[0]),
        Edge::new(outbound_id, t.clone(), inbound_ids[1]),
        Edge::new(outbound_id, t.clone(), inbound_ids[2]),
        Edge::new(outbound_id, t.clone(), inbound_ids[3]),
        Edge::new(outbound_id, t, inbound_ids[4]),
    ]);
    let range = db.get_edges(q.into()).unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_piped<D: Datastore>(db: &TestDatabase<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();

    let inbound_id = create_edge_from(db, outbound_v.id);

    let query_1 = SpecificVertexQuery::single(outbound_v.id)
        .outbound()
        .unwrap()
        .limit(1)
        .t(models::Identifier::new("test_edge_type").unwrap());
    let range = db.get_edges(query_1.clone().into()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(
            outbound_v.id,
            models::Identifier::new("test_edge_type").unwrap(),
            inbound_id
        )
    );

    let query_2 = query_1
        .inbound()
        .unwrap()
        .limit(1)
        .inbound()
        .unwrap()
        .limit(1)
        .t(models::Identifier::new("test_edge_type").unwrap());
    let range = db.get_edges(query_2.into()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(
            outbound_v.id,
            models::Identifier::new("test_edge_type").unwrap(),
            inbound_id
        )
    );
}

fn check_edge_range(range: &[models::Edge], expected_outbound_id: Uuid, expected_length: usize) {
    assert_eq!(range.len(), expected_length);
    let mut covered_ids: HashSet<Uuid> = HashSet::new();
    let t = models::Identifier::new("test_edge_type").unwrap();

    for edge in range {
        assert_eq!(edge.outbound_id, expected_outbound_id);
        assert_eq!(edge.t, t);
        assert!(!covered_ids.contains(&edge.inbound_id));
        covered_ids.insert(edge.inbound_id);
    }
}
