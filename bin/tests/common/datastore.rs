use indradb::*;
use std::marker::PhantomData;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicUsize, Ordering};
use super::http::request;
use std::thread::sleep;
use hyper::client::Client;
use std::time::Duration;
use hyper::status::StatusCode;

const START_PORT: usize = 1024;

lazy_static! {
    static ref PORT: AtomicUsize = AtomicUsize::new(START_PORT);
}

#[derive(Debug)]
pub struct HttpDatastore<H: HttpTransaction> {
    port: usize,
    server: Child,
    phantom_http_transaction: PhantomData<H>,
}

impl<H: HttpTransaction> Default for HttpDatastore<H> {
    // Ignore is here because otherwise we get noisy results - it's used in
    // macros which the compiler doesn't seem to pick up on
    #[allow(dead_code)]
    fn default() -> Self {
        let port = PORT.fetch_add(1, Ordering::SeqCst);

        let server = Command::new("../target/debug/indradb-server")
            .envs(hashmap!{"PORT" => port.to_string()})
            .spawn()
            .expect("Server failed to start");

        let client = Client::new();

        for _ in 0..5 {
            let req = request(
                &client,
                port,
                "GET",
                "/",
                vec![],
            );
            let res = req.send();

            if let Ok(res) = res {
                if res.status == StatusCode::NotFound {
                    return HttpDatastore {
                        port: port,
                        server: server,
                        phantom_http_transaction: PhantomData,
                    };
                }
            }

            sleep(Duration::from_secs(1));
        }

        panic!("Server failed to initialize after a few seconds");
    }
}

impl<H: HttpTransaction> Drop for HttpDatastore<H> {
    fn drop(&mut self) {
        if let Err(err) = self.server.kill() {
            panic!(format!("Could not drop server instance: {}", err))
        }
    }
}

impl<H: HttpTransaction> Datastore<H> for HttpDatastore<H> {
    fn transaction(&self) -> Result<H, Error> {
        Ok(H::new(self.port))
    }
}

pub trait HttpTransaction: Transaction {
    fn new(usize) -> Self;
}
