use super::{Id, Datastore, Transaction, Request, Response, ErrorResponse, models};
use std::collections::{BTreeMap, HashSet};
use core::ops::{Add, Sub};
use chrono::duration::Duration;
use chrono::UTC;
use chrono::naive::datetime::NaiveDateTime;
use serde_json::Value as JsonValue;
use std::marker::PhantomData;
use rand::{thread_rng, Rng};

fn create_test_properties(name: &str) -> BTreeMap<String, JsonValue> {
	let mut props = BTreeMap::new();
	props.insert("name".to_string(), JsonValue::String(name.to_string()));
	props
}

fn new_review_edge(outbound_id: i64, inbound_id: i64, weight: f32) -> Request {
	Request::SetEdge(models::Edge::new(outbound_id, "review".to_string(), inbound_id, weight))
}

fn new_purchased_edge(outbound_id: i64, inbound_id: i64, weight: f32) -> Request {
	Request::SetEdge(models::Edge::new(outbound_id, "purchased".to_string(), inbound_id, weight))
}

fn new_follows_edge(outbound_id: i64, inbound_id: i64, weight: f32) -> Request {
	Request::SetEdge(models::Edge::new(outbound_id, "follows".to_string(), inbound_id, weight))
}

pub struct DatastoreTestSandbox<D: Datastore<T, I>, T: Transaction, I: Id> {
	name: String,

	owner_id: I,
	owner_secret: String,

	pub datastore: D,
	pub vertices: Vec<models::Vertex>,
	pub accounts: Vec<I>,

	phantom_transaction: PhantomData<T>
}

impl<D: Datastore<T, I>, T: Transaction, I: Id> DatastoreTestSandbox<D, T, I> {
	pub fn new(name: String, datastore: D) -> DatastoreTestSandbox<D, T, I> {
		return DatastoreTestSandbox{
			name: name.clone(),
			owner_id: I::default(),
			owner_secret: "".to_string(),
			datastore: datastore,
			vertices: Vec::new(),
			accounts: Vec::new(),
			phantom_transaction: PhantomData
		};
	}

	fn generate_email(&self, prefix: &str) -> String {
		format!("{}-{}@nutrino.com", prefix, self.name.clone())
	}

	fn transaction(&self) -> T {
		self.datastore.transaction(self.owner_id.clone()).expect("Expected to be able to create a transaction")
	}

	fn search_id(&self, t: &str, name: &str) -> i64 {
		for vertex in self.vertices.iter() {
			if vertex.t != t {
				continue;
			}

			if vertex.properties.get("name").unwrap().as_str().unwrap().to_string() != name.to_string() {
				continue;
			}

			return vertex.id;
		}

		panic!("Could not find vertex with type=\"{}\" and name=\"{}\"", t, name);
	}

	fn fake_id(&self) -> i64 {
		let mut actual_ids: HashSet<i64> = HashSet::new();

		for vertex in self.vertices.iter() {
			actual_ids.insert(vertex.id);
		}

		let mut rng = thread_rng();

		loop {
			let candidate_id = rng.gen::<i64>();

			if !actual_ids.contains(&candidate_id.clone()) {
				return candidate_id;
			}
		}
	}

	fn jill_id(&self) -> i64 {
		self.search_id("user", "Jill")
	}

	fn bob_id(&self) -> i64 {
		self.search_id("user", "Bob")
	}

	fn christopher_id(&self) -> i64 {
		self.search_id("user", "Christopher")
	}

	fn memento_id(&self) -> i64 {
		self.search_id("movie", "Memento")
	}

	fn inception_id(&self) -> i64 {
		self.search_id("movie", "Inception")
	}

