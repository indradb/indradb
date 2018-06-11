#![recursion_limit = "1024"]

extern crate common;

use std::env;

/// App for running the GraphQL server.
fn main() {
    let port_str = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port = port_str
        .parse::<u16>()
        .expect("Could not parse environment variable `PORT`");
    common::start_server(port);
}
