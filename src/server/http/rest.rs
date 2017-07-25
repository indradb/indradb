use iron::prelude::*;
use iron::status;
use braid::{Transaction, Type, EdgeKey, VertexQuery, EdgeQuery};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use super::util::*;

pub fn create_vertex(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let t = get_query_param::<Type>(query_params, "type", true)?
        .unwrap();
    let response = datastore_request(trans.create_vertex(t))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn get_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<VertexQuery>(query_params)?;
    let response = datastore_request(trans.get_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}

pub fn delete_vertices(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<VertexQuery>(query_params)?;
    datastore_request(trans.delete_vertices(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn create_edge(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
    let outbound_id: Uuid = get_url_param(req, "outbound_id")?;
    let t: Type = get_url_param(req, "t")?;
    let inbound_id: Uuid = get_url_param(req, "inbound_id")?;
    let query_params = get_query_params(req)?;
    let weight = get_weight_query_param(query_params)?;
    let key = EdgeKey::new(outbound_id, t, inbound_id);
    datastore_request(trans.create_edge(key, weight))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn get_edges(req: &mut Request) -> IronResult<Response> {
    let trans = get_transaction(req)?;
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
    let trans = get_transaction(req)?;
    let query_params = get_query_params(req)?;
    let q = get_obj_query_param::<EdgeQuery>(query_params)?;
    datastore_request(trans.delete_edges(q))?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &()))
}

pub fn script(req: &mut Request) -> IronResult<Response> {
    let name: String = get_url_param(req, "name")?;

    let payload = match read_optional_json(&mut req.body)? {
        Some(val) => val,
        None => JsonValue::Null,
    };

    let trans = get_transaction(req)?;
    let account_id = get_account_id(req);
    let response = execute_script(name, &payload, &trans, account_id)?;
    datastore_request(trans.commit())?;
    Ok(to_response(status::Ok, &response))
}
