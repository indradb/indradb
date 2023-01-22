use super::util;
use crate::{expect_err, models, Database, Datastore, Error, QueryExt};

fn setup_vertex_with_indexed_property<D: Datastore>(db: &Database<D>, property_name: &models::Identifier) -> u64 {
    db.index_property(property_name.clone()).unwrap();
    let v = models::Vertex::new(models::Identifier::new("test_vertex_type").unwrap());
    db.create_vertex(&v).unwrap();
    let q = models::SpecificVertexQuery::single(v.id);
    db.set_properties(q, property_name.clone(), serde_json::Value::Bool(true))
        .unwrap();
    v.id
}

fn setup_edge_with_indexed_property<D: Datastore>(
    db: &Database<D>,
    property_name: &models::Identifier,
) -> models::Edge {
    db.index_property(property_name.clone()).unwrap();
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t, inbound_v.id);
    let q = models::SpecificEdgeQuery::single(edge.clone());
    db.create_edge(&edge).unwrap();
    db.set_properties(q, property_name.clone(), serde_json::Value::Bool(true))
        .unwrap();
    edge
}

pub fn should_not_query_unindexed_vertex_property<D: Datastore>(db: &Database<D>) {
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyPresenceQuery::new(models::Identifier::new("foo").unwrap()),
    );
    expect_err!(result, Error::NotIndexed);
}

pub fn should_not_query_unindexed_edge_property<D: Datastore>(db: &Database<D>) {
    let result = util::get_edges(
        db,
        models::EdgeWithPropertyPresenceQuery::new(models::Identifier::new("foo").unwrap()),
    );
    expect_err!(result, Error::NotIndexed);
}

