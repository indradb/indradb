use super::util::*;
use super::context;
use indradb::{EdgeDirection, EdgeKey, EdgeQuery, Transaction, Type, Vertex, VertexQuery, Edge, VertexMetadata, EdgeMetadata};
use iron::headers::{ContentType, Encoding, Headers, TransferEncoding};
use iron::prelude::*;
use iron::status;
use iron::typemap::TypeMap;
use script;
use serde_json;
use serde_json::value::Value as JsonValue;
use std::thread::spawn;
use uuid::Uuid;
use juniper;
use juniper::FieldResult;

pub fn script(req: &mut Request) -> IronResult<Response> {
    // Get the inputs
    let name: String = get_url_param(req, "name")?;
    let payload = read_json(&mut req.body)?.unwrap_or_else(|| JsonValue::Null);
    let (path, contents) = get_script_file(name)?;

    match script::execute(&contents, &path, payload) {
        Ok(value) => Ok(to_response(status::Ok, &value)),
        Err(err) => {
            let error_message = format!("Script failed: {:?}", err);
            Err(create_iron_error(
                status::InternalServerError,
                error_message,
            ))
        }
    }
}

pub fn mapreduce(req: &mut Request) -> IronResult<Response> {
    // Get the inputs
    let name: String = get_url_param(req, "name")?;
    let payload = read_json(&mut req.body)?.unwrap_or_else(|| JsonValue::Null);
    let (path, contents) = get_script_file(name)?;

    // Construct a response
    let mut hs = Headers::new();
    hs.set(ContentType(get_json_mime()));
    hs.set(TransferEncoding(vec![Encoding::Chunked]));

    let (sender, receiver) = script::bounded(1);

    spawn(move || {
        script::execute_mapreduce(contents, path, payload, sender);
    });

    Ok(Response {
        status: Some(status::Ok),
        headers: hs,
        extensions: TypeMap::new(),
        body: Some(Box::new(receiver)),
    })
}

pub struct GraphQLQuery;

graphql_object!(RootQuery: context::Context |&self| {
    field api_version() -> &str {
        "1.0"
    }

    field vertices(&executor, q: VertexQuery) -> FieldResult<Vec<Vertex>> {
        let trans = &executor.context().trans;
        Ok(trans.get_vertices(&q)?)
    }

    field vertex_count(&executor) -> FieldResult<String> {
        let trans = &executor.context().trans;
        Ok(trans.get_vertex_count()?.to_string())
    }

    field edges(&executor, q: EdgeQuery) -> FieldResult<Vec<Edge>> {
        let trans = &executor.context().trans;
        Ok(trans.get_edges(&q)?)
    }

    field edge_count(&executor, id: Uuid, type_filter: Option<Type>, direction: EdgeDirection) -> FieldResult<String> {
        let trans = &executor.context().trans;
        Ok(trans.get_edge_count(id, type_filter.as_ref(), direction)?.to_string())
    }

    field vertex_metadata(&executor, q: VertexQuery, name: String) -> FieldResult<Vec<VertexMetadata>> {
        let trans = &executor.context().trans;
        Ok(trans.get_vertex_metadata(&q, &name)?)
    }

    field edge_metadata(&executor, q: EdgeQuery, name: String) -> FieldResult<Vec<EdgeMetadata>> {
        let trans = &executor.context().trans;
        Ok(trans.get_edge_metadata(&q, &name)?)
    }
});

pub struct RootMutation;

graphql_object!(RootMutation: context::Context |&self| {
    field create_vertex(&executor, vertex: Vertex) -> FieldResult<bool> {
        let trans = &executor.context().trans;
        Ok(trans.create_vertex(&vertex)?)
    }

    field create_vertex_from_type(&executor, t: Type) -> FieldResult<Uuid> {
        let trans = &executor.context().trans;
        Ok(trans.create_vertex_from_type(t)?)
    }

    field delete_vertices(&executor, q: VertexQuery) -> FieldResult<()> {
        let trans = &executor.context().trans;
        trans.delete_vertices(&q)?;
        Ok(())
    }

    field create_edge(&executor, key: EdgeKey) -> FieldResult<bool> {
        let trans = &executor.context().trans;
        Ok(trans.create_edge(&key)?)
    }

    field delete_edges(&executor, q: EdgeQuery) -> FieldResult<()> {
        let trans = &executor.context().trans;
        trans.delete_edges(&q)?;
        Ok(())
    }

    field set_vertex_metadata(&executor, q: VertexQuery, name: String, value: String) -> FieldResult<()> {
        let value_json: JsonValue = serde_json::from_str(&value)?;
        let trans = &executor.context().trans;
        trans.set_vertex_metadata(&q, &name, &value_json)?;
        Ok(())
    }

    field delete_vertex_metadata(&executor, q: VertexQuery, name: String) -> FieldResult<()> {
        let trans = &executor.context().trans;
        trans.delete_vertex_metadata(&q, &name)?;
        Ok(())
    }

    field set_edge_metadata(&executor, q: EdgeQuery, name: String, value: String) -> FieldResult<()> {
        let value_json: JsonValue = serde_json::from_str(&value)?;
        let trans = &executor.context().trans;
        trans.set_edge_metadata(&q, &name, &value_json)?;
        Ok(())
    }

    field delete_edge_metadata(&executor, q: EdgeQuery, name: String) -> FieldResult<()> {
        let trans = &executor.context().trans;
        trans.delete_edge_metadata(&q, &name)?;
        Ok(())
    }
});
