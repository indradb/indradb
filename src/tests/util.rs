use chrono::{UTC, DateTime, Duration as ChronoDuration};
use super::super::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use models;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;
use std::cmp::{min, max};

pub fn create_edge_from<D, T>(trans: &T, outbound_id: Uuid) -> Uuid
    where D: Datastore<T>,
          T: Transaction
{
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(inbound_vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(1.0).unwrap();
    trans.create_edge(models::Edge::new_with_current_datetime(outbound_id, edge_t, inbound_id, weight)).unwrap();
    inbound_id
}

pub fn create_edges<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>) -> (Uuid, [Uuid; 5])
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();
    let inbound_ids: [Uuid; 5] = [
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id),
        create_edge_from::<D, T>(&trans, outbound_id)
    ];

    trans.commit().unwrap();

    (outbound_id, inbound_ids)
}

pub fn create_time_range_queryable_edges<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>) -> (Uuid, DateTime<UTC>, DateTime<UTC>, [Uuid; 5])
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();

    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);

    sleep(Duration::new(2, 0));
    let start_time = UTC::now().checked_sub_signed(ChronoDuration::seconds(1)).unwrap();
    let inbound_ids = [create_edge_from::<D, T>(&trans, outbound_id),
                       create_edge_from::<D, T>(&trans, outbound_id),
                       create_edge_from::<D, T>(&trans, outbound_id),
                       create_edge_from::<D, T>(&trans, outbound_id),
                       create_edge_from::<D, T>(&trans, outbound_id)];
    let end_time = UTC::now().checked_add_signed(ChronoDuration::seconds(1)).unwrap();
    sleep(Duration::new(2, 0));

    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);
    create_edge_from::<D, T>(&trans, outbound_id);

    trans.commit().unwrap();

    (outbound_id, start_time, end_time, inbound_ids)
}

pub fn create_edge_to<D, T>(trans: &T, inbound_id: Uuid) -> Uuid
    where D: Datastore<T>,
          T: Transaction
{
    let outbound_vertex_t = models::Type::new("test_outbound_vertex_type".to_string()).unwrap();
    let outbound_id = trans.create_vertex(outbound_vertex_t).unwrap();
    let edge_t = models::Type::new("test_edge_type".to_string()).unwrap();
    let weight = models::Weight::new(1.0).unwrap();
    trans.create_edge(models::Edge::new_with_current_datetime(outbound_id, edge_t, inbound_id, weight)).unwrap();
    inbound_id
}

pub fn create_reversed_edges<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>) -> (Uuid, [Uuid; 5])
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(inbound_vertex_t).unwrap();
    let outbound_ids = [create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id)];

    trans.commit().unwrap();

    (inbound_id, outbound_ids)
}

pub fn create_time_range_queryable_reversed_edges<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>) -> (Uuid, DateTime<UTC>, DateTime<UTC>, [Uuid; 5])
    where D: Datastore<T>,
          T: Transaction
{
    let trans = sandbox.transaction();
    let inbound_vertex_t = models::Type::new("test_inbound_vertex_type".to_string()).unwrap();
    let inbound_id = trans.create_vertex(inbound_vertex_t).unwrap();
    
    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);

    sleep(Duration::new(2, 0));
    let start_time = UTC::now();
    let outbound_ids = [create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id),
                        create_edge_to::<D, T>(&trans, inbound_id)];

    // We add some padding to the end time because some implementations only
    // have second-level accuracy, in which case we'd get errors
    let end_time = UTC::now().checked_add_signed(ChronoDuration::seconds(1)).unwrap();
    sleep(Duration::new(2, 0));

    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);
    create_edge_to::<D, T>(&trans, inbound_id);

    trans.commit().unwrap();

    (inbound_id, start_time, end_time, outbound_ids)
}

pub fn min_uuid(uuids: &[Uuid]) -> Uuid {
    let mut min_uuid = uuids[0];

    for uuid in uuids {
        min_uuid = min(min_uuid, *uuid);
    }

    return min_uuid
}

pub fn max_uuid(uuids: &[Uuid]) -> Uuid {
    let mut max_uuid = uuids[0];

    for uuid in uuids {
        max_uuid = max(max_uuid, *uuid);
    }

    return max_uuid
}
