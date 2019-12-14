#![feature(test)]

extern crate common;
#[macro_use]
extern crate indradb;
extern crate test;

const TEST_PORT: u16 = 27616;

full_bench_impl!({
    use common::client_datastore::ClientDatastore;
    use common::server;
    use futures::executor::LocalPool;
    use futures::prelude::*;
    use futures::task::LocalSpawn;
    use std::net::ToSocketAddrs;

    let addr = format!("127.0.0.1:{}", TEST_PORT)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let exec = LocalPool::new();
    let spawner = exec.spawner();
    let f = server::run(addr, indradb::MemoryDatastore::default(), exec.spawner());
    spawner
        .spawn_local_obj(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())).into())
        .unwrap();
    ClientDatastore::new(TEST_PORT, exec)
});
