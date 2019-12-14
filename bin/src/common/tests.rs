use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::net::ToSocketAddrs;

use crate::client_datastore::ClientDatastore;
use crate::server;

use indradb::util::generate_temporary_path;
use indradb::{Datastore, Transaction};
use futures::prelude::*;
use futures::executor::LocalPool;
use futures::task::LocalSpawn;

const START_PORT: u16 = 27616;

lazy_static! {
    static ref CURRENT_PORT: AtomicUsize = AtomicUsize::new(START_PORT as usize);
}

full_test_impl!({
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    let addr = format!("127.0.0.1:{}", port).to_socket_addrs().unwrap().next().unwrap();

    let exec = LocalPool::new();
    let spawner = exec.spawner();
    let f = server::run(addr, indradb::MemoryDatastore::default(), exec.spawner());
    spawner.spawn_local_obj(Box::pin(f.map_err(|err| panic!(err)).map(|_|())).into()).unwrap();
    ClientDatastore::new(port as u16, exec)
});

#[test]
fn should_create_rocksdb_datastore() {
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    let addr = format!("127.0.0.1:{}", port).to_socket_addrs().unwrap().next().unwrap();

    let exec = LocalPool::new();
    let spawner = exec.spawner();
    let datastore = indradb::RocksdbDatastore::new(&generate_temporary_path(), None, false).unwrap();
    let f = server::run(addr, datastore, exec.spawner());
    spawner.spawn_local_obj(Box::pin(f.map_err(|err| panic!(err)).map(|_|())).into()).unwrap();

    // Just make sure we can run a command
    let datastore = ClientDatastore::new(port as u16, exec);
    let trans = datastore.transaction().unwrap();
    let count = trans.get_vertex_count().unwrap();

    assert_eq!(count, 0);
}
