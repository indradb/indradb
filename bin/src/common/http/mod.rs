mod endpoints;
mod util;

use graphql;
use std::u16;
use actix::{System, SyncArbiter};
use std::sync::Arc;
use statics;
use actix_web::{middleware, http, server, App};

/// Starts a new server on the given port.
pub fn start_server(port: u16) {
    let sys = System::new("indradb");
    let schema = Arc::new(graphql::Schema::new(graphql::RootQuery, graphql::RootMutation));

    let addr = SyncArbiter::start(*statics::WEB_WORKER_POOL_SIZE as usize, move || {
        graphql::Executor::new(schema.clone())
    });

    let s = server::new(move || {
        App::with_state(endpoints::AppState { executor: addr.clone() })
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/graphql", |r| r.method(http::Method::POST).with2(endpoints::graphql_handler))
            .resource("/script/{name}", |r| r.method(http::Method::POST).h(endpoints::script_handler))
            .resource("/mapreduce/{name}", |r| r.method(http::Method::POST).h(endpoints::mapreduce_handler))
            .default_resource(|r| r.f(endpoints::not_found_handler))
    });

    s.bind(&format!("0.0.0.0:{}", port)).expect("Expected to be able to bind to server").start();

    let _ = sys.run();
}
