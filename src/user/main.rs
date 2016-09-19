extern crate clap;
extern crate nutrino;
#[macro_use] extern crate common;

use clap::{Arg, App, SubCommand};
use std::error::Error;
use common::datastore;
use nutrino::Datastore;

fn main() {
    // Parse command line arguments
    let email_arg = Arg::with_name("EMAIL").help("Email of account to manage").required(true).index(1);

    let matches = App::new("nutrino-user")
        .version("0.1")
        .about("User management for Nutrino")
        .subcommand(SubCommand::with_name("add").arg(email_arg.clone()))
        .subcommand(SubCommand::with_name("remove").arg(email_arg.clone()))
        .get_matches();

    let datastore = datastore();

    if let Some(matches) = matches.subcommand_matches("add") {
        let email = matches.value_of("EMAIL").unwrap().to_string();

        match datastore.create_account(email) {
            Ok(secret) => println!("Account secret: {}", secret),
            Err(err) => exit_with_err!("Could not create account: {}", err.description())
        }
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let email = matches.value_of("EMAIL").unwrap().to_string();

        if let Err(err) = datastore.delete_account(email) {
            exit_with_err!("Could not delete account: {}", err.description());
        }
    } else {
        exit_with_err!("No action specified");
    }
}
