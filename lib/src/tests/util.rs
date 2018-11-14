use super::super::{Id, Datastore, Transaction};
use chrono::offset::Utc;
use chrono::DateTime;
use models;

pub fn create_edge_from<T: Transaction>(trans: &T, outbound_id: Id) -> Id {
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type").unwrap();
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t, inbound_v.id.clone());
    trans.create_edge(&key).unwrap();
    inbound_v.id
}

pub fn create_edges<D: Datastore>(datastore: &mut D) -> (Id, [Id; 5]) {
    let trans = datastore.transaction().unwrap();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    let inbound_ids: [Id; 5] = [
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
    ];

    (outbound_v.id, inbound_ids)
}

pub fn create_time_range_queryable_edges<D: Datastore>(
    datastore: &mut D,
) -> (Id, DateTime<Utc>, DateTime<Utc>, [Id; 5]) {
    let trans = datastore.transaction().unwrap();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    trans.create_vertex(&outbound_v).unwrap();

    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());

    let start_time = Utc::now();
    let inbound_ids = [
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
        create_edge_from(&trans, outbound_v.id.clone()),
    ];
    let end_time = Utc::now();

    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());
    create_edge_from(&trans, outbound_v.id.clone());

    (outbound_v.id, start_time, end_time, inbound_ids)
}
