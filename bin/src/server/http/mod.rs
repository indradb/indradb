mod middleware;
mod response_chan;
mod rest;
mod transaction;
mod util;

use iron::prelude::*;
use router::Router;
use std::u16;

/// Starts a new server on the given port.
pub fn start(port: u16) {
    let mut router = Router::new();

    router.post("/transaction", transaction::transaction, "transaction");

    router.put(
        "/edge/:outbound_id/:t/:inbound_id",
        rest::create_edge,
        "create_edge",
    );
    router.get("/edge", rest::get_edges, "get_edges");
    router.delete("/edge", rest::delete_edges, "delete_edges");

    router.get("/vertex", rest::get_vertices, "get_vertices");
    router.post("/vertex", rest::create_vertex, "create_vertex");
    router.delete("/vertex", rest::delete_vertices, "delete_vertices");

    router.post("/script/:name", rest::script, "script");
    router.post("/mapreduce/:name", rest::mapreduce, "mapreduce");

    let binding = format!("0.0.0.0:{}", port);
    println!("Listening on {}", binding);

    let mut chain = Chain::new(router);
    chain.link_before(middleware::BasicAuthMiddleware::new());
    chain.link_after(middleware::ErrorMiddleware::new());
    Iron::new(chain).http(&*binding).unwrap();
}
