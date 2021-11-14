use crate::{
    Datastore, EdgeKey, EdgeQueryExt, Identifier, SpecificEdgeQuery, SpecificVertexQuery, Transaction, Vertex,
    VertexQueryExt,
};

use serde_json::Value as JsonValue;
use uuid::Uuid;

pub fn should_handle_vertex_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let t = Identifier::new("test_edge_type").unwrap();
    let v = Vertex::new(t);
    trans.create_vertex(&v).unwrap();
    let q = SpecificVertexQuery::single(v.id).property("foo");

    // Check to make sure there's no initial value
    let result = trans.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    trans.set_vertex_properties(q.clone(), &JsonValue::Bool(true)).unwrap();
    let result = trans.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, JsonValue::Bool(true));

    // Set and get the value as false
    trans.set_vertex_properties(q.clone(), &JsonValue::Bool(false)).unwrap();
    let result = trans.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_vertex_properties(q.clone()).unwrap();
    let result = trans.get_vertex_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_vertex_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let t = Identifier::new("a_vertex").unwrap();
    let v1 = &Vertex::new(t.clone());
    let v2 = &Vertex::new(t.clone());
    let v3 = &Vertex::new(t);
    trans.create_vertex(v1).unwrap();
    trans.create_vertex(v2).unwrap();
    trans.create_vertex(v3).unwrap();
    let q1 = SpecificVertexQuery::single(v1.id);
    let q2 = SpecificVertexQuery::single(v2.id);
    let q3 = SpecificVertexQuery::single(v3.id);

    // Check to make sure there are no initial properties
    let all_result = trans.get_all_vertex_properties(q2.clone()).unwrap();
    assert_eq!(all_result.len(), 1);
    assert_eq!(all_result[0].props.len(), 0);

    // Set and get some properties for v2
    trans
        .set_vertex_properties(q2.clone().property("a"), &JsonValue::Bool(false))
        .unwrap();
    trans
        .set_vertex_properties(q2.clone().property("b"), &JsonValue::Bool(true))
        .unwrap();

    let result_1 = trans.get_all_vertex_properties(q1).unwrap();
    assert_eq!(result_1.len(), 1);
    assert_eq!(result_1[0].props.len(), 0);

    let result_2 = trans.get_all_vertex_properties(q2).unwrap();
    assert_eq!(result_2.len(), 1);
    assert_eq!(result_2[0].props.len(), 2);
    assert_eq!(result_2[0].props[0].name, "a");
    assert_eq!(result_2[0].props[0].value, JsonValue::Bool(false));
    assert_eq!(result_2[0].props[1].name, "b");
    assert_eq!(result_2[0].props[1].value, JsonValue::Bool(true));

    let result_3 = trans.get_all_vertex_properties(q3).unwrap();
    assert_eq!(result_3.len(), 1);
    assert_eq!(result_3[0].props.len(), 0);
}

pub fn should_not_set_invalid_vertex_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let q = SpecificVertexQuery::single(Uuid::default()).property("foo");
    trans.set_vertex_properties(q.clone(), &JsonValue::Null).unwrap();
    let result = trans.get_vertex_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_vertex_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let q = SpecificVertexQuery::single(Uuid::default()).property("foo");

    trans.delete_vertex_properties(q).unwrap();

    let v = Vertex::new(Identifier::new("foo").unwrap());
    trans.create_vertex(&v).unwrap();

    let q = SpecificVertexQuery::single(v.id).property("foo");
    trans.delete_vertex_properties(q).unwrap();
}

pub fn should_handle_edge_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = Identifier::new("test_edge_type").unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let key = EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    let q = SpecificEdgeQuery::single(key.clone()).property("edge-property");

    trans.create_edge(&key).unwrap();

    // Check to make sure there's no initial value
    let result = trans.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    trans.set_edge_properties(q.clone(), &JsonValue::Bool(true)).unwrap();
    let result = trans.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    assert_eq!(result[0].value, JsonValue::Bool(true));

    // Set and get the value as false
    trans.set_edge_properties(q.clone(), &JsonValue::Bool(false)).unwrap();
    let result = trans.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    assert_eq!(result[0].value, JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_edge_properties(q.clone()).unwrap();
    let result = trans.get_edge_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_edge_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let key = EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    let eq = SpecificEdgeQuery::single(key.clone());
    let q1 = eq.clone().property("edge-prop-1");
    let q2 = eq.clone().property("edge-prop-2");

    trans.create_edge(&key).unwrap();

    // Check to make sure there's no initial value
    let result = trans.get_all_edge_properties(eq.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 0);

    // Set and get the value as true
    trans.set_edge_properties(q1.clone(), &JsonValue::Bool(false)).unwrap();
    trans.set_edge_properties(q2.clone(), &JsonValue::Bool(true)).unwrap();

    let result = trans.get_all_edge_properties(eq.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 2);
    assert_eq!(result[0].props[0].name, "edge-prop-1");
    assert_eq!(result[0].props[0].value, JsonValue::Bool(false));
    assert_eq!(result[0].props[1].name, "edge-prop-2");
    assert_eq!(result[0].props[1].value, JsonValue::Bool(true));

    // Delete & check that they are deleted
    trans.delete_edge_properties(q1).unwrap();
    trans.delete_edge_properties(q2).unwrap();

    let result = trans.get_all_edge_properties(eq).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 0);
}

pub fn should_not_set_invalid_edge_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let key = EdgeKey::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    let q = SpecificEdgeQuery::single(key).property("bar");
    trans.set_edge_properties(q.clone(), &JsonValue::Null).unwrap();
    let result = trans.get_edge_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_edge_properties<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let key = EdgeKey::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    trans
        .delete_edge_properties(SpecificEdgeQuery::single(key).property("bar"))
        .unwrap();

    let outbound_v = Vertex::new(Identifier::new("foo").unwrap());
    let inbound_v = Vertex::new(Identifier::new("foo").unwrap());
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();

    let key = EdgeKey::new(outbound_v.id, Identifier::new("baz").unwrap(), inbound_v.id);
    trans.create_edge(&key).unwrap();
    trans
        .delete_edge_properties(SpecificEdgeQuery::single(key).property("bleh"))
        .unwrap();
}
