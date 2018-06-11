use indradb::Datastore;
use iron::Request;
use juniper;
use proxy_datastore;
use statics;

pub struct Context {
    pub trans: proxy_datastore::ProxyTransaction,
}

impl Context {
    pub fn new(trans: proxy_datastore::ProxyTransaction) -> Self {
        Self { trans: trans }
    }
}

impl juniper::Context for Context {}

pub fn factory(_: &mut Request) -> Context {
    // TODO: does juniper have a way to propagate errors in contexts?
    let trans = statics::DATASTORE
        .transaction()
        .expect("Expected to be able to grab a transaction");
    Context::new(trans)
}
