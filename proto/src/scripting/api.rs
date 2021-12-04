use super::converters;

use indradb::{Error, Transaction};

pub fn create_vertex_from_type<T: indradb::Transaction + Send + Sync + 'static>(
    trans: T,
    t: converters::Identifier,
) -> Result<converters::Uuid, Error> {
    Ok(converters::Uuid::new(trans.create_vertex_from_type(t.0)?))
}

pub fn get_vertices<T: indradb::Transaction + Send + Sync + 'static>(
    trans: T,
    q: converters::VertexQuery,
) -> Result<Vec<converters::Vertex>, Error> {
    Ok(trans
        .get_vertices(q.0)?
        .into_iter()
        .map(converters::Vertex::new)
        .collect())
}

pub fn delete_vertices<T: indradb::Transaction + Send + Sync + 'static>(
    trans: T,
    q: converters::VertexQuery,
) -> Result<(), Error> {
    trans.delete_vertices(q.0)
}

pub fn get_vertex_count<T: indradb::Transaction + Send + Sync + 'static>(trans: T, _: ()) -> Result<u64, Error> {
    trans.get_vertex_count()
}

pub fn create_edge<T: indradb::Transaction + Send + Sync + 'static>(
    trans: T,
    key: converters::EdgeKey,
) -> Result<bool, Error> {
    Ok(trans.create_edge(&key.0)?)
}

pub fn get_edges<T: indradb::Transaction + Send + Sync + 'static>(
    trans: T,
    q: converters::EdgeQuery,
) -> Result<Vec<converters::Edge>, Error> {
    Ok(trans.get_edges(q.0)?.into_iter().map(converters::Edge::new).collect())
}

pub fn delete_edges<T: indradb::Transaction + Send + Sync + 'static>(
    trans: T,
    q: converters::EdgeQuery,
) -> Result<(), Error> {
    trans.delete_edges(q.0)?;
    Ok(())
}

pub fn get_edge_count<T: indradb::Transaction + Send + Sync + 'static>(
    trans: T,
    (id, type_filter, direction): (
        converters::Uuid,
        Option<converters::Identifier>,
        converters::EdgeDirection,
    ),
) -> Result<u64, Error> {
    Ok(trans.get_edge_count(id.0, type_filter.as_ref().map(|t| &t.0), direction.0)?)
}
