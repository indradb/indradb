use indradb::*;
use std::marker::PhantomData;
use std::process::{Child, Command};
use uuid::Uuid;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::http;
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use reqwest::{Method, StatusCode};

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

        let mut envs = HashMap::new();
        envs.insert("PORT", port.to_string());

        let server = Command::new("../target/debug/indradb-server")
            .envs(envs)
            .spawn()
            .expect("Server failed to start");

        for _ in 0..5 {
            let result = http::request(
                port,
                Uuid::default(),
                "".to_string(),
                Method::Post,
                "/transaction",
                &vec![],
                Some(json!([]))
            );

            if let Ok(response) = result {
                if response.status() == StatusCode::Ok {
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
    fn has_account(&self, id: Uuid) -> Result<bool, Error> {
        Ok(id == Uuid::default())
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
        Ok(id == Uuid::default() && &secret[..] == "")
    }

    fn transaction(&self, account_id: Uuid) -> Result<H, Error> {
        Ok(H::new(self.port, account_id, "".to_string()))
    }
}

pub trait HttpTransaction: Transaction {
    fn new(usize, Uuid, String) -> Self;
}
