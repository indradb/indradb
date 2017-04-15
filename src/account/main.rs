#[macro_use]
extern crate clap;
extern crate braid;
#[macro_use]
extern crate common;
extern crate uuid;

use clap::{Arg, App, SubCommand};
use common::datastore;
use braid::Datastore;
use uuid::Uuid;

/// App for managing accounts
fn main() {
    let matches = App::new("braid-account")
        .version("0.1")
        .about("User management for Braid")
        .subcommand(SubCommand::with_name("add"))
        .subcommand(SubCommand::with_name("remove")
            .arg(Arg::with_name("ID").help("ID of account").required(true).index(1)))
        .get_matches();

    let datastore = datastore();

    if let Some(_) = matches.subcommand_matches("add") {
        match datastore.create_account() {
            Ok((id, secret)) => {
                println!("Account ID: {}", id);
                println!("Account secret: {}", secret);
            },
            Err(err) => exit_with_err!("Could not create account: {:?}", err),
        }
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let id = value_t!(matches, "ID", Uuid).unwrap();

        if let Err(err) = datastore.delete_account(id) {
            exit_with_err!("Could not delete account: {:?}", err);
        }
    } else {
        exit_with_err!("No action specified");
    }
}
