use crate::{Database, Datastore, Edge, Identifier, QueryExt, SpecificEdgeQuery, SpecificVertexQuery, Vertex};

use uuid::Uuid;

pub fn should_handle_vertex_properties<D: Datastore>(db: &Database<D>) {
    let t = Identifier::new("test_vertex_type").unwrap();
    let v = Vertex::new(t);
    db.create_vertex(&v).unwrap();
    let q = SpecificVertexQuery::single(v.id).property(Identifier::new("foo").unwrap());

    // Check to make sure there's no initial value
    let result = db.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_vertex_properties(q.clone(), serde_json::Value::Bool(true))
        .unwrap();
    let result = db.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, serde_json::Value::Bool(true));

    // Set and get the value as false
    db.set_vertex_properties(q.clone(), serde_json::Value::Bool(false))
        .unwrap();
    let result = db.get_vertex_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, serde_json::Value::Bool(false));

    // Delete & check that it's deleted
    db.delete_vertex_properties(q.clone()).unwrap();
    let result = db.get_vertex_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_vertex_properties<D: Datastore>(db: &Database<D>) {
    let t = Identifier::new("a_vertex").unwrap();
    let v1 = &Vertex::new(t.clone());
    let v2 = &Vertex::new(t.clone());
    let v3 = &Vertex::new(t);
    db.create_vertex(v1).unwrap();
    db.create_vertex(v2).unwrap();
    db.create_vertex(v3).unwrap();
    let q1 = SpecificVertexQuery::single(v1.id);
    let q2 = SpecificVertexQuery::single(v2.id);
    let q3 = SpecificVertexQuery::single(v3.id);

    // Check to make sure there are no initial properties
    let all_result = db.get_all_vertex_properties(q2.clone().into()).unwrap();
    assert_eq!(all_result.len(), 0);

    // Set and get some properties for v2
    db.set_vertex_properties(
        q2.clone().property(Identifier::new("a").unwrap()),
        serde_json::Value::Bool(false),
    )
    .unwrap();
    db.set_vertex_properties(
        q2.clone().property(Identifier::new("b").unwrap()),
        serde_json::Value::Bool(true),
    )
    .unwrap();

    let result_1 = db.get_all_vertex_properties(q1.into()).unwrap();
    assert_eq!(result_1.len(), 0);

    let result_2 = db.get_all_vertex_properties(q2.into()).unwrap();
    assert_eq!(result_2.len(), 1);
    assert_eq!(result_2[0].props.len(), 2);
    assert_eq!(result_2[0].props[0].name, Identifier::new("a").unwrap());
    assert_eq!(result_2[0].props[0].value, serde_json::Value::Bool(false));
    assert_eq!(result_2[0].props[1].name, Identifier::new("b").unwrap());
    assert_eq!(result_2[0].props[1].value, serde_json::Value::Bool(true));

    let result_3 = db.get_all_vertex_properties(q3.into()).unwrap();
    assert_eq!(result_3.len(), 0);
}

pub fn should_not_set_invalid_vertex_properties<D: Datastore>(db: &Database<D>) {
    let q = SpecificVertexQuery::single(Uuid::default()).property(Identifier::new("foo").unwrap());
    db.set_vertex_properties(q.clone(), serde_json::Value::Null).unwrap();
    let result = db.get_vertex_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_vertex_properties<D: Datastore>(db: &Database<D>) {
    let q = SpecificVertexQuery::single(Uuid::default()).property(Identifier::new("foo").unwrap());

    db.delete_vertex_properties(q).unwrap();

    let v = Vertex::new(Identifier::new("foo").unwrap());
    db.create_vertex(&v).unwrap();

    let q = SpecificVertexQuery::single(v.id).property(Identifier::new("foo").unwrap());
    db.delete_vertex_properties(q).unwrap();
}

pub fn should_handle_edge_properties<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let edge = Edge::new(outbound_v.id, edge_t, inbound_v.id);
    let q = SpecificEdgeQuery::single(edge.clone()).property(Identifier::new("edge-property").unwrap());

    db.create_edge(&edge).unwrap();

    // Check to make sure there's no initial value
    let result = db.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_edge_properties(q.clone(), serde_json::Value::Bool(true))
        .unwrap();
    let result = db.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].edge, edge);
    assert_eq!(result[0].value, serde_json::Value::Bool(true));

    // Set and get the value as false
    db.set_edge_properties(q.clone(), serde_json::Value::Bool(false))
        .unwrap();
    let result = db.get_edge_properties(q.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].edge, edge);
    assert_eq!(result[0].value, serde_json::Value::Bool(false));

    // Delete & check that it's deleted
    db.delete_edge_properties(q.clone()).unwrap();
    let result = db.get_edge_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_edge_properties<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let edge = Edge::new(outbound_v.id, edge_t, inbound_v.id);
    let eq = SpecificEdgeQuery::single(edge.clone());
    let q1 = eq.clone().property(Identifier::new("edge-prop-1").unwrap());
    let q2 = eq.clone().property(Identifier::new("edge-prop-2").unwrap());

    db.create_edge(&edge).unwrap();

    // Check to make sure there's no initial value
    let result = db.get_all_edge_properties(eq.clone().into()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_edge_properties(q1.clone(), serde_json::Value::Bool(false))
        .unwrap();
    db.set_edge_properties(q2.clone(), serde_json::Value::Bool(true))
        .unwrap();

    let result = db.get_all_edge_properties(eq.clone().into()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 2);
    assert_eq!(result[0].props[0].name, Identifier::new("edge-prop-1").unwrap());
    assert_eq!(result[0].props[0].value, serde_json::Value::Bool(false));
    assert_eq!(result[0].props[1].name, Identifier::new("edge-prop-2").unwrap());
    assert_eq!(result[0].props[1].value, serde_json::Value::Bool(true));

    // Delete & check that they are deleted
    db.delete_edge_properties(q1).unwrap();
    db.delete_edge_properties(q2).unwrap();

    let result = db.get_all_edge_properties(eq.into()).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_set_invalid_edge_properties<D: Datastore>(db: &Database<D>) {
    let edge = Edge::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    let q = SpecificEdgeQuery::single(edge).property(Identifier::new("bar").unwrap());
    db.set_edge_properties(q.clone(), serde_json::Value::Null).unwrap();
    let result = db.get_edge_properties(q).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_edge_properties<D: Datastore>(db: &Database<D>) {
    let edge = Edge::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    db.delete_edge_properties(SpecificEdgeQuery::single(edge).property(Identifier::new("bar").unwrap()))
        .unwrap();

    let outbound_v = Vertex::new(Identifier::new("foo").unwrap());
    let inbound_v = Vertex::new(Identifier::new("foo").unwrap());
    db.create_vertex(&outbound_v).unwrap();
    db.create_vertex(&inbound_v).unwrap();

    let edge = Edge::new(outbound_v.id, Identifier::new("baz").unwrap(), inbound_v.id);
    db.create_edge(&edge).unwrap();
    db.delete_edge_properties(SpecificEdgeQuery::single(edge).property(Identifier::new("bleh").unwrap()))
        .unwrap();
}
