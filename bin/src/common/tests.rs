use client_datastore::ClientDatastore;
use server;
use std::thread::spawn;
use std::sync::atomic::{Ordering, AtomicUsize};

const START_PORT: u16 = 27616;

lazy_static! {
    static ref CURRENT_PORT: AtomicUsize = AtomicUsize::new(START_PORT as usize);
}

full_test_impl!({
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);
    spawn(move || server::start(&format!("127.0.0.1:{}", port)));
    ClientDatastore::new(port as u16)
});
