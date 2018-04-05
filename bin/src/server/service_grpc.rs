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

const METHOD_INDRA_DB_PING: ::grpcio::Method<super::request::PingRequest, super::response::PingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/IndraDB/Ping",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INDRA_DB_TRANSACTION: ::grpcio::Method<super::request::TransactionRequest, super::response::TransactionResponse> = ::grpcio::Method {
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

    pub fn ping_opt(&self, req: &super::request::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::response::PingResponse> {
        self.client.unary_call(&METHOD_INDRA_DB_PING, req, opt)
    }

    pub fn ping(&self, req: &super::request::PingRequest) -> ::grpcio::Result<super::response::PingResponse> {
        self.ping_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ping_async_opt(&self, req: &super::request::PingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::response::PingResponse>> {
        self.client.unary_call_async(&METHOD_INDRA_DB_PING, req, opt)
    }

    pub fn ping_async(&self, req: &super::request::PingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::response::PingResponse>> {
        self.ping_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn transaction_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::request::TransactionRequest>, ::grpcio::ClientDuplexReceiver<super::response::TransactionResponse>)> {
        self.client.duplex_streaming(&METHOD_INDRA_DB_TRANSACTION, opt)
    }

    pub fn transaction(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::request::TransactionRequest>, ::grpcio::ClientDuplexReceiver<super::response::TransactionResponse>)> {
        self.transaction_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait IndraDb {
    fn ping(&self, ctx: ::grpcio::RpcContext, req: super::request::PingRequest, sink: ::grpcio::UnarySink<super::response::PingResponse>);
    fn transaction(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::request::TransactionRequest>, sink: ::grpcio::DuplexSink<super::response::TransactionResponse>);
}

pub fn create_indra_db<S: IndraDb + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INDRA_DB_PING, move |ctx, req, resp| {
        instance.ping(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_INDRA_DB_TRANSACTION, move |ctx, req, resp| {
        instance.transaction(ctx, req, resp)
    });
    builder.build()
}
