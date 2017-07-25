use iron::prelude::*;
use iron::status;
use braid::{Transaction, Error, EdgeKey, VertexQuery, EdgeQuery};
use common::ProxyTransaction;
use serde_json::value::Value as JsonValue;
use serde_json;
use serde::ser::Serialize;
use std::u16;
use super::util::*;
use uuid::Uuid;

pub fn transaction(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let mut idx: u16 = 0;
    let mut jsonable_res: Vec<JsonValue> = Vec::new();

    if let JsonValue::Array(items) = read_required_json(&mut req.body)? {
        for item in items {
            if let JsonValue::Object(obj) = item {
                let action = match get_required_json_string_param(&obj, "action") {
                    Ok(value) => value,
                    Err(err) => {
                        let message = format!("Item #{}: {}", idx, err);
                        return Err(create_iron_error(status::BadRequest, message));
                    }
                };

                let result: Result<JsonValue, IronError> = match &action[..] {
                    "create_vertex" => create_vertex(&trans, &obj),
                    "get_vertices" => get_vertices(&trans, &obj),
                    "delete_vertices" => delete_vertices(&trans, &obj),

                    "create_edge" => create_edge(&trans, &obj),
                    "get_edges" => get_edges(&trans, &obj),
                    "delete_edges" => delete_edges(&trans, &obj),
                    "get_edge_count" => get_edge_count(&trans, &obj),

                    "run_script" => {
                        let account_id = get_account_id(req);
                        run_script(&trans, &obj, account_id)
                    }

                    _ => {
                        Err(create_iron_error(
                            status::BadRequest,
                            "Unknown action".to_string(),
                        ))
                    }
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
    } else {
        return Err(create_iron_error(
            status::BadRequest,
            "Request body should be an array".to_string(),
        ));
    }

    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &jsonable_res))
}

fn create_vertex(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let t = get_required_json_type_param(item, "type")?;
    execute_item(trans.create_vertex(t))
}

fn get_vertices(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_required_json_obj_param::<VertexQuery>(item, "query")?;
    execute_item(trans.get_vertices(q))
}

fn delete_vertices(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_required_json_obj_param::<VertexQuery>(item, "query")?;
    execute_item(trans.delete_vertices(q))
}

fn create_edge(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let key = get_required_json_obj_param::<EdgeKey>(item, "key")?;
    let weight = get_required_json_weight_param(item, "weight")?;
    execute_item(trans.create_edge(key, weight))
}

fn get_edges(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_required_json_obj_param::<EdgeQuery>(item, "query")?;
    execute_item(trans.get_edges(q))
}

fn delete_edges(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_required_json_obj_param::<EdgeQuery>(item, "query")?;
    execute_item(trans.delete_edges(q))
}

fn get_edge_count(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
) -> Result<JsonValue, IronError> {
    let q = get_required_json_obj_param::<EdgeQuery>(item, "query")?;
    execute_item(trans.get_edge_count(q))
}

fn run_script(
    trans: &ProxyTransaction,
    item: &serde_json::Map<String, JsonValue>,
    account_id: Uuid,
) -> Result<JsonValue, IronError> {
    let name: String = get_required_json_string_param(item, "name")?;

    match item.get("payload") {
        Some(val) => execute_script(name, &val, trans, account_id),
        None => execute_script(name, &JsonValue::Null, trans, account_id),
    }
}

fn execute_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
    let result = datastore_request(result)?;

    match serde_json::to_value(&result) {
        Ok(val) => Ok(val),
        Err(err) => Err(create_iron_error(
            status::InternalServerError,
            format!("Could not serialize results: {}", err),
        )),
    }
}
