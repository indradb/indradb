use client_datastore::ClientDatastore;
use indradb::util::generate_temporary_path;
use indradb::{Datastore, Transaction};
use server;
use std::panic::catch_unwind;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use futures::{Future, Stream};
use futures::sync::mpsc::channel;

const START_PORT: u16 = 27616;

lazy_static! {
    static ref CURRENT_PORT: AtomicUsize = AtomicUsize::new(START_PORT as usize);
}

full_test_impl!({
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    ClientDatastore::new(port as u16, "memory://".to_string())
});

#[test]
fn should_create_rocksdb_datastore() {
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    let connection_string = format!("rocksdb://{}", generate_temporary_path());
    let datastore = ClientDatastore::new(port as u16, connection_string);

    // Just make sure we can run a command
    let trans = datastore.transaction().unwrap();
    let count = trans.get_vertex_count().unwrap();
    assert_eq!(count, 0);
}

#[test]
fn should_panic_on_bad_connection_string() {
    let zero = Duration::from_secs(0);

    let result = catch_unwind(|| {
        let (_, shutdown_receiver) = channel::<()>(1);
        let shutdown_receiver = shutdown_receiver
            .into_future()
            .map(|_| {})
            .map_err(|_| unreachable!());
        server::run_until("127.0.0.1:9999", "foo://", 1, zero, shutdown_receiver)
    });

    assert!(result.is_err());
}
