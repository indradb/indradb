#[macro_use]
extern crate clap;
extern crate braid;
#[macro_use]
extern crate common;

use clap::{App, SubCommand};
use braid::PostgresDatastore;
use std::env;

/// App for managing databases
fn main() {
    let matches = App::new("braid-db")
        .version("0.1")
        .about("Manages braid datastores, and their underlying databases")
        .subcommand(SubCommand::with_name("init"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        let connection_string = env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
        
        if !connection_string.starts_with("postgres://") {
            panic!("`braid-db init` can only be run on postgres datastores");
        }

        if let Err(err) = PostgresDatastore::create_schema(connection_string) {
            exit_with_err!("Could not create the database schema: {:?}", err);
        }
    } else {
        exit_with_err!("No action specified");
    }
}
