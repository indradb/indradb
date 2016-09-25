extern crate nutrino;

use std::env;
use nutrino::PostgresDatastore;

// Based off of https://github.com/rust-lang/rfcs/issues/1078
#[macro_export]
macro_rules! exit_with_err {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;

            if let Err(e) = write!(&mut ::std::io::stderr(), "{}\n", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
            }

            ::std::process::exit(1);
        }
    )
}

pub fn datastore() -> PostgresDatastore {
    let pool_size = match env::var("DATABASE_POOL_SIZE") {
        Ok(str_val) => Some(str_val.parse().expect("Invalid DATABASE_POOL_SIZE: Must be an integer")),
        Err(_) => None
    };

    let connection_string = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => "postgresql://localhost:5432/nutrino".to_string()
    };

    let secret = env::var("SECRET").unwrap_or("".to_string());
    PostgresDatastore::new(pool_size, connection_string, secret)
}
