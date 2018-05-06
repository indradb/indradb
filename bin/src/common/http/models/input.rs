use chrono::DateTime;
use chrono::offset::Utc;
use indradb;
use juniper::{FieldResult, ID};
use std::str::FromStr;
use uuid::Uuid;

fn build_from_vertex_query(
    q: indradb::VertexQuery,
    metadata: Option<Vec<String>>,
    outbound: Option<InputPipeEdgeQuery>,
    inbound: Option<InputPipeEdgeQuery>,
) -> FieldResult<Vec<Query>> {
    let mut queries = vec![];

    if let Some(metadata) = metadata {
        for metadata_name in &metadata {
            queries.push(Query::VertexMetadata(q.clone(), metadata_name.clone()));
        }
    }

    if let Some(outbound) = outbound {
        if outbound.limit < 0 {
            return Err("limit must be non-negative".into());
        }

        let type_filter = outbound.type_filter.map(indradb::Type::new).transpose()?;
        let base_query = q.clone().outbound_edges(
            type_filter,
            outbound.high_filter,
            outbound.low_filter,
            outbound.limit as u32,
        );
        let inner_outbound = outbound.outbound.map(|o| *o);
        let inner_inbound = outbound.inbound.map(|o| *o);
        queries.extend(build_from_edge_query(
            base_query,
            outbound.metadata,
            inner_outbound,
            inner_inbound,
        )?);
    }

    if let Some(inbound) = inbound {
        if inbound.limit < 0 {
            return Err("limit must be non-negative".into());
        }

        let type_filter = inbound.type_filter.map(indradb::Type::new).transpose()?;
        let base_query = q.clone().inbound_edges(
            type_filter,
            inbound.high_filter,
            inbound.low_filter,
            inbound.limit as u32,
        );
        let inner_outbound = inbound.outbound.map(|o| *o);
        let inner_inbound = inbound.inbound.map(|o| *o);
        queries.extend(build_from_edge_query(
            base_query,
            inbound.metadata,
            inner_outbound,
            inner_inbound,
        )?);
    }

    if queries.len() == 0 {
        queries.push(Query::Vertex(q));
    }

    Ok(queries)
}

fn build_from_edge_query(
    q: indradb::EdgeQuery,
    metadata: Option<Vec<String>>,
    outbound: Option<InputPipeVertexQuery>,
    inbound: Option<InputPipeVertexQuery>,
) -> FieldResult<Vec<Query>> {
    let mut queries = vec![];

    if let Some(metadata) = metadata {
        for metadata_name in &metadata {
            queries.push(Query::EdgeMetadata(q.clone(), metadata_name.to_string()));
        }
    }

    if let Some(outbound) = outbound {
        if outbound.limit < 0 {
            return Err("limit must be non-negative".into());
        }

        let base_query = q.clone().outbound_vertices(outbound.limit as u32);
        let inner_outbound = outbound.outbound.map(|o| *o);
        let inner_inbound = outbound.inbound.map(|o| *o);
        queries.extend(build_from_vertex_query(
            base_query,
            outbound.metadata,
            inner_outbound,
            inner_inbound,
        )?);
    }

    if let Some(inbound) = inbound {
        if inbound.limit < 0 {
            return Err("limit must be non-negative".into());
        }

        let base_query = q.clone().inbound_vertices(inbound.limit as u32);
        let inner_outbound = inbound.outbound.map(|o| *o);
        let inner_inbound = inbound.inbound.map(|o| *o);
        queries.extend(build_from_vertex_query(
            base_query,
            inbound.metadata,
            inner_outbound,
            inner_inbound,
        )?);
    }

    if queries.len() == 0 {
        queries.push(Query::Edge(q));
    }

    Ok(queries)
}

#[graphql(description = "Represents a uniquely identifiable key to an edge.")]
#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputEdgeKey {
    #[graphql(description = "The id of the outbound vertex.")]
    pub outbound_id: ID,

    #[graphql(description = "The type of the edge.")]
    pub t: String,

    #[graphql(description = "The id of the inbound vertex.")]
    pub inbound_id: ID,
}

impl From<InputEdgeKey> for FieldResult<indradb::EdgeKey> {
    fn from(key: InputEdgeKey) -> FieldResult<indradb::EdgeKey> {
        let outbound_id = Uuid::parse_str(&key.outbound_id)?;
        let t = indradb::Type::new(key.t)?;
        let inbound_id = Uuid::parse_str(&key.inbound_id)?;
        Ok(indradb::EdgeKey::new(outbound_id, t, inbound_id))
    }
}

#[graphql(description = "Specifies what kind of items should be piped from one type of query to another.")]
#[derive(Clone, Debug, Copy, GraphQLEnum)]
pub enum InputEdgeDirection {
    Outbound,
    Inbound,
}

