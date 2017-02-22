use braid::*;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::cell::RefCell;
use uuid::Uuid;

use super::accounts::{create_account, delete_account};

// We need to remember all of the secrets for the accounts that were created, to make
// authenticated requests. This is stored in TLS to avoid storing it in the struct. We cannot
// store this in the struct because it'd make `create_account` require a mutable self reference,
// which the `Datastore` trait does not define.
thread_local! {
    static ACCOUNT_IDS: RefCell<BTreeMap<Uuid, String>> = RefCell::new(BTreeMap::new());
}

#[derive(Clone, Debug)]
pub struct HttpDatastore<H: HttpTransaction<T>, T: Transaction<Uuid>> {
    port: i32,
    phantom_http_transaction: PhantomData<H>,
    phantom_transaction: PhantomData<T>,
}

impl<H: HttpTransaction<T>, T: Transaction<Uuid>> HttpDatastore<H, T> {
    // Ignore is here because otherwise we get noisy results - it's used in
    // macros which the compiler doesn't seem to pick up on
    #[allow(dead_code)]
    pub fn new(port: i32) -> HttpDatastore<H, T> {
        HttpDatastore {
            port: port,
            phantom_http_transaction: PhantomData,
            phantom_transaction: PhantomData,
        }
    }
}

impl<H: HttpTransaction<T>, T: Transaction<Uuid>> Datastore<T, Uuid> for HttpDatastore<H, T> {
    fn has_account(&self, _: Uuid) -> Result<bool, Error> {
        panic!("Unimplemented")
    }

    fn create_account(&self, email: String) -> Result<(Uuid, String), Error> {
        let (account_id, secret) = create_account(email)?;

        ACCOUNT_IDS.with(|account_ids| {
            let ref mut account_ids: BTreeMap<Uuid, String> = *account_ids.borrow_mut();
            account_ids.insert(account_id, secret.clone());
        });

        Ok((account_id, secret))
    }

    fn delete_account(&self, account_id: Uuid) -> Result<(), Error> {
        delete_account(account_id)
    }

    fn auth(&self, _: Uuid, _: String) -> Result<bool, Error> {
        panic!("Unimplemented")
    }

    fn transaction(&self, account_id: Uuid) -> Result<T, Error> {
        let secret = ACCOUNT_IDS.with(|account_ids| {
            let ref account_ids: BTreeMap<Uuid, String> = *account_ids.borrow();
            account_ids.get(&account_id).unwrap().clone()
        });

        Ok(H::new(self.port, account_id, secret))
    }
}

pub trait HttpTransaction<T: Transaction<Uuid>> {
    fn new(i32, Uuid, String) -> T;
}
