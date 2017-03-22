use iron::prelude::*;
use iron::status;
use braid::{Vertex, Edge, Transaction, Type, VertexQuery, Error};
use serde_json::value::Value as JsonValue;
use serde_json;
use chrono::{DateTime, UTC};
use std::u16;
use uuid::Uuid;
use super::util::*;

pub fn get_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q_json = get_query_param::<JsonValue>(query_params, "q".to_string(), true)?.unwrap_or_else(|| JsonValue::Null);
    
    let q = match serde_json::from_value::<VertexQuery>(q_json) {
        Ok(q) => q,
        Err(_) => {
            return Err(create_iron_error(status::BadRequest, "Bad payload: expected vertex query".to_string()))
        }
    };

    let response = datastore_request(trans.get_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn get_vertex(req: &mut Request) -> IronResult<Response> {
    let id: Uuid = get_url_param(req, "id")?;
    let trans = get_transaction(req)?;
    let q = VertexQuery::Vertex(id);
    let response = datastore_request(trans.get_vertices(q))?;
    datastore_request(trans.commit())?;

    debug_assert!(response.len() <= 1);

    if response.len() == 0 {
        Err(convert_to_iron_error(Error::VertexNotFound))
    } else {
        Ok(to_response(status::Ok, &response[0]))
    }
}

pub fn create_vertex(req: &mut Request) -> IronResult<Response> {
    let obj = read_json_object(&mut req.body)?;
    let t = get_required_json_type_param(&obj, "type")?;
    let trans = get_transaction(req)?;
    let response = datastore_request(trans.create_vertex(t))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn set_vertex(req: &mut Request) -> IronResult<Response> {
    let id: Uuid = get_url_param(req, "id")?;
    let obj = read_json_object(&mut req.body)?;
    let t = get_required_json_type_param(&obj, "type")?;
    let v = Vertex::new(id, t);
    let trans = get_transaction(req)?;
    datastore_request(trans.set_vertex(v))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn delete_vertex(req: &mut Request) -> IronResult<Response> {
    let id: Uuid = get_url_param(req, "id")?;
    let trans = get_transaction(req)?;
    datastore_request(trans.delete_vertex(id))?;
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
