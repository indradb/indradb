use std::collections::HashSet;

use super::util::{create_edge_from, create_edges, create_time_range_queryable_edges};
use crate::{
    models, Datastore, EdgeDirection, EdgeKey, SpecificEdgeQuery, SpecificVertexQuery, QueryExt,
};

use chrono::offset::Utc;
use chrono::Timelike;
use uuid::Uuid;

pub fn should_get_a_valid_edge<D: Datastore>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    datastore.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t.clone(), inbound_v.id);

    // Record the start and end time. Round off the the nanoseconds off the
    // start time, since some implementations may not have that level of
    // accuracy.
    let start_time = Utc::now().with_nanosecond(0).unwrap();
    datastore.create_edge(&key).unwrap();
    let end_time = Utc::now();

    let e = datastore.get_edges(SpecificEdgeQuery::single(key).into()).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].key.outbound_id, outbound_v.id);
    assert_eq!(e[0].key.t, edge_t);
    assert_eq!(e[0].key.inbound_id, inbound_v.id);
    assert!(e[0].created_datetime >= start_time);
    assert!(e[0].created_datetime <= end_time);
}

pub fn should_not_get_an_invalid_edge<D: Datastore>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    datastore.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    let e = datastore
        .get_edges(SpecificEdgeQuery::single(EdgeKey::new(outbound_v.id, edge_t.clone(), Uuid::default())).into())
        .unwrap();
    assert_eq!(e.len(), 0);
    let e = datastore
        .get_edges(SpecificEdgeQuery::single(EdgeKey::new(Uuid::default(), edge_t, inbound_v.id)).into())
        .unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_create_a_valid_edge<D: Datastore>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    datastore.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    // Set the edge and check
    let key = models::EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    datastore.create_edge(&key).unwrap();
    let e = datastore
        .get_edges(SpecificEdgeQuery::single(key.clone()).into())
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    datastore.create_edge(&key).unwrap();

    // First check that getting a single edge will still...get a single edge
    let e = datastore
        .get_edges(SpecificEdgeQuery::single(key.clone()).into())
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);

    // REGRESSION: Second check that getting an edge range will only fetch a
    // single edge
    let e = datastore
        .get_edges(SpecificVertexQuery::single(outbound_v.id).outbound().limit(10).into())
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);
}

pub fn should_not_create_an_invalid_edge<D: Datastore>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t, Uuid::default());
    let result = datastore.create_edge(&key);
    assert_eq!(result.unwrap(), false);
}

pub fn should_delete_a_valid_edge<D: Datastore>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    datastore.create_vertex(&inbound_v).unwrap();

    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    datastore.create_edge(&key).unwrap();

    let q = SpecificEdgeQuery::single(key);
    datastore
        .set_edge_properties(
            q.clone().property(models::Identifier::new("foo").unwrap()),
            serde_json::Value::Bool(true),
        )
        .unwrap();

    datastore.delete_edges(q.clone().into()).unwrap();
    let e = datastore.get_edges(q.into()).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_not_delete_an_invalid_edge<D: Datastore>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    datastore
        .delete_edges(SpecificEdgeQuery::single(EdgeKey::new(outbound_v.id, edge_t, Uuid::default())).into())
        .unwrap();
}

pub fn should_get_an_edge_count<D: Datastore>(datastore: &D) {
    let (outbound_id, _) = create_edges(datastore);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = datastore
        .get_edge_count(outbound_id, Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D: Datastore>(datastore: &D) {
    let (outbound_id, _) = create_edges(datastore);
    let count = datastore
        .get_edge_count(outbound_id, None, EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore>(datastore: &D) {
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = datastore
        .get_edge_count(Uuid::default(), Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_inbound_edge_count<D: Datastore>(datastore: &D) {
    let (_, inbound_ids) = create_edges(datastore);
    let count = datastore
        .get_edge_count(inbound_ids[0], None, EdgeDirection::Inbound)
        .unwrap();
    assert_eq!(count, 1);
}

pub fn should_get_an_edge_range<D: Datastore>(datastore: &D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = datastore
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(start_time)
                .high(end_time)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_with_no_type<D: Datastore>(datastore: &D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let range = datastore
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .low(start_time)
                .high(end_time)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_range<D: Datastore>(datastore: &D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let t = models::Identifier::new("foo").unwrap();
    let range = datastore
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(start_time)
                .high(end_time)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 0);
}

pub fn should_get_edges_with_no_high<D: Datastore>(datastore: &D) {
    let (outbound_id, start_time, _, _) = create_time_range_queryable_edges(datastore);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = datastore
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(start_time)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 10);
}

pub fn should_get_edges_with_no_low<D: Datastore>(datastore: &D) {
    let (outbound_id, _, end_time, _) = create_time_range_queryable_edges(datastore);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = datastore
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .high(end_time)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 10);
}

pub fn should_get_edges_with_no_time<D: Datastore>(datastore: &D) {
    let (outbound_id, _, _, _) = create_time_range_queryable_edges(datastore);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = datastore
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(100)
                .t(t)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 15);
}

pub fn should_get_no_edges_for_reversed_time<D: Datastore>(datastore: &D) {
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let range = datastore
        .get_edges(
            SpecificVertexQuery::single(outbound_id)
                .outbound()
                .limit(10)
                .t(t)
                .low(end_time)
                .high(start_time)
                .into(),
        )
        .unwrap();
    check_edge_range(&range, outbound_id, 0);
}

pub fn should_get_edges<D: Datastore>(datastore: &D) {
    let (outbound_id, _, _, inbound_ids) = create_time_range_queryable_edges(datastore);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let q = SpecificEdgeQuery::new(vec![
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[0]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[1]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[2]),
        EdgeKey::new(outbound_id, t.clone(), inbound_ids[3]),
        EdgeKey::new(outbound_id, t, inbound_ids[4]),
    ]);
    let range = datastore.get_edges(q.into()).unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_piped<D: Datastore>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();

    let inbound_id = create_edge_from(datastore, outbound_v.id);

    let query_1 = SpecificVertexQuery::single(outbound_v.id)
        .outbound()
        .limit(1)
        .t(models::Identifier::new("test_edge_type").unwrap());
    let range = datastore.get_edges(query_1.clone().into()).unwrap();
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
    let range = datastore.get_edges(query_2.into()).unwrap();
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
