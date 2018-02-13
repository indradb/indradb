use iron::prelude::*;
use iron::status;
use indradb::{EdgeKey, EdgeQuery, Error, Transaction, Type, VertexQuery};
use common::ProxyTransaction;
use serde_json::value::Value as JsonValue;
use serde_json;
use serde::ser::Serialize;
use std::u16;
use regex;
use std::path::Path;
use statics;
use std::fs::File;
use script;
use std::io::Read;
use super::util::*;

lazy_static! {
    static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[\w-_]+(\.lua)?$").unwrap();
}

pub fn script(req: &mut Request) -> IronResult<Response> {
    let name: String = get_url_param(req, "name")?;

    if !SCRIPT_NAME_VALIDATOR.is_match(&name[..]) {
        return Err(create_iron_error(
            status::BadRequest,
            "Invalid script name".to_string(),
        ));
    }

    let payload: JsonValue = read_json(&mut req.body)?.unwrap_or_else(|| JsonValue::Null);
    let path = Path::new(&statics::SCRIPT_ROOT[..]).join(name);

    let mut file = File::open(&path)
        .map_err(|_| create_iron_error(status::NotFound, "Could not load script".to_string()))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|_| create_iron_error(status::NotFound, "Could not read script".to_string()))?;

    let value = script::run(&contents, &path, payload).map_err(|err| {
        let error_message = format!("Script failed: {:?}", err);
        create_iron_error(status::InternalServerError, error_message)
    })?;

    Ok(to_response(status::Ok, &value))
}

pub fn transaction(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let mut idx: u16 = 0;
    let mut jsonable_res: Vec<JsonValue> = Vec::new();
    let body: Vec<JsonValue> = read_json(&mut req.body)?.unwrap_or_else(|| Vec::new());

    for item in body {
        if let JsonValue::Object(obj) = item {
            let action = get_json_obj_value::<String>(&obj, "action").map_err(|err| {
                let message = format!("Item #{}: {}", idx, err);
                create_iron_error(status::BadRequest, message)
            })?;

            let result: Result<JsonValue, IronError> = match &action[..] {
                "create_vertex" => create_vertex(&trans, &obj),
                "get_vertices" => get_vertices(&trans, &obj),
                "delete_vertices" => delete_vertices(&trans, &obj),

                "create_edge" => create_edge(&trans, &obj),
                "get_edges" => get_edges(&trans, &obj),
                "delete_edges" => delete_edges(&trans, &obj),
                "get_edge_count" => get_edge_count(&trans, &obj),

                "get_global_metadata" => get_global_metadata(&trans, &obj),
                "set_global_metadata" => set_global_metadata(&trans, &obj),
                "delete_global_metadata" => delete_global_metadata(&trans, &obj),

                "get_vertex_metadata" => get_vertex_metadata(&trans, &obj),
                "set_vertex_metadata" => set_vertex_metadata(&trans, &obj),
                "delete_vertex_metadata" => delete_vertex_metadata(&trans, &obj),

                "get_edge_metadata" => get_edge_metadata(&trans, &obj),
                "set_edge_metadata" => set_edge_metadata(&trans, &obj),
                "delete_edge_metadata" => delete_edge_metadata(&trans, &obj),

                _ => Err(create_iron_error(
                    status::BadRequest,
                    "Unknown action".to_string(),
                )),
            };

            match result {
                Err(err) => {
                    let message = format!("Item #{}: {}", idx, err);
                    return Err(create_iron_error(status::BadRequest, message));
                }
                Ok(value) => {
                    jsonable_res.push(value);
                }
            }
        } else {
            return Err(create_iron_error(
                status::BadRequest,
                format!("Item #{}: Invalid type", idx),
            ));
        }

        idx += 1;
    }

    trans.commit().map_err(|err| create_iron_error(status::InternalServerError, format!("Could not commit transaction: {}", err)))?;
    Ok(to_response(status::Ok, &jsonable_res))
}

fn create_vertex(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let t = get_json_obj_value::<Type>(item, "type")?;
    execute_item(trans.create_vertex(t))
}

fn get_vertices(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    execute_item(trans.get_vertices(q))
}

fn delete_vertices(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    execute_item(trans.delete_vertices(q))
}

fn create_edge(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let key = get_json_obj_value::<EdgeKey>(item, "key")?;
    execute_item(trans.create_edge(key))
}

fn get_edges(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    execute_item(trans.get_edges(q))
}

fn delete_edges(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    execute_item(trans.delete_edges(q))
}

fn get_edge_count(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    execute_item(trans.get_edge_count(q))
}

fn get_global_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.get_global_metadata(name))
}

fn set_global_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let name = get_json_obj_value::<String>(item, "name")?;
    let value = get_json_obj_value::<JsonValue>(item, "value")?;
    execute_item(trans.set_global_metadata(name, value))
}

fn delete_global_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.delete_global_metadata(name))
}

fn get_vertex_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.get_vertex_metadata(q, name))
}

fn set_vertex_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    let value = get_json_obj_value::<JsonValue>(item, "value")?;
    execute_item(trans.set_vertex_metadata(q, name, value))
}

fn delete_vertex_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<VertexQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.delete_vertex_metadata(q, name))
}

fn get_edge_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.get_edge_metadata(q, name))
}

fn set_edge_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    let value = get_json_obj_value::<JsonValue>(item, "value")?;
    execute_item(trans.set_edge_metadata(q, name, value))
}

fn delete_edge_metadata(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_json_obj_value::<EdgeQuery>(item, "query")?;
    let name = get_json_obj_value::<String>(item, "name")?;
    execute_item(trans.delete_edge_metadata(q, name))
}

fn execute_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
    let value = result.map_err(|err| create_iron_error(status::InternalServerError, format!("{}", err)))?;

    Ok(
        serde_json::to_value(value).map_err(|err| {
            create_iron_error(
                status::InternalServerError,
                format!("Could not serialize results: {}", err),
            )
        })?,
    )
}