pub fn should_index_existing_vertex_property<D: Datastore>(db: &Database<D>) {
    // Setup
    let property_name = models::Identifier::new("existing-vertex-property").unwrap();
    let other_property_name = models::Identifier::new("some-other-property").unwrap();
    let v = models::Vertex::new(models::Identifier::new("test_vertex_type").unwrap());
    db.create_vertex(&v).unwrap();
    let q = models::SpecificVertexQuery::single(v.id);
    db.set_properties(q.clone(), property_name.clone(), serde_json::Value::Bool(true))
        .unwrap();

    // Index property
    db.index_property(property_name.clone()).unwrap();

    // Get the vertex
    let result = util::get_vertices(db, models::VertexWithPropertyPresenceQuery::new(property_name.clone())).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);

    // Get the vertex with piped queries
    let result = util::get_vertices(db, q.clone().with_property(property_name.clone()).unwrap()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    let result = util::get_vertices(db, q.clone().without_property(property_name.clone()).unwrap()).unwrap();
    assert!(result.is_empty());

    // Check against another property
    let result = util::get_vertices(db, q.clone().without_property(other_property_name.clone()).unwrap());
    expect_err!(result, Error::NotIndexed);
    db.index_property(other_property_name.clone()).unwrap();
    let result = util::get_vertices(db, q.without_property(other_property_name).unwrap()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
}

pub fn should_index_existing_edge_property<D: Datastore>(db: &Database<D>) {
    // Setup
    let property_name = models::Identifier::new("existing-edge-property").unwrap();
    let other_property_name = models::Identifier::new("some-other-property").unwrap();
    let vertex_t = models::Identifier::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t, inbound_v.id);
    let q = models::SpecificEdgeQuery::single(edge.clone());
    db.create_edge(&edge).unwrap();
    db.set_properties(q.clone(), property_name.clone(), serde_json::Value::Bool(true))
        .unwrap();

    // Index property
    db.index_property(property_name.clone()).unwrap();

    // Get the edge
    let result = util::get_edges(db, models::EdgeWithPropertyPresenceQuery::new(property_name.clone())).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);

    // Get the edge with a piped query
    let result = util::get_edges(db, q.clone().with_property(property_name.clone()).unwrap()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
    let result = util::get_edges(db, q.clone().without_property(property_name.clone()).unwrap()).unwrap();
    assert!(result.is_empty());

    // Check against another property
    let result = util::get_edges(db, q.clone().without_property(other_property_name.clone()).unwrap());
    expect_err!(result, Error::NotIndexed);
    db.index_property(other_property_name.clone()).unwrap();
    let result = util::get_edges(db, q.without_property(other_property_name).unwrap()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
}

pub fn should_delete_indexed_vertex_property<D: Datastore>(db: &Database<D>) {
    let property_name = models::Identifier::new("deletable-vertex-property").unwrap();
    let id = setup_vertex_with_indexed_property(db, &property_name);
    let q = models::SpecificVertexQuery::single(id);
    db.delete(q.clone()).unwrap();
    let result = util::get_vertices(db, models::VertexWithPropertyPresenceQuery::new(property_name)).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_delete_indexed_edge_property<D: Datastore>(db: &Database<D>) {
    let property_name = models::Identifier::new("deletable-edge-property").unwrap();
    let edge = setup_edge_with_indexed_property(db, &property_name);
    let q = models::SpecificEdgeQuery::single(edge);
    db.delete(q.clone()).unwrap();
    let result = util::get_edges(db, models::EdgeWithPropertyPresenceQuery::new(property_name)).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_update_indexed_vertex_property<D: Datastore>(db: &Database<D>) {
    let json_true = serde_json::Value::Bool(true);
    let json_false = serde_json::Value::Bool(false);
    let property_name = models::Identifier::new("updateable-vertex-property").unwrap();

    // Ensure errors happen when attempting to query before the property is indexed
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyValueQuery::new(property_name.clone(), json_true.clone()),
    );
    expect_err!(result, Error::NotIndexed);

    let id = setup_vertex_with_indexed_property(db, &property_name);
    let q = models::SpecificVertexQuery::single(id);
    db.set_properties(q.clone(), property_name.clone(), json_false.clone())
        .unwrap();

    // property foo should not be the old value
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyValueQuery::new(property_name.clone(), json_true.clone()),
    )
    .unwrap();
    assert_eq!(result.len(), 0);
    let result = util::get_vertices(
        db,
        q.clone()
            .with_property_equal_to(property_name.clone(), json_true.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 0);
    let result = util::get_vertices(
        db,
        q.clone()
            .with_property_not_equal_to(property_name.clone(), json_true.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);

    // property foo should be the new value
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyValueQuery::new(property_name.clone(), json_false.clone()),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = util::get_vertices(
        db,
        q.clone()
            .with_property_equal_to(property_name.clone(), json_false.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = util::get_vertices(
        db,
        q.with_property_not_equal_to(property_name.clone(), json_false.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_update_indexed_edge_property<D: Datastore>(db: &Database<D>) {
    let json_true = serde_json::Value::Bool(true);
    let json_false = serde_json::Value::Bool(false);
    let property_name = models::Identifier::new("updateable-edge-property").unwrap();

    let result = util::get_edges(
        db,
        models::EdgeWithPropertyValueQuery::new(property_name.clone(), json_true.clone()),
    );
    expect_err!(result, Error::NotIndexed);

    let edge = setup_edge_with_indexed_property(db, &property_name);
    let q = models::SpecificEdgeQuery::single(edge.clone());
    db.set_properties(q.clone(), property_name.clone(), json_false.clone())
        .unwrap();

    // property foo should not be the old value
    let result = util::get_edges(
        db,
        models::EdgeWithPropertyValueQuery::new(property_name.clone(), json_true.clone()),
    )
    .unwrap();
    assert_eq!(result.len(), 0);
    let result = util::get_edges(
        db,
        q.clone()
            .with_property_equal_to(property_name.clone(), json_true.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 0);
    let result = util::get_edges(
        db,
        q.clone()
            .with_property_not_equal_to(property_name.clone(), json_true)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge.clone());

    // property foo should be the new value
    let result = util::get_edges(
        db,
        models::EdgeWithPropertyValueQuery::new(property_name.clone(), json_false.clone()),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
    let result = util::get_edges(
        db,
        q.clone()
            .with_property_equal_to(property_name.clone(), json_false.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
    let result = util::get_edges(
        db,
        q.with_property_not_equal_to(property_name.clone(), json_false).unwrap(),
    )
    .unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_query_indexed_vertex_property_empty<D: Datastore>(db: &Database<D>) {
    let property_name = models::Identifier::new("queryable-vertex-property").unwrap();
    db.index_property(property_name.clone()).unwrap();
    let result = util::get_vertices(db, models::VertexWithPropertyPresenceQuery::new(property_name)).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_query_indexed_edge_property_empty<D: Datastore>(db: &Database<D>) {
    let property_name = models::Identifier::new("queryable-edge-property").unwrap();
    db.index_property(property_name.clone()).unwrap();
    let result = util::get_edges(db, models::EdgeWithPropertyPresenceQuery::new(property_name)).unwrap();
    assert_eq!(result.len(), 0);
}
