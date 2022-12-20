use crate::{
    Datastore, EdgeKey, Identifier, SpecificEdgeQuery, SpecificVertexQuery, Vertex, QueryExt,
};

use uuid::Uuid;

pub fn should_handle_vertex_properties<D: Datastore>(datastore: &D) {
    let t = Identifier::new("test_vertex_type").unwrap();
    let v = Vertex::new(t);
    datastore.create_vertex(&v).unwrap();
    let q = SpecificVertexQuery::single(v.id).property(Identifier::new("foo").unwrap());

    // Check to make sure there's no initial value
    let result = datastore.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    datastore
        .set_vertex_properties(q.clone(), serde_json::Value::Bool(true))
        .unwrap();
    let result = datastore.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, serde_json::Value::Bool(true));

    // Set and get the value as false
    datastore
        .set_vertex_properties(q.clone(), serde_json::Value::Bool(false))
        .unwrap();
    let result = datastore.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, serde_json::Value::Bool(false));

    // Delete & check that it's deleted
    datastore.delete_vertex_properties(q.clone()).unwrap();
    let result = datastore.get_vertex_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_vertex_properties<D: Datastore>(datastore: &D) {
    let t = Identifier::new("a_vertex").unwrap();
    let v1 = &Vertex::new(t.clone());
    let v2 = &Vertex::new(t.clone());
    let v3 = &Vertex::new(t);
    datastore.create_vertex(v1).unwrap();
    datastore.create_vertex(v2).unwrap();
    datastore.create_vertex(v3).unwrap();
    let q1 = SpecificVertexQuery::single(v1.id);
    let q2 = SpecificVertexQuery::single(v2.id);
    let q3 = SpecificVertexQuery::single(v3.id);

    // Check to make sure there are no initial properties
    let all_result = datastore.get_all_vertex_properties(q2.clone().into()).unwrap();
    assert_eq!(all_result.len(), 1);
    assert_eq!(all_result[0].props.len(), 0);

    // Set and get some properties for v2
    datastore
        .set_vertex_properties(
            q2.clone().property(Identifier::new("a").unwrap()),
            serde_json::Value::Bool(false),
        )
        .unwrap();
    datastore
        .set_vertex_properties(
            q2.clone().property(Identifier::new("b").unwrap()),
            serde_json::Value::Bool(true),
        )
        .unwrap();

    let result_1 = datastore.get_all_vertex_properties(q1.into()).unwrap();
    assert_eq!(result_1.len(), 1);
    assert_eq!(result_1[0].props.len(), 0);

    let result_2 = datastore.get_all_vertex_properties(q2.into()).unwrap();
    assert_eq!(result_2.len(), 1);
    assert_eq!(result_2[0].props.len(), 2);
    assert_eq!(result_2[0].props[0].name, Identifier::new("a").unwrap());
    assert_eq!(result_2[0].props[0].value, serde_json::Value::Bool(false));
    assert_eq!(result_2[0].props[1].name, Identifier::new("b").unwrap());
    assert_eq!(result_2[0].props[1].value, serde_json::Value::Bool(true));

    let result_3 = datastore.get_all_vertex_properties(q3.into()).unwrap();
    assert_eq!(result_3.len(), 1);
    assert_eq!(result_3[0].props.len(), 0);
}

