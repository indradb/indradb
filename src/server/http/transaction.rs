use iron::prelude::*;
use iron::status;
use braid::{Vertex, Edge, Transaction, Error};
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

    match read_required_json(&mut req.body) {
        Ok(JsonValue::Array(items)) => {
            for item in items {
                match item {
                    JsonValue::Object(obj) => {
                        let action = get_required_json_string_param(&obj, "action")?;

                        let result: Result<JsonValue, IronError> = match &action[..] {
                            "get_vertex_range" => get_vertex_range(&trans, &obj),
                            "get_vertex" => get_vertex(&trans, &obj),
                            "create_vertex" => create_vertex(&trans, &obj),
                            "set_vertex" => set_vertex(&trans, &obj),
                            "delete_vertex" => delete_vertex(&trans, &obj),
                            
                            "get_edge" => get_edge(&trans, &obj),
                            "set_edge" => set_edge(&trans, &obj),
                            "delete_edge" => delete_edge(&trans, &obj),

                            "get_edge_count" => get_edge_count(&trans, &obj),
                            "get_edge_range" => get_edge_range(&trans, &obj),
                            "get_edge_time_range" => get_edge_time_range(&trans, &obj),

                            "get_reversed_edge_count" => get_reversed_edge_count(&trans, &obj),
                            "get_reversed_edge_range" => get_reversed_edge_range(&trans, &obj),
                            "get_reversed_edge_time_range" => {
                                get_reversed_edge_time_range(&trans, &obj)
                            }

                            _ => {
                                return Err(create_iron_error(status::BadRequest,
                                                             "Unknown action".to_string()))
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

                        idx += 1;
                    }
                    _ => {
                        return Err(create_iron_error(status::BadRequest,
                                                     format!("Item #{}: Invalid type", idx)))
                    }
                }
            }
        }
        _ => {
            return Err(create_iron_error(status::BadRequest,
                                         "Request body should be an array".to_string()))
        }
    }

    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &jsonable_res))
}

fn get_vertex_range(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let start_id = get_optional_json_uuid_param(item, "start_id")?.unwrap_or(Uuid::default());
    let limit = parse_limit(get_optional_json_u16_param(item, "limit")?);
    execute_item(trans.get_vertex_range(start_id, limit))
}

fn get_vertex(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let id = get_required_json_uuid_param(item, "id")?;
    execute_item(trans.get_vertex(id))
}

fn create_vertex(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let t = get_required_json_type_param(item, "type")?;
    execute_item(trans.create_vertex(t))
}

fn set_vertex(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let id = get_required_json_uuid_param(item, "id")?;
    let t = get_required_json_type_param(item, "type")?;
    execute_item(trans.set_vertex(Vertex::new(id, t)))
}

fn delete_vertex(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let id = get_required_json_uuid_param(item, "id")?;
    execute_item(trans.delete_vertex(id))
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
    let offset = get_optional_json_u64_param(item, "offset")?.unwrap_or(0);
    execute_item(trans.get_edge_range(outbound_id, t, offset, limit))
}

fn get_edge_time_range(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let outbound_id = get_required_json_uuid_param(item, "outbound_id")?;
    let t = get_optional_json_type_param(item, "type")?;
    let limit = parse_limit(get_optional_json_u16_param(item, "limit")?);
    let high = get_optional_json_datetime_param(item, "high")?;
    let low = get_optional_json_datetime_param(item, "low")?;
    execute_item(trans.get_edge_time_range(outbound_id, t, high, low, limit))
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
    let offset = get_optional_json_u64_param(item, "offset")?.unwrap_or(0);
    execute_item(trans.get_reversed_edge_range(inbound_id, t, offset, limit))
}

fn get_reversed_edge_time_range(trans: &ProxyTransaction, item: &serde_json::Map<String, JsonValue>) -> Result<JsonValue, IronError> {
    let inbound_id = get_required_json_uuid_param(item, "inbound_id")?;
    let t = get_optional_json_type_param(item, "type")?;
    let limit = parse_limit(get_optional_json_u16_param(item, "limit")?);
    let high = get_optional_json_datetime_param(item, "high")?;
    let low = get_optional_json_datetime_param(item, "low")?;
    execute_item(trans.get_reversed_edge_time_range(inbound_id, t, high, low, limit))
}

fn execute_item<T: Serialize>(result: Result<T, Error>) -> Result<JsonValue, IronError> {
    let result = datastore_request(result)?;

    match serde_json::to_value(&result) {
        Ok(val) => Ok(val),
        Err(err) => Err(create_iron_error(status::InternalServerError, format!("Could not serialize results: {}", err)))
    }
}
