use std::net::ToSocketAddrs;

use crate::client_datastore::ClientDatastore;
use crate::server;

use async_std::net::TcpListener;
use futures::prelude::*;
use indradb::util::generate_temporary_path;
use indradb::{Datastore, Transaction};
use async_std::task::{block_on, spawn_local};

full_test_impl!({
    block_on(async move {
        let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap();
        let listener = TcpListener::bind(&addr).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let f = server::run(listener, indradb::MemoryDatastore::default());
        spawn_local(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())));
        ClientDatastore::new(port as u16)
    })
});

#[test]
fn should_create_rocksdb_datastore() {
    let count = block_on(async move {
        let addr = "127.0.0.1:0".to_socket_addrs().unwrap().next().unwrap();
        let listener = TcpListener::bind(&addr).await.unwrap();
        let port = listener.local_addr().unwrap().port();

        let datastore = indradb::RocksdbDatastore::new(&generate_temporary_path(), None, false).unwrap();
        let f = server::run(listener, datastore);
        spawn_local(Box::pin(f.map_err(|err| panic!(err)).map(|_| ())));

        // Just make sure we can run a command
        let datastore = ClientDatastore::new(port as u16);
        let trans = datastore.transaction().unwrap();
        trans.get_vertex_count().unwrap()
    });

    assert_eq!(count, 0);
}