	fn create_test_vertex(&mut self, t: &str, name: Option<&str>) -> i64 {
		let mut trans = self.datastore.transaction(self.owner_id.clone()).expect("Expected to be able to create a transaction");

		let props = match name {
			Some(name) => create_test_properties(name),
			None => BTreeMap::new()
		};

		trans.request(Request::CreateVertex(t.to_string(), props.clone()));
		let item = single_response_from_transaction(&mut trans);

		let id = match item {
			Ok(Response::VertexId(id)) => id,
			_ => {
				assert!(false, format!("Unexpected response: {:?}", item));
				-1
			}
		};

		self.vertices.push(models::Vertex::new_with_properties(id, t.to_string(), props));
		id
	}

	fn register_account(&mut self, email: &str) -> (I, String) {
		let (id, secret) = self.datastore.create_account(email.to_string()).expect("Expected to be able to create an account");
		self.accounts.push(id.clone());
		(id, secret)
	}

	pub fn setup(&mut self) {
		// First create a couple of accounts
		let owner_email = self.generate_email("owner");
		let (owner_id, owner_secret) = self.register_account(&owner_email[..]);
		self.owner_id = owner_id;
		self.owner_secret = owner_secret;

		// Insert some users
		let jill_id = self.create_test_vertex("user", Some("Jill"));
		let bob_id = self.create_test_vertex("user", Some("Bob"));
		let christopher_id = self.create_test_vertex("user", Some("Christopher"));

		// Insert some movies
		let doodlebug_id = self.create_test_vertex("movie", Some("Doodlebug"));
		let following_id = self.create_test_vertex("movie", Some("Following"));
		let memento_id = self.create_test_vertex("movie", Some("Memento"));
		let insomnia_id = self.create_test_vertex("movie", Some("Insomnia"));
		let batman_begins_id = self.create_test_vertex("movie", Some("Batman Begins"));
		let prestige_id = self.create_test_vertex("movie", Some("The Prestige"));
		let dark_knight_id = self.create_test_vertex("movie", Some("The Dark Knight"));
		let inception_id = self.create_test_vertex("movie", Some("Inception"));
		let dark_knight_rises_id = self.create_test_vertex("movie", Some("The Dark Knight Rises"));
		let interstellar_id = self.create_test_vertex("movie", Some("Interstellar"));

		// Create a new transaction for inserting all the test edges
		let mut trans = self.datastore.transaction(self.owner_id.clone()).expect("Expected to be able to create a transaction");

		// Jill isn't a fan
		trans.request(new_review_edge(jill_id, inception_id, -0.8));
		trans.request(new_review_edge(jill_id, dark_knight_rises_id, -0.9));
		trans.request(new_review_edge(jill_id, interstellar_id, -0.8));

		// Bob likes some stuff
		trans.request(new_purchased_edge(bob_id, inception_id, 1.0));
		trans.request(new_purchased_edge(bob_id, interstellar_id, 1.0));
		trans.request(new_review_edge(bob_id, memento_id, 0.2));
		trans.request(new_review_edge(bob_id, insomnia_id, -1.0));
		trans.request(new_review_edge(bob_id, batman_begins_id, 0.7));
		trans.request(new_review_edge(bob_id, prestige_id, 0.8));
		trans.request(new_review_edge(bob_id, dark_knight_id, 0.9));
		trans.request(new_review_edge(bob_id, inception_id, 1.0));
		trans.request(new_review_edge(bob_id, dark_knight_rises_id, 0.8));
		trans.request(new_review_edge(bob_id, interstellar_id, 1.0));

		// Christopher really likes these movies
		trans.request(new_purchased_edge(christopher_id, doodlebug_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, following_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, memento_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, insomnia_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, batman_begins_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, prestige_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, dark_knight_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, inception_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, dark_knight_rises_id, 1.0));
		trans.request(new_purchased_edge(christopher_id, interstellar_id, 1.0));
		trans.request(new_review_edge(christopher_id, batman_begins_id, 1.0));
		trans.request(new_review_edge(christopher_id, prestige_id, 1.0));
		trans.request(new_review_edge(christopher_id, dark_knight_id, 1.0));
		trans.request(new_review_edge(christopher_id, inception_id, 1.0));
		trans.request(new_review_edge(christopher_id, dark_knight_rises_id, 1.0));
		trans.request(new_review_edge(christopher_id, interstellar_id, 1.0));

		// Jill and Bob follow each other; Christopher is anti-social
		trans.request(new_follows_edge(jill_id, bob_id, 1.0));
		trans.request(new_follows_edge(bob_id, jill_id, 1.0));

		assert!(trans.commit().is_ok());
	}

