use super::util;
use crate::{ijson, Database, Datastore, Edge, Error, Identifier, QueryExt, SpecificEdgeQuery, SpecificVertexQuery};
use uuid::Uuid;

pub fn should_handle_vertex_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let t = Identifier::new("test_vertex_type")?;
    let id = db.create_vertex_from_type(t)?;
    let q = SpecificVertexQuery::single(id);

    // Check to make sure there's no initial value
    let result = util::get_vertex_properties(db, q.clone().properties()?.name(Identifier::new("foo")?))?;
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_properties(q.clone(), Identifier::new("foo")?, &ijson!(true))?;
    let result = util::get_vertex_properties(db, q.clone().properties()?.name(Identifier::new("foo")?))?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    assert_eq!(result[0].value, ijson!(true));

    // Set and get the value as false
    db.set_properties(q.clone(), Identifier::new("foo")?, &ijson!(false))?;
    let result = util::get_vertex_properties(db, q.clone().properties()?.name(Identifier::new("foo")?))?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    assert_eq!(result[0].value, ijson!(false));

    // Delete & check that it's deleted
    db.delete(q.clone().properties()?.name(Identifier::new("foo")?))?;
    let result = util::get_vertex_properties(db, q.properties()?.name(Identifier::new("foo")?))?;
    assert_eq!(result.len(), 0);

    Ok(())
}

pub fn should_get_all_vertex_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let t = Identifier::new("a_vertex")?;
    let v1 = db.create_vertex_from_type(t)?;
    let v2 = db.create_vertex_from_type(t)?;
    let v3 = db.create_vertex_from_type(t)?;
    let q1 = SpecificVertexQuery::single(v1);
    let q2 = SpecificVertexQuery::single(v2);
    let q3 = SpecificVertexQuery::single(v3);

    // Check to make sure there are no initial properties
    let all_result = util::get_all_vertex_properties(db, q2.clone())?;
    assert_eq!(all_result.len(), 0);

    // Set and get some properties for v2
    db.set_properties(q2.clone(), Identifier::new("a")?, &ijson!(false))?;
    db.set_properties(q2.clone(), Identifier::new("b")?, &ijson!(true))?;

    let result_1 = util::get_all_vertex_properties(db, q1)?;
    assert_eq!(result_1.len(), 0);

    let result_2 = util::get_all_vertex_properties(db, q2)?;
    assert_eq!(result_2.len(), 1);
    assert_eq!(result_2[0].props.len(), 2);
    assert_eq!(result_2[0].props[0].name, Identifier::new("a")?);
    assert_eq!(result_2[0].props[0].value, ijson!(false));
    assert_eq!(result_2[0].props[1].name, Identifier::new("b")?);
    assert_eq!(result_2[0].props[1].value, ijson!(true));

    let result_3 = util::get_all_vertex_properties(db, q3)?;
    assert_eq!(result_3.len(), 0);

    Ok(())
}

pub fn should_not_set_invalid_vertex_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let q = SpecificVertexQuery::single(Uuid::default());
    db.set_properties(q.clone(), Identifier::new("foo")?, &ijson!(null))?;
    let result = util::get_vertex_properties(db, q.properties()?.name(Identifier::new("foo")?))?;
    assert_eq!(result.len(), 0);
    Ok(())
}

pub fn should_not_delete_invalid_vertex_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let q = SpecificVertexQuery::single(Uuid::default())
        .properties()?
        .name(Identifier::new("foo")?);
    db.delete(q)?;

    let id = db.create_vertex_from_type(Identifier::new("foo")?)?;

    let q = SpecificVertexQuery::single(id)
        .properties()?
        .name(Identifier::new("foo")?);
    db.delete(q)?;

    Ok(())
}

