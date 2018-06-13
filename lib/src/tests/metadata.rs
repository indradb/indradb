use super::super::{Datastore, EdgeKey, EdgeQuery, Transaction, Type, Vertex, VertexQuery};
use serde_json::Value as JsonValue;
use util::generate_random_secret;
use uuid::Uuid;

pub fn should_handle_vertex_metadata<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let t = Type::new("test_edge_type".to_string()).unwrap();
    let v = Vertex::new(t);
    trans.create_vertex(&v).unwrap();
    let name = format!("vertex-metadata-{}", generate_random_secret(8));
    let q = VertexQuery::Vertices { ids: vec![v.id] };

    // Check to make sure there's no initial value
    let result = trans.get_vertex_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    trans
        .set_vertex_metadata(&q, &name, &JsonValue::Bool(true))
        .unwrap();
    let result = trans.get_vertex_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, JsonValue::Bool(true));

    // Set and get the value as false
    trans
        .set_vertex_metadata(&q, &name, &JsonValue::Bool(false))
        .unwrap();
    let result = trans.get_vertex_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, v.id);
    assert_eq!(result[0].value, JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_vertex_metadata(&q, &name).unwrap();
    let result = trans.get_vertex_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_set_invalid_vertex_metadata<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![Uuid::default()],
    };
    trans
        .set_vertex_metadata(&q, "foo", &JsonValue::Null)
        .unwrap();
    let result = trans.get_vertex_metadata(&q, "foo").unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_vertex_metadata<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![Uuid::default()],
    };
    trans.delete_vertex_metadata(&q, "foo").unwrap();

    let v = Vertex::new(Type::new("foo".to_string()).unwrap());
    trans.create_vertex(&v).unwrap();

    let q = VertexQuery::Vertices { ids: vec![v.id] };
    trans.delete_vertex_metadata(&q, "foo").unwrap();
}

pub fn should_handle_edge_metadata<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = Type::new("test_edge_type".to_string()).unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t.clone());
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = Type::new("test_edge_type".to_string()).unwrap();
    let key = EdgeKey::new(outbound_v.id, edge_t.clone(), inbound_v.id);
    let q = EdgeQuery::Edges {
        keys: vec![key.clone()],
    };
    let name = format!("edge-metadata-{}", generate_random_secret(8));

    trans.create_edge(&key).unwrap();

    // Check to make sure there's no initial value
    let result = trans.get_edge_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    trans
        .set_edge_metadata(&q, &name, &JsonValue::Bool(true))
        .unwrap();
    let result = trans.get_edge_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    assert_eq!(result[0].value, JsonValue::Bool(true));

    // Set and get the value as false
    trans
        .set_edge_metadata(&q, &name, &JsonValue::Bool(false))
        .unwrap();
    let result = trans.get_edge_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].key, key);
    assert_eq!(result[0].value, JsonValue::Bool(false));

    // Delete & check that it's deleted
    trans.delete_edge_metadata(&q, &name).unwrap();
    let result = trans.get_edge_metadata(&q, &name).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_set_invalid_edge_metadata<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let q = EdgeQuery::Edges {
        keys: vec![
            EdgeKey::new(
                Uuid::default(),
                Type::new("foo".to_string()).unwrap(),
                Uuid::default(),
            ),
        ],
    };
    trans
        .set_edge_metadata(&q, "bar", &JsonValue::Null)
        .unwrap();
    let result = trans.get_edge_metadata(&q, "bar").unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_edge_metadata<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let q = EdgeQuery::Edges {
        keys: vec![
            EdgeKey::new(
                Uuid::default(),
                Type::new("foo".to_string()).unwrap(),
                Uuid::default(),
            ),
        ],
    };
    trans.delete_edge_metadata(&q, "bar").unwrap();

    let outbound_v = Vertex::new(Type::new("foo".to_string()).unwrap());
    let inbound_v = Vertex::new(Type::new("foo".to_string()).unwrap());
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();

    let key = EdgeKey::new(
        outbound_v.id,
        Type::new("baz".to_string()).unwrap(),
        inbound_v.id,
    );
    trans.create_edge(&key).unwrap();
    trans
        .delete_edge_metadata(&EdgeQuery::Edges { keys: vec![key] }, "bleh")
        .unwrap();
}
