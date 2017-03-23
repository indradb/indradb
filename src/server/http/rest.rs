use iron::prelude::*;
use iron::status;
use braid::{Edge, Transaction, Type};
use serde_json::value::Value as JsonValue;
use chrono::{DateTime, UTC};
use std::u16;
use uuid::Uuid;
use super::util::*;

pub fn get_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_vertex_query_param(query_params)?;
    let response = datastore_request(trans.get_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn create_vertex(req: &mut Request) -> IronResult<Response> {
    let obj = read_json_object(&mut req.body)?;
    let t = get_required_json_type_param(&obj, "type")?;
    let trans = get_transaction(req)?;
    let response = datastore_request(trans.create_vertex(t))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn set_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_vertex_query_param(query_params)?;
    let t = get_query_param::<Type>(query_params, "type".to_string(), true)?.unwrap();
    datastore_request(trans.set_vertices(q, t))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))    
}

pub fn delete_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_vertex_query_param(query_params)?;
    datastore_request(trans.delete_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn get_edge(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "type")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let trans = get_transaction(req)?;
    let response = datastore_request(trans.get_edge(outbound_id, t, inbound_id))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn set_edge(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "type")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let obj = read_json_object(&mut req.body)?;
    let weight = get_required_json_weight_param(&obj, "weight")?;
    let e = Edge::new_with_current_datetime(outbound_id, t, inbound_id, weight);

    let trans = get_transaction(req)?;
    datastore_request(trans.set_edge(e))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn delete_edge(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "type")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;

    let trans = get_transaction(req)?;
    datastore_request(trans.delete_edge(outbound_id, t, inbound_id))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn get_edge_range(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Option<Type> = get_optional_url_param(req, "type")?;
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let action = &get_query_param::<String>(query_params, "action".to_string(), true)?.unwrap()[..];

    match action {
        "count" => {
            let response = datastore_request(trans.get_edge_count(outbound_id, t))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &response))
        },
        _ => {
            let limit = parse_limit(get_query_param::<u16>(query_params, "limit".to_string(), false)?);
            let high = get_query_param::<DateTime<UTC>>(
                query_params,
                "high".to_string(),
                false
            )?;
            let low = get_query_param::<DateTime<UTC>>(
                query_params,
                "low".to_string(),
                false
            )?;
            let response = datastore_request(trans.get_edge_range(outbound_id, t, high, low, limit))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &response))
        }
    }
}

pub fn get_reversed_edge_range(req: &mut Request) -> IronResult<Response> {
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let t: Option<Type> = get_optional_url_param(req, "type")?;
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let action = &get_query_param::<String>(query_params, "action".to_string(), true)?.unwrap()[..];

    match action {
        "count" => {
            let response = datastore_request(trans.get_reversed_edge_count(inbound_id, t))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &response))
        },
        _ => {
            let limit = parse_limit(get_query_param::<u16>(query_params, "limit".to_string(), false)?);
            let high = get_query_param::<DateTime<UTC>>(
                query_params,
                "high".to_string(),
                false
            )?;
            let low = get_query_param::<DateTime<UTC>>(
                query_params,
                "low".to_string(),
                false
            )?;
            let response = datastore_request(trans.get_reversed_edge_range(inbound_id, t, high, low, limit))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &response))
        }
    }
}

pub fn script(req: &mut Request) -> IronResult<Response> {
    let name: String = get_url_param(req, "name")?;

    let payload = match read_optional_json(&mut req.body)? {
        Some(val) => val,
        None => JsonValue::Null,
    };

    let trans = get_transaction(req)?;
    let account_id = get_account_id(req);
    let response = execute_script(name, payload, &trans, account_id)?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}
