use futures::prelude::*;
use common::{ProxyDatastore, datastore};
use std::sync::Arc;
use grpc;
use grpcio;
use futures::stream::Stream;

#[derive(Clone)]
pub struct IndraDbService {
    datastore: Arc<ProxyDatastore>
}

impl IndraDbService {
    pub fn new() -> Self {
        Self { datastore: Arc::new(datastore()) }
    }
}

impl grpc::IndraDb for IndraDbService {
    fn transaction(&self, ctx: grpcio::RpcContext, stream: grpcio::RequestStream<grpc::TransactionRequest>, sink: grpcio::DuplexSink<grpc::TransactionResponse>) {
        let datastore = self.datastore.clone();

        let f = stream.for_each(|request| {
            println!("{:?}", request);
        });

        ctx.spawn(f)
    }
}
