use iron::prelude::*;
use iron::status;
use indradb::{EdgeKey, EdgeQuery, Transaction, Type, VertexQuery};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use regex;
use script;
use statics;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use super::util::*;

lazy_static! {
    static ref SCRIPT_NAME_VALIDATOR: regex::Regex = regex::Regex::new(r"^[\w-_]+(\.lua)?$").unwrap();
}

pub fn create_vertex(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let query_params = get_query_params(req)?;
    let t = get_query_param::<Type>(query_params, "type", true)?.unwrap();
    let response = datastore_request(trans.create_vertex(t))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn get_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<VertexQuery>(query_params)?;
    let response = datastore_request(trans.get_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn delete_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<VertexQuery>(query_params)?;
    datastore_request(trans.delete_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn create_edge(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "t")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let key = EdgeKey::new(outbound_id, t, inbound_id);
    datastore_request(trans.create_edge(key))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn get_edges(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<EdgeQuery>(query_params)?;
    let action = get_query_param::<String>(query_params, "action", false)?;

    if action == Some("count".to_string()) {
        let response = datastore_request(trans.get_edge_count(q))?;
        Ok(to_response(status::Ok, &response))
    } else {
        let response = datastore_request(trans.get_edges(q))?;
        Ok(to_response(status::Ok, &response))
    }
}

pub fn delete_edges(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction()?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<EdgeQuery>(query_params)?;
    datastore_request(trans.delete_edges(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn script(req: &mut Request) -> IronResult<Response> {
    let name: String = get_url_param(req, "name")?;

    if !SCRIPT_NAME_VALIDATOR.is_match(&name[..]) {
        return Err(create_iron_error(
            status::BadRequest,
            "Invalid script name".to_string(),
        ));
    }

    let payload = match read_optional_json(&mut req.body)? {
        Some(val) => val,
        None => JsonValue::Null,
    };

    let path = Path::new(&statics::SCRIPT_ROOT[..]).join(name);

    let contents = match File::open(&path) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(_) => Err(create_iron_error(
                    status::NotFound,
                    "Could not read script".to_string(),
                )),
            }
        }
        Err(_) => Err(create_iron_error(
            status::NotFound,
            "Could not load script".to_string(),
        )),
    }?;

    match script::run(&contents, &path, payload) {
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
