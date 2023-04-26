use std::collections::HashSet;

use super::util;
use crate::{
    ijson, models, AllEdgeQuery, Database, Datastore, Edge, Error, QueryExt, SpecificEdgeQuery, SpecificVertexQuery,
};

use uuid::Uuid;

pub fn should_get_all_edges<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (outbound_id, inbound_ids) = util::create_edges(db)?;
    let edges = util::get_edges(db, AllEdgeQuery)?;
    assert_eq!(
        edges,
        inbound_ids
            .into_iter()
            .map(|id| Edge::new(outbound_id, models::Identifier::new("test_edge_type").unwrap(), id))
            .collect::<Vec<Edge>>()
    );
    Ok(())
}

pub fn should_get_a_valid_edge<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;
    let edge_t = models::Identifier::new("test_edge_type")?;
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);

    db.create_edge(&edge)?;

    let e = util::get_edges(db, SpecificEdgeQuery::single(edge))?;
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].outbound_id, outbound_id);
    assert_eq!(e[0].t, edge_t);
    assert_eq!(e[0].inbound_id, inbound_id);
    Ok(())
}

pub fn should_not_get_an_invalid_edge<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;
    let edge_t = models::Identifier::new("test_edge_type")?;

    let e = util::get_edges(
        db,
        SpecificEdgeQuery::single(Edge::new(outbound_id, edge_t, Uuid::default())),
    )?;
    assert_eq!(e.len(), 0);
    let e = util::get_edges(
        db,
        SpecificEdgeQuery::single(Edge::new(Uuid::default(), edge_t, inbound_id)),
    )?;
    assert_eq!(e.len(), 0);
    Ok(())
}

pub fn should_create_a_valid_edge<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;
    let edge_t = models::Identifier::new("test_edge_type")?;

    // Set the edge and check
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);
    db.create_edge(&edge)?;
    let e = util::get_edges(db, SpecificEdgeQuery::single(edge.clone()))?;
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    db.create_edge(&edge)?;

    // First check that getting a single edge will still...get a single edge
    let e = util::get_edges(db, SpecificEdgeQuery::single(edge.clone()))?;
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // REGRESSION: Second check that getting an edge range will only fetch a
    // single edge
    let e = util::get_edges(db, SpecificVertexQuery::single(outbound_id).outbound()?.limit(10))?;
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);
    Ok(())
}

pub fn should_not_create_an_invalid_edge<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let outbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v)?;
    let edge_t = models::Identifier::new("test_edge_type")?;
    let edge = models::Edge::new(outbound_v.id, edge_t, Uuid::default());
    let result = db.create_edge(&edge);
    assert_eq!(result?, false);
    Ok(())
}

pub fn should_delete_a_valid_edge<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_edge_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;

    let edge_t = models::Identifier::new("test_edge_type")?;
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);
    db.create_edge(&edge)?;

    let q = SpecificEdgeQuery::single(edge);
    db.set_properties(q.clone(), models::Identifier::new("foo")?, &ijson!(true))?;

    db.delete(q.clone())?;
    let e = util::get_edges(db, q)?;
    assert_eq!(e.len(), 0);
    Ok(())
}

pub fn should_not_delete_an_invalid_edge<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_edge_type")?;
    let outbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v)?;
    let edge_t = models::Identifier::new("test_edge_type")?;
    db.delete(SpecificEdgeQuery::single(Edge::new(
        outbound_v.id,
        edge_t,
        Uuid::default(),
    )))?;
    Ok(())
}

pub fn should_get_edges_with_no_type<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (outbound_id, _) = util::create_edges(db)?;
    let range = util::get_edges(db, SpecificVertexQuery::single(outbound_id).outbound()?.limit(10))?;
    check_edge_range(&range, outbound_id, 5)?;
    Ok(())
}

pub fn should_get_edge_range<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (outbound_id, _) = util::create_edges(db)?;
    let t = models::Identifier::new("test_edge_type")?;
    let range = util::get_edges(db, SpecificVertexQuery::single(outbound_id).outbound()?.limit(100).t(t))?;
    check_edge_range(&range, outbound_id, 5)?;
    Ok(())
}

pub fn should_get_edges<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (outbound_id, inbound_ids) = util::create_edges(db)?;
    let t = models::Identifier::new("test_edge_type")?;
    let q = SpecificEdgeQuery::new(vec![
        Edge::new(outbound_id, t, inbound_ids[0]),
        Edge::new(outbound_id, t, inbound_ids[1]),
        Edge::new(outbound_id, t, inbound_ids[2]),
        Edge::new(outbound_id, t, inbound_ids[3]),
        Edge::new(outbound_id, t, inbound_ids[4]),
    ]);
    let range = util::get_edges(db, q)?;
    check_edge_range(&range, outbound_id, 5)?;
    Ok(())
}

pub fn should_get_edges_piped<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let outbound_v = models::Vertex::new(vertex_t);
    db.create_vertex(&outbound_v)?;

    let inbound_id = util::create_edge_from(db, outbound_v.id)?;

    let query_1 = SpecificVertexQuery::single(outbound_v.id)
        .outbound()?
        .limit(1)
        .t(models::Identifier::new("test_edge_type")?);
    let range = util::get_edges(db, query_1.clone())?;
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(outbound_v.id, models::Identifier::new("test_edge_type")?, inbound_id)
    );

    let query_2 = query_1
        .inbound()?
        .limit(1)
        .inbound()?
        .limit(1)
        .t(models::Identifier::new("test_edge_type")?);
    let range = util::get_edges(db, query_2)?;
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(outbound_v.id, models::Identifier::new("test_edge_type")?, inbound_id)
    );
    Ok(())
}

fn check_edge_range(range: &[models::Edge], expected_outbound_id: Uuid, expected_length: usize) -> Result<(), Error> {
    assert_eq!(range.len(), expected_length);
    let mut covered_ids: HashSet<Uuid> = HashSet::new();
    let t = models::Identifier::new("test_edge_type")?;

    for edge in range {
        assert_eq!(edge.outbound_id, expected_outbound_id);
        assert_eq!(edge.t, t);
        assert!(!covered_ids.contains(&edge.inbound_id));
        covered_ids.insert(edge.inbound_id);
    }

    Ok(())
}
