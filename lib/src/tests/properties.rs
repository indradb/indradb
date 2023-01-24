use super::util;
use crate::util::extract_count;
use crate::{
    errors, expect_err, ijson, AllVertexQuery, CountQueryExt, Database, Datastore, Edge, Identifier, PipePropertyQuery,
    PipeWithPropertyPresenceQuery, QueryExt, SpecificEdgeQuery, SpecificVertexQuery,
};
use uuid::Uuid;

pub fn should_handle_vertex_properties<D: Datastore>(db: &Database<D>) {
    let t = Identifier::new("test_vertex_type").unwrap();
    let id = db.create_vertex_from_type(t).unwrap();
    let q = SpecificVertexQuery::single(id);

    // Check to make sure there's no initial value
    let result = util::get_vertex_properties(
        db,
        q.clone().properties().unwrap().name(Identifier::new("foo").unwrap()),
    )
    .unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_properties(q.clone(), Identifier::new("foo").unwrap(), &ijson!(true))
        .unwrap();
    let result = util::get_vertex_properties(
        db,
        q.clone().properties().unwrap().name(Identifier::new("foo").unwrap()),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    assert_eq!(result[0].value, ijson!(true));

    // Set and get the value as false
    db.set_properties(q.clone(), Identifier::new("foo").unwrap(), &ijson!(false))
        .unwrap();
    let result = util::get_vertex_properties(
        db,
        q.clone().properties().unwrap().name(Identifier::new("foo").unwrap()),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, id);
    assert_eq!(result[0].value, ijson!(false));

    // Delete & check that it's deleted
    db.delete(q.clone().properties().unwrap().name(Identifier::new("foo").unwrap()))
        .unwrap();
    let result =
        util::get_vertex_properties(db, q.properties().unwrap().name(Identifier::new("foo").unwrap())).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_vertex_properties<D: Datastore>(db: &Database<D>) {
    let t = Identifier::new("a_vertex").unwrap();
    let v1 = db.create_vertex_from_type(t).unwrap();
    let v2 = db.create_vertex_from_type(t).unwrap();
    let v3 = db.create_vertex_from_type(t).unwrap();
    let q1 = SpecificVertexQuery::single(v1);
    let q2 = SpecificVertexQuery::single(v2);
    let q3 = SpecificVertexQuery::single(v3);

    // Check to make sure there are no initial properties
    let all_result = util::get_all_vertex_properties(db, q2.clone()).unwrap();
    assert_eq!(all_result.len(), 0);

    // Set and get some properties for v2
    db.set_properties(q2.clone(), Identifier::new("a").unwrap(), &ijson!(false))
        .unwrap();
    db.set_properties(q2.clone(), Identifier::new("b").unwrap(), &ijson!(true))
        .unwrap();

    let result_1 = util::get_all_vertex_properties(db, q1).unwrap();
    assert_eq!(result_1.len(), 0);

    let result_2 = util::get_all_vertex_properties(db, q2).unwrap();
    assert_eq!(result_2.len(), 1);
    assert_eq!(result_2[0].props.len(), 2);
    assert_eq!(result_2[0].props[0].name, Identifier::new("a").unwrap());
    assert_eq!(result_2[0].props[0].value, ijson!(false));
    assert_eq!(result_2[0].props[1].name, Identifier::new("b").unwrap());
    assert_eq!(result_2[0].props[1].value, ijson!(true));

    let result_3 = util::get_all_vertex_properties(db, q3).unwrap();
    assert_eq!(result_3.len(), 0);
}

pub fn should_not_set_invalid_vertex_properties<D: Datastore>(db: &Database<D>) {
    let q = SpecificVertexQuery::single(Uuid::default());
    db.set_properties(q.clone(), Identifier::new("foo").unwrap(), &ijson!(null))
        .unwrap();
    let result =
        util::get_vertex_properties(db, q.properties().unwrap().name(Identifier::new("foo").unwrap())).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_vertex_properties<D: Datastore>(db: &Database<D>) {
    let q = SpecificVertexQuery::single(Uuid::default())
        .properties()
        .unwrap()
        .name(Identifier::new("foo").unwrap());
    db.delete(q).unwrap();

    let id = db.create_vertex_from_type(Identifier::new("foo").unwrap()).unwrap();

    let q = SpecificVertexQuery::single(id)
        .properties()
        .unwrap()
        .name(Identifier::new("foo").unwrap());
    db.delete(q).unwrap();
}

pub fn should_handle_edge_properties<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let inbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let edge = Edge::new(outbound_id, edge_t, inbound_id);
    let q = SpecificEdgeQuery::single(edge.clone());

    db.create_edge(&edge).unwrap();

    // Check to make sure there's no initial value
    let result = util::get_edge_properties(
        db,
        q.clone()
            .properties()
            .unwrap()
            .name(Identifier::new("edge-property").unwrap()),
    )
    .unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_properties(q.clone(), Identifier::new("edge-property").unwrap(), &ijson!(true))
        .unwrap();
    let result = util::get_edge_properties(
        db,
        q.clone()
            .properties()
            .unwrap()
            .name(Identifier::new("edge-property").unwrap()),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].edge, edge);
    assert_eq!(result[0].value, ijson!(true));

    // Set and get the value as false
    db.set_properties(q.clone(), Identifier::new("edge-property").unwrap(), &ijson!(false))
        .unwrap();
    let result = util::get_edge_properties(
        db,
        q.clone()
            .properties()
            .unwrap()
            .name(Identifier::new("edge-property").unwrap()),
    )
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].edge, edge);
    assert_eq!(result[0].value, ijson!(false));

    // Delete & check that it's deleted
    db.delete(q.clone()).unwrap();
    let result = util::get_edge_properties(
        db,
        q.properties().unwrap().name(Identifier::new("edge-property").unwrap()),
    )
    .unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_get_all_edge_properties<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let outbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let inbound_id = db.create_vertex_from_type(vertex_t).unwrap();
    let edge_t = Identifier::new("test_edge_type").unwrap();
    let edge = Edge::new(outbound_id, edge_t, inbound_id);
    let eq = SpecificEdgeQuery::single(edge.clone());

    db.create_edge(&edge).unwrap();

    // Check to make sure there's no initial value
    let result = util::get_all_edge_properties(db, eq.clone()).unwrap();
    assert_eq!(result.len(), 0);

    // Set and get the value as true
    db.set_properties(eq.clone(), Identifier::new("edge-prop-1").unwrap(), &ijson!(false))
        .unwrap();
    db.set_properties(eq.clone(), Identifier::new("edge-prop-2").unwrap(), &ijson!(true))
        .unwrap();

    let result = util::get_all_edge_properties(db, eq.clone()).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].props.len(), 2);
    assert_eq!(result[0].props[0].name, Identifier::new("edge-prop-1").unwrap());
    assert_eq!(result[0].props[0].value, ijson!(false));
    assert_eq!(result[0].props[1].name, Identifier::new("edge-prop-2").unwrap());
    assert_eq!(result[0].props[1].value, ijson!(true));

    // Delete & check that they are deleted
    db.delete(
        eq.clone()
            .properties()
            .unwrap()
            .name(Identifier::new("edge-prop-1").unwrap()),
    )
    .unwrap();
    db.delete(
        eq.clone()
            .properties()
            .unwrap()
            .name(Identifier::new("edge-prop-2").unwrap()),
    )
    .unwrap();

    let result = util::get_all_edge_properties(db, eq).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_set_invalid_edge_properties<D: Datastore>(db: &Database<D>) {
    let edge = Edge::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    let q = SpecificEdgeQuery::single(edge);
    db.set_properties(q.clone(), Identifier::new("bar").unwrap(), &ijson!(null))
        .unwrap();
    let result = util::get_edge_properties(db, q.properties().unwrap().name(Identifier::new("bar").unwrap())).unwrap();
    assert_eq!(result.len(), 0);
}

