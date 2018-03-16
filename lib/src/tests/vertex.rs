use super::super::{Datastore, Transaction, VertexQuery};
use super::util::{create_edge_from, create_edges};
use models;
use std::collections::HashSet;
use std::u32;
use uuid::Uuid;

pub fn should_get_all_vertices<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();

    let mut inserted_ids = vec![
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
    ];

    inserted_ids.sort();
    let range = trans
        .get_vertices(&VertexQuery::All {
            start_id: None,
            limit: u32::MAX,
        })
        .unwrap();

    assert!(range.len() >= 5);

    let mut covered_ids: HashSet<Uuid> = HashSet::new();

    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, vertex_t);
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}

pub fn should_get_all_vertices_with_zero_limit<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();

    let mut inserted_ids = vec![
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
    ];

    inserted_ids.sort();
    let range = trans
        .get_vertices(&VertexQuery::All {
            start_id: None,
            limit: 0,
        })
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_all_vertices_out_of_range<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();

    let mut inserted_ids = vec![
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
    ];

    inserted_ids.sort();
    let range = trans
        .get_vertices(&VertexQuery::All {
            start_id: Some(Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()),
            limit: u32::MAX,
        })
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_single_vertices<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let inserted_id = trans.create_vertex(&vertex_t).unwrap();
    let range = trans
        .get_vertices(&VertexQuery::Vertices {
            ids: vec![inserted_id],
        })
        .unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);
    assert_eq!(range[0].t.0, "test_vertex_type");
}

pub fn should_get_single_vertices_nonexisting<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    trans.create_vertex(&vertex_t).unwrap();
    let range = trans
        .get_vertices(&VertexQuery::Vertices {
            ids: vec![Uuid::default()],
        })
        .unwrap();
    assert_eq!(range.len(), 0);
}

pub fn should_get_vertices<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let mut inserted_ids = vec![
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
        trans.create_vertex(&vertex_t).unwrap(),
    ];

    inserted_ids.sort();
    let range = trans
        .get_vertices(&VertexQuery::Vertices {
            ids: vec![
                inserted_ids[0],
                inserted_ids[1],
                inserted_ids[2],
                Uuid::default(),
            ],
        })
        .unwrap();

    assert!(range.len() == 3);

    let mut covered_ids: HashSet<Uuid> = HashSet::new();

    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, vertex_t);
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}

pub fn should_get_vertices_piped<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();

    let inserted_id_1 = trans.create_vertex(&vertex_t).unwrap();
    let inserted_id_2 = create_edge_from::<D, T>(&trans, inserted_id_1);

    // This query should get `inserted_id_2`
    let query_1 = VertexQuery::Vertices {
        ids: vec![inserted_id_1],
    }.outbound_edges(
        Some(models::Type::new("test_edge_type".to_string()).unwrap()),
        None,
        None,
        1,
    )
        .inbound_vertices(1);
    let range = trans.get_vertices(&query_1).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id_2);

    // This query should get `inserted_id_1`
    let query_2 = query_1
        .inbound_edges(
            Some(models::Type::new("test_edge_type".to_string()).unwrap()),
            None,
            None,
            1,
        )
        .outbound_vertices(1);
    let range = trans.get_vertices(&query_2).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id_1);
}

pub fn should_delete_a_valid_vertex<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let q = VertexQuery::Vertices {
        ids: vec![outbound_id],
    };
    trans.delete_vertices(&q).unwrap();
    let v = trans.get_vertices(&q).unwrap();
    assert_eq!(v.len(), 0);
    let t = models::Type::new("test_edge_type".to_string()).unwrap();
    let count = trans
        .get_edge_count(outbound_id, Some(&t), models::EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_not_delete_an_invalid_vertex<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let trans = datastore.transaction().unwrap();
    trans
        .delete_vertices(&VertexQuery::Vertices {
            ids: vec![Uuid::default()],
        })
        .unwrap();
}

pub fn should_get_a_vertex_count<D, T>(datastore: &mut D)
where
    D: Datastore<T>,
    T: Transaction,
{
    let vertex_t = models::Type::new("test_vertex_type".to_string()).unwrap();
    let trans = datastore.transaction().unwrap();
    trans.create_vertex(&vertex_t).unwrap();
    let count = trans.get_vertex_count().unwrap();
    assert!(count >= 1);
}
