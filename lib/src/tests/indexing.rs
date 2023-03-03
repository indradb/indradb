use super::util;
use crate::{expect_err, ijson, models, Database, Datastore, Error, QueryExt};
use uuid::Uuid;

fn setup_vertex_with_indexed_property<D: Datastore>(
    db: &Database<D>,
    property_name: models::Identifier,
) -> Result<Uuid, Error> {
    db.index_property(property_name)?;
    let id = db.create_vertex_from_type(models::Identifier::new("test_vertex_type")?)?;
    let q = models::SpecificVertexQuery::single(id);
    db.set_properties(q, property_name, &ijson!(true))?;
    Ok(id)
}

fn setup_edge_with_indexed_property<D: Datastore>(
    db: &Database<D>,
    property_name: models::Identifier,
) -> Result<models::Edge, Error> {
    db.index_property(property_name)?;
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;
    let edge_t = models::Identifier::new("test_edge_type")?;
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);
    let q = models::SpecificEdgeQuery::single(edge.clone());
    db.create_edge(&edge)?;
    db.set_properties(q, property_name, &ijson!(true))?;
    Ok(edge)
}

pub fn should_not_query_unindexed_vertex_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyPresenceQuery::new(models::Identifier::new("foo")?),
    );
    expect_err!(result, Error::NotIndexed);
    Ok(())
}

pub fn should_not_query_unindexed_edge_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let result = util::get_edges(
        db,
        models::EdgeWithPropertyPresenceQuery::new(models::Identifier::new("foo")?),
    );
    expect_err!(result, Error::NotIndexed);
    Ok(())
}

pub fn should_index_existing_vertex_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    // Setup
    let property_name = models::Identifier::new("existing-vertex-property")?;
    let other_property_name = models::Identifier::new("some-other-property")?;
    let id = db.create_vertex_from_type(models::Identifier::new("test_vertex_type")?)?;
    let q = models::SpecificVertexQuery::single(id);
    db.set_properties(q.clone(), property_name, &ijson!(true))?;

    // Index property
    db.index_property(property_name)?;

    // Get the vertex
    let result = util::get_vertices(db, models::VertexWithPropertyPresenceQuery::new(property_name))?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);

    // Get the vertex with piped queries
    let result = util::get_vertices(db, q.clone().with_property(property_name)?)?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = util::get_vertices(db, q.clone().without_property(property_name)?)?;
    assert!(result.is_empty());

    // Check against another property
    let result = util::get_vertices(db, q.clone().without_property(other_property_name)?);
    expect_err!(result, Error::NotIndexed);
    db.index_property(other_property_name.clone())?;
    let result = util::get_vertices(db, q.without_property(other_property_name)?)?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    Ok(())
}

pub fn should_index_existing_edge_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    // Setup
    let property_name = models::Identifier::new("existing-edge-property")?;
    let other_property_name = models::Identifier::new("some-other-property")?;
    let vertex_t = models::Identifier::new("test_vertex_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;
    let edge_t = models::Identifier::new("test_edge_type")?;
    let edge = models::Edge::new(outbound_id, edge_t, inbound_id);

    let q = models::SpecificEdgeQuery::single(edge.clone());
    db.create_edge(&edge)?;
    db.set_properties(q.clone(), property_name, &ijson!(true))?;

    // Index property
    db.index_property(property_name)?;

    // Get the edge
    let result = util::get_edges(db, models::EdgeWithPropertyPresenceQuery::new(property_name))?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);

    // Get the edge with a piped query
    let result = util::get_edges(db, q.clone().with_property(property_name)?)?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
    let result = util::get_edges(db, q.clone().without_property(property_name)?)?;
    assert!(result.is_empty());

    // Check against another property
    let result = util::get_edges(db, q.clone().without_property(other_property_name)?);
    expect_err!(result, Error::NotIndexed);
    db.index_property(other_property_name)?;
    let result = util::get_edges(db, q.without_property(other_property_name)?)?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
    Ok(())
}

pub fn should_delete_indexed_vertex_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let property_name = models::Identifier::new("deletable-vertex-property")?;
    let id = setup_vertex_with_indexed_property(db, property_name)?;
    let q = models::SpecificVertexQuery::single(id);
    db.delete(q.clone())?;
    let result = util::get_vertices(db, models::VertexWithPropertyPresenceQuery::new(property_name))?;
    assert_eq!(result.len(), 0);
    Ok(())
}

pub fn should_delete_indexed_edge_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let property_name = models::Identifier::new("deletable-edge-property")?;
    let edge = setup_edge_with_indexed_property(db, property_name)?;
    let q = models::SpecificEdgeQuery::single(edge);
    db.delete(q.clone())?;
    let result = util::get_edges(db, models::EdgeWithPropertyPresenceQuery::new(property_name))?;
    assert_eq!(result.len(), 0);
    Ok(())
}

