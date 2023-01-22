use std::collections::HashSet;

use super::util;
use crate::{
    models, AllEdgeQuery, Database, Datastore, Edge, EdgeDirection, QueryExt, SpecificEdgeQuery, SpecificVertexQuery,
};

pub fn should_get_all_edges<D: Datastore>(db: &Database<D>) {
    let (outbound_id, inbound_ids) = util::create_edges(db);
    let edges = util::get_edges(db, AllEdgeQuery).unwrap();
    assert_eq!(
        edges,
        inbound_ids
            .into_iter()
            .map(|id| Edge::new(outbound_id, models::Identifier::new("test_edge_type").unwrap(), id))
            .collect::<Vec<Edge>>()
    );
}

pub fn should_get_a_valid_edge<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let inbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);

    db.create_edge(&edge).unwrap();

    let e = util::get_edges(db, SpecificEdgeQuery::single(edge)).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].outbound_id, outbound_id);
    assert_eq!(e[0].t, edge_t);
    assert_eq!(e[0].inbound_id, inbound_id);
}

pub fn should_not_get_an_invalid_edge<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let inbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    let e = util::get_edges(db, SpecificEdgeQuery::single(Edge::new(outbound_id, edge_t, 0))).unwrap();
    assert_eq!(e.len(), 0);
    let e = util::get_edges(db, SpecificEdgeQuery::single(Edge::new(0, edge_t, inbound_id))).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_create_a_valid_edge<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let inbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    // Set the edge and check
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);
    db.create_edge(&edge).unwrap();
    let e = util::get_edges(db, SpecificEdgeQuery::single(edge.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    db.create_edge(&edge).unwrap();

    // First check that getting a single edge will still...get a single edge
    let e = util::get_edges(db, SpecificEdgeQuery::single(edge.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // REGRESSION: Second check that getting an edge range will only fetch a
    // single edge
    let e = util::get_edges(
        db,
        SpecificVertexQuery::single(outbound_id).outbound().unwrap().limit(10),
    )
    .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);
}

pub fn should_not_create_an_invalid_edge<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_id, edge_t, 0);
    let result = db.create_edge(&edge);
    assert_eq!(result.unwrap(), false);
}

pub fn should_delete_a_valid_edge<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_edge_type").unwrap();
    let outbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let inbound_id = db.create_vertex_from_type(vertex_t).unwrap();

    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);
    db.create_edge(&edge).unwrap();

    let q = SpecificEdgeQuery::single(edge);
    db.set_properties(
        q.clone(),
        models::Identifier::new("foo").unwrap(),
        serde_json::Value::Bool(true),
    )
    .unwrap();

    db.delete(q.clone()).unwrap();
    let e = util::get_edges(db, q).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_not_delete_an_invalid_edge<D: Datastore>(db: &Database<D>) {
    let outbound_id = db
        .create_vertex_from_type(models::Identifier::new("test_edge_type").unwrap())
        .unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    db.delete(SpecificEdgeQuery::single(Edge::new(outbound_id, edge_t, 0)))
        .unwrap();
}

pub fn should_get_an_edge_count<D: Datastore>(db: &Database<D>) {
    let (outbound_id, _) = util::create_edges(db);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = util::get_edge_count(db, outbound_id, Some(t), EdgeDirection::Outbound).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D: Datastore>(db: &Database<D>) {
    let (outbound_id, _) = util::create_edges(db);
    let count = util::get_edge_count(db, outbound_id, None, EdgeDirection::Outbound).unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore>(db: &Database<D>) {
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = util::get_edge_count(db, 0, Some(t), EdgeDirection::Outbound).unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_inbound_edge_count<D: Datastore>(db: &Database<D>) {
    let (_, inbound_ids) = util::create_edges(db);
    let count = util::get_edge_count(db, inbound_ids[0], None, EdgeDirection::Inbound).unwrap();
    assert_eq!(count, 1);
}

pub fn should_get_edges_with_no_type<D: Datastore>(db: &Database<D>) {
    let (outbound_id, _) = util::create_edges(db);
    let range = util::get_edges(
        db,
        SpecificVertexQuery::single(outbound_id).outbound().unwrap().limit(10),
    )
    .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edge_range<D: Datastore>(db: &Database<D>) {
    let (outbound_id, _) = util::create_edges(db);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = util::get_edges(
        db,
        SpecificVertexQuery::single(outbound_id)
            .outbound()
            .unwrap()
            .limit(100)
            .t(t),
    )
    .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges<D: Datastore>(db: &Database<D>) {
    let (outbound_id, inbound_ids) = util::create_edges(db);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let q = SpecificEdgeQuery::new(vec![
        Edge::new(outbound_id, t, inbound_ids[0]),
        Edge::new(outbound_id, t, inbound_ids[1]),
        Edge::new(outbound_id, t, inbound_ids[2]),
        Edge::new(outbound_id, t, inbound_ids[3]),
        Edge::new(outbound_id, t, inbound_ids[4]),
    ]);
    let range = util::get_edges(db, q).unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_piped<D: Datastore>(db: &Database<D>) {
    let outbound_id = db
        .create_vertex_from_type(models::Identifier::new("test_vertex_type").unwrap())
        .unwrap();
    let inbound_id = util::create_edge_from(db, outbound_id);

    let query_1 = SpecificVertexQuery::single(outbound_id)
        .outbound()
        .unwrap()
        .limit(1)
        .t(models::Identifier::new("test_edge_type").unwrap());
    let range = util::get_edges(db, query_1.clone()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(
            outbound_id,
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
    let range = util::get_edges(db, query_2).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(
            outbound_id,
            models::Identifier::new("test_edge_type").unwrap(),
            inbound_id
        )
    );
}

fn check_edge_range(range: &[models::Edge], expected_outbound_id: u64, expected_length: usize) {
    assert_eq!(range.len(), expected_length);
    let mut covered_ids: HashSet<u64> = HashSet::new();
    let t = models::Identifier::new("test_edge_type").unwrap();

    for edge in range {
        assert_eq!(edge.outbound_id, expected_outbound_id);
        assert_eq!(edge.t, t);
        assert!(!covered_ids.contains(&edge.inbound_id));
        covered_ids.insert(edge.inbound_id);
    }
}
