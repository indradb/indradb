use std::sync::atomic::AtomicUsize;
use client_datastore::ClientDatastore;
use server;
use std::thread::spawn;
use std::sync::atomic::Ordering;
use indradb::util::generate_temporary_path;
use std::path::Path;
use std::panic::catch_unwind;
use indradb::{Datastore, Transaction};

const START_PORT: u16 = 27616;

lazy_static! {
    static ref CURRENT_PORT: AtomicUsize = AtomicUsize::new(START_PORT as usize);
}

full_test_impl!({
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    spawn(move || server::start(&format!("127.0.0.1:{}", port), "memory://"));
    ClientDatastore::new(port as u16)
});

#[test]
fn should_create_rocksdb_datastore() {
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);

    spawn(move || server::start(&format!("127.0.0.1:{}", port), &format!("rocksdb://{}", generate_temporary_path())));

    // Just make sure we can run a command
    let datastore = ClientDatastore::new(port as u16);
    let trans = datastore.transaction().unwrap();
    let count = trans.get_vertex_count().unwrap();

    assert_eq!(count, 0);
}

#[test]
fn should_panic_on_bad_connection_string() {
    let result = catch_unwind(|| server::start("127.0.0.1:9999", "foo://"));
    assert!(result.is_err());
}
