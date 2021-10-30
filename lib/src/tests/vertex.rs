use super::super::{Datastore, EdgeQueryExt, RangeVertexQuery, SpecificVertexQuery, Transaction, VertexQueryExt};
use super::util::{create_edge_from, create_edges};
use crate::models;
use std::collections::HashSet;
use uuid::Uuid;

pub fn should_create_vertex_from_type<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_vertex_type").unwrap();
    trans.create_vertex_from_type(t).unwrap();
}

pub fn should_get_range_vertices<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let mut inserted_ids = create_vertices(&trans);

    let range = trans.get_vertices(RangeVertexQuery::new()).unwrap();

    assert!(range.len() >= 5);

    let mut covered_ids: HashSet<Uuid> = HashSet::new();

    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, models::Type::new("test_vertex_type").unwrap());
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}

pub fn should_get_no_vertices_with_zero_limit<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    create_vertices(&trans);
    let range = trans.get_vertices(RangeVertexQuery::new().limit(0)).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_range_vertices_out_of_range<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    create_vertices(&trans);
    let range = trans
        .get_vertices(RangeVertexQuery::new().start_id(Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()))
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_no_vertices_with_type_filter<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let type_filter = models::Type::new("foo").unwrap();
    create_vertices(&trans);
    let range = trans.get_vertices(RangeVertexQuery::new().t(type_filter)).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_single_vertex<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let vertex = models::Vertex::new(vertex_t);
    trans.create_vertex(&vertex).unwrap();
    let range = trans.get_vertices(SpecificVertexQuery::single(vertex.id)).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, vertex.id);
    assert_eq!(range[0].t.0, "test_vertex_type");
}

pub fn should_get_single_vertex_nonexisting<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let vertex = models::Vertex::new(vertex_t);
    trans.create_vertex(&vertex).unwrap();
    let range = trans
        .get_vertices(SpecificVertexQuery::single(Uuid::default()))
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_vertices<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let mut inserted_ids = create_vertices(&trans);

    let range = trans
        .get_vertices(SpecificVertexQuery::new(vec![
            inserted_ids[0],
            inserted_ids[1],
            inserted_ids[2],
            Uuid::default(),
        ]))
        .unwrap();

    assert!(range.len() == 3);

    let mut covered_ids: HashSet<Uuid> = HashSet::new();

    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, models::Type::new("test_vertex_type").unwrap());
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}

pub fn should_get_vertices_piped<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();

    let v = models::Vertex::new(vertex_t);
    trans.create_vertex(&v).unwrap();
    let inserted_id = create_edge_from(&trans, v.id);

    // This query should get `inserted_id`
    let query_1 = SpecificVertexQuery::single(v.id)
        .outbound()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .limit(1);
    let range = trans.get_vertices(query_1.clone()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get `inserted_id`
    let query_2 = SpecificVertexQuery::single(v.id)
        .outbound()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .limit(1)
        .t(models::Type::new("test_inbound_vertex_type").unwrap());
    let range = trans.get_vertices(query_2).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get nothing
    let query_3 = SpecificVertexQuery::single(v.id)
        .outbound()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .limit(1)
        .t(models::Type::new("foo").unwrap());
    let range = trans.get_vertices(query_3).unwrap();
    assert_eq!(range.len(), 0);

    // This query should get `v`
    let query_4 = query_1.inbound().limit(1).t(edge_t).outbound().limit(1);
    let range = trans.get_vertices(query_4).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0], v);
}

pub fn should_delete_a_valid_outbound_vertex<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let q = SpecificVertexQuery::single(outbound_id);
    trans
        .set_vertex_properties(q.clone().property("foo"), &models::JsonValue::new(serde_json::Value::Bool(true)))
        .unwrap();
    trans.delete_vertices(q.clone()).unwrap();
    let v = trans.get_vertices(q).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Type::new("test_edge_type").unwrap();
    let count = trans
        .get_edge_count(outbound_id, Some(&t), models::EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_delete_a_valid_inbound_vertex<D: Datastore>(datastore: &mut D) {
    let (_, inbound_ids) = create_edges(datastore);
    let inbound_id = inbound_ids[0];
    let trans = datastore.transaction().unwrap();
    let q = SpecificVertexQuery::single(inbound_id);
    trans.delete_vertices(q.clone()).unwrap();
    let v = trans.get_vertices(q).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Type::new("test_edge_type").unwrap();
    let count = trans
        .get_edge_count(inbound_id, Some(&t), models::EdgeDirection::Inbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    trans
        .delete_vertices(SpecificVertexQuery::single(Uuid::default()))
        .unwrap();
}

pub fn should_get_a_vertex_count<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let v = models::Vertex::new(vertex_t);
    trans.create_vertex(&v).unwrap();
    let count = trans.get_vertex_count().unwrap();
    assert!(count >= 1);
}

fn create_vertices<T>(trans: &T) -> Vec<Uuid>
where
    T: Transaction,
{
    let t = models::Type::new("test_vertex_type").unwrap();

    let vertices = vec![
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t),
    ];

    for vertex in &vertices {
        trans.create_vertex(vertex).unwrap();
    }

    let mut vertex_ids: Vec<Uuid> = vertices.into_iter().map(|v| v.id).collect();
    vertex_ids.sort();
    vertex_ids
}