	pub fn teardown(&self) {
		for id in self.accounts.iter() {
			self.datastore.delete_account(id.clone()).expect("Expected to be able to delete the account");
		}
	}
}

#[macro_export]
macro_rules! test_datastore_impl {
	(fn datastore() -> $typ:ty $code:block) => (
		fn datastore() -> $typ $code

		#[test]
		fn auth_bad_username() {
			datastore_test::run(datastore(), "auth_bad_username", |sandbox| {
				::datastore_test::auth_bad_username(sandbox)
			});
		}

		#[test]
		fn auth_bad_password() {
			::datastore_test::run(datastore(), "auth_bad_password", |sandbox| {
				::datastore_test::auth_bad_password(sandbox)
			});
		}

		#[test]
		fn auth_good() {
			::datastore_test::run(datastore(), "auth_good", |sandbox| {
				::datastore_test::auth_good(sandbox)
			});
		}

		#[test]
		fn has_account_existing() {
			::datastore_test::run(datastore(), "has_account_existing", |sandbox| {
				::datastore_test::has_account_existing(sandbox)
			});
		}

		#[test]
		fn has_account_nonexisting() {
			::datastore_test::run(datastore(), "has_account_nonexisting", |sandbox| {
				::datastore_test::has_account_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_account_nonexisting() {
			::datastore_test::run(datastore(), "delete_account_nonexisting", |sandbox| {
				::datastore_test::delete_account_nonexisting(sandbox)
			});
		}

		#[test]
		fn get_vertex_existing() {
			::datastore_test::run(datastore(), "get_vertex_existing", |sandbox| {
				::datastore_test::get_vertex_existing(sandbox)
			});
		}

		#[test]
		fn get_vertex_nonexisting() {
			::datastore_test::run(datastore(), "get_vertex_nonexisting", |sandbox| {
				::datastore_test::get_vertex_nonexisting(sandbox)
			});
		}

		#[test]
		fn create_vertex() {
			::datastore_test::run(datastore(), "create_vertex", |sandbox| {
				::datastore_test::create_vertex(sandbox)
			});
		}

		#[test]
		fn set_vertex_existing() {
			::datastore_test::run(datastore(), "set_vertex_existing", |sandbox| {
				::datastore_test::set_vertex_existing(sandbox)
			});
		}

		#[test]
		fn set_vertex_nonexisting() {
			::datastore_test::run(datastore(), "set_vertex_nonexisting", |sandbox| {
				::datastore_test::set_vertex_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_vertex_existing() {
			::datastore_test::run(datastore(), "delete_vertex_existing", |sandbox| {
				::datastore_test::delete_vertex_existing(sandbox)
			});
		}

		#[test]
		fn delete_vertex_nonexisting() {
			::datastore_test::run(datastore(), "delete_vertex_nonexisting", |sandbox| {
				::datastore_test::delete_vertex_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_vertex_bad_permissions() {
			::datastore_test::run(datastore(), "delete_vertex_bad_permissions", |sandbox| {
				::datastore_test::delete_vertex_bad_permissions(sandbox)
			});
		}

		#[test]
		fn get_edge_existing() {
			::datastore_test::run(datastore(), "get_edge_existing", |sandbox| {
				::datastore_test::get_edge_existing(sandbox)
			});
		}

		#[test]
		fn get_edge_nonexisting() {
			::datastore_test::run(datastore(), "get_edge_nonexisting", |sandbox| {
				::datastore_test::get_edge_nonexisting(sandbox)
			});
		}

		#[test]
		fn set_edge_existing() {
			::datastore_test::run(datastore(), "set_edge_existing", |sandbox| {
				::datastore_test::set_edge_existing(sandbox)
			});
		}

		#[test]
		fn set_edge_nonexisting() {
			::datastore_test::run(datastore(), "set_edge_nonexisting", |sandbox| {
				::datastore_test::set_edge_nonexisting(sandbox)
			});
		}

		#[test]
		fn set_edge_bad_weight() {
			::datastore_test::run(datastore(), "set_edge_bad_weight", |sandbox| {
				::datastore_test::set_edge_bad_weight(sandbox)
			});
		}

		#[test]
		fn set_edge_bad_permissions() {
			::datastore_test::run(datastore(), "set_edge_bad_permissions", |sandbox| {
				::datastore_test::set_edge_bad_permissions(sandbox)
			});
		}

		#[test]
		fn delete_edge_existing() {
			::datastore_test::run(datastore(), "delete_edge_existing", |sandbox| {
				::datastore_test::delete_edge_existing(sandbox)
			});
		}

		#[test]
		fn delete_edge_nonexisting() {
			::datastore_test::run(datastore(), "delete_edge_nonexisting", |sandbox| {
				::datastore_test::delete_edge_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_edge_bad_permissions() {
			::datastore_test::run(datastore(), "delete_edge_bad_permissions", |sandbox| {
				::datastore_test::delete_edge_bad_permissions(sandbox)
			});
		}

		#[test]
		fn get_edge_count_existing() {
			::datastore_test::run(datastore(), "get_edge_count_existing", |sandbox| {
				::datastore_test::get_edge_count_existing(sandbox)
			});
		}

		#[test]
		fn get_edge_count_nonexisting() {
			::datastore_test::run(datastore(), "get_edge_count_nonexisting", |sandbox| {
				::datastore_test::get_edge_count_nonexisting(sandbox)
			});
		}

		#[test]
		fn get_edge_range_existing() {
			::datastore_test::run(datastore(), "get_edge_range_existing", |sandbox| {
				::datastore_test::get_edge_range_existing(sandbox)
			});
		}

		#[test]
		fn get_edge_range_nonexisting() {
			::datastore_test::run(datastore(), "get_edge_range_nonexisting", |sandbox| {
				::datastore_test::get_edge_range_nonexisting(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_full() {
			::datastore_test::run(datastore(), "get_edge_time_range_full", |sandbox| {
				::datastore_test::get_edge_time_range_full(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_empty() {
			::datastore_test::run(datastore(), "get_edge_time_range_empty", |sandbox| {
				::datastore_test::get_edge_time_range_empty(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_no_high() {
			::datastore_test::run(datastore(), "get_edge_time_range_no_high", |sandbox| {
				::datastore_test::get_edge_time_range_no_high(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_no_low() {
			::datastore_test::run(datastore(), "get_edge_time_range_no_low", |sandbox| {
				::datastore_test::get_edge_time_range_no_low(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_no_time() {
			::datastore_test::run(datastore(), "get_edge_time_range_no_time", |sandbox| {
				::datastore_test::get_edge_time_range_no_time(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_reversed_time() {
			::datastore_test::run(datastore(), "get_edge_time_range_reversed_time", |sandbox| {
				::datastore_test::get_edge_time_range_reversed_time(sandbox)
			});
		}

		#[test]
		fn get_local_metadata_existing() {
			::datastore_test::run(datastore(), "get_local_metadata_existing", |sandbox| {
				::datastore_test::get_local_metadata_existing(sandbox)
			});
		}

		#[test]
		fn get_local_metadata_nonexisting() {
			::datastore_test::run(datastore(), "get_local_metadata_nonexisting", |sandbox| {
				::datastore_test::get_local_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn set_local_metadata_existing() {
			::datastore_test::run(datastore(), "set_local_metadata_existing", |sandbox| {
				::datastore_test::set_local_metadata_existing(sandbox)
			});
		}

		#[test]
		fn set_local_metadata_nonexisting() {
			::datastore_test::run(datastore(), "set_local_metadata_nonexisting", |sandbox| {
				::datastore_test::set_local_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_local_metadata_existing() {
			::datastore_test::run(datastore(), "delete_local_metadata_existing", |sandbox| {
				::datastore_test::delete_local_metadata_existing(sandbox)
			});
		}

		#[test]
		fn delete_local_metadata_nonexisting() {
			::datastore_test::run(datastore(), "delete_local_metadata_nonexisting", |sandbox| {
				::datastore_test::delete_local_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn get_global_metadata_existing() {
			::datastore_test::run(datastore(), "get_global_metadata_existing", |sandbox| {
				::datastore_test::get_global_metadata_existing(sandbox)
			});
		}

		#[test]
		fn get_global_metadata_nonexisting() {
			::datastore_test::run(datastore(), "get_global_metadata_nonexisting", |sandbox| {
				::datastore_test::get_global_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn set_global_metadata_existing() {
			::datastore_test::run(datastore(), "set_global_metadata_existing", |sandbox| {
				::datastore_test::set_global_metadata_existing(sandbox)
			});
		}

		#[test]
		fn set_global_metadata_nonexisting() {
			::datastore_test::run(datastore(), "set_global_metadata_nonexisting", |sandbox| {
				::datastore_test::set_global_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_global_metadata_existing() {
			::datastore_test::run(datastore(), "delete_global_metadata_existing", |sandbox| {
				::datastore_test::delete_global_metadata_existing(sandbox)
			});
		}

		#[test]
		fn delete_global_metadata_nonexisting() {
			::datastore_test::run(datastore(), "delete_global_metadata_nonexisting", |sandbox| {
				::datastore_test::delete_global_metadata_nonexisting(sandbox)
			});
		}
	)
}

pub fn run<D, T, I, C>(datastore: D, name: &str, test: C) where
	D: Datastore<T, I>,
	T: Transaction,
	I: Id,
	C: FnOnce(&mut DatastoreTestSandbox<D, T, I>) -> ()
{
	let mut sandbox = DatastoreTestSandbox::new(name.to_string(), datastore);
	sandbox.setup();
    test(&mut sandbox);
	sandbox.teardown();
}

fn response_from_transaction<T: Transaction>(trans: &mut T, len: usize) -> Vec<Result<Response, ErrorResponse>> {
	let result = trans.commit();
	assert!(result.is_ok());
	let payload = result.unwrap();
	assert_eq!(payload.len(), len);
	payload
}

fn single_response_from_transaction<T: Transaction>(trans: &mut T) -> Result<Response, ErrorResponse> {
	let payload = response_from_transaction(trans, 1);
	payload.get(0).unwrap().clone()
}

pub fn auth_bad_username<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(I::default(), "foobar".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn auth_bad_password<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id.clone(), "bad_token".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn auth_good<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id.clone(), sandbox.owner_secret.clone());
	assert!(auth.is_ok());
	assert!(auth.unwrap());
}

pub fn has_account_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(sandbox.owner_id.clone());
	assert!(results.is_ok());
	assert!(results.unwrap());
}

pub fn has_account_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(I::default());
	assert!(results.is_ok());
	assert!(!results.unwrap());
}

pub fn delete_account_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let result = sandbox.datastore.delete_account(I::default());
	assert!(result.is_err());
}

pub fn get_vertex_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetVertex(sandbox.jill_id()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Vertex(v)) => {
			assert_eq!(v.id, sandbox.jill_id());
			assert_eq!(v.t, "user".to_string());
			let expected_properties = create_test_properties("Jill");
			assert_eq!(v.properties, expected_properties);
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	};
}

pub fn get_vertex_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let fake_id = sandbox.fake_id();
	let mut trans = sandbox.transaction();
	trans.request(Request::GetVertex(fake_id));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, fake_id),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	};
}

pub fn create_vertex<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let props = create_test_properties("Jill 2.0");

	let mut trans = sandbox.transaction();
	trans.request(Request::CreateVertex("user".to_string(), props));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::VertexId(id)) => assert!(id > 0),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn set_vertex_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	// First create a vertex
	let created_id = sandbox.create_test_vertex("movie", None);

	// Now update the vertex & double-check the results
	let props = create_test_properties("Trainspotting 2");
	let v = models::Vertex::new_with_properties(created_id, "movie".to_string(), props.clone());

	let mut trans = sandbox.transaction();
	trans.request(Request::SetVertex(v.clone()));
	trans.request(Request::GetVertex(created_id));

	let payload = response_from_transaction(&mut trans, 2);
	let set_vertex_item = payload.get(0).unwrap().clone();
	let get_vertex_item = payload.get(1).unwrap().clone();

	match set_vertex_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", set_vertex_item))
	};

	match get_vertex_item {
		Ok(Response::Vertex(set_v)) => {
			assert_eq!(set_v.id, created_id);
			assert_eq!(set_v.properties, props);
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_vertex_item))
	};
}

pub fn set_vertex_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let fake_id = sandbox.fake_id();
	let v = models::Vertex::new(fake_id, "movie".to_string());

	let mut trans = sandbox.transaction();
	trans.request(Request::SetVertex(v.clone()));

	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, fake_id),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_vertex_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	// First create a vertex
	let id = sandbox.create_test_vertex("movie", None);

	// Create some edges
	let mut trans1 = sandbox.transaction();
	trans1.request(Request::SetEdge(models::Edge::new(id, "follows".to_string(), sandbox.jill_id(), 1.0)));
	trans1.request(Request::SetEdge(models::Edge::new(id, "review".to_string(), sandbox.memento_id(), 1.0)));
	trans1.request(Request::SetEdge(models::Edge::new(sandbox.christopher_id(), "follows".to_string(), id, 1.0)));
	let res = trans1.commit();
	assert!(res.is_ok());

	// Delete the edge and make sure everything was cleared
	let mut trans2 = sandbox.transaction();
	trans2.request(Request::DeleteVertex(id));
	trans2.request(Request::GetVertex(id));
	trans2.request(Request::GetEdgeCount(id, "follows".to_string()));
	trans2.request(Request::GetEdgeCount(id, "review".to_string()));
	trans2.request(Request::GetEdgeCount(sandbox.christopher_id(), "follows".to_string()));

	let payload = response_from_transaction(&mut trans2, 5);
	let delete_item = payload.get(0).unwrap().clone();
	let get_vertex_item = payload.get(1).unwrap().clone();
	let get_outbound_follows_item = payload.get(2).unwrap().clone();
	let get_reviews_item = payload.get(3).unwrap().clone();
	let get_inbound_follows_item = payload.get(4).unwrap().clone();

	match delete_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", delete_item))
	};

	match get_vertex_item {
		Err(ErrorResponse::VertexDoesNotExist(_)) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", get_vertex_item))
	};

	let check_zero_count_item = |item| {
		match item {
			Ok(Response::Count(count)) => assert_eq!(count, 0),
			_ => assert!(false, format!("Unexpected response: {:?}", item))
		};
	};

	check_zero_count_item(get_outbound_follows_item);
	check_zero_count_item(get_reviews_item);
	check_zero_count_item(get_inbound_follows_item);
}

pub fn delete_vertex_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let fake_id = sandbox.fake_id();
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteVertex(fake_id));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, fake_id),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_vertex_bad_permissions<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (id, secret) = sandbox.register_account("isolated");
	let mut trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	trans.request(Request::DeleteVertex(sandbox.jill_id()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, sandbox.jill_id()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdge(sandbox.jill_id(), "review".to_string(), sandbox.inception_id()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Edge(e)) => {
			assert_eq!(e.outbound_id, sandbox.jill_id());
			assert_eq!(e.t, "review".to_string());
			assert_eq!(e.inbound_id, sandbox.inception_id());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdge(sandbox.jill_id(), "review".to_string(), sandbox.fake_id()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::EdgeDoesNotExist(_, _, _)) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn set_edge_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	// This also tests adding a new type that didn't previously exist
	let e1 = models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id(), 0.5);
	let e2 = models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id(), -0.5);

	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdge(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id()));
	trans.request(Request::SetEdge(e1.clone()));
	trans.request(Request::GetEdge(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id()));
	trans.request(Request::SetEdge(e2.clone()));
	trans.request(Request::GetEdge(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id()));

	let payload = response_from_transaction(&mut trans, 5);
	let get_edge_nonexisting_item = payload.get(0).unwrap().clone();
	let create_edge_item = payload.get(1).unwrap().clone();
	let get_edge_created_item = payload.get(2).unwrap().clone();
	let set_edge_item = payload.get(3).unwrap().clone();
	let get_edge_existing_item = payload.get(4).unwrap().clone();

	match get_edge_nonexisting_item {
		Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id)) => {
			assert_eq!(outbound_id, sandbox.jill_id());
			assert_eq!(t, "blocks".to_string());
			assert_eq!(inbound_id, sandbox.christopher_id());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_edge_nonexisting_item))
	};

	match create_edge_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", create_edge_item))
	};

	match set_edge_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", create_edge_item))
	};

	let check_set_edge = |item, expected_edge| {
		match item {
			Ok(Response::Edge(set_e)) => assert_eq!(set_e, expected_edge),
			_ => assert!(false, format!("Unexpected response: {:?}", item))
		};
	};

	check_set_edge(get_edge_created_item, e1);
	check_set_edge(get_edge_existing_item, e2);
}

pub fn set_edge_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let fake_id = sandbox.fake_id();
	let mut trans = sandbox.transaction();
	trans.request(Request::SetEdge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), fake_id, 0.5)));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, fake_id),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn set_edge_bad_weight<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::SetEdge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.bob_id(), 1.01)));
	trans.request(Request::SetEdge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.bob_id(), -1.01)));

	let payload = response_from_transaction(&mut trans, 2);

	let check_item = |item| {
		match item {
			Err(ErrorResponse::WeightOutOfRange) => (),
			_ => assert!(false, format!("Unexpected response: {:?}", item))
		}
	};

	check_item(payload.get(0).unwrap().clone());
	check_item(payload.get(1).unwrap().clone());
}

pub fn set_edge_bad_permissions<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (id, secret) = sandbox.register_account("isolated");
	let mut trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	trans.request(Request::SetEdge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id(), 0.5)));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert!(id == sandbox.jill_id() || id == sandbox.christopher_id()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_edge_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let bob_id = sandbox.bob_id();
	let christopher_id = sandbox.christopher_id();

	let mut trans = sandbox.transaction();
	let e = models::Edge::new(bob_id, "blocks".to_string(), christopher_id, 0.5);
	trans.request(Request::SetEdge(e.clone()));
	trans.request(Request::GetEdge(bob_id, "blocks".to_string(), christopher_id));
	trans.request(Request::DeleteEdge(bob_id, "blocks".to_string(), christopher_id));
	trans.request(Request::GetEdge(bob_id, "blocks".to_string(), christopher_id));

	let payload = response_from_transaction(&mut trans, 4);
	let set_edge_item = payload.get(0).unwrap().clone();
	let get_edge_before_item = payload.get(1).unwrap().clone();
	let delete_item = payload.get(2).unwrap().clone();
	let get_edge_after_item = payload.get(3).unwrap().clone();

	match set_edge_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", set_edge_item))
	};

	match get_edge_before_item {
		Ok(Response::Edge(_)) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", get_edge_before_item))
	};

	match delete_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", delete_item))
	};

	match get_edge_after_item {
		Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id)) => {
			assert_eq!(outbound_id, bob_id);
			assert_eq!(t, "blocks".to_string());
			assert_eq!(inbound_id, christopher_id);
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_edge_after_item))
	};
}

pub fn delete_edge_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let fake_id = sandbox.fake_id();
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteEdge(sandbox.jill_id(), "blocks".to_string(), fake_id));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id)) => {
			assert_eq!(outbound_id, sandbox.jill_id());
			assert_eq!(t, "blocks".to_string());
			assert_eq!(inbound_id, fake_id);
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_edge_bad_permissions<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let fake_id = sandbox.fake_id();
	let (id, secret) = sandbox.register_account("isolated");
	let mut trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	trans.request(Request::DeleteEdge(sandbox.jill_id(), "blocks".to_string(), fake_id));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id)) => {
			assert_eq!(outbound_id, sandbox.jill_id());
			assert_eq!(t, "blocks".to_string());
			assert_eq!(inbound_id, fake_id);
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_count_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeCount(sandbox.christopher_id(), "purchased".to_string()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Count(count)) => assert_eq!(count, 10),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_count_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeCount(sandbox.fake_id(), "purchased".to_string()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Count(count)) => assert_eq!(count, 0),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_range_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "purchased".to_string(), 0, 5));
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "purchased".to_string(), 5, 0));
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "purchased".to_string(), 5, 5));

	let payload = response_from_transaction(&mut trans, 3);

	let check_item = |item, count| {
		match item {
			Ok(Response::Edges(edges)) => {
				assert_eq!(edges.len(), count);
				let mut covered_ids: HashSet<i64> = HashSet::new();

				for edge in edges.iter() {
					assert_eq!(edge.outbound_id, sandbox.christopher_id());
					assert_eq!(edge.t, "purchased".to_string());
					assert_eq!(edge.weight, 1.0);
					assert_eq!(edge.properties.len(), 0);
					assert!(!covered_ids.contains(&edge.inbound_id));
					covered_ids.insert(edge.inbound_id);
				}
			},
			_ => assert!(false, format!("Unexpected response: {:?}", item))
		}
	};

	check_item(payload.get(0).unwrap().clone(), 5);
	check_item(payload.get(1).unwrap().clone(), 0);
	check_item(payload.get(2).unwrap().clone(), 5);
}

pub fn get_edge_range_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "foo".to_string(), 0, 10));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Edges(edges)) => assert_eq!(edges.len(), 0),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_time_range_full<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), get_after(), get_before(), 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_empty<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "foo".to_string(), get_after(), get_before(), 10);
	check_edge_time_range(sandbox, request, 0);
}

