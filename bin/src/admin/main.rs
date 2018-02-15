#![recursion_limit = "1024"]

extern crate clap;
extern crate common;
extern crate indradb;
extern crate uuid;

use clap::{App, SubCommand};
use indradb::PostgresDatastore;
use std::env;
use std::process::exit;

/// App for managing databases
fn main() {
    let matches = App::new("indradb-admin")
        .version("0.1")
        .about("Manages indradb datastores, and their underlying databases")
        .subcommand(SubCommand::with_name("init"))
        .get_matches();

    if matches.subcommand_matches("init").is_some() {
        let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());

        if !connection_string.starts_with("postgres://") {
            panic!("`indradb-admin init` can only be run on postgres datastores");
        }

        if let Err(err) = PostgresDatastore::create_schema(connection_string) {
            eprintln!("Could not create the database schema: {:?}", err);
            exit(1);
        }
    } else {
        eprintln!("No action specified");
        exit(1);
    }
}
