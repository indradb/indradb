mod middleware;
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

    router.get("/edge/_/:inbound_id/:type", rest::get_reversed_edge_range, "get_reversed_edge_range");
    router.get("/edge/:outbound_id/:type/:inbound_id", rest::get_edge, "get_edge");
    router.put("/edge/:outbound_id/:type/:inbound_id", rest::set_edge, "set_edge");
    router.delete("/edge/:outbound_id/:type/:inbound_id", rest::delete_edge, "delete_edge");
    router.get("/edge/:outbound_id/:type/_", rest::get_edge_range, "get_edge_range");

    router.get("/vertex", rest::get_vertices, "get_vertices");
    router.post("/vertex", rest::create_vertex, "create_vertex");
    router.put("/vertex", rest::set_vertices, "set_vertices");
    router.delete("/vertex", rest::delete_vertices, "delete_vertices");

    router.post("/script/:name", rest::script, "script");

    let binding = format!("0.0.0.0:{}", port);
    println!("Listening on {}", binding);

    let mut chain = Chain::new(router);
    chain.link_before(middleware::BasicAuthMiddleware::new());
    chain.link_after(middleware::ErrorMiddleware::new());
    Iron::new(chain).http(&*binding).unwrap();
}
