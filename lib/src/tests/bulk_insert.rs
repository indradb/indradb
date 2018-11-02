use super::super::{Datastore, BulkInsertItem, VertexQuery, EdgeQuery, EdgeKey, Transaction, Type, Vertex};
use serde_json::Value as JsonValue;
use chrono::offset::Utc;
use chrono::Timelike;

pub fn should_bulk_insert<D: Datastore>(datastore: &mut D) {
    let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
    let outbound_v = Vertex::new(vertex_t.clone());
    let inbound_v = Vertex::new(vertex_t.clone());

    let items = vec![
        BulkInsertItem::Vertex(outbound_v.clone()),
        BulkInsertItem::Vertex(inbound_v.clone()),
    ];

    datastore.bulk_insert(items.into_iter()).unwrap();

    // Record the start and end time. Round off the the nanoseconds off the
    // start time, since some implementations may not have that level of
    // accuracy.
    let start_time = Utc::now().with_nanosecond(0).unwrap();
    let edge_t = Type::new("test_edge_type".to_string()).unwrap();
    let key = EdgeKey::new(outbound_v.id, edge_t.clone(), inbound_v.id);

    let items = vec![
        BulkInsertItem::Edge(key.clone()),
        BulkInsertItem::VertexProperty(outbound_v.id, "vertex_property_name".to_string(), JsonValue::String("vertex_property_value".to_string())),
        BulkInsertItem::EdgeProperty(key.clone(), "edge_property_name".to_string(), JsonValue::String("edge_property_value".to_string())),
    ];

    datastore.bulk_insert(items.into_iter()).unwrap();

    let end_time = Utc::now();

    let trans = datastore.transaction().unwrap();

    let vertices = trans.get_vertices(&VertexQuery::Vertices {
        ids: vec![outbound_v.id, inbound_v.id]
    }).unwrap();

    assert_eq!(vertices.len(), 2);
    assert_eq!(vertices[0].id, outbound_v.id);
    assert_eq!(vertices[0].t, outbound_v.t);
    assert_eq!(vertices[1].id, inbound_v.id);
    assert_eq!(vertices[1].t, inbound_v.t);

    let edges = trans.get_edges(&EdgeQuery::Edges {
        keys: vec![key.clone()],
    }).unwrap();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].key.outbound_id, outbound_v.id);
    assert_eq!(edges[0].key.t, edge_t);
    assert_eq!(edges[0].key.inbound_id, inbound_v.id);
    assert!(edges[0].created_datetime >= start_time);
    assert!(edges[0].created_datetime <= end_time);

    let vertex_properties = trans.get_vertex_properties(&VertexQuery::Vertices {
        ids: vec![outbound_v.id]
    }, "vertex_property_name").unwrap();

    assert_eq!(vertex_properties.len(), 1);
    assert_eq!(vertex_properties[0].id, outbound_v.id);
    assert_eq!(vertex_properties[0].value, JsonValue::String("vertex_property_value".to_string()));

    let edge_properties = trans.get_edge_properties(&EdgeQuery::Edges {
        keys: vec![key.clone()],
    }, "edge_property_name").unwrap();

    assert_eq!(edge_properties.len(), 1);
    assert_eq!(edge_properties[0].key, key);
    assert_eq!(edge_properties[0].value, JsonValue::String("edge_property_value".to_string()));
}

pub fn should_not_bulk_insert_an_invalid_vertex<D: Datastore>(datastore: &mut D) {
    let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
    let vertex = Vertex::new(vertex_t.clone());

    let trans = datastore.transaction().unwrap();
    assert!(trans.create_vertex(&vertex).unwrap());

    let items = vec![BulkInsertItem::Vertex(vertex.clone())];
    assert!(datastore.bulk_insert(items.into_iter()).is_err());
}

pub fn should_not_bulk_insert_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let vertex_t = Type::new("test_vertex_type".to_string()).unwrap();
    let v1 = Vertex::new(vertex_t.clone());
    let v2 = Vertex::new(vertex_t.clone());

    let trans = datastore.transaction().unwrap();
    assert!(trans.create_vertex(&v1).unwrap());

    let edge_t = Type::new("test_edge_type".to_string()).unwrap();

    let items = vec![BulkInsertItem::Edge(EdgeKey::new(v1.id, edge_t.clone(), v2.id))];
    assert!(datastore.bulk_insert(items.into_iter()).is_err());

    let items = vec![BulkInsertItem::Edge(EdgeKey::new(v2.id, edge_t.clone(), v1.id))];
    assert!(datastore.bulk_insert(items.into_iter()).is_err());
}