pub fn get_edge_time_range_no_high<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), Option::None, get_before(), 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_no_low<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), get_after(), Option::None, 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_no_time<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), Option::None, Option::None, 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_reversed_time<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), get_after(), get_after(), 10);
	check_edge_time_range(sandbox, request, 0);
}

pub fn check_edge_time_range<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>, request: Request, expected_length: usize) {
	let mut trans = sandbox.transaction();
	trans.request(request);
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Edges(edges)) => {
			assert_eq!(edges.len(), expected_length);
			let mut covered_ids: HashSet<i64> = HashSet::new();

			for edge in edges.iter() {
				assert_eq!(edge.outbound_id, sandbox.christopher_id());
				assert_eq!(edge.t, "review".to_string());
				assert_eq!(edge.weight, 1.0);
				assert_eq!(edge.properties.len(), 0);
				assert!(!covered_ids.contains(&edge.inbound_id));
				covered_ids.insert(edge.inbound_id);
			}
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_before() -> Option<NaiveDateTime> {
	let time = UTC::now().sub(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}

pub fn get_after() -> Option<NaiveDateTime> {
	let time = UTC::now().add(Duration::days(1));
	Option::Some(NaiveDateTime::from_timestamp(time.timestamp(), 0))
}

pub fn get_local_metadata_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn get_local_metadata_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn set_local_metadata_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn set_local_metadata_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn delete_local_metadata_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn delete_local_metadata_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn get_global_metadata_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn get_global_metadata_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn set_global_metadata_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn set_global_metadata_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn delete_global_metadata_existing<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}

pub fn delete_global_metadata_nonexisting<D: Datastore<T, I>, T: Transaction, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	assert!(false, "Not implemented");
}
