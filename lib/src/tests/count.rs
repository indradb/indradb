use super::util;
use crate::util::extract_count;
use crate::{
    errors, expect_err, ijson, AllVertexQuery, CountQuery, CountQueryExt, Database, Datastore, EdgeDirection,
    Error, Identifier, PipePropertyQuery, PipeQuery, PipeWithPropertyPresenceQuery,
    QueryExt, SpecificVertexQuery,
};
use uuid::Uuid;

pub fn should_get_a_vertex_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = Identifier::new("test_vertex_type")?;
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
    let q = PipeQuery {
        inner: Box::new(AllVertexQuery.count()?.into()),
        direction: EdgeDirection::Outbound,
        limit: 1,
        t: None,
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
    Ok(())
}

pub fn should_get_an_edge_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (outbound_id, _) = util::create_edges(db)?;
    let t = Identifier::new("test_edge_type")?;
    let count = util::get_edge_count(db, outbound_id, Some(t), EdgeDirection::Outbound)?;
    assert_eq!(count, 5);
    Ok(())
}

pub fn should_get_an_edge_count_with_no_type<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (outbound_id, _) = util::create_edges(db)?;
    let count = util::get_edge_count(db, outbound_id, None, EdgeDirection::Outbound)?;
    assert_eq!(count, 5);
    Ok(())
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let t = Identifier::new("test_edge_type")?;
    let count = util::get_edge_count(db, Uuid::default(), Some(t), EdgeDirection::Outbound)?;
    assert_eq!(count, 0);
    Ok(())
}

pub fn should_get_an_inbound_edge_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let (_, inbound_ids) = util::create_edges(db)?;
    let count = util::get_edge_count(db, inbound_ids[0], None, EdgeDirection::Inbound)?;
    assert_eq!(count, 1);
    Ok(())
}

pub fn should_get_an_edge_properties_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = Identifier::new("test_vertex_type")?;
    let id = db.create_vertex_from_type(vertex_t)?;
    let q = SpecificVertexQuery::single(id);
    let count = extract_count(db.get(q.outbound()?.properties()?.count()?)?).unwrap();
    assert!(count == 0);
    Ok(())
}

pub fn should_get_a_vertex_properties_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let vertex_t = Identifier::new("test_vertex_type")?;
    let id = db.create_vertex_from_type(vertex_t)?;
    let q = SpecificVertexQuery::single(id);
    db.set_properties(q.clone(), Identifier::new("foo")?, &ijson!(true))?;
    let count = extract_count(db.get(q.properties()?.name(Identifier::new("foo")?).count()?)?).unwrap();
    assert!(count >= 1);
    Ok(())
}

pub fn should_not_set_properties_on_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    let result = db.set_properties(AllVertexQuery.count()?, Identifier::new("foo")?, &ijson!(true));
    expect_err!(result, errors::Error::OperationOnQuery);
    Ok(())
}

pub fn should_not_pipe_properties_on_vertex_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    // We have to build the query without it's constructor because the
    // constructor will catch this issue and trigger a `ValidationError`.
    let q = PipePropertyQuery {
        inner: Box::new(AllVertexQuery.count()?.into()),
        name: None,
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
    Ok(())
}

pub fn should_not_pipe_property_presence_on_vertex_count<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    // We have to build the query without it's constructor because the
    // constructor will catch this issue and trigger a `ValidationError`.
    let q = PipeWithPropertyPresenceQuery {
        inner: Box::new(AllVertexQuery.count()?.into()),
        name: Identifier::new("foo")?,
        exists: true,
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
    Ok(())
}

pub fn should_not_run_nested_count_query<D: Datastore>(db: &Database<D>) -> Result<(), Error> {
    // We have to build the query without it's constructor because the
    // constructor will catch this issue and trigger a `ValidationError`.
    let q = CountQuery {
        inner: Box::new(
            CountQuery {
                inner: Box::new(AllVertexQuery.count()?.into()),
            }
            .into(),
        ),
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
    Ok(())
}
