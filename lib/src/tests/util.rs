use super::super::{Datastore, Transaction};
use chrono::DateTime;
use chrono::offset::Utc;
use models;
use uuid::Uuid;

pub fn create_edge_from<D, T>(trans: &T, outbound_id: Uuid) -> Uuid
where
    D: Datastore<T>,
    T: Transaction,
{
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t, inbound_v.id);
    trans.create_edge(&key).unwrap();
    inbound_v.id
}

pub fn create_edges<D, T>(datastore: &mut D) -> (Uuid, [Uuid; 5])
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
    ];

    (outbound_v.id, inbound_ids)
}

pub fn create_time_range_queryable_edges<D, T>(datastore: &mut D) -> (Uuid, DateTime<Utc>, DateTime<Utc>, [Uuid; 5])
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    trans.create_vertex(&outbound_v).unwrap();

    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);

    let start_time = Utc::now();
    let inbound_ids = [
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
        create_edge_from::<D, T>(&trans, outbound_v.id),
    ];
    let end_time = Utc::now();

    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);
    create_edge_from::<D, T>(&trans, outbound_v.id);

    (outbound_v.id, start_time, end_time, inbound_ids)
}
