use crate::errors::{Error, Result};
use crate::util::{extract_count, extract_edge_properties, extract_edges, extract_vertex_properties, extract_vertices};
use crate::{models, CountQueryExt, Database, Datastore, QueryExt};

use uuid::Uuid;

pub(crate) fn create_edge_from<D: Datastore>(db: &Database<D>, outbound_id: Uuid) -> Result<Uuid> {
    let inbound_vertex_t = models::Identifier::new("test_inbound_vertex_type")?;
    let inbound_v = models::Vertex::new(inbound_vertex_t);
    db.create_vertex(&inbound_v)?;
    let edge_t = models::Identifier::new("test_edge_type")?;
    let edge = models::Edge::new(outbound_id, edge_t, inbound_v.id);
    db.create_edge(&edge)?;
    Ok(inbound_v.id)
}

pub(crate) fn create_edges<D: Datastore>(db: &Database<D>) -> Result<(Uuid, [Uuid; 5])> {
    let outbound_vertex_t = models::Identifier::new("test_outbound_vertex_type")?;
    let outbound_v = models::Vertex::new(outbound_vertex_t);
    db.create_vertex(&outbound_v)?;
    let inbound_ids: [Uuid; 5] = [
        create_edge_from(db, outbound_v.id)?,
        create_edge_from(db, outbound_v.id)?,
        create_edge_from(db, outbound_v.id)?,
        create_edge_from(db, outbound_v.id)?,
        create_edge_from(db, outbound_v.id)?,
    ];

    Ok((outbound_v.id, inbound_ids))
}

pub(crate) fn get_vertices<D: Datastore, Q: Into<models::Query>>(
    db: &Database<D>,
    q: Q,
) -> Result<Vec<models::Vertex>> {
    Ok(extract_vertices(db.get(q)?).unwrap())
}

pub(crate) fn get_vertex_count<D: Datastore>(db: &Database<D>) -> Result<u64> {
    Ok(extract_count(db.get(models::AllVertexQuery.count().unwrap())?).unwrap())
}

pub(crate) fn get_edges<D: Datastore, Q: Into<models::Query>>(db: &Database<D>, q: Q) -> Result<Vec<models::Edge>> {
    Ok(extract_edges(db.get(q)?).unwrap())
}

pub(crate) fn get_edge_count<D: Datastore>(
    db: &Database<D>,
    id: Uuid,
    t: Option<models::Identifier>,
    direction: models::EdgeDirection,
) -> Result<u64> {
    let q = models::SpecificVertexQuery::single(id);

    let q = match direction {
        models::EdgeDirection::Outbound => q.outbound().unwrap(),
        models::EdgeDirection::Inbound => q.inbound().unwrap(),
    };

    let q: models::Query = if let Some(t) = t {
        q.t(t).count().unwrap().into()
    } else {
        q.count().unwrap().into()
    };

    Ok(extract_count(db.get(q)?).unwrap())
}

pub(crate) fn get_vertex_properties<D: Datastore>(
    db: &Database<D>,
    q: models::PipePropertyQuery,
) -> Result<Vec<models::VertexProperty>> {
    let props = extract_vertex_properties(db.get(q)?).unwrap();
    if props.len() > 1 {
        Err(Error::Unsupported)
    } else {
        let iter = props.into_iter().flat_map(|vps| {
            vps.props
                .into_iter()
                .map(move |vp| models::VertexProperty::new(vps.vertex.id, vp.value))
        });
        Ok(iter.collect())
    }
}

pub(crate) fn get_all_vertex_properties<D: Datastore, Q: Into<models::Query>>(
    db: &Database<D>,
    q: Q,
) -> Result<Vec<models::VertexProperties>> {
    // `QueryExt::properties()` not used here because this function is not
    // generic in order to keep this object safe.
    let props_query = models::PipePropertyQuery::new(Box::new(q.into()))?;
    let props = extract_vertex_properties(db.get(props_query)?).unwrap();
    Ok(props)
}

pub(crate) fn get_edge_properties<D: Datastore>(
    db: &Database<D>,
    q: models::PipePropertyQuery,
) -> Result<Vec<models::EdgeProperty>> {
    let props = extract_edge_properties(db.get(q)?).unwrap();
    if props.len() > 1 {
        Err(Error::Unsupported)
    } else {
        let iter = props.into_iter().flat_map(move |eps| {
            let iter = eps
                .props
                .into_iter()
                .map(|ep| models::EdgeProperty::new(eps.edge.clone(), ep.value));
            iter.collect::<Vec<models::EdgeProperty>>()
        });
        Ok(iter.collect())
    }
}

pub(crate) fn get_all_edge_properties<D: Datastore, Q: Into<models::Query>>(
    db: &Database<D>,
    q: Q,
) -> Result<Vec<models::EdgeProperties>> {
    // `QueryExt::properties()` not used here because this function is not
    // generic in order to keep this object safe.
    let props_query = models::PipePropertyQuery::new(Box::new(q.into()))?;
    Ok(extract_edge_properties(db.get(props_query)?).unwrap())
}
