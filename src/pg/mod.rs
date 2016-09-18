extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate num_cpus;

mod datastore;
mod tests;

pub use self::datastore::{PostgresDatastore, PostgresTransaction};
