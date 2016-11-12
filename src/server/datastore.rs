use common::{ProxyDatastore, datastore};

lazy_static! {
    pub static ref DATASTORE: ProxyDatastore = datastore();
}
