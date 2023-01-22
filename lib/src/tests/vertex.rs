use std::collections::HashSet;

use super::util;
use crate::util::extract_count;
use crate::{
    errors, expect_err, models, AllVertexQuery, CountQueryExt, Database, Datastore, QueryExt, RangeVertexQuery,
    SpecificVertexQuery,
};

use uuid::Uuid;

pub fn should_create_vertex_from_type<D: Datastore>(db: &Database<D>) {
    let t = models::Identifier::new("test_vertex_type").unwrap();
    db.create_vertex_from_type(t).unwrap();
}

pub fn should_get_all_vertices<D: Datastore>(db: &Database<D>) {
    let inserted_ids = create_vertices(db);
    let range = util::get_vertices(db, AllVertexQuery).unwrap();
    check_has_all_vertices(range, inserted_ids);
}

pub fn should_get_range_vertices<D: Datastore>(db: &Database<D>) {
    let inserted_ids = create_vertices(db);
    let range = util::get_vertices(db, RangeVertexQuery::new()).unwrap();
    check_has_all_vertices(range, inserted_ids);
}

pub fn should_get_no_vertices_with_zero_limit<D: Datastore>(db: &Database<D>) {
    create_vertices(db);
    let range = util::get_vertices(db, RangeVertexQuery::new().limit(0)).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_range_vertices_out_of_range<D: Datastore>(db: &Database<D>) {
    create_vertices(db);
    let range = util::get_vertices(
        db,
        RangeVertexQuery::new().start_id(Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()),
    )
    .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_no_vertices_with_type_filter<D: Datastore>(db: &Database<D>) {
    let type_filter = models::Identifier::new("foo").unwrap();
    create_vertices(db);
    let range = util::get_vertices(db, RangeVertexQuery::new().t(type_filter)).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_single_vertex<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let vertex = models::Vertex::new(vertex_t);
    db.create_vertex(&vertex).unwrap();
    let range = util::get_vertices(db, SpecificVertexQuery::single(vertex.id)).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, id);
    assert_eq!(range[0].t.as_str(), "test_vertex_type");
}

pub fn should_get_single_vertex_nonexisting<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let vertex = models::Vertex::new(vertex_t);
    db.create_vertex(&vertex).unwrap();
    let range = util::get_vertices(db, SpecificVertexQuery::single(Uuid::default())).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_vertices<D: Datastore>(db: &Database<D>) {
    let mut inserted_ids = create_vertices(db);

    let range = util::get_vertices(
        db,
        SpecificVertexQuery::new(vec![inserted_ids[0], inserted_ids[1], inserted_ids[2], Uuid::default()]),
    )
    .unwrap();

    assert!(range.len() == 3);

    let mut covered_ids: HashSet<Uuid> = HashSet::new();

    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, models::Identifier::new("test_vertex_type").unwrap());
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}

pub fn should_get_vertices_piped<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    let v = models::Vertex::new(vertex_t);
    db.create_vertex(&v).unwrap();
    let inserted_id = util::create_edge_from(db, v.id);

    // This query should get `inserted_id`
    let query_1 = SpecificVertexQuery::single(v.id)
        .outbound()
        .unwrap()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .unwrap()
        .limit(1);
    let range = util::get_vertices(db, query_1.clone()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get `inserted_id`
    let query_2 = SpecificVertexQuery::single(v.id)
        .outbound()
        .unwrap()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .unwrap()
        .limit(1)
        .t(models::Identifier::new("test_inbound_vertex_type").unwrap());
    let range = util::get_vertices(db, query_2).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get nothing
    let query_3 = SpecificVertexQuery::single(v.id)
        .outbound()
        .unwrap()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .unwrap()
        .limit(1)
        .t(models::Identifier::new("foo").unwrap());
    let range = util::get_vertices(db, query_3).unwrap();
    assert_eq!(range.len(), 0);

    // This query should get `v`
    let query_4 = query_1
        .inbound()
        .unwrap()
        .limit(1)
        .t(edge_t)
        .outbound()
        .unwrap()
        .limit(1);
    let range = util::get_vertices(db, query_4).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0], v);
}

pub fn should_delete_a_valid_outbound_vertex<D: Datastore>(db: &Database<D>) {
    let (outbound_id, _) = util::create_edges(db);
    let q = SpecificVertexQuery::single(outbound_id);
    db.set_properties(
        q.clone(),
        models::Identifier::new("foo").unwrap(),
        serde_json::Value::Bool(true),
    )
    .unwrap();
    db.delete(q.clone()).unwrap();
    let v = util::get_vertices(db, q).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = util::get_edge_count(db, outbound_id, Some(&t), models::EdgeDirection::Outbound).unwrap();
    assert_eq!(count, 0);
}

pub fn should_delete_a_valid_inbound_vertex<D: Datastore>(db: &Database<D>) {
    let (_, inbound_ids) = util::create_edges(db);
    let inbound_id = inbound_ids[0];
    let q = SpecificVertexQuery::single(inbound_id);
    db.delete(q.clone()).unwrap();
    let v = util::get_vertices(db, q).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = util::get_edge_count(db, inbound_id, Some(&t), models::EdgeDirection::Inbound).unwrap();
    assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D: Datastore>(db: &Database<D>) {
    db.delete(SpecificVertexQuery::single(Uuid::default())).unwrap();
}

pub fn should_get_a_vertex_count<D: Datastore>(db: &Database<D>) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let v = models::Vertex::new(vertex_t);
    db.create_vertex(&v).unwrap();
    let count = util::get_vertex_count(db).unwrap();
    assert!(count >= 1);
    let count = extract_count(db.get(SpecificVertexQuery::single(v.id).count().unwrap()).unwrap()).unwrap();
    assert!(count >= 1);
}

pub fn should_not_delete_on_vertex_count<D: Datastore>(db: &Database<D>) {
    let result = db.delete(AllVertexQuery.count().unwrap());
    expect_err!(result, errors::Error::OperationOnQuery);
}

pub fn should_not_pipe_on_vertex_count<D: Datastore>(db: &Database<D>) {
    // We have to build the query without it's constructor because the
    // constructor will catch this issue and trigger a `ValidationError`.
    let q = models::PipeQuery {
        inner: Box::new(AllVertexQuery.count().unwrap().into()),
        direction: models::EdgeDirection::Outbound,
        limit: 1,
        t: None,
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
}

fn check_has_all_vertices(range: Vec<models::Vertex>, mut inserted_ids: Vec<Uuid>) {
    assert!(range.len() >= 5);
    let mut covered_ids: HashSet<Uuid> = HashSet::new();
    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, models::Identifier::new("test_vertex_type").unwrap());
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}

fn create_vertices<D: Datastore>(db: &Database<D>) -> Vec<Uuid> {
    let t = models::Identifier::new("test_vertex_type").unwrap();

    let vertices = vec![
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t),
    ];

    for vertex in &vertices {
        db.create_vertex(vertex).unwrap();
    }

    let mut vertex_ids: Vec<Uuid> = vertices.into_iter().map(|v| v.id).collect();
    vertex_ids.sort();
    vertex_ids
}
