#![feature(test)]

extern crate indradb_proto as proto;
#[macro_use]
extern crate indradb;

use std::net::ToSocketAddrs;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::runtime::Runtime;

full_bench_impl!({
    let rt = Runtime::new().unwrap();

    let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap();
    let listener = rt.block_on(TcpListener::bind(&addr)).unwrap();
    let port = listener.local_addr().unwrap().port();
    rt.spawn(proto::run_server(
        Arc::new(indradb::MemoryDatastore::default()),
        listener,
    ));

    proto::tests::ClientDatastore::new(port as u16, rt)
});