pub fn should_not_delete_invalid_edge_properties<D: Datastore>(db: &Database<D>) {
    let edge = Edge::new(Uuid::default(), Identifier::new("foo").unwrap(), Uuid::default());
    db.delete(
        SpecificEdgeQuery::single(edge)
            .properties()
            .unwrap()
            .name(Identifier::new("bar").unwrap()),
    )
    .unwrap();

    let outbound_id = db.create_vertex_from_type(Identifier::new("foo").unwrap()).unwrap();
    let inbound_id = db.create_vertex_from_type(Identifier::new("foo").unwrap()).unwrap();

    let edge = Edge::new(outbound_id, Identifier::new("baz").unwrap(), inbound_id);
    db.create_edge(&edge).unwrap();
    db.delete(
        SpecificEdgeQuery::single(edge)
            .properties()
            .unwrap()
            .name(Identifier::new("bleh").unwrap()),
    )
    .unwrap();
}

pub fn should_get_an_edge_properties_count<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let id = db.create_vertex_from_type(vertex_t).unwrap();
    let q = SpecificVertexQuery::single(id);
    let count = extract_count(
        db.get(q.outbound().unwrap().properties().unwrap().count().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert!(count == 0);
}

pub fn should_get_a_vertex_properties_count<D: Datastore>(db: &Database<D>) {
    let vertex_t = Identifier::new("test_vertex_type").unwrap();
    let id = db.create_vertex_from_type(vertex_t).unwrap();
    let q = SpecificVertexQuery::single(id);
    db.set_properties(q.clone(), Identifier::new("foo").unwrap(), &ijson!(true))
        .unwrap();
    let count = extract_count(
        db.get(
            q.properties()
                .unwrap()
                .name(Identifier::new("foo").unwrap())
                .count()
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(count >= 1);
}

pub fn should_not_set_properties_on_count<D: Datastore>(db: &Database<D>) {
    let result = db.set_properties(
        AllVertexQuery.count().unwrap(),
        Identifier::new("foo").unwrap(),
        &ijson!(true),
    );
    expect_err!(result, errors::Error::OperationOnQuery);
}

pub fn should_not_pipe_properties_on_vertex_count<D: Datastore>(db: &Database<D>) {
    // We have to build the query without it's constructor because the
    // constructor will catch this issue and trigger a `ValidationError`.
    let q = PipePropertyQuery {
        inner: Box::new(AllVertexQuery.count().unwrap().into()),
        name: None,
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
}

pub fn should_not_pipe_property_presence_on_vertex_count<D: Datastore>(db: &Database<D>) {
    // We have to build the query without it's constructor because the
    // constructor will catch this issue and trigger a `ValidationError`.
    let q = PipeWithPropertyPresenceQuery {
        inner: Box::new(AllVertexQuery.count().unwrap().into()),
        name: Identifier::new("foo").unwrap(),
        exists: true,
    };
    let result = db.get(q);
    expect_err!(result, errors::Error::OperationOnQuery);
}
