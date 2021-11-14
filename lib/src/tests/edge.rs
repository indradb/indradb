use std::collections::HashSet;

use super::util::{create_edge_from, create_edges, create_time_range_queryable_edges};
use crate::{
    models, Datastore, EdgeDirection, EdgeKey, EdgeQueryExt, SpecificEdgeQuery, SpecificVertexQuery, Transaction,
    VertexQueryExt,
};

use chrono::offset::Utc;
use chrono::Timelike;
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub fn should_get_a_valid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();

    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t.clone(), inbound_v.id);

    // Record the start and end time. Round off the the nanoseconds off the
    // start time, since some implementations may not have that level of
    // accuracy.
    let start_time = Utc::now().with_nanosecond(0).unwrap();
    trans.create_edge(&key).unwrap();
    let end_time = Utc::now();

    let e = trans.get_edges(SpecificEdgeQuery::single(key)).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].key.outbound_id, outbound_v.id);
    assert_eq!(e[0].key.t, edge_t);
    assert_eq!(e[0].key.inbound_id, inbound_v.id);
    assert!(e[0].created_datetime >= start_time);
    assert!(e[0].created_datetime <= end_time);
}

pub fn should_not_get_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();

    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    let e = trans
        .get_edges(SpecificEdgeQuery::single(EdgeKey::new(
            outbound_v.id,
            edge_t.clone(),
            Uuid::default(),
        )))
        .unwrap();
    assert_eq!(e.len(), 0);
    let e = trans
        .get_edges(SpecificEdgeQuery::single(EdgeKey::new(
            Uuid::default(),
            edge_t,
            inbound_v.id,
        )))
        .unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_create_a_valid_edge<D: Datastore>(datastore: &mut D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let trans = datastore.transaction().unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    // Set the edge and check
    let key = models::EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    trans.create_edge(&key).unwrap();
    let e = trans.get_edges(SpecificEdgeQuery::single(key.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    trans.create_edge(&key).unwrap();

    // First check that getting a single edge will still...get a single edge
    let e = trans.get_edges(SpecificEdgeQuery::single(key.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);

    // REGRESSION: Second check that getting an edge range will only fetch a
    // single edge
    let e = trans
        .get_edges(SpecificVertexQuery::single(outbound_v.id).outbound().limit(10))
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);
}

pub fn should_not_create_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t, Uuid::default());
    let result = trans.create_edge(&key);
    assert_eq!(result.unwrap(), false);
}

pub fn should_delete_a_valid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Identifier::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();

    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    trans.create_edge(&key).unwrap();

    let q = SpecificEdgeQuery::single(key);
    trans
        .set_edge_properties(q.clone().property("foo"), &JsonValue::Bool(true))
        .unwrap();

    trans.delete_edges(q.clone()).unwrap();
    let e = trans.get_edges(q).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_not_delete_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Identifier::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    trans
        .delete_edges(SpecificEdgeQuery::single(EdgeKey::new(
            outbound_v.id,
            edge_t,
            Uuid::default(),
        )))
        .unwrap();
}

pub fn should_get_an_edge_count<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = trans
        .get_edge_count(outbound_id, Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let count = trans
        .get_edge_count(outbound_id, None, EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = trans
        .get_edge_count(Uuid::default(), Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_inbound_edge_count<D: Datastore>(datastore: &mut D) {
    let (_, inbound_ids) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let count = trans
        .get_edge_count(inbound_ids[0], None, EdgeDirection::Inbound)
        .unwrap();
    assert_eq!(count, 1);
}

pub fn should_get_an_edge_range<D: Datastore>(datastore: &mut D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = trans
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(start_time)
                .high(end_time),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_with_no_type<D: Datastore>(datastore: &mut D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let range = trans
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .low(start_time)
                .high(end_time),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_range<D: Datastore>(datastore: &mut D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("foo").unwrap();
    let range = trans
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(start_time)
                .high(end_time),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 0);
}

pub fn should_get_edges_with_no_high<D: Datastore>(datastore: &mut D) {
    let (outbound_id, start_time, _, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = trans
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(start_time),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 10);
}

pub fn should_get_edges_with_no_low<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = trans
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .high(end_time),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 10);
}

pub fn should_get_edges_with_no_time<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _, _, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = trans
        .get_edges(SpecificVertexQuery::single(outbound_id).outbound().limit(100).t(t))
        .unwrap();
    check_edge_range(&range, outbound_id, 15);
}

pub fn should_get_no_edges_for_reversed_time<D: Datastore>(datastore: &mut D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = trans
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(end_time)
                .high(start_time),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 0);
}

pub fn should_get_edges<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _, _, inbound_ids) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Identifier::new("test_edge_type").unwrap();
    let q = SpecificEdgeQuery::new(vec![
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[0]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[1]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[2]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[3]),
        EdgeKey::new(outbound_id, t, inbound_ids[4]),
    ]);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_piped<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();

    let inbound_id = create_edge_from(&trans, outbound_v.id);

    let query_1 = SpecificVertexQuery::single(outbound_v.id)
        .outbound()
        .limit(1)
        .t(models::Identifier::new("test_edge_type").unwrap());
    let range = trans.get_edges(query_1.clone()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0].key,
        models::EdgeKey::new(
            outbound_v.id,
            models::Identifier::new("test_edge_type").unwrap(),
            inbound_id
        )
    );

    let query_2 = query_1
        .inbound()
        .limit(1)
        .inbound()
        .limit(1)
        .t(models::Identifier::new("test_edge_type").unwrap());
    let range = trans.get_edges(query_2).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0].key,
        models::EdgeKey::new(
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
        assert_eq!(edge.key.outbound_id, expected_outbound_id);
        assert_eq!(edge.key.t, t);
        assert!(!covered_ids.contains(&edge.key.inbound_id));
        covered_ids.insert(edge.key.inbound_id);
    }
}
