mod middleware;
mod endpoints;
mod util;

use iron::prelude::*;
use router::Router;
use std::u16;

/// Starts a new server on the given port.
pub fn start(port: u16) {
    let mut router = Router::new();

    router.post("/script/:name", endpoints::script, "script");
    router.post("/transaction", endpoints::transaction, "transaction");

    let binding = format!("0.0.0.0:{}", port);
    println!("Listening on {}", binding);

    let mut chain = Chain::new(router);
    chain.link_after(middleware::ErrorMiddleware::new());
    Iron::new(chain).http(&*binding).unwrap();
}
