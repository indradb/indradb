extern crate clap;
extern crate nutrino;
extern crate rand;
#[macro_use] extern crate common;

use clap::{Arg, App, SubCommand};
use rand::{Rng, OsRng};
use std::error::Error;
use common::datastore;
use nutrino::Datastore;

fn generate_random_password() -> String {
    let mut chars = vec![];
    let mut rng = OsRng::new().unwrap();

    for _ in 0..10 {
        let c: u8 = rng.gen_range(b'!', b'~') as u8;
        chars.push(c);
    }

    String::from_utf8(chars).unwrap()
}

fn add_account(email: String, secret: String) {
    match datastore().create_account(email, secret) {
        Ok(id) => println!("Your account id is: {}", id),
        Err(err) => exit_with_err!("Could not create account: {}", err.description())
    }
}

fn remove_account(email: String) {
    let datastore = datastore();

    let account_id = match datastore.get_account_id(email) {
        Ok(Some(id)) => id,
        Ok(None) => exit_with_err!("Account not found"),
        Err(err) => exit_with_err!("Could not lookup account: {}", err.description())
    };

    if let Err(err) = datastore.delete_account(account_id) {
        exit_with_err!("Could not delete account: {}", err.description());
    }
}

fn main() {
    // Parse command line arguments
    let email_arg = Arg::with_name("EMAIL").help("Email of account to manage").required(true).index(1);
    let secret_arg = Arg::with_name("secret").short("s").long("secret").takes_value(true).help("Secret value");

    let matches = App::new("nutrino-user")
        .version("0.1")
        .about("User management for Nutrino")
        .subcommand(SubCommand::with_name("add")
            .arg(email_arg.clone())
            .arg(secret_arg))
        .subcommand(SubCommand::with_name("remove")
            .arg(email_arg.clone()))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let email = matches.value_of("EMAIL").unwrap().to_string();

        let secret = match matches.value_of("secret") {
            Some(val) => val.to_string(),
            None => {
                let secret = generate_random_password();
                println!("Your secret is: {}", secret);
                secret
            }
        };

        add_account(email, secret);
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let email = matches.value_of("EMAIL").unwrap().to_string();
        remove_account(email);
    } else {
        exit_with_err!("No action specified");
    }
}
