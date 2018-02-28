use super::super::{Datastore, EdgeDirection, EdgeKey, EdgeQuery, Transaction, VertexQuery};
use models;
use uuid::Uuid;
use chrono::offset::Utc;
use chrono::Timelike;
use super::util::{create_edge_from, create_edges, create_time_range_queryable_edges};
use std::collections::HashSet;

pub fn should_get_a_valid_edge<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();

    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);

    // Record the start and end time. Round off the the nanoseconds off the
    // start time, since some implementations may not have that level of
    // accuracy.
    let start_time = Utc::now().with_nanosecond(0).unwrap();
    trans.create_edge(key).unwrap();
    let end_time = Utc::now();

    let e = trans
        .get_edges(EdgeQuery::Edges {
            keys: vec![EdgeKey::new(outbound_id, edge_t.clone(), inbound_id)],
        })
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].key.outbound_id, outbound_id);
    assert_eq!(e[0].key.t, edge_t);
    assert_eq!(e[0].key.inbound_id, inbound_id);
    assert!(e[0].created_datetime >= start_time);
    assert!(e[0].created_datetime <= end_time);
}

pub fn should_not_get_an_invalid_edge<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();

    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();

    let e = trans
        .get_edges(EdgeQuery::Edges {
            keys: vec![EdgeKey::new(outbound_id, edge_t.clone(), Uuid::default())],
        })
        .unwrap();
    assert_eq!(e.len(), 0);
    let e = trans
        .get_edges(EdgeQuery::Edges {
            keys: vec![EdgeKey::new(Uuid::default(), edge_t, inbound_id)],
        })
        .unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_create_a_valid_edge<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let trans = datastore.transaction().unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t.clone()).unwrap();

    // Set the edge and check
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    trans.create_edge(key.clone()).unwrap();
    let e = trans
        .get_edges(EdgeQuery::Edges {
            keys: vec![key.clone()],
        })
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    trans.create_edge(key.clone()).unwrap();

    // First check that getting a single edge will still...get a single edge
    let e = trans
        .get_edges(EdgeQuery::Edges {
            keys: vec![key.clone()],
        })
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);

    // REGRESSION: Second check that getting an edge range will only fetch a
    // single edge
    let e = trans
        .get_edges(
            VertexQuery::Vertices {
                ids: vec![outbound_id],
            }.outbound_edges(None, None, None, 10),
        )
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(key, e[0].key);
}

pub fn should_not_create_an_invalid_edge<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), Uuid::default());
    let result = trans.create_edge(key);
    assert_eq!(result.unwrap(), false);
}

pub fn should_delete_a_valid_edge<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t.clone()).unwrap();
    let inbound_id = trans.create_vertex(vertex_t).unwrap();

    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t.clone(), inbound_id);
    trans.create_edge(key.clone()).unwrap();
    trans
        .delete_edges(EdgeQuery::Edges {
            keys: vec![key.clone()],
        })
        .unwrap();
    let e = trans
        .get_edges(EdgeQuery::Edges { keys: vec![key] })
        .unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_not_delete_an_invalid_edge<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    trans
        .delete_edges(EdgeQuery::Edges {
            keys: vec![EdgeKey::new(outbound_id, edge_t, Uuid::default())],
        })
        .unwrap();
}

pub fn should_get_an_edge_count<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans
        .get_edge_count(outbound_id, Some(t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let count = trans
        .get_edge_count(outbound_id, None, EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans
        .get_edge_count(Uuid::default(), Some(t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_inbound_edge_count<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (_, inbound_ids) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let count = trans
        .get_edge_count(inbound_ids[0], None, EdgeDirection::Inbound)
        .unwrap();
    assert_eq!(count, 1);
}

pub fn should_get_an_edge_range<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    }.outbound_edges(Some(t), Some(end_time), Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_with_no_type<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    }.outbound_edges(None, Some(end_time), Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_no_edges_for_an_invalid_range<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("foo".to_string()).unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    }.outbound_edges(Some(t), Some(end_time), Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 0);
}

pub fn should_get_edges_with_no_high<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, start_time, _, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    }.outbound_edges(Some(t), None, Some(start_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 10);
}

pub fn should_get_edges_with_no_low<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, _, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    }.outbound_edges(Some(t), Some(end_time), None, 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 10);
}

pub fn should_get_edges_with_no_time<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, _, _, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    }.outbound_edges(Some(t), None, None, 100);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 15);
}

pub fn should_get_no_edges_for_reversed_time<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, start_time, end_time, _) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    }.outbound_edges(Some(t), Some(start_time), Some(end_time), 10);
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 0);
}

pub fn should_get_edges<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, _, _, inbound_ids) = create_time_range_queryable_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let q = EdgeQuery::Edges {
        keys: vec![
            EdgeKey::new(outbound_id, t.clone(), inbound_ids[0]),
            EdgeKey::new(outbound_id, t.clone(), inbound_ids[1]),
            EdgeKey::new(outbound_id, t.clone(), inbound_ids[2]),
            EdgeKey::new(outbound_id, t.clone(), inbound_ids[3]),
            EdgeKey::new(outbound_id, t.clone(), inbound_ids[4]),
        ],
    };
    let range = trans.get_edges(q).unwrap();
    check_edge_range(&range, outbound_id, 5);
}

pub fn should_get_edges_piped<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();

    let inserted_id_1 = trans.create_vertex(vertex_t.clone()).unwrap();
    let inserted_id_2 = create_edge_from::<D, T>(&trans, inserted_id_1);

    // This query should get `inserted_id_2`
    let query_1 = VertexQuery::Vertices {
        ids: vec![inserted_id_1],
    }.outbound_edges(
        Some(models::Type::new("test_edge_type".to_string()).unwrap()),
        None,
        None,
        1,
    );
    let range = trans.get_edges(query_1.clone()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0].key,
        models::EdgeKey::new(
            inserted_id_1,
            models::Type::new("test_edge_type".to_string()).unwrap(),
            inserted_id_2
        )
    );

    // This query should get `inserted_id_1`
    let query_2 = query_1.inbound_vertices(1).inbound_edges(
        Some(models::Type::new("test_edge_type".to_string()).unwrap()),
        None,
        None,
        1,
    );
    let range = trans.get_edges(query_2).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0].key,
        models::EdgeKey::new(
            inserted_id_1,
            models::Type::new("test_edge_type".to_string()).unwrap(),
            inserted_id_2
        )
    );
}

fn check_edge_range(range: &[models::Edge], expected_outbound_id: Uuid, expected_length: usize) {
    assert_eq!(range.len(), expected_length);
    let mut covered_ids: HashSet<Uuid> = HashSet::new();
    let t = models::Type::new("test_edge_type".to_string()).unwrap();

    for edge in range {
        assert_eq!(edge.key.outbound_id, expected_outbound_id);
        assert_eq!(edge.key.t, t);
        assert!(!covered_ids.contains(&edge.key.inbound_id));
        covered_ids.insert(edge.key.inbound_id);
    }
}
