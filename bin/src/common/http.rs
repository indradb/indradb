use actix::prelude::*;
use graphql;
use script;
use mapreduce;
use serde_json::value::Value as JsonValue;
use actix_web::{middleware, http, server, App, AsyncResponder, HttpRequest, HttpResponse, HttpMessage, Json, State, FutureResponse, Path, ws, Error};
use futures::{Future, Stream};
use std::u16;
use actix::{System, SyncArbiter};
use std::sync::Arc;
use statics;

pub struct AppState {
    graphql: Addr<Syn, graphql::Executor>,
    script: Addr<Syn, script::Executor>
}

fn graphql_handler(state: State<AppState>, data: Json<graphql::Request>) -> FutureResponse<HttpResponse> {
    state.graphql.send(data.0).from_err().and_then(|res| {
        Ok(HttpResponse::Ok().content_type("application/json").body(res?.to_string()))
    }).responder()
}

fn script_handler(state: State<AppState>, path: Path<String>, data: Json<JsonValue>) -> FutureResponse<HttpResponse> {
    state.script.send(script::Request::new(path.to_string(), data.0)).from_err().and_then(|res| {
        Ok(HttpResponse::Ok().content_type("application/json").body(res?.to_string()))
    }).responder()
}

fn mapreduce_handler(req: HttpRequest<AppState>, path: Path<String>, data: Json<JsonValue>) -> Result<HttpResponse, Error> {
    ws::start(req, mapreduce::Executor::new(script::Request::new(path.to_string(), data.0)))
}

fn not_found_handler(_: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "error": "No route found"
    }))
}

/// Starts a new server on the given port.
pub fn start_server(port: u16) {
    let sys = System::new("indradb");
    let schema = Arc::new(graphql::Schema::new(graphql::RootQuery, graphql::RootMutation));

    let graphql_addr = SyncArbiter::start(*statics::POOL_SIZE as usize, move || {
        graphql::Executor::new(schema.clone())
    });

    let script_addr = SyncArbiter::start(*statics::POOL_SIZE as usize, move || {
        script::Executor::new()
    });

    let s = server::new(move || {
        let state = AppState {
            graphql: graphql_addr.clone(),
            script: script_addr.clone(),
        };

        App::with_state(state)
            .middleware(middleware::Logger::default())
            .resource("/graphql", |r| r.method(http::Method::POST).with2(graphql_handler))
            .resource("/script/{name}", |r| r.method(http::Method::POST).with3(script_handler))
            .resource("/mapreduce/{name}", |r| r.method(http::Method::POST).with3(mapreduce_handler))
            .default_resource(|r| r.f(not_found_handler))
    });

    s.bind(&format!("0.0.0.0:{}", port)).expect("Expected to be able to bind to server").start();

    let _ = sys.run();
}
