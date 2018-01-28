use chrono::DateTime;
use chrono::offset::Utc;
use super::super::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use models;
use uuid::Uuid;

pub fn create_edge_from<D, T>(trans: &T, outbound_id: Uuid) -> Uuid
where
    D: Datastore<T>,
    T: Transaction,
{
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(inbound_vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let key = models::EdgeKey::new(outbound_id, edge_t, inbound_id);
    trans.create_edge(key).unwrap();
    inbound_id
}

pub fn create_edges<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>) -> (Uuid, [Uuid; 5])
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
    ];

    trans.commit().unwrap();

    (outbound_id, inbound_ids)
}

pub fn create_time_range_queryable_edges<D, T>(
    sandbox: &mut DatastoreTestSandbox<D, T>,
) -> (Uuid, DateTime<Utc>, DateTime<Utc>, [Uuid; 5])
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = sandbox.transaction();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();

    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);

    let start_time = Utc::now();
    let inbound_ids = [
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
    ];
    let end_time = Utc::now();

    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);

    trans.commit().unwrap();

    (outbound_id, start_time, end_time, inbound_ids)
}
