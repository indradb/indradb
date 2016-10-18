use nutrino::*;
use std::process::Command;
use rand::{Rng, thread_rng};
use std::collections::BTreeMap;
use regex::Regex;
use std::marker::PhantomData;
use std::env;
use std::str;
use std::cell::RefCell;

lazy_static! {
    static ref ACCOUNT_ID_MATCHER: Regex = Regex::new(r"Account ID: (\d+)").unwrap();
    static ref ACCOUNT_SECRET_MATCHER: Regex = Regex::new(r"Account secret: (.+)").unwrap();
}

// We need to remember all of the secrets for the accounts that were created, to make
// authenticated requests. This is stored in TLS to avoid storing it in the struct. We cannot
// store this in the struct because it'd make `create_account` require a mutable self reference,
// which the `Datastore` trait does not define.
thread_local! {
    static ACCOUNT_IDS: RefCell<BTreeMap<i64, String>> = RefCell::new(BTreeMap::new());
}

#[derive(Clone, Debug)]
pub struct HttpDatastore<H: HttpTransaction<T>, T: Transaction<i64>> {
	port: i32,
    phantom_http_transaction: PhantomData<H>,
    phantom_transaction: PhantomData<T>
}

impl<H: HttpTransaction<T>, T: Transaction<i64>> HttpDatastore<H, T> {
	pub fn new(port: i32) -> HttpDatastore<H, T> {
		HttpDatastore {
			port: port,
            phantom_http_transaction: PhantomData,
            phantom_transaction: PhantomData
		}
	}
}

impl<H: HttpTransaction<T>, T: Transaction<i64>> Datastore<T, i64> for HttpDatastore<H, T> {
	fn has_account(&self, user_id: i64) -> Result<bool, Error> {
        panic!("Unimplemented")
	}

	fn create_account(&self, email: String) -> Result<(i64, String), Error> {
        let random_value: String = thread_rng().gen_ascii_chars().take(10).collect();
        let email = format!("bin-test-{}@nutrino.com", random_value);

        let create_user_output = Command::new("./target/debug/nutrino-user")
            .arg("add")
            .arg(email)
            .output();

        match create_user_output {
            Ok(output) => {
                if !output.status.success() {
                    Err(Error::Unexpected(format!("Unexpected exit status running `nutrino-user`: {}", output.status)))
                } else {
                    let stdout = str::from_utf8(&output.stdout).unwrap();
                    let account_id: i64 = ACCOUNT_ID_MATCHER.captures(stdout).unwrap().at(1).unwrap().parse::<i64>().unwrap();
                    let secret: String = ACCOUNT_SECRET_MATCHER.captures(stdout).unwrap().at(1).unwrap().to_string();

                    ACCOUNT_IDS.with(|account_ids| {
                        let ref mut account_ids: BTreeMap<i64, String> = *account_ids.borrow_mut();
                        account_ids.insert(account_id, secret.clone());
                    });

                    Ok((account_id, secret))
                }
            },
            Err(err) => {
                Err(Error::Unexpected(format!("Could not run `nutrino-user`: {}", err)))
            }
        }
	}

	fn delete_account(&self, user_id: i64) -> Result<(), Error> {
        let remove_user_status = Command::new("./target/debug/nutrino-user")
            .arg("remove")
            .arg(user_id.to_string())
            .status();

        match remove_user_status {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(Error::Unexpected(format!("Unexpected exit status running `nutrino-user`: {}", status.code().unwrap())))
                }
            },
            Err(err) => {
                Err(Error::Unexpected(format!("Unexpected error running `nutrino-user`: {}", err)))
            }
        }
	}

	fn auth(&self, user_id: i64, secret: String) -> Result<bool, Error> {
        panic!("Unimplemented")
	}

	fn transaction(&self, user_id: i64) -> Result<T, Error> {
        let secret = ACCOUNT_IDS.with(|account_ids| {
            let ref account_ids: BTreeMap<i64, String> = *account_ids.borrow();
            account_ids.get(&user_id).unwrap().clone()
        });

        Ok(H::new(self.port, user_id, secret))
	}
}

pub trait HttpTransaction<T: Transaction<i64>> {
    fn new(i32, i64, String) -> T;
}
