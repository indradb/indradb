use super::super::{Datastore, Transaction};
use crate::models;
use uuid::Uuid;

pub fn create_edge_from<T: Transaction>(trans: &T, outbound_id: Uuid) -> Uuid {
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type").unwrap();
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    let key = models::Edge::new(outbound_id, edge_t, inbound_v.id);
    trans.create_edge(&key).unwrap();
    inbound_v.id
}

pub fn create_edges<D: Datastore>(datastore: &mut D) -> (Uuid, [Uuid; 5]) {
    let trans = datastore.transaction().unwrap();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from(&trans, outbound_v.id),
        create_edge_from(&trans, outbound_v.id),
        create_edge_from(&trans, outbound_v.id),
        create_edge_from(&trans, outbound_v.id),
        create_edge_from(&trans, outbound_v.id),
    ];

    (outbound_v.id, inbound_ids)
}
