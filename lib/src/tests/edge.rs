use super::super::{
    Datastore, EdgeDirection, Edge, EdgeQueryExt, SpecificEdgeQuery, SpecificVertexQuery, Transaction,
    VertexQueryExt,
};
use super::util::{create_edge_from, create_edges};
use models;
use uuid::Uuid;

pub fn should_get_a_valid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();

    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t.clone());
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t.clone(), inbound_v.id);
    trans.create_edge(&edge).unwrap();

    let e = trans.get_edges(SpecificEdgeQuery::single(edge.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].outbound_id, outbound_v.id);
    assert_eq!(e[0].t, edge_t);
    assert_eq!(e[0].inbound_id, inbound_v.id);
}

pub fn should_not_get_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();

    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t.clone());
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();

    let e = trans
        .get_edges(SpecificEdgeQuery::single(Edge::new(
            outbound_v.id,
            edge_t.clone(),
            Uuid::default(),
        )))
        .unwrap();;
    assert_eq!(e.len(), 0);
    let e = trans
        .get_edges(SpecificEdgeQuery::single(Edge::new(
            Uuid::default(),
            edge_t.clone(),
            inbound_v.id,
        )))
        .unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_create_a_valid_edge<D: Datastore>(datastore: &mut D) {
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let trans = datastore.transaction().unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t.clone());
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();

    // Set the edge and check
    let edge = models::Edge::new(outbound_v.id, edge_t.clone(), inbound_v.id);
    trans.create_edge(&edge).unwrap();
    let e = trans.get_edges(SpecificEdgeQuery::single(edge.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // `create_edge` should support the ability of updating an existing edge
    // - test for that
    trans.create_edge(&edge).unwrap();

    // First check that getting a single edge will still...get a single edge
    let e = trans.get_edges(SpecificEdgeQuery::single(edge.clone())).unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);

    // REGRESSION: Second check that getting an edge range will only fetch a
    // single edge
    let e = trans
        .get_edges(SpecificVertexQuery::single(outbound_v.id).outbound(10))
        .unwrap();
    assert_eq!(e.len(), 1);
    assert_eq!(edge, e[0]);
}

pub fn should_not_create_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t.clone(), Uuid::default());
    let result = trans.create_edge(&edge);
    assert_eq!(result.unwrap(), false);
}

pub fn should_delete_a_valid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    let inbound_v = models::Vertex::new(vertex_t.clone());
    trans.create_vertex(&outbound_v).unwrap();
    trans.create_vertex(&inbound_v).unwrap();

    let edge_t = models::Type::new("test_edge_type").unwrap();
    let edge = models::Edge::new(outbound_v.id, edge_t.clone(), inbound_v.id);
    trans.create_edge(&edge).unwrap();
    trans.delete_edges(SpecificEdgeQuery::single(edge.clone())).unwrap();
    let e = trans.get_edges(SpecificEdgeQuery::single(edge.clone())).unwrap();
    assert_eq!(e.len(), 0);
}

pub fn should_not_delete_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_edge_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t.clone());
    trans.create_vertex(&outbound_v).unwrap();
    let edge_t = models::Type::new("test_edge_type").unwrap();
    trans
        .delete_edges(SpecificEdgeQuery::single(Edge::new(
            outbound_v.id,
            edge_t,
            Uuid::default(),
        )))
        .unwrap();
}

pub fn should_get_an_edge_count<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type").unwrap();
    let count = trans
        .get_edge_count(outbound_id, Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_with_no_type<D: Datastore>(datastore: &mut D) {
    let (outbound_id, _) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let count = trans
        .get_edge_count(outbound_id, None, EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 5);
}

pub fn should_get_an_edge_count_for_an_invalid_edge<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let t = models::Type::new("test_edge_type").unwrap();
    let count = trans
        .get_edge_count(Uuid::default(), Some(&t), EdgeDirection::Outbound)
        .unwrap();
    assert_eq!(count, 0);
}

pub fn should_get_an_inbound_edge_count<D: Datastore>(datastore: &mut D) {
    let (_, inbound_ids) = create_edges(datastore);
    let trans = datastore.transaction().unwrap();
    let count = trans
        .get_edge_count(inbound_ids[0], None, EdgeDirection::Inbound)
        .unwrap();
    assert_eq!(count, 1);
}

pub fn should_get_edges_piped<D: Datastore>(datastore: &mut D) {
    let trans = datastore.transaction().unwrap();
    let vertex_t = models::Type::new("test_vertex_type").unwrap();
    let outbound_v = models::Vertex::new(vertex_t);
    trans.create_vertex(&outbound_v).unwrap();

    let inbound_id = create_edge_from(&trans, outbound_v.id);

    let query_1 = SpecificVertexQuery::single(outbound_v.id)
        .outbound(1)
        .t(models::Type::new("test_edge_type").unwrap());
    let range = trans.get_edges(query_1.clone()).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(outbound_v.id, models::Type::new("test_edge_type").unwrap(), inbound_id)
    );

    let query_2 = query_1
        .inbound(1)
        .inbound(1)
        .t(models::Type::new("test_edge_type").unwrap());
    let range = trans.get_edges(query_2).unwrap();
    assert_eq!(range.len(), 1);
    assert_eq!(
        range[0],
        models::Edge::new(outbound_v.id, models::Type::new("test_edge_type").unwrap(), inbound_id)
    );
}
