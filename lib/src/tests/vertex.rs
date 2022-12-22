use std::collections::HashSet;

use super::util::{create_edge_from, create_edges};
use crate::{models, Datastore, QueryExt, RangeVertexQuery, SpecificVertexQuery};
use crate::compat::DatastoreV3CompatExt;

use uuid::Uuid;

pub fn should_create_vertex_from_type<D: DatastoreV3CompatExt>(datastore: &D) {
    let t = models::Identifier::new("test_vertex_type").unwrap();
    datastore.create_vertex_from_type(t).unwrap();
}

pub fn should_get_range_vertices<D: DatastoreV3CompatExt>(datastore: &D) {
    let mut inserted_ids = create_vertices(datastore);

    let range = datastore.get_vertices(RangeVertexQuery::new().into()).unwrap();

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

pub fn should_get_no_vertices_with_zero_limit<D: DatastoreV3CompatExt>(datastore: &D) {
    create_vertices(datastore);
    let range = datastore.get_vertices(RangeVertexQuery::new().limit(0).into()).unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_range_vertices_out_of_range<D: DatastoreV3CompatExt>(datastore: &D) {
    create_vertices(datastore);
    let range = datastore
        .get_vertices(
            RangeVertexQuery::new()
                .start_id(Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap())
                .into(),
        )
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_no_vertices_with_type_filter<D: DatastoreV3CompatExt>(datastore: &D) {
    let type_filter = models::Identifier::new("foo").unwrap();
    create_vertices(datastore);
    let range = datastore
        .get_vertices(RangeVertexQuery::new().t(type_filter).into())
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_single_vertex<D: DatastoreV3CompatExt>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let vertex = models::Vertex::new(vertex_t);
    datastore.create_vertex(&vertex).unwrap();
    let range = datastore
        .get_vertices(SpecificVertexQuery::single(vertex.id).into())
        .unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, vertex.id);
    assert_eq!(range[0].t.0, "test_vertex_type");
}

pub fn should_get_single_vertex_nonexisting<D: DatastoreV3CompatExt>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let vertex = models::Vertex::new(vertex_t);
    datastore.create_vertex(&vertex).unwrap();
    let range = datastore
        .get_vertices(SpecificVertexQuery::single(Uuid::default()).into())
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_vertices<D: DatastoreV3CompatExt>(datastore: &D) {
    let mut inserted_ids = create_vertices(datastore);

    let range = datastore
        .get_vertices(
            SpecificVertexQuery::new(vec![inserted_ids[0], inserted_ids[1], inserted_ids[2], Uuid::default()]).into(),
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

pub fn should_get_vertices_piped<D: DatastoreV3CompatExt>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();

    let v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&v).unwrap();
    let inserted_id = create_edge_from(datastore, v.id);

    // This query should get `inserted_id`
    let query_1 = SpecificVertexQuery::single(v.id)
        .outbound()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .limit(1);
    let range = datastore.get_vertices(query_1.clone().into()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get `inserted_id`
    let query_2 = SpecificVertexQuery::single(v.id)
        .outbound()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .limit(1)
        .t(models::Identifier::new("test_inbound_vertex_type").unwrap());
    let range = datastore.get_vertices(query_2.into()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get nothing
    let query_3 = SpecificVertexQuery::single(v.id)
        .outbound()
        .limit(1)
        .t(edge_t.clone())
        .inbound()
        .limit(1)
        .t(models::Identifier::new("foo").unwrap());
    let range = datastore.get_vertices(query_3.into()).unwrap();
    assert_eq!(range.len(), 0);

    // This query should get `v`
    let query_4 = query_1.inbound().limit(1).t(edge_t).outbound().limit(1);
    let range = datastore.get_vertices(query_4.into()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0], v);
}

pub fn should_delete_a_valid_outbound_vertex<D: DatastoreV3CompatExt>(datastore: &D) {
    let (outbound_id, _) = create_edges(datastore);
    let q = SpecificVertexQuery::single(outbound_id);
    datastore
        .set_vertex_properties(
            q.clone().property(models::Identifier::new("foo").unwrap()),
            serde_json::Value::Bool(true),
        )
        .unwrap();
    datastore.delete_vertices(q.clone().into()).unwrap();
    let v = datastore.get_vertices(q.into()).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = datastore
        .get_edge_count(outbound_id, Some(&t), models::EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_delete_a_valid_inbound_vertex<D: DatastoreV3CompatExt>(datastore: &D) {
    let (_, inbound_ids) = create_edges(datastore);
    let inbound_id = inbound_ids[0];
    let q = SpecificVertexQuery::single(inbound_id);
    datastore.delete_vertices(q.clone().into()).unwrap();
    let v = datastore.get_vertices(q.into()).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Identifier::new("test_edge_type").unwrap();
    let count = datastore
        .get_edge_count(inbound_id, Some(&t), models::EdgeDirection::Inbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D: DatastoreV3CompatExt>(datastore: &D) {
    datastore
        .delete_vertices(SpecificVertexQuery::single(Uuid::default()).into())
        .unwrap();
}

pub fn should_get_a_vertex_count<D: DatastoreV3CompatExt>(datastore: &D) {
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let v = models::Vertex::new(vertex_t);
    datastore.create_vertex(&v).unwrap();
    let count = datastore.get_vertex_count().unwrap();
    assert!(count >= 1);
}

fn create_vertices<D: DatastoreV3CompatExt>(datastore: &D) -> Vec<Uuid> {
    let t = models::Identifier::new("test_vertex_type").unwrap();

    let vertices = vec![
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t.clone()),
        models::Vertex::new(t),
    ];

    for vertex in &vertices {
        datastore.create_vertex(vertex).unwrap();
    }

    let mut vertex_ids: Vec<Uuid> = vertices.into_iter().map(|v| v.id).collect();
    vertex_ids.sort();
    vertex_ids
}