pub fn should_update_indexed_vertex_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let json_true = ijson!(true);
    let json_false = ijson!(false);
    let property_name = models::Identifier::new("updateable-vertex-property")?;

    // Ensure errors happen when attempting to query before the property is indexed
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyValueQuery::new(property_name, json_true.clone()),
    );
    expect_err!(result, Error::NotIndexed);

    let id = setup_vertex_with_indexed_property(db, property_name)?;
    let q = models::SpecificVertexQuery::single(id);
    db.set_properties(q.clone(), property_name, &json_false)?;

    // property foo should not be the old value
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyValueQuery::new(property_name, json_true.clone()),
    )?;
    assert_eq!(result.len(), 0);
    let result = util::get_vertices(db, q.clone().with_property_equal_to(property_name, json_true.clone())?)?;
    assert_eq!(result.len(), 0);
    let result = util::get_vertices(
        db,
        q.clone().with_property_not_equal_to(property_name, json_true.clone())?,
    )?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);

    // property foo should be the new value
    let result = util::get_vertices(
        db,
        models::VertexWithPropertyValueQuery::new(property_name, json_false.clone()),
    )?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = util::get_vertices(db, q.clone().with_property_equal_to(property_name, json_false.clone())?)?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    let result = util::get_vertices(db, q.with_property_not_equal_to(property_name, json_false.clone())?)?;
    assert_eq!(result.len(), 0);

    Ok(())
}

pub fn should_update_indexed_edge_property<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let json_true = ijson!(true);
    let json_false = ijson!(false);
    let property_name = models::Identifier::new("updateable-edge-property")?;

    let result = util::get_edges(
        db,
        models::EdgeWithPropertyValueQuery::new(property_name, json_true.clone()),
    );
    expect_err!(result, Error::NotIndexed);

    let edge = setup_edge_with_indexed_property(db, property_name)?;
    let q = models::SpecificEdgeQuery::single(edge.clone());
    db.set_properties(q.clone(), property_name, &json_false)?;

    // property foo should not be the old value
    let result = util::get_edges(
        db,
        models::EdgeWithPropertyValueQuery::new(property_name, json_true.clone()),
    )?;
    assert_eq!(result.len(), 0);
    let result = util::get_edges(db, q.clone().with_property_equal_to(property_name, json_true.clone())?)?;
    assert_eq!(result.len(), 0);
    let result = util::get_edges(db, q.clone().with_property_not_equal_to(property_name, json_true)?)?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge.clone());

    // property foo should be the new value
    let result = util::get_edges(
        db,
        models::EdgeWithPropertyValueQuery::new(property_name, json_false.clone()),
    )?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
    let result = util::get_edges(db, q.clone().with_property_equal_to(property_name, json_false.clone())?)?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], edge);
    let result = util::get_edges(db, q.with_property_not_equal_to(property_name, json_false)?)?;
    assert_eq!(result.len(), 0);

    Ok(())
}

pub fn should_query_indexed_vertex_property_empty<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let property_name = models::Identifier::new("queryable-vertex-property")?;
    db.index_property(property_name)?;
    let result = util::get_vertices(db, models::VertexWithPropertyPresenceQuery::new(property_name))?;
    assert_eq!(result.len(), 0);
    Ok(())
}

pub fn should_query_indexed_edge_property_empty<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let property_name = models::Identifier::new("queryable-edge-property")?;
    db.index_property(property_name)?;
    let result = util::get_edges(db, models::EdgeWithPropertyPresenceQuery::new(property_name))?;
    assert_eq!(result.len(), 0);
    Ok(())
}

/// Tests for a regression found by the fuzzer:
/// https://github.com/indradb/indradb/issues/278
pub fn should_get_vertex_with_property_value_empty<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let property_name = models::Identifier::new("II")?;
    db.index_property(property_name)?;
    let results = util::get_vertices(
        db,
        models::VertexWithPropertyValueQuery::new(property_name, ijson!(null)),
    )?;
    assert_eq!(results.len(), 0);
    Ok(())
}

/// Tests for a regression:
/// https://github.com/indradb/indradb/issues/236
pub fn should_pipe_not_indexed_errors<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let q = models::VertexWithPropertyValueQuery::new(models::Identifier::new("Name")?, ijson!("John"));
    let result = db.get(q);
    expect_err!(result, Error::NotIndexed);

    let q = models::RangeVertexQuery::new()
        .t(models::Identifier::new("pipe-not-indexed-error-vertex-property")?)
        .limit(5)
        .with_property_equal_to(models::Identifier::new("Name")?, ijson!("John"))?;
    let result = db.get(q);
    expect_err!(result, Error::NotIndexed);

    Ok(())
}
