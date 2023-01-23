use super::util;
use crate::{
    ijson, CountQueryExt, Database, Datastore, Edge, Identifier, NamedProperty, QueryExt, QueryOutputValue,
    SpecificVertexQuery, Vertex, VertexProperties,
};

pub fn should_get_nested_include_query<D: Datastore>(db: &Database<D>) {
    let (outbound_id, inbound_ids) = util::create_edges(db);
    let q = SpecificVertexQuery::single(outbound_id)
        .include()
        .outbound()
        .unwrap()
        .include()
        .count()
        .unwrap();
    let output = db.get(q).unwrap();
    assert_eq!(
        output,
        vec![
            QueryOutputValue::Vertices(vec![Vertex::new(
                outbound_id,
                Identifier::new("test_outbound_vertex_type").unwrap()
            )]),
            QueryOutputValue::Edges(
                inbound_ids
                    .into_iter()
                    .map(|id| Edge::new(outbound_id, Identifier::new("test_edge_type").unwrap(), id))
                    .collect()
            ),
            QueryOutputValue::Count(5),
        ]
    );
}

pub fn should_get_unnested_include_query<D: Datastore>(db: &Database<D>) {
    let id = db.create_vertex_from_type(Identifier::new("foo").unwrap()).unwrap();
    let q = SpecificVertexQuery::single(id);
    db.set_properties(q.clone(), Identifier::new("bar").unwrap(), &ijson!(true))
        .unwrap();
    let output = db.get(q.include().properties().unwrap()).unwrap();
    assert_eq!(
        output,
        vec![
            QueryOutputValue::Vertices(vec![Vertex::new(id, Identifier::new("foo").unwrap())]),
            QueryOutputValue::VertexProperties(vec![VertexProperties::new(
                Vertex::new(id, Identifier::new("foo").unwrap()),
                vec![NamedProperty::new(Identifier::new("bar").unwrap(), ijson!(true)),],
            )])
        ]
    );
}

pub fn should_include_with_property_presence<D: Datastore>(db: &Database<D>) {
    let id = db.create_vertex_from_type(Identifier::new("foo").unwrap()).unwrap();
    let q = SpecificVertexQuery::single(id);
    db.index_property(Identifier::new("bar").unwrap()).unwrap();
    db.set_properties(q.clone(), Identifier::new("bar").unwrap(), &ijson!(true))
        .unwrap();
    let output = db
        .get(
            q.clone()
                .include()
                .with_property(Identifier::new("bar").unwrap())
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        output,
        vec![
            QueryOutputValue::Vertices(vec![Vertex::new(id, Identifier::new("foo").unwrap())]),
            QueryOutputValue::Vertices(vec![Vertex::new(id, Identifier::new("foo").unwrap())]),
        ]
    );
    let output = db
        .get(
            q.include()
                .with_property_equal_to(Identifier::new("bar").unwrap(), ijson!(true))
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        output,
        vec![
            QueryOutputValue::Vertices(vec![Vertex::new(id, Identifier::new("foo").unwrap())]),
            QueryOutputValue::Vertices(vec![Vertex::new(id, Identifier::new("foo").unwrap())]),
        ]
    );
}
