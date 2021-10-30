use uuid::Uuid;
use crate::{Datastore, models, Error, Transaction, VertexQueryExt, EdgeQueryExt};

fn setup_vertex_with_indexed_property<D: Datastore>(datastore: &mut D) -> Uuid {
    datastore.index_vertex_property("foo").unwrap();
    let trans = datastore.transaction().unwrap();
    let v = models::Vertex::new(models::Type::new("test_vertex_type").unwrap());
    trans.create_vertex(&v).unwrap();
    let q = models::SpecificVertexQuery::single(v.id).property("foo");
    trans.set_vertex_properties(q.clone(), &models::JsonValue::new(serde_json::Value::Bool(true))).unwrap();
    v.id
}

fn setup_edge_with_indexed_property<D: Datastore>(datastore: &mut D) -> (Uuid, Uuid) {
    datastore.index_edge_property("foo").unwrap();
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    let q = models::SpecificEdgeQuery::single(key.clone()).property("foo");
    trans.create_edge(&key).unwrap();
    trans.set_edge_properties(q.clone(), &models::JsonValue::new(serde_json::Value::Bool(true))).unwrap();
    (outbound_v.id, inbound_v.id)
}

pub fn should_not_query_unindexed_vertex_property<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let result = trans.get_vertices(models::PropertyPresenceVertexQuery::new("foo"));
    match result {
        Err(Error::NotIndexed) => (),
        _ => assert!(false, "unexpected result: {:?}", result),
    }
}

pub fn should_not_query_unindexed_edge_property<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let result = trans.get_edges(models::PropertyPresenceEdgeQuery::new("foo"));
    match result {
        Err(Error::NotIndexed) => (),
        _ => assert!(false, "unexpected result: {:?}", result),
    }
}

pub fn should_index_existing_vertex_property<D: Datastore>(datastore: &mut D) {
    // Setup
    let trans = datastore.transaction().unwrap();
    let v = models::Vertex::new(models::Type::new("test_vertex_type").unwrap());
    trans.create_vertex(&v).unwrap();
    let q = models::SpecificVertexQuery::single(v.id);
    trans.set_vertex_properties(q.clone().property("existing-vertex-property"), &models::JsonValue::new(serde_json::Value::Bool(true))).unwrap();

    // Index property
    datastore.index_vertex_property("existing-vertex-property").unwrap();

    // Get the vertex
    let result = trans.get_vertices(models::PropertyPresenceVertexQuery::new("existing-vertex-property")).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);

    // Get the vertex with a piped query
    let result = trans.get_vertices(q.with_property("existing-vertex-property")).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
}

pub fn should_index_existing_edge_property<D: Datastore>(datastore: &mut D) {
    // Setup
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    let q = models::SpecificEdgeQuery::single(key.clone());
    trans.create_edge(&key).unwrap();
    trans.set_edge_properties(q.clone().property("existing-edge-property"), &models::JsonValue::new(serde_json::Value::Bool(true))).unwrap();

    // Index property
    datastore.index_edge_property("existing-edge-property").unwrap();

    // Get the edge
    let result = trans.get_edges(models::PropertyPresenceEdgeQuery::new("existing-edge-property")).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);

    // Get the edge with a piped query
    let result = trans.get_edges(q.with_property("existing-edge-property")).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
}

