use braid::*;
use std::marker::PhantomData;
use std::process::{Command, Child};
use uuid::Uuid;
use std::sync::Mutex;
use super::http::request;
use std::thread::sleep;
use hyper::client::Client;
use std::time::Duration;
use hyper::status::StatusCode;

const START_PORT: usize = 1024;

lazy_static! {
    static ref PORT: Mutex<usize> = Mutex::new(START_PORT);
}

#[derive(Debug)]
pub struct HttpDatastore<H: HttpTransaction> {
    port: usize,
    server: Child,
    phantom_http_transaction: PhantomData<H>,
}

impl<H: HttpTransaction> HttpDatastore<H> {
    // Ignore is here because otherwise we get noisy results - it's used in
    // macros which the compiler doesn't seem to pick up on
    #[allow(dead_code)]
    pub fn new() -> HttpDatastore<H> {
        let port = {
            let mut port = PORT.lock().unwrap();
            *port += 1;
            (*port).clone()
        };

        let server = Command::new("../target/debug/braid-server")
            .envs(hashmap!{"PORT" => port.to_string()})
            .spawn()
            .expect("Server failed to start");

        let client = Client::new();

        for _ in 0..5 {
            let req = request(&client, port, Uuid::default(), "".to_string(), "GET", "/".to_string(), vec![]);
            let res = req.send();

            if let Ok(res) = res {
                if res.status == StatusCode::NotFound {
                    return  HttpDatastore {
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
    fn has_account(&self, id: Uuid) -> Result<bool, Error> {
        return Ok(id == Uuid::default())
    }

    fn create_account(&self) -> Result<(Uuid, String), Error> {
        Ok((Uuid::default(), "".to_string()))
    }

    fn delete_account(&self, _: Uuid) -> Result<(), Error> {
        // Don't actually do anything, because all data is process-local and
        // will die with the process.
        Ok(())
    }

    fn auth(&self, id: Uuid, secret: String) -> Result<bool, Error> {
        return Ok(id == Uuid::default() && &secret[..] == "")
    }

    fn transaction(&self, account_id: Uuid) -> Result<H, Error> {
        Ok(H::new(self.port, account_id, "".to_string()))
    }
}

pub trait HttpTransaction: Transaction {
    fn new(usize, Uuid, String) -> Self;
}
