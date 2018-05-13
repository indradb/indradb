use juniper;
use proxy_datastore;

pub struct Context {
    pub trans: proxy_datastore::ProxyTransaction,
}

impl Context {
    pub fn new(trans: proxy_datastore::ProxyTransaction) -> Self {
        Self { trans: trans }
    }
}

impl juniper::Context for Context {}
