use super::context::Context;
use super::models::*;
use indradb::{EdgeDirection, EdgeKey, Transaction, Type, Vertex};
use juniper::{FieldResult, RootNode, ID};
use serde_json;
use serde_json::value::Value as JsonValue;
use std::str::FromStr;
use uuid::Uuid;

pub struct RootQuery;

graphql_object!(RootQuery: Context |&self| {
    field api_version() -> &str {
        "1.0"
    }

    field query(&executor, q: InputRootQuery) -> FieldResult<Vec<OutputItem>> {
        let trans = &executor.context().trans;

        match q.to_indradb_query()? {
            Query::Vertex(q) => {
                let vertices = trans.get_vertices(&q)?;
                Ok(vertices.into_iter().map(OutputItem::from).collect())
            },
            Query::Edge(q) => {
                let edges = trans.get_edges(&q)?;
                Ok(edges.into_iter().map(OutputItem::from).collect())
            },
            Query::VertexMetadata(q, name) => {
                let vertex_metadata = trans.get_vertex_metadata(&q, &name)?;
                Ok(vertex_metadata.into_iter().map(OutputItem::from).collect())
            },
            Query::EdgeMetadata(q, name) => {
                let edge_metadata = trans.get_edge_metadata(&q, &name)?;
                Ok(edge_metadata.into_iter().map(OutputItem::from).collect())
            }
        }
    }

    field vertex_count(&executor) -> FieldResult<String> {
        let trans = &executor.context().trans;
        Ok(trans.get_vertex_count()?.to_string())
    }

    field edge_count(&executor, id: ID, type_filter: Option<String>, direction: InputEdgeDirection) -> FieldResult<String> {
        let id = Uuid::from_str(&id)?;
        let type_filter = type_filter.map(Type::new).transpose()?;
        let direction = EdgeDirection::from(direction);
        let trans = &executor.context().trans;
        Ok(trans.get_edge_count(id, type_filter.as_ref(), direction)?.to_string())
    }
});

pub struct RootMutation;

graphql_object!(RootMutation: Context |&self| {
    field create_vertex(&executor, vertex: InputVertex) -> FieldResult<bool> {
        let trans = &executor.context().trans;
        let vertex = FieldResult::<Vertex>::from(vertex)?;
        Ok(trans.create_vertex(&vertex)?)
    }

    field create_vertex_from_type(&executor, t: String) -> FieldResult<ID> {
        let trans = &executor.context().trans;
        let t = Type::new(t)?;
        let id = trans.create_vertex_from_type(t)?;
        Ok(ID::from(id.hyphenated().to_string()))
    }

    field create_edge(&executor, key: InputEdgeKey) -> FieldResult<bool> {
        let trans = &executor.context().trans;
        let key = FieldResult::<EdgeKey>::from(key)?;
        Ok(trans.create_edge(&key)?)
    }

    field delete(&executor, q: InputRootQuery) -> FieldResult<bool> {
        let trans = &executor.context().trans;

        match q.to_indradb_query()? {
            Query::Vertex(q) => {
                trans.delete_vertices(&q)?;
            },
            Query::Edge(q) => {
                trans.delete_edges(&q)?;
            },
            Query::VertexMetadata(q, name) => {
                trans.delete_vertex_metadata(&q, &name)?;
            },
            Query::EdgeMetadata(q, name) => {
                trans.delete_edge_metadata(&q, &name)?;
            }
        }

        Ok(true)
    }

    field set_metadata(&executor, q: InputRootQuery, value: String) -> FieldResult<bool> {
        let value_json: JsonValue = serde_json::from_str(&value)?;
        let trans = &executor.context().trans;

        match q.to_indradb_query()? {
            Query::VertexMetadata(q, name) => {
                trans.set_vertex_metadata(&q, &name, &value_json)?;
            },
            Query::EdgeMetadata(q, name) => {
                trans.set_edge_metadata(&q, &name, &value_json)?;
            },
            _ => {
                return Err("The query is not for metadata".into());
            }
        }

        Ok(true)
    }
});

pub type Schema = RootNode<'static, RootQuery, RootMutation>;
