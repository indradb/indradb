use super::{Datastore, Transaction};
use traits::Id;
use std::marker::PhantomData;

pub struct DatastoreTestSandbox<D: Datastore<T, I>, T: Transaction<I>, I: Id> {
	pub name: String,

	pub owner_id: I,
	pub owner_secret: String,

	pub datastore: D,
	pub accounts: Vec<I>,

	phantom_transaction: PhantomData<T>
}

impl<D: Datastore<T, I>, T: Transaction<I>, I: Id> DatastoreTestSandbox<D, T, I> {
	pub fn new(datastore: D) -> DatastoreTestSandbox<D, T, I> {
		return DatastoreTestSandbox{
			name: "".to_string(),
			owner_id: I::default(),
			owner_secret: "".to_string(),
			datastore: datastore,
			accounts: Vec::new(),
			phantom_transaction: PhantomData
		};
	}

	pub fn generate_unique_string(&self, prefix: &str) -> String {
		format!("{}-{}", prefix, self.name.replace(" ", "-"))
	}

	pub fn transaction(&self) -> T {
		self.datastore.transaction(self.owner_id).unwrap()
	}

	pub fn register_account(&mut self, email: &str) -> (I, String) {
		let (id, secret) = self.datastore.create_account(email.to_string()).expect("Expected to be able to create an account");
		self.accounts.push(id);
		(id, secret)
	}

	pub fn setup(&mut self, name: &str) {
		// Set the name first
		self.name = name.to_string();

		// Create a couple of accounts
		let owner_email = self.generate_unique_string("owner");
		let (owner_id, owner_secret) = self.register_account(&owner_email[..]);
		self.owner_id = owner_id;
		self.owner_secret = owner_secret;
	}

	pub fn teardown(&self) {
		// Delete account data
		for id in self.accounts.iter() {
			self.datastore.delete_account(id.clone()).unwrap();
		}
	}
}
