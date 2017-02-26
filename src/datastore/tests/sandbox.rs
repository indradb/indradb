use datastore::{Datastore, Transaction};
use uuid::Uuid;
use std::marker::PhantomData;

pub struct DatastoreTestSandbox<D: Datastore<T>, T: Transaction> {
    pub name: String,

    pub owner_id: Uuid,
    pub owner_secret: String,

    pub datastore: D,
    pub accounts: Vec<Uuid>,

    phantom_transaction: PhantomData<T>,
}

impl<D: Datastore<T>, T: Transaction> DatastoreTestSandbox<D, T> {
    pub fn new(datastore: D) -> DatastoreTestSandbox<D, T> {
        DatastoreTestSandbox {
            name: "".to_string(),
            owner_id: Uuid::default(),
            owner_secret: "".to_string(),
            datastore: datastore,
            accounts: Vec::new(),
            phantom_transaction: PhantomData,
        }
    }

    pub fn generate_unique_string(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.name.replace(" ", "-"))
    }

    pub fn transaction(&self) -> T {
        self.datastore.transaction(self.owner_id).unwrap()
    }

    pub fn register_account(&mut self, email: &str) -> (Uuid, String) {
        let (id, secret) = self.datastore
            .create_account(email.to_string())
            .expect("Expected to be able to create an account");
        self.accounts.push(id);
        (id, secret)
    }

    pub fn setup(&mut self, name: &str) {
        // Set the test name
        self.name = name.to_string();

        // Create an account
        let owner_email = self.generate_unique_string("owner");
        let (owner_id, owner_secret) = self.register_account(&owner_email[..]);
        self.owner_id = owner_id;
        self.owner_secret = owner_secret;
    }

    pub fn teardown(&self) {
        // Delete account data
        for id in &self.accounts {
            self.datastore.delete_account(*id).unwrap();
        }
    }
}
