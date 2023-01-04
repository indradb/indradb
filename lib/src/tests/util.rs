use crate::{models, Database, Datastore};

use uuid::Uuid;

pub fn create_edge_from<D: Datastore>(db: &Database<D>, outbound_id: Uuid) -> Uuid {
    let inbound_vertex_t = models::Identifier::new("test_inbound_vertex_type").unwrap();
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    db.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_id, edge_t, inbound_v.id);
    db.create_edge(&edge).unwrap();
    inbound_v.id
}

pub fn create_edges<D: Datastore>(db: &Database<D>) -> (Uuid, [Uuid; 5]) {
    let outbound_vertex_t = models::Identifier::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    db.create_vertex(&outbound_v).unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
        create_edge_from(db, outbound_v.id),
    ];

    (outbound_v.id, inbound_ids)
}
