use std::collections::HashSet;
use std::error::Error as StdError;

use super::util;
use crate::util::extract_count;
use crate::{
    errors, expect_err, ijson, models, AllVertexQuery, CountQueryExt, Database, Datastore, Error, QueryExt,
    RangeVertexQuery, SpecificVertexQuery,
};

use uuid::Uuid;

pub fn should_create_vertex_from_type<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let t = models::Identifier::new("test_vertex_type")?;
    db.create_vertex_from_type(t)?;
    Ok(())
}

pub fn should_get_all_vertices<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let inserted_ids = create_vertices(db)?;
    let range = util::get_vertices(db, AllVertexQuery)?;
    check_has_all_vertices(range, inserted_ids);
    Ok(())
}

pub fn should_get_range_vertices<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let inserted_ids = create_vertices(db)?;
    let range = util::get_vertices(db, RangeVertexQuery::new())?;
    check_has_all_vertices(range, inserted_ids);
    Ok(())
}

pub fn should_get_no_vertices_with_zero_limit<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    create_vertices(db)?;
    let range = util::get_vertices(db, RangeVertexQuery::new().limit(0))?;
    assert_eq!(range.len(), 0);
    Ok(())
}

pub fn should_get_range_vertices_out_of_range<D: Datastore>(db: &Database<D>) -> Result<(), Box<dyn StdError>> {
    create_vertices(db)?;
    let range = util::get_vertices(
        db,
        RangeVertexQuery::new().start_id(Uuid::parse_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")?),
    )?;
    assert_eq!(range.len(), 0);
    Ok(())
}

pub fn should_get_no_vertices_with_type_filter<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let type_filter = models::Identifier::new("foo")?;
    create_vertices(db)?;
    let range = util::get_vertices(db, RangeVertexQuery::new().t(type_filter))?;
    assert_eq!(range.len(), 0);
    Ok(())
}

pub fn should_get_single_vertex<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let id = db.create_vertex_from_type(vertex_t)?;
    let range = util::get_vertices(db, SpecificVertexQuery::single(id))?;
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, id);
    assert_eq!(range[0].t.as_str(), "test_vertex_type");
    Ok(())
}

pub fn should_get_single_vertex_nonexisting<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    db.create_vertex_from_type(vertex_t)?;
    let range = util::get_vertices(db, SpecificVertexQuery::single(Uuid::default()))?;
    assert_eq!(range.len(), 0);
    Ok(())
}

pub fn should_get_vertices<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let mut inserted_ids = create_vertices(db)?;

    let range = util::get_vertices(
        db,
        SpecificVertexQuery::new(vec![inserted_ids[0], inserted_ids[1], inserted_ids[2], Uuid::default()]),
    )?;

    assert!(range.len() == 3);

    let mut covered_ids: HashSet<Uuid> = HashSet::new();

    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, models::Identifier::new("test_vertex_type")?);
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }

    Ok(())
}

pub fn should_get_vertices_piped<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let edge_t = models::Identifier::new("test_edge_type")?;
    let id = db.create_vertex_from_type(vertex_t)?;
    let inserted_id = util::create_edge_from(db, id)?;

    // This query should get `inserted_id`
    let query_1 = SpecificVertexQuery::single(id)
        .outbound()?
        .limit(1)
        .t(edge_t)
        .inbound()?
        .limit(1);
    let range = util::get_vertices(db, query_1.clone())?;
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get `inserted_id`
    let query_2 = SpecificVertexQuery::single(id)
        .outbound()?
        .limit(1)
        .t(edge_t)
        .inbound()?
        .limit(1)
        .t(models::Identifier::new("test_inbound_vertex_type")?);
    let range = util::get_vertices(db, query_2)?;
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, inserted_id);

    // This query should get nothing
    let query_3 = SpecificVertexQuery::single(id)
        .outbound()?
        .limit(1)
        .t(edge_t)
        .inbound()?
        .limit(1)
        .t(models::Identifier::new("foo")?);
    let range = util::get_vertices(db, query_3)?;
    assert_eq!(range.len(), 0);

    // This query should get `v`
    let query_4 = query_1.inbound()?.limit(1).t(edge_t).outbound()?.limit(1);
    let range = util::get_vertices(db, query_4)?;
    assert_eq!(range.len(), 1);
    assert_eq!(range[0].id, id);

    Ok(())
}

pub fn should_delete_a_valid_outbound_vertex<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (outbound_id, _) = util::create_edges(db)?;
    let q = SpecificVertexQuery::single(outbound_id);
    db.set_properties(q.clone(), models::Identifier::new("foo")?, &ijson!(true))?;
    db.delete(q.clone())?;
    let v = util::get_vertices(db, q)?;
    assert_eq!(v.len(), 0);
    let t = models::Identifier::new("test_edge_type")?;
    let count = util::get_edge_count(db, outbound_id, Some(t), models::EdgeDirection::Outbound)?;
    assert_eq!(count, 0);
    Ok(())
}

pub fn should_delete_a_valid_inbound_vertex<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (_, inbound_ids) = util::create_edges(db)?;
    let inbound_id = inbound_ids[0];
    let q = SpecificVertexQuery::single(inbound_id);
    db.delete(q.clone())?;
    let v = util::get_vertices(db, q)?;
    assert_eq!(v.len(), 0);
    let t = models::Identifier::new("test_edge_type")?;
    let count = util::get_edge_count(db, inbound_id, Some(t), models::EdgeDirection::Inbound)?;
    assert_eq!(count, 0);
    Ok(())
}

pub fn should_not_delete_an_invalid_vertex<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    db.delete(SpecificVertexQuery::single(Uuid::default()))
}

pub fn should_get_a_vertex_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let id = db.create_vertex_from_type(vertex_t)?;
    let count = util::get_vertex_count(db)?;
    assert!(count >= 1);
    let count = extract_count(db.get(SpecificVertexQuery::single(id).count()?)?).unwrap();
    assert!(count >= 1);
    Ok(())
}

pub fn should_not_delete_on_vertex_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let result = db.delete(AllVertexQuery.count()?);
    expect_err!(result, errors::Error::OperationOnQuery);
    Ok(())
}

pub fn should_not_pipe_on_vertex_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    // We have to build the query without it's constructor because the
    // constructor will catch this issue and trigger a `ValidationError`.
    let q = models::PipeQuery {
        inner: Box::new(AllVertexQuery.count()?.into()),
        direction: models::EdgeDirection::Outbound,
        limit: 1,
        t: None,
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
    Ok(())
}

fn check_has_all_vertices(range: Vec<models::Vertex>, mut inserted_ids: Vec<Uuid>) {
    assert!(range.len() >= 5);
    let mut covered_ids: HashSet<Uuid> = HashSet::new();
    for vertex in &range {
        if let Ok(index) = inserted_ids.binary_search(&vertex.id) {
            assert_eq!(vertex.t, models::Identifier::new("test_vertex_type").unwrap());
            inserted_ids.remove(index);
        }

        assert!(!covered_ids.contains(&vertex.id));
        covered_ids.insert(vertex.id);
    }
}

fn create_vertices<D: Datastore>(db: &Database<D>) -> Result<Vec<Uuid>, Error> {
    let t = models::Identifier::new("test_vertex_type")?;
    let mut ids = Vec::with_capacity(5);
    for _i in 0..5 {
        let id = db.create_vertex_from_type(t)?;
        ids.push(id);
    }
    Ok(ids)
}
