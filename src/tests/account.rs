use super::super::{Datastore, Transaction};
use super::sandbox::DatastoreTestSandbox;
use errors::Error;
use uuid::Uuid;

pub fn should_fail_auth_with_a_bad_username<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let auth = sandbox.datastore.auth(Uuid::default(), "foobar".to_string());
    assert!(!auth.unwrap());
}

pub fn should_fail_auth_with_a_bad_password<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let auth = sandbox.datastore.auth(sandbox.owner_id, "bad_token".to_string());
    assert!(!auth.unwrap());
}

pub fn should_successfully_auth_with_good_credentials<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let auth = sandbox.datastore.auth(sandbox.owner_id, sandbox.owner_secret.clone());
    assert!(auth.unwrap());
}

pub fn should_lookup_valid_accounts<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let results = sandbox.datastore.has_account(sandbox.owner_id);
    assert!(results.unwrap());
}

pub fn should_fail_to_lookup_invalid_accounts<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let results = sandbox.datastore.has_account(Uuid::default());
    assert!(!results.unwrap());
}

pub fn should_fail_when_attempting_to_delete_invalid_accounts<D, T>(sandbox: &mut DatastoreTestSandbox<D, T>)
    where D: Datastore<T>,
          T: Transaction
{
    let result = sandbox.datastore.delete_account(Uuid::default());
    assert_eq!(result.unwrap_err(), Error::AccountNotFound);
}
