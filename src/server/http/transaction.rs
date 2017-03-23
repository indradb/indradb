use iron::prelude::*;
use iron::status;
use braid::{Edge, Transaction, Error};
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
                    "get_vertices" => get_vertices(&trans, &obj),
                    "create_vertex" => create_vertex(&trans, &obj),
                    "set_vertices" => set_vertices(&trans, &obj),
                    "delete_vertices" => delete_vertices(&trans, &obj),
                    
                    "get_edge" => get_edge(&trans, &obj),
                    "set_edge" => set_edge(&trans, &obj),
                    "delete_edge" => delete_edge(&trans, &obj),

                    "get_edge_count" => get_edge_count(&trans, &obj),
                    "get_edge_range" => get_edge_range(&trans, &obj),

                    "get_reversed_edge_count" => get_reversed_edge_count(&trans, &obj),
                    "get_reversed_edge_range" => get_reversed_edge_range(&trans, &obj),

                    "run_script" => {
                        let account_id = get_account_id(req);
                        run_script(&trans, &obj, account_id)
                    },

                    _ => {
                        Err(create_iron_error(status::BadRequest, "Unknown action".to_string()))
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
                return Err(create_iron_error(status::BadRequest, format!("Item #{}: Invalid type", idx)))
            }

            idx += 1;
        }
    } else {
        return Err(create_iron_error(status::BadRequest, "Request body should be an array".to_string()))
    }

    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &jsonable_res))
}

fn get_vertices(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let q = get_required_json_vertex_query_param(item, "query")?;
    execute_item(trans.get_vertices(q))
}

fn create_vertex(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let t = get_required_json_type_param(item, "type")?;
    execute_item(trans.create_vertex(t))
}

fn set_vertices(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let q = get_required_json_vertex_query_param(item, "query")?;
    let t = get_required_json_type_param(item, "type")?;
    execute_item(trans.set_vertices(q, t))
}

fn delete_vertices(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let q = get_required_json_vertex_query_param(item, "query")?;
    execute_item(trans.delete_vertices(q))
}

fn get_edge(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let outbound_id = get_required_json_uuid_param(item, "outbound_id")?;
    let t = get_required_json_type_param(item, "type")?;
    let inbound_id = get_required_json_uuid_param(item, "inbound_id")?;
    execute_item(trans.get_edge(outbound_id, t, inbound_id))
}

fn set_edge(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let outbound_id = get_required_json_uuid_param(item, "outbound_id")?;
    let t = get_required_json_type_param(item, "type")?;
    let inbound_id = get_required_json_uuid_param(item, "inbound_id")?;
    let weight = get_required_json_weight_param(item, "weight")?;
    execute_item(trans.set_edge(Edge::new_with_current_datetime(outbound_id, t, inbound_id, weight)))
}

fn delete_edge(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let outbound_id = get_required_json_uuid_param(item, "outbound_id")?;
    let t = get_required_json_type_param(item, "type")?;
    let inbound_id = get_required_json_uuid_param(item, "inbound_id")?;
    execute_item(trans.delete_edge(outbound_id, t, inbound_id))
}

fn get_edge_count(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let outbound_id = get_required_json_uuid_param(item, "outbound_id")?;
    let t = get_optional_json_type_param(item, "type")?;
    execute_item(trans.get_edge_count(outbound_id, t))
}

fn get_edge_range(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let outbound_id = get_required_json_uuid_param(item, "outbound_id")?;
    let t = get_optional_json_type_param(item, "type")?;
    let limit = parse_limit(get_optional_json_u16_param(item, "limit")?);
    let high = get_optional_json_datetime_param(item, "high")?;
    let low = get_optional_json_datetime_param(item, "low")?;
    execute_item(trans.get_edge_range(outbound_id, t, high, low, limit))
}

fn get_reversed_edge_count(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let inbound_id = get_required_json_uuid_param(item, "inbound_id")?;
    let t = get_optional_json_type_param(item, "type")?;
    execute_item(trans.get_reversed_edge_count(inbound_id, t))
}

fn get_reversed_edge_range(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let inbound_id = get_required_json_uuid_param(item, "inbound_id")?;
    let t = get_optional_json_type_param(item, "type")?;
    let limit = parse_limit(get_optional_json_u16_param(item, "limit")?);
    let high = get_optional_json_datetime_param(item, "high")?;
    let low = get_optional_json_datetime_param(item, "low")?;
    execute_item(trans.get_reversed_edge_range(inbound_id, t, high, low, limit))
}

fn run_script(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>, account_id: Uuid) -> Result<JsonValue, IronError> {
    let name: String = get_required_json_string_param(item, "name")?;
    let payload: JsonValue = match item.get("payload") {
        Some(val) => val.clone(),
        None => JsonValue::Null
    };
    execute_script(name, payload, trans, account_id)
}

fn execute_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
    let result = datastore_request(result)?;

    match serde_json::to_value(&result) {
        Ok(val) => Ok(val),
        Err(err) => Err(create_iron_error(status::InternalServerError, format!("Could not serialize results: {}", err)))
    }
}
