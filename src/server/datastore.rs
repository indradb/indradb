use common::datastore;
use nutrino::PostgresDatastore;

lazy_static! {
    pub static ref DATASTORE: PostgresDatastore = datastore();
}
