use super::util;
use crate::{
    AllVertexQuery, Database, Datastore, Edge, Identifier, QueryExt, QueryOutputValue, SpecificVertexQuery, Vertex,
};

pub fn should_get_with_include_query<D: Datastore>(db: &Database<D>) {
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
            QueryOutputValue::Vertices(vec![Vertex::with_id(
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
