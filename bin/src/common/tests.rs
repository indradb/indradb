use std::net::ToSocketAddrs;

use crate::client_datastore::ClientDatastore;
use crate::server;

use futures::prelude::*;
use futures::task::LocalSpawn;
use indradb::util::generate_temporary_path;
use indradb::{Datastore, Transaction};
use async_std::net::TcpListener;
use futures::executor::LocalPool;

full_test_impl!({
    let mut exec = LocalPool::new();
    
    let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap();
    let listener = exec.run_until(async {
        TcpListener::bind(&addr).await
    }).unwrap();
    let port = listener.local_addr().unwrap().port();

    let f = server::run(listener, indradb::MemoryDatastore::default(), exec.spawner());
    exec.spawner()
        .spawn_local_obj(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())).into())
        .unwrap();
    ClientDatastore::new(port, exec)
});

#[test]
fn should_create_rocksdb_datastore() {
    let mut exec = LocalPool::new();
    
    let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap();
    let listener = exec.run_until(async {
        TcpListener::bind(&addr).await
    }).unwrap();
    let port = listener.local_addr().unwrap().port();

    let datastore = indradb::RocksdbDatastore::new(&generate_temporary_path(), None, false).unwrap();
    let f = server::run(listener, datastore, exec.spawner());
    exec.spawner()
        .spawn_local_obj(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())).into())
        .unwrap();
    
    // Just make sure we can run a command
    let datastore = ClientDatastore::new(port as u16, exec);
    let trans = datastore.transaction().unwrap();
    let count = trans.get_vertex_count().unwrap();
    assert_eq!(count, 0);
}
