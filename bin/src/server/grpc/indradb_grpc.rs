// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_INDRA_DB_TRANSACTION: ::grpcio::Method<super::indradb::TransactionRequest, super::indradb::TransactionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/IndraDB/Transaction",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct IndraDbClient {
    client: ::grpcio::Client,
}

impl IndraDbClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        IndraDbClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn transaction_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::indradb::TransactionRequest>, ::grpcio::ClientDuplexReceiver<super::indradb::TransactionResponse>)> {
        self.client.duplex_streaming(&METHOD_INDRA_DB_TRANSACTION, opt)
    }

    pub fn transaction(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::indradb::TransactionRequest>, ::grpcio::ClientDuplexReceiver<super::indradb::TransactionResponse>)> {
        self.transaction_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait IndraDb {
    fn transaction(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::indradb::TransactionRequest>, sink: ::grpcio::DuplexSink<super::indradb::TransactionResponse>);
}

pub fn create_indra_db<S: IndraDb + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_INDRA_DB_TRANSACTION, move |ctx, req, resp| {
        instance.transaction(ctx, req, resp)
    });
    builder.build()
}
