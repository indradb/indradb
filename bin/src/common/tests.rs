use client_datastore::ClientDatastore;
use executor;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread::spawn;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

const START_PORT: u16 = 27616;

lazy_static! {
    static ref CURRENT_PORT: AtomicUsize = AtomicUsize::new(START_PORT as usize);
}

full_test_impl!({
    let port = (*CURRENT_PORT).fetch_add(1, Ordering::SeqCst);

    spawn(move || {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port as u16);
        let datastore = indradb::MemoryDatastore::default();
        executor::run(addr, 1, datastore).expect("Server did not start");
    });
    
    ClientDatastore::new(port as u16)
});
