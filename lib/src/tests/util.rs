use super::super::{Datastore, Transaction};
use crate::models;

pub fn create_edge_from<T: Transaction>(trans: &T, outbound_id: u64) -> u64 {
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type").unwrap();
    let inbound_id = trans.create_vertex(&inbound_vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);
    trans.create_edge(&edge).unwrap();
    inbound_id
}

pub fn create_edges<D: Datastore>(datastore: &mut D) -> (u64, [u64; 5]) {
    let trans = datastore.transaction().unwrap();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type").unwrap();
    let outbound_id = trans.create_vertex(&outbound_vertex_t).unwrap();
    let inbound_ids: [u64; 5] = [
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
        create_edge_from(&trans, outbound_id),
    ];

    (outbound_id, inbound_ids)
}
