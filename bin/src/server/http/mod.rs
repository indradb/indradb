mod middleware;
mod endpoints;
mod util;

use iron::prelude::*;
use router::Router;
use std::u16;
use juniper_iron::GraphQLHandler;

pub fn context_factory(_: &mut Request) -> () {
    ()
}

/// Starts a new server on the given port.
pub fn start(port: u16) {
    let mut router = Router::new();

    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        endpoints::GraphQLQuery,
        endpoints::GraphQLMutation,
    );

    router.any("/graphql", graphql_endpoint, "graphql");
    router.post("/script/:name", endpoints::script, "script");
    router.post("/mapreduce/:name", endpoints::mapreduce, "mapreduce");

    let binding = format!("0.0.0.0:{}", port);
    println!("Listening on {}", binding);

    let mut chain = Chain::new(router);
    chain.link_after(middleware::ErrorMiddleware::new());
    Iron::new(chain).http(&*binding).unwrap();
}