impl From<InputEdgeDirection> for indradb::EdgeDirection {
    fn from(direction: InputEdgeDirection) -> indradb::EdgeDirection {
        match direction {
            InputEdgeDirection::Outbound => indradb::EdgeDirection::Outbound,
            InputEdgeDirection::Inbound => indradb::EdgeDirection::Inbound,
        }
    }
}

#[graphql(description = "A query for vertices, edges, vertex metadata, and edge metadata.")]
#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputRootQuery {
    vertex_range: Option<InputVertexRangeQuery>,
    vertices: Option<InputVerticesQuery>,
    edges: Option<InputEdgesQuery>,
}

impl InputRootQuery {
    pub fn queries(self) -> FieldResult<Vec<Query>> {
        match (self.vertex_range, self.vertices, self.edges) {
            (Some(vertex_range), None, None) => {
                if vertex_range.limit < 0 {
                    return Err("limit must be non-negative".into());
                }

                let base_query = indradb::VertexQuery::All {
                    start_id: vertex_range
                        .start_id
                        .map(|i| Uuid::from_str(&i))
                        .transpose()?,
                    limit: vertex_range.limit as u32,
                };

                build_from_vertex_query(
                    base_query,
                    vertex_range.metadata,
                    vertex_range.outbound,
                    vertex_range.inbound,
                )
            }
            (None, Some(vertices), None) => {
                let ids: Result<Vec<Uuid>, _> = vertices.ids.iter().map(|i| Uuid::from_str(&i)).collect();

                let base_query = indradb::VertexQuery::Vertices { ids: ids? };

                build_from_vertex_query(
                    base_query,
                    vertices.metadata,
                    vertices.outbound,
                    vertices.inbound,
                )
            }
            (None, None, Some(edges)) => {
                let keys: Result<Vec<indradb::EdgeKey>, _> = edges
                    .keys
                    .into_iter()
                    .map(FieldResult::<indradb::EdgeKey>::from)
                    .collect();

                let base_query = indradb::EdgeQuery::Edges { keys: keys? };

                build_from_edge_query(base_query, edges.metadata, edges.outbound, edges.inbound)
            }
            (None, None, None) => Err("No query".into()),
            _ => Err("Only one query can be set".into()),
        }
    }
}

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputVertexRangeQuery {
    pub start_id: Option<ID>,
    pub limit: i32,
    pub metadata: Option<Vec<String>>,
    pub outbound: Option<InputPipeEdgeQuery>,
    pub inbound: Option<InputPipeEdgeQuery>,
}

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputVerticesQuery {
    pub ids: Vec<ID>,
    pub metadata: Option<Vec<String>>,
    pub outbound: Option<InputPipeEdgeQuery>,
    pub inbound: Option<InputPipeEdgeQuery>,
}

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputEdgesQuery {
    pub keys: Vec<InputEdgeKey>,
    pub metadata: Option<Vec<String>>,
    pub outbound: Option<InputPipeVertexQuery>,
    pub inbound: Option<InputPipeVertexQuery>,
}

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputPipeVertexQuery {
    pub limit: i32,
    pub metadata: Option<Vec<String>>,
    pub outbound: Option<Box<InputPipeEdgeQuery>>,
    pub inbound: Option<Box<InputPipeEdgeQuery>>,
}

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputPipeEdgeQuery {
    pub type_filter: Option<String>,
    pub high_filter: Option<DateTime<Utc>>,
    pub low_filter: Option<DateTime<Utc>>,
    pub limit: i32,
    pub metadata: Option<Vec<String>>,
    pub outbound: Option<Box<InputPipeVertexQuery>>,
    pub inbound: Option<Box<InputPipeVertexQuery>>,
}

#[derive(Clone, Debug)]
pub enum Query {
    Vertex(indradb::VertexQuery),
    Edge(indradb::EdgeQuery),
    VertexMetadata(indradb::VertexQuery, String),
    EdgeMetadata(indradb::EdgeQuery, String),
}

#[graphql(description = "A vertex.")]
#[derive(Clone, Debug, GraphQLInputObject)]
pub struct InputVertex {
    #[graphql(description = "The id of the vertex.")]
    pub id: ID,

    #[graphql(description = "The type of the vertex.")]
    pub t: String,
}

impl From<InputVertex> for FieldResult<indradb::Vertex> {
    fn from(vertex: InputVertex) -> FieldResult<indradb::Vertex> {
        let id = Uuid::parse_str(&vertex.id)?;
        let t = indradb::Type::new(vertex.t)?;
        Ok(indradb::Vertex::with_id(id, t))
    }
}