pub fn should_delete_indexed_vertex_property<D: Datastore>(datastore: &mut D) {
    let id = setup_vertex_with_indexed_property(datastore);
    let trans = datastore.transaction().unwrap();
    let q = models::SpecificVertexQuery::single(id);
    trans.delete_vertices(q.clone()).unwrap();
    let result = trans.get_vertices(models::PropertyPresenceVertexQuery::new("foo")).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_delete_indexed_edge_property<D: Datastore>(datastore: &mut D) {
    let (inbound_id, outbound_id) = setup_edge_with_indexed_property(datastore);
    let trans = datastore.transaction().unwrap();
    let key = models::EdgeKey::new(outbound_id, models::Type::new("test_edge_type").unwrap(), inbound_id);
    let q = models::SpecificEdgeQuery::single(key);
    trans.delete_edges(q.clone()).unwrap();
    let result = trans.get_edges(models::PropertyPresenceEdgeQuery::new("foo")).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_update_indexed_vertex_property<D: Datastore>(datastore: &mut D) {
    let json_true = models::JsonValue::new(serde_json::Value::Bool(true));
    let json_false = models::JsonValue::new(serde_json::Value::Bool(false));

    let id = setup_vertex_with_indexed_property(datastore);
    let trans = datastore.transaction().unwrap();
    let q = models::SpecificVertexQuery::single(id);
    trans.set_vertex_properties(q.clone().property("foo"), &json_false).unwrap();

    // property foo should not be the old value
    let result = trans.get_vertices(models::PropertyValueVertexQuery::new("foo", json_true.clone(), true)).unwrap();
    assert_eq!(result.len(), 0);
    let result = trans.get_vertices(models::PropertyValueVertexQuery::new("foo", json_true.clone(), false)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = trans.get_vertices(q.clone().with_property_equal_to("foo", json_true.clone())).unwrap();
    assert_eq!(result.len(), 0);
    let result = trans.get_vertices(q.clone().with_property_not_equal_to("foo", json_true)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);

    // property foo should be the new value
    let result = trans.get_vertices(models::PropertyValueVertexQuery::new("foo", json_false.clone(), true)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = trans.get_vertices(models::PropertyValueVertexQuery::new("foo", json_false.clone(), false)).unwrap();
    assert_eq!(result.len(), 0);
    let result = trans.get_vertices(q.clone().with_property_equal_to("foo", json_false.clone())).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = trans.get_vertices(q.with_property_not_equal_to("foo", json_false)).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_update_indexed_edge_property<D: Datastore>(datastore: &mut D) {
    let json_true = models::JsonValue::new(serde_json::Value::Bool(true));
    let json_false = models::JsonValue::new(serde_json::Value::Bool(false));

    let (inbound_id, outbound_id) = setup_edge_with_indexed_property(datastore);
    let trans = datastore.transaction().unwrap();
    let key = models::EdgeKey::new(outbound_id, models::Type::new("test_edge_type").unwrap(), inbound_id);
    let q = models::SpecificEdgeQuery::single(key.clone());
    trans.set_edge_properties(q.clone().property("foo"), &json_false).unwrap();

    // property foo should not be the old value
    let result = trans.get_edges(models::PropertyValueEdgeQuery::new("foo", json_true.clone(), true)).unwrap();
    assert_eq!(result.len(), 0);
    let result = trans.get_edges(models::PropertyValueEdgeQuery::new("foo", json_true.clone(), false)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key.clone());
    let result = trans.get_edges(q.clone().with_property_equal_to("foo", json_true.clone())).unwrap();
    assert_eq!(result.len(), 0);
    let result = trans.get_edges(q.clone().with_property_not_equal_to("foo", json_true)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key.clone());

    // property foo should be the new value
    let result = trans.get_edges(models::PropertyValueEdgeQuery::new("foo", json_false.clone(), true)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    let result = trans.get_edges(models::PropertyValueEdgeQuery::new("foo", json_false.clone(), false)).unwrap();
    assert_eq!(result.len(), 0);
    let result = trans.get_edges(q.clone().with_property_equal_to("foo", json_false.clone())).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    let result = trans.get_edges(q.with_property_not_equal_to("foo", json_false)).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_query_indexed_vertex_property_empty<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    datastore.index_vertex_property("vertex-property").unwrap();
    let result = trans.get_vertices(models::PropertyPresenceVertexQuery::new("vertex-property")).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_query_indexed_edge_property_empty<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    datastore.index_edge_property("edge-property").unwrap();
    let result = trans.get_edges(models::PropertyPresenceEdgeQuery::new("edge-property")).unwrap();
    assert_eq!(result.len(), 0);
}
