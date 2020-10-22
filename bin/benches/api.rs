#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

full_bench_impl!({
    use async_std::net::TcpListener;
    use common::client_datastore::ClientDatastore;
    use common::server;
    use futures::executor::LocalPool;
    use futures::prelude::*;
    use futures::task::LocalSpawn;
    use std::net::ToSocketAddrs;

    let mut exec = LocalPool::new();

    let addr = format!("127.0.0.1:0").to_socket_addrs().unwrap().next().unwrap();
    let listener = exec.run_until(async { TcpListener::bind(&addr).await }).unwrap();
    let port = listener.local_addr().unwrap().port();

    let f = server::run(listener, indradb::MemoryDatastore::default(), exec.spawner());
    exec.spawner()
        .spawn_local_obj(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())).into())
        .unwrap();
    ClientDatastore::new(port, exec)
});