pub fn should_handle_edge_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = Identifier::new("test_vertex_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;
    let edge_t = Identifier::new("test_edge_type")?;
    let edge = Edge::new(outbound_id, edge_t, inbound_id);
    let q = SpecificEdgeQuery::single(edge.clone());

    db.create_edge(&edge)?;

    // Check to make sure there's no initial value
    let result = util::get_edge_properties(db, q.clone().properties()?.name(Identifier::new("edge-property")?))?;
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_properties(q.clone(), Identifier::new("edge-property")?, &ijson!(true))?;
    let result = util::get_edge_properties(db, q.clone().properties()?.name(Identifier::new("edge-property")?))?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].edge, edge);
    assert_eq!(result[0].value, ijson!(true));

    // Set and get the value as false
    db.set_properties(q.clone(), Identifier::new("edge-property")?, &ijson!(false))?;
    let result = util::get_edge_properties(db, q.clone().properties()?.name(Identifier::new("edge-property")?))?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].edge, edge);
    assert_eq!(result[0].value, ijson!(false));

    // Delete & check that it's deleted
    db.delete(q.clone())?;
    let result = util::get_edge_properties(db, q.properties()?.name(Identifier::new("edge-property")?))?;
    assert_eq!(result.len(), 0);

    Ok(())
}

pub fn should_get_all_edge_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = Identifier::new("test_vertex_type")?;
    let outbound_id = db.create_vertex_from_type(vertex_t)?;
    let inbound_id = db.create_vertex_from_type(vertex_t)?;
    let edge_t = Identifier::new("test_edge_type")?;
    let edge = Edge::new(outbound_id, edge_t, inbound_id);
    let eq = SpecificEdgeQuery::single(edge.clone());

    db.create_edge(&edge)?;

    // Check to make sure there's no initial value
    let result = util::get_all_edge_properties(db, eq.clone())?;
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_properties(eq.clone(), Identifier::new("edge-prop-1")?, &ijson!(false))?;
    db.set_properties(eq.clone(), Identifier::new("edge-prop-2")?, &ijson!(true))?;

    let result = util::get_all_edge_properties(db, eq.clone())?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 2);
    assert_eq!(result[0].props[0].name, Identifier::new("edge-prop-1")?);
    assert_eq!(result[0].props[0].value, ijson!(false));
    assert_eq!(result[0].props[1].name, Identifier::new("edge-prop-2")?);
    assert_eq!(result[0].props[1].value, ijson!(true));

    // Delete & check that they are deleted
    db.delete(eq.clone().properties()?.name(Identifier::new("edge-prop-1")?))?;
    db.delete(eq.clone().properties()?.name(Identifier::new("edge-prop-2")?))?;

    let result = util::get_all_edge_properties(db, eq)?;
    assert_eq!(result.len(), 0);

    Ok(())
}

pub fn should_not_set_invalid_edge_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let edge = Edge::new(Uuid::default(), Identifier::new("foo")?, Uuid::default());
    let q = SpecificEdgeQuery::single(edge);
    db.set_properties(q.clone(), Identifier::new("bar")?, &ijson!(null))?;
    let result = util::get_edge_properties(db, q.properties()?.name(Identifier::new("bar")?))?;
    assert_eq!(result.len(), 0);
    Ok(())
}

pub fn should_not_delete_invalid_edge_properties<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let edge = Edge::new(Uuid::default(), Identifier::new("foo")?, Uuid::default());
    db.delete(
        SpecificEdgeQuery::single(edge)
            .properties()?
            .name(Identifier::new("bar")?),
    )?;

    let outbound_id = db.create_vertex_from_type(Identifier::new("foo")?)?;
    let inbound_id = db.create_vertex_from_type(Identifier::new("foo")?)?;

    let edge = Edge::new(outbound_id, Identifier::new("baz")?, inbound_id);
    db.create_edge(&edge)?;
    db.delete(
        SpecificEdgeQuery::single(edge)
            .properties()?
            .name(Identifier::new("bleh")?),
    )?;

    Ok(())
}
