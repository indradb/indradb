use nutrino::*;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::cell::RefCell;

use super::accounts::{create_account, delete_account};

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
	fn has_account(&self, _: i64) -> Result<bool, Error> {
        panic!("Unimplemented")
	}

	fn create_account(&self, email: String) -> Result<(i64, String), Error> {
        let (account_id, secret) = try!(create_account(email));

        ACCOUNT_IDS.with(|account_ids| {
            let ref mut account_ids: BTreeMap<i64, String> = *account_ids.borrow_mut();
            account_ids.insert(account_id, secret.clone());
        });

        Ok((account_id, secret))
	}

	fn delete_account(&self, user_id: i64) -> Result<(), Error> {
        delete_account(user_id)
	}

	fn auth(&self, _: i64, _: String) -> Result<bool, Error> {
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
