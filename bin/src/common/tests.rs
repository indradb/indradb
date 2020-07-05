use std::net::ToSocketAddrs;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::client_datastore::ClientDatastore;
use crate::server;

use futures::prelude::*;
use indradb::util::generate_temporary_path;
use indradb::{Datastore, Transaction};
use async_std::task::{block_on, spawn_local};

const START_PORT: u16 = 27616;

lazy_static! {
    static ref CURRENT_PORT: AtomicUsize = AtomicUsize::new(START_PORT as usize);
}

full_test_impl!({
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    let addr = format!("127.0.0.1:{}", port).to_socket_addrs().unwrap().next().unwrap();

    block_on(async move {
        let f = server::run(addr, indradb::MemoryDatastore::default());
        spawn_local(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())));
        ClientDatastore::new(port as u16)
    })
});

#[test]
fn should_create_rocksdb_datastore() {
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    let addr = format!("127.0.0.1:{}", port).to_socket_addrs().unwrap().next().unwrap();

    let count = block_on(async move {
        let datastore = indradb::RocksdbDatastore::new(&generate_temporary_path(), None, false).unwrap();
        let f = server::run(addr, datastore);
        spawn_local(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())));

        // Just make sure we can run a command
        let datastore = ClientDatastore::new(port as u16);
        let trans = datastore.transaction().unwrap();
        trans.get_vertex_count().unwrap()
    });

    assert_eq!(count, 0);
}
