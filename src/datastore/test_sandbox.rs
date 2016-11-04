use super::{Datastore, Transaction};
use traits::Id;
use models;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub struct DatastoreTestSandbox<D: Datastore<T, I>, T: Transaction<I>, I: Id> {
	pub name: String,

	pub owner_id: I,
	pub owner_secret: String,

	pub datastore: D,
	pub vertices: BTreeMap<String, BTreeMap<String, I>>,
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
			vertices: BTreeMap::new(),
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

	pub fn search_id(&self, t: &str, name: &str) -> I {
		let container = self.vertices.get(t).unwrap();
		container.get(name).unwrap().clone()
	}

	pub fn create_test_vertex(&mut self, t: &str, name: &str) -> I {
		let trans = self.datastore.transaction(self.owner_id).unwrap();
		let id = trans.create_vertex(t.to_string()).unwrap();
		trans.commit().unwrap();

		if !self.vertices.contains_key(t) {
			self.vertices.insert(t.to_string(), BTreeMap::new());
		}

		let container = self.vertices.get_mut(t).unwrap();
		container.insert(name.to_string(), id);
		id
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

pub fn insert_sample_data<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	// Insert some users
	let jill_id = sandbox.create_test_vertex("user", "Jill");
	let bob_id = sandbox.create_test_vertex("user", "Bob");
	let christopher_id = sandbox.create_test_vertex("user", "Christopher");

	// Insert some movies
	let doodlebug_id = sandbox.create_test_vertex("movie", "Doodlebug");
	let following_id = sandbox.create_test_vertex("movie", "Following");
	let memento_id = sandbox.create_test_vertex("movie", "Memento");
	let insomnia_id = sandbox.create_test_vertex("movie", "Insomnia");
	let batman_begins_id = sandbox.create_test_vertex("movie", "Batman Begins");
	let prestige_id = sandbox.create_test_vertex("movie", "The Prestige");
	let dark_knight_id = sandbox.create_test_vertex("movie", "The Dark Knight");
	let inception_id = sandbox.create_test_vertex("movie", "Inception");
	let dark_knight_rises_id = sandbox.create_test_vertex("movie", "The Dark Knight Rises");
	let interstellar_id = sandbox.create_test_vertex("movie", "Interstellar");

	// Create a new transaction for inserting all the test edges
	let trans = sandbox.transaction();

	// Jill isn't a fan
	new_review_edge(&trans, jill_id, inception_id, -0.8);
	new_review_edge(&trans, jill_id, dark_knight_rises_id, -0.9);
	new_review_edge(&trans, jill_id, interstellar_id, -0.8);

	// Bob likes some stuff
	new_purchased_edge(&trans, bob_id, inception_id, 1.0);
	new_purchased_edge(&trans, bob_id, interstellar_id, 1.0);
	new_review_edge(&trans, bob_id, memento_id, 0.2);
	new_review_edge(&trans, bob_id, insomnia_id, -1.0);
	new_review_edge(&trans, bob_id, batman_begins_id, 0.7);
	new_review_edge(&trans, bob_id, prestige_id, 0.8);
	new_review_edge(&trans, bob_id, dark_knight_id, 0.9);
	new_review_edge(&trans, bob_id, inception_id, 1.0);
	new_review_edge(&trans, bob_id, dark_knight_rises_id, 0.8);
	new_review_edge(&trans, bob_id, interstellar_id, 1.0);

	// Christopher really likes these movies
	new_purchased_edge(&trans, christopher_id, doodlebug_id, 1.0);
	new_purchased_edge(&trans, christopher_id, following_id, 1.0);
	new_purchased_edge(&trans, christopher_id, memento_id, 1.0);
	new_purchased_edge(&trans, christopher_id, insomnia_id, 1.0);
	new_purchased_edge(&trans, christopher_id, batman_begins_id, 1.0);
	new_purchased_edge(&trans, christopher_id, prestige_id, 1.0);
	new_purchased_edge(&trans, christopher_id, dark_knight_id, 1.0);
	new_purchased_edge(&trans, christopher_id, inception_id, 1.0);
	new_purchased_edge(&trans, christopher_id, dark_knight_rises_id, 1.0);
	new_purchased_edge(&trans, christopher_id, interstellar_id, 1.0);
	new_review_edge(&trans, christopher_id, batman_begins_id, 1.0);
	new_review_edge(&trans, christopher_id, prestige_id, 1.0);
	new_review_edge(&trans, christopher_id, dark_knight_id, 1.0);
	new_review_edge(&trans, christopher_id, inception_id, 1.0);
	new_review_edge(&trans, christopher_id, dark_knight_rises_id, 1.0);
	new_review_edge(&trans, christopher_id, interstellar_id, 1.0);

	// Jill and Bob follow each other; Christopher is anti-social
	new_follows_edge(&trans, jill_id, bob_id, 1.0);
	new_follows_edge(&trans, bob_id, jill_id, 1.0);

	trans.commit().unwrap();
}

fn new_review_edge<T: Transaction<I>, I: Id>(trans: &T, outbound_id: I, inbound_id: I, weight: f32) {
	trans.set_edge(models::Edge::new(outbound_id, "review".to_string(), inbound_id, weight)).unwrap()
}

fn new_purchased_edge<T: Transaction<I>, I: Id>(trans: &T, outbound_id: I, inbound_id: I, weight: f32) {
	trans.set_edge(models::Edge::new(outbound_id, "purchased".to_string(), inbound_id, weight)).unwrap()
}

fn new_follows_edge<T: Transaction<I>, I: Id>(trans: &T, outbound_id: I, inbound_id: I, weight: f32) {
	trans.set_edge(models::Edge::new(outbound_id, "follows".to_string(), inbound_id, weight)).unwrap()
}
