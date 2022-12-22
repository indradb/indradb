use crate::{models, Datastore};

use uuid::Uuid;

pub fn create_edge_from<D: Datastore>(datastore: &D, outbound_id: Uuid) -> Uuid {
    let inbound_vertex_t = models::Identifier::new("test_inbound_vertex_type").unwrap();
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    datastore.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Identifier::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t, inbound_v.id);
    datastore.create_edge(&key).unwrap();
    inbound_v.id
}

pub fn create_edges<D: Datastore>(datastore: &D) -> (Uuid, [Uuid; 5]) {
    let outbound_vertex_t = models::Identifier::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    datastore.create_vertex(&outbound_v).unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
        create_edge_from(datastore, outbound_v.id),
    ];

    (outbound_v.id, inbound_ids)
}
