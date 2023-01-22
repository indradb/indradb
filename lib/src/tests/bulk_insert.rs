use super::util;
use crate::{
    BulkInsertItem, Database, Datastore, Edge, Identifier, QueryExt, SpecificEdgeQuery, SpecificVertexQuery, Vertex,
};

pub fn should_bulk_insert<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_v = Vertex::new(1_000_000, vertex_t);
    let inbound_v = Vertex::new(1_000_001, vertex_t);

    let items = vec![BulkInsertItem::Vertex(outbound_v), BulkInsertItem::Vertex(inbound_v)];

    db.bulk_insert(items).unwrap();

    let edge_t = Identifier::new("test_edge_type").unwrap();
    let edge = Edge::new(outbound_v.id, edge_t, inbound_v.id);

    let items = vec![
        BulkInsertItem::Edge(edge),
        BulkInsertItem::VertexProperty(
            outbound_v.id,
            Identifier::new("vertex_property_name").unwrap(),
            serde_json::Value::String("vertex_property_value".to_string()),
        ),
        BulkInsertItem::EdgeProperty(
            edge,
            Identifier::new("edge_property_name").unwrap(),
            serde_json::Value::String("edge_property_value".to_string()),
        ),
    ];

    db.bulk_insert(items).unwrap();

    let vertices = util::get_vertices(db, SpecificVertexQuery::new(vec![outbound_v.id, inbound_v.id])).unwrap();

    assert_eq!(vertices.len(), 2);
    assert_eq!(vertices[0].id, outbound_v.id);
    assert_eq!(vertices[0].t, outbound_v.t);
    assert_eq!(vertices[1].id, inbound_v.id);
    assert_eq!(vertices[1].t, inbound_v.t);

    let edges = util::get_edges(db, SpecificEdgeQuery::single(edge)).unwrap();

    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].outbound_id, outbound_v.id);
    assert_eq!(edges[0].t, edge_t);
    assert_eq!(edges[0].inbound_id, inbound_v.id);

    let vertex_properties = util::get_vertex_properties(
        db,
        SpecificVertexQuery::single(outbound_v.id)
            .properties()
            .unwrap()
            .name(Identifier::new("vertex_property_name").unwrap()),
    )
    .unwrap();

    assert_eq!(vertex_properties.len(), 1);
    assert_eq!(vertex_properties[0].id, outbound_v.id);
    assert_eq!(
        vertex_properties[0].value,
        serde_json::Value::String("vertex_property_value".to_string())
    );

    let edge_properties = util::get_edge_properties(
        db,
        SpecificEdgeQuery::single(edge)
            .properties()
            .unwrap()
            .name(Identifier::new("edge_property_name").unwrap()),
    )
    .unwrap();

    assert_eq!(edge_properties.len(), 1);
    assert_eq!(edge_properties[0].edge, edge);
    assert_eq!(
        edge_properties[0].value,
        serde_json::Value::String("edge_property_value".to_string())
    );
}

// Bulk insert allows for redundant vertex insertion
pub fn should_bulk_insert_a_redundant_vertex<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let id = db.create_vertex_from_type(vertex_t).unwrap();
    let items = vec![BulkInsertItem::Vertex(Vertex::new(id, vertex_t))];
    assert!(db.bulk_insert(items).is_ok());
}

// As an optimization, bulk insert does not verify that the vertices
// associated with an inserted edge exist; this verifies that
pub fn should_bulk_insert_an_invalid_edge<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let v1_id = db.create_vertex_from_type(vertex_t).unwrap();
    let v2 = Vertex::new(u64::max_value(), vertex_t);

    let edge_t = Identifier::new("test_edge_type").unwrap();

    let items = vec![BulkInsertItem::Edge(Edge::new(v1_id, edge_t, v2.id))];
    assert!(db.bulk_insert(items).is_ok());
    let items = vec![BulkInsertItem::Edge(Edge::new(v2.id, edge_t, v1_id))];
    assert!(db.bulk_insert(items).is_ok());
}
