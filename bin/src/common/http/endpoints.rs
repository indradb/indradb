use super::util::*;
use super::context;
use super::models::*;
use indradb::{EdgeDirection, EdgeKey, EdgeQuery, Transaction, Type, Vertex, VertexQuery, Edge, VertexMetadata, EdgeMetadata};
use iron::headers::{ContentType, Encoding, Headers, TransferEncoding};
use iron::prelude::*;
use iron::status;
use iron::typemap::TypeMap;
use script;
use serde_json::value::Value as JsonValue;
use std::thread::spawn;
use uuid::Uuid;
use juniper::{FieldResult, FieldError, ID, RootNode};

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

pub struct RootQuery;

graphql_object!(RootQuery: context::Context |&self| {
    field api_version() -> &str {
        "1.0"
    }

    field query(&executor, q: InputRootQuery) -> FieldResult<Vec<OutputItem>> {
        let trans = &executor.context().trans;

        let results: FieldResult<Vec<Vec<OutputItem>>> = q.queries()?.into_iter().map(|q| -> FieldResult<Vec<OutputItem>> {
            match q {
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
        }).collect();

        Ok(results?.into_iter().flat_map(|v| v).collect())
    }

    field vertex_count(&executor) -> FieldResult<String> {
        let trans = &executor.context().trans;
        Ok(trans.get_vertex_count()?.to_string())
    }

    field edge_count(&executor, id: Uuid, type_filter: Option<InputType>, direction: InputEdgeDirection) -> FieldResult<String> {
        let type_filter = type_filter.map(FieldResult::<Type>::from).transpose()?;
        let direction = EdgeDirection::from(direction);
        let trans = &executor.context().trans;
        Ok(trans.get_edge_count(id, type_filter.as_ref(), direction)?.to_string())
    }
});

pub struct RootMutation;

graphql_object!(RootMutation: context::Context |&self| {
    field create_vertex(&executor, vertex: InputVertex) -> FieldResult<bool> {
        let trans = &executor.context().trans;
        let vertex = FieldResult::<Vertex>::from(vertex)?;
        Ok(trans.create_vertex(&vertex)?)
    }

    field create_vertex_from_type(&executor, t: InputType) -> FieldResult<ID> {
        let trans = &executor.context().trans;
        let t = FieldResult::<Type>::from(t)?;
        let id = trans.create_vertex_from_type(t)?;
        Ok(ID::from(id.hyphenated().to_string()))
    }

    field create_edge(&executor, key: InputEdgeKey) -> FieldResult<bool> {
        let trans = &executor.context().trans;
        let key = FieldResult::<EdgeKey>::from(key)?;
        Ok(trans.create_edge(&key)?)
    }

    field delete(&executor, q: InputRootQuery) -> FieldResult<()> {
        // let trans = &executor.context().trans;
        // trans.delete_vertices(&q)?;
        // Ok(())
        unimplemented!();
    }

    field set_metadata(&executor, q: InputRootQuery, value: String) -> FieldResult<()> {
        // let value_json: JsonValue = serde_json::from_str(&value)?;
        // let trans = &executor.context().trans;
        // trans.set_vertex_metadata(&q, &name, &value_json)?;
        // Ok(())
        unimplemented!();
    }
});

pub type Schema = RootNode<'static, RootQuery, RootMutation>;