pub fn should_not_set_invalid_vertex_properties<D: Datastore>(datastore: &D) {
    let q = SpecificVertexQuery::single(Uuid::default()).property(Identifier::new("foo").unwrap());
    datastore
        .set_vertex_properties(q.clone(), serde_json::Value::Null)
        .unwrap();
    let result = datastore.get_vertex_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_vertex_properties<D: Datastore>(datastore: &D) {
    let q = SpecificVertexQuery::single(Uuid::default()).property(Identifier::new("foo").unwrap());

    datastore.delete_vertex_properties(q).unwrap();

    let v = Vertex::new(Identifier::new("foo").unwrap());
    datastore.create_vertex(&v).unwrap();

    let q = SpecificVertexQuery::single(v.id).property(Identifier::new("foo").unwrap());
    datastore.delete_vertex_properties(q).unwrap();
}

pub fn should_handle_edge_properties<D: Datastore>(datastore: &D) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    datastore.create_vertex(&inbound_v).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let key = EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    let q = SpecificEdgeQuery::single(key.clone()).property(Identifier::new("edge-property").unwrap());

    datastore.create_edge(&key).unwrap();

    // Check to make sure there's no initial value
    let result = datastore.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    datastore
        .set_edge_properties(q.clone(), serde_json::Value::Bool(true))
        .unwrap();
    let result = datastore.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    assert_eq!(result[0].value, serde_json::Value::Bool(true));

    // Set and get the value as false
    datastore
        .set_edge_properties(q.clone(), serde_json::Value::Bool(false))
        .unwrap();
    let result = datastore.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    assert_eq!(result[0].value, serde_json::Value::Bool(false));

    // Delete & check that it's deleted
    datastore.delete_edge_properties(q.clone()).unwrap();
    let result = datastore.get_edge_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_edge_properties<D: Datastore>(datastore: &D) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    datastore.create_vertex(&inbound_v).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let key = EdgeKey::new(outbound_v.id, edge_t, inbound_v.id);
    let eq = SpecificEdgeQuery::single(key.clone());
    let q1 = eq.clone().property(Identifier::new("edge-prop-1").unwrap());
    let q2 = eq.clone().property(Identifier::new("edge-prop-2").unwrap());

    datastore.create_edge(&key).unwrap();

    // Check to make sure there's no initial value
    let result = datastore.get_all_edge_properties(eq.clone().into()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 0);

    // Set and get the value as true
    datastore
        .set_edge_properties(q1.clone(), serde_json::Value::Bool(false))
        .unwrap();
    datastore
        .set_edge_properties(q2.clone(), serde_json::Value::Bool(true))
        .unwrap();

    let result = datastore.get_all_edge_properties(eq.clone().into()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 2);
    assert_eq!(result[0].props[0].name, Identifier::new("edge-prop-1").unwrap());
    assert_eq!(result[0].props[0].value, serde_json::Value::Bool(false));
    assert_eq!(result[0].props[1].name, Identifier::new("edge-prop-2").unwrap());
    assert_eq!(result[0].props[1].value, serde_json::Value::Bool(true));

    // Delete & check that they are deleted
    datastore.delete_edge_properties(q1).unwrap();
    datastore.delete_edge_properties(q2).unwrap();

    let result = datastore.get_all_edge_properties(eq.into()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 0);
}

pub fn should_not_set_invalid_edge_properties<D: Datastore>(datastore: &D) {
    let key = EdgeKey::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    let q = SpecificEdgeQuery::single(key).property(Identifier::new("bar").unwrap());
    datastore
        .set_edge_properties(q.clone(), serde_json::Value::Null)
        .unwrap();
    let result = datastore.get_edge_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_edge_properties<D: Datastore>(datastore: &D) {
    let key = EdgeKey::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    datastore
        .delete_edge_properties(SpecificEdgeQuery::single(key).property(Identifier::new("bar").unwrap()))
        .unwrap();

    let outbound_v = Vertex::new(Identifier::new("foo").unwrap());
    let inbound_v = Vertex::new(Identifier::new("foo").unwrap());
    datastore.create_vertex(&outbound_v).unwrap();
    datastore.create_vertex(&inbound_v).unwrap();

    let key = EdgeKey::new(outbound_v.id, Identifier::new("baz").unwrap(), inbound_v.id);
    datastore.create_edge(&key).unwrap();
    datastore
        .delete_edge_properties(SpecificEdgeQuery::single(key).property(Identifier::new("bleh").unwrap()))
        .unwrap();
}
