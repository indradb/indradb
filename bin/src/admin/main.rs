#[macro_use]
extern crate clap;
extern crate common;
extern crate indradb;
extern crate uuid;

use clap::{App, Arg, SubCommand};
use indradb::{Datastore, PostgresDatastore};
use common::datastore;
use std::env;
use uuid::Uuid;
use std::process::exit;

/// App for managing databases
fn main() {
    let matches = App::new("indradb-admin")
        .version("0.1")
        .about("Manages indradb datastores, and their underlying databases")
        .subcommand(SubCommand::with_name("init"))
        .subcommand(SubCommand::with_name("add-account"))
        .subcommand(
            SubCommand::with_name("remove-account").arg(
                Arg::with_name("ID")
                    .help("ID of account")
                    .required(true)
                    .index(1),
            ),
        )
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
    } else if matches.subcommand_matches("add-account").is_some() {
        let datastore = datastore();

        match datastore.create_account() {
            Ok((id, secret)) => {
                println!("Account ID: {}", id);
                println!("Account secret: {}", secret);
            }
            Err(err) => {
                eprintln!("Could not create account: {:?}", err);
                exit(1);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("remove-account") {
        let datastore = datastore();
        let id = value_t!(matches, "ID", Uuid).unwrap();

        if let Err(err) = datastore.delete_account(id) {
            eprintln!("Could not delete account: {:?}", err);
            exit(1);
        }
    } else {
        eprintln!("No action specified");
        exit(1);
    }
}
