use iron::prelude::*;
use iron::status;
use nutrino::{Vertex, Edge, Transaction, Type};
use std::io::Read;
use serde_json::value::Value as JsonValue;
use script;
use std::path::Path;
use chrono::{DateTime, UTC};
use regex;
use std::fs::File;
use std::u16;
use uuid::Uuid;
use super::util::*;
use statics;

lazy_static! {
	static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[\w-_]+(\.lua)?$").unwrap();
}

pub fn get_vertex_range(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let offset = get_query_param::<u64>(query_params, "offset".to_string(), false)?.unwrap_or(0);
    let limit = parse_limit(get_query_param::<u16>(query_params, "limit".to_string(), false)?);
    let result = datastore_request(trans.get_vertex_range(offset, limit))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn get_vertex(req: &mut Request) -> IronResult<Response> {
    let id: Uuid = get_url_param(req, "id")?;
    let trans = get_transaction(req)?;
    let result = datastore_request(trans.get_vertex(id))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn create_vertex(req: &mut Request) -> IronResult<Response> {
    let obj = read_json_object(&mut req.body)?;
    let t = get_required_json_type_param(&obj, "type")?;
    let trans = get_transaction(req)?;
    let result = datastore_request(trans.create_vertex(t))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn set_vertex(req: &mut Request) -> IronResult<Response> {
    let id: Uuid = get_url_param(req, "id")?;
    let obj = read_json_object(&mut req.body)?;
    let t = get_required_json_type_param(&obj, "type")?;
    let v = Vertex::new(id, t);
    let trans = get_transaction(req)?;
    let result = datastore_request(trans.set_vertex(v))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn delete_vertex(req: &mut Request) -> IronResult<Response> {
    let id: Uuid = get_url_param(req, "id")?;
    let trans = get_transaction(req)?;
    let result = datastore_request(trans.delete_vertex(id))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn get_edge(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "type")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let trans = get_transaction(req)?;
    let result = datastore_request(trans.get_edge(outbound_id, t, inbound_id))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn set_edge(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "type")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let obj = read_json_object(&mut req.body)?;
    let weight = get_required_json_weight_param(&obj, "weight")?;
    let e = Edge::new_with_current_datetime(outbound_id, t, inbound_id, weight);

    let trans = get_transaction(req)?;
    let result = datastore_request(trans.set_edge(e))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn delete_edge(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "type")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;

    let trans = get_transaction(req)?;
    let result = datastore_request(trans.delete_edge(outbound_id, t, inbound_id))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &result))
}

pub fn get_edge_range(req: &mut Request) -> IronResult<Response> {
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Option<Type> = get_optional_url_param(req, "type")?;
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let action = &get_query_param::<String>(query_params, "action".to_string(), true)?.unwrap()[..];

    match action {
        "count" => {
            let result = datastore_request(trans.get_edge_count(outbound_id, t))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &result))
        }
        "time" => {
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
            let result = datastore_request(trans.get_edge_time_range(outbound_id, t, high, low, limit))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &result))
        }
        "position" => {
            let limit = parse_limit(get_query_param::<u16>(query_params, "limit".to_string(), false)?);
            let offset = get_query_param::<u64>(query_params, "offset".to_string(), false)?.unwrap_or(0);
            let result = datastore_request(trans.get_edge_range(outbound_id, t, offset, limit))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &result))
        }
        _ => Err(create_iron_error(status::BadRequest, "Invalid `action`".to_string())),
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
            let result = datastore_request(trans.get_reversed_edge_count(inbound_id, t))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &result))
        }
        "time" => {
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
            let result = datastore_request(trans.get_reversed_edge_time_range(inbound_id, t, high, low, limit))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &result))
        }
        "position" => {
            let limit = parse_limit(get_query_param::<u16>(query_params, "limit".to_string(), false)?);
            let offset = get_query_param::<u64>(query_params, "offset".to_string(), false)?.unwrap_or(0);
            let result = datastore_request(trans.get_reversed_edge_range(inbound_id, t, offset, limit))?;
            datastore_request(trans.commit())?;
            Ok(to_response(status::Ok, &result))
        }
        _ => Err(create_iron_error(status::BadRequest, "Invalid `action`".to_string())),
    }
}

pub fn script(req: &mut Request) -> IronResult<Response> {
    let script_name: String = get_url_param(req, "name")?;

    if !SCRIPT_NAME_VALIDATOR.is_match(&script_name[..]) {
        return Err(create_iron_error(status::BadRequest, "Invalid script name".to_string()));
    }

    let arg = match read_optional_json(&mut req.body)? {
        Some(val) => val,
        None => JsonValue::Null,
    };

    let path = Path::new(&statics::SCRIPT_ROOT[..]).join(script_name);

    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            return Err(create_iron_error(status::NotFound, "Could not load script".to_string()))
        }
    };

    let mut payload = String::new();

    if let Err(err) = f.read_to_string(&mut payload) {
        return Err(create_iron_error(status::InternalServerError,
                                     format!("Could not read script contents: {}", err)));
    }

    let account_id = get_account_id(req);
    let trans = get_transaction(req)?;

    match script::run(trans, account_id, &payload[..], arg) {
        Ok(val) => Ok(to_response(status::Ok, &val)),
        Err(err) => {
            Err(create_iron_error(status::InternalServerError,
                                  format!("Script failed: {:?}", err)))
        }
    }
}
