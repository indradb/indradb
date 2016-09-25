use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use super::test_util::*;
use requests::Request;
use responses::{Response, ErrorResponse};
use models;
use traits::Id;
use std::collections::HashSet;
use serde_json::Value as JsonValue;

pub fn auth_bad_username<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(I::default(), "foobar".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn auth_bad_password<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id, "bad_token".to_string());
	assert!(auth.is_ok());
	assert!(!auth.unwrap());
}

pub fn auth_good<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let auth = sandbox.datastore.auth(sandbox.owner_id, sandbox.owner_secret.clone());
	assert!(auth.is_ok());
	assert!(auth.unwrap());
}

pub fn has_account_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(sandbox.owner_id);
	assert!(results.is_ok());
	assert!(results.unwrap());
}

pub fn has_account_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let results = sandbox.datastore.has_account(I::default());
	assert!(results.is_ok());
	assert!(!results.unwrap());
}

pub fn delete_account_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let result = sandbox.datastore.delete_account(I::default());
	assert!(result.is_err());
}

pub fn get_vertex_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn get_vertex_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetVertex(I::default()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, I::default()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	};
}

pub fn create_vertex<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let props = create_test_properties("Jill 2.0");

	let mut trans = sandbox.transaction();
	trans.request(Request::CreateVertex("user".to_string(), props));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::VertexId(_)) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn set_vertex_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn set_vertex_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let v = models::Vertex::new(I::default(), "movie".to_string());

	let mut trans = sandbox.transaction();
	trans.request(Request::SetVertex(v.clone()));

	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, I::default()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_vertex_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn delete_vertex_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteVertex(I::default()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, I::default()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_vertex_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (id, _) = sandbox.register_account("isolated");
	let mut trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	trans.request(Request::DeleteVertex(sandbox.jill_id()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, sandbox.jill_id()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn get_edge_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdge(sandbox.jill_id(), "review".to_string(), I::default()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::EdgeDoesNotExist(_, _, _)) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn set_edge_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn set_edge_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::SetEdge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), I::default(), 0.5)));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert_eq!(id, I::default()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn set_edge_bad_weight<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn set_edge_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (id, _) = sandbox.register_account("isolated");
	let mut trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	trans.request(Request::SetEdge(models::Edge::new(sandbox.jill_id(), "blocks".to_string(), sandbox.christopher_id(), 0.5)));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::VertexDoesNotExist(id)) => assert!(id == sandbox.jill_id() || id == sandbox.christopher_id()),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_edge_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
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

pub fn delete_edge_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteEdge(sandbox.jill_id(), "blocks".to_string(), I::default()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id)) => {
			assert_eq!(outbound_id, sandbox.jill_id());
			assert_eq!(t, "blocks".to_string());
			assert_eq!(inbound_id, I::default());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn delete_edge_bad_permissions<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let (id, _) = sandbox.register_account("isolated");
	let mut trans = sandbox.datastore.transaction(id).expect("Expected to be able to create a transaction");
	trans.request(Request::DeleteEdge(sandbox.jill_id(), "blocks".to_string(), I::default()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::EdgeDoesNotExist(outbound_id, t, inbound_id)) => {
			assert_eq!(outbound_id, sandbox.jill_id());
			assert_eq!(t, "blocks".to_string());
			assert_eq!(inbound_id, I::default());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_count_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeCount(sandbox.christopher_id(), "purchased".to_string()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Count(count)) => assert_eq!(count, 10),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_count_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeCount(I::default(), "purchased".to_string()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Count(count)) => assert_eq!(count, 0),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_range_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "purchased".to_string(), 0, 5));
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "purchased".to_string(), 5, 0));
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "purchased".to_string(), 5, 5));

	let payload = response_from_transaction(&mut trans, 3);

	let check_item = |item, count| {
		match item {
			Ok(Response::Edges(edges)) => {
				assert_eq!(edges.len(), count);
				let mut covered_ids: HashSet<I> = HashSet::new();

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

pub fn get_edge_range_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetEdgeRange(sandbox.christopher_id(), "foo".to_string(), 0, 10));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Edges(edges)) => assert_eq!(edges.len(), 0),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn get_edge_time_range_full<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), get_after(), get_before(), 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_empty<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "foo".to_string(), get_after(), get_before(), 10);
	check_edge_time_range(sandbox, request, 0);
}

pub fn get_edge_time_range_no_high<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), Option::None, get_before(), 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_no_low<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), get_after(), Option::None, 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_no_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), Option::None, Option::None, 10);
	check_edge_time_range(sandbox, request, 6);
}

pub fn get_edge_time_range_reversed_time<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let request = Request::GetEdgeTimeRange(sandbox.christopher_id(), "review".to_string(), get_after(), get_after(), 10);
	check_edge_time_range(sandbox, request, 0);
}

fn check_edge_time_range<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>, request: Request<I>, expected_length: usize) {
	let mut trans = sandbox.transaction();
	trans.request(request);
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Edges(edges)) => {
			assert_eq!(edges.len(), expected_length);
			let mut covered_ids: HashSet<I> = HashSet::new();

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

pub fn local_get_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local")));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Metadata(JsonValue::Bool(is_director))) => assert!(is_director),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn local_get_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetMetadata(Some(sandbox.owner_id), "".to_string()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::MetadataDoesNotExist(Some(owner_id), key)) => {
			assert_eq!(owner_id, sandbox.owner_id);
			assert_eq!("".to_string(), key);
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn local_set_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::SetMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local"), JsonValue::String("test".to_string())));
	trans.request(Request::GetMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local")));

	let payload = response_from_transaction(&mut trans, 2);
	let set_item = payload.get(0).unwrap().clone();
	let get_item = payload.get(1).unwrap().clone();

	match set_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", set_item))
	};

	match get_item {
		Ok(Response::Metadata(JsonValue::String(val))) => {
			assert_eq!(val, "test".to_string());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_item))
	};
}

pub fn local_set_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::SetMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local-2"), JsonValue::String("test".to_string())));
	trans.request(Request::GetMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local-2")));

	let payload = response_from_transaction(&mut trans, 2);
	let set_item = payload.get(0).unwrap().clone();
	let get_item = payload.get(1).unwrap().clone();

	match set_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", set_item))
	};

	match get_item {
		Ok(Response::Metadata(JsonValue::String(val))) => {
			assert_eq!(val, "test".to_string());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_item))
	};
}

pub fn local_delete_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local")));
	trans.request(Request::GetMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local")));

	let payload = response_from_transaction(&mut trans, 2);
	let delete_item = payload.get(0).unwrap().clone();
	let get_item = payload.get(1).unwrap().clone();

	match delete_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", delete_item))
	};

	match get_item {
		Err(ErrorResponse::MetadataDoesNotExist(Some(owner_id), key)) => {
			assert_eq!(owner_id, sandbox.owner_id);
			assert_eq!(key, sandbox.generate_unique_string("local"));
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_item))
	};
}

pub fn local_delete_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteMetadata(Some(sandbox.owner_id), sandbox.generate_unique_string("local-2")));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::MetadataDoesNotExist(Some(owner_id), key)) => {
			assert_eq!(owner_id, sandbox.owner_id);
			assert_eq!(key, sandbox.generate_unique_string("local-2"));
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn global_get_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetMetadata(None, sandbox.generate_unique_string("global")));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Ok(Response::Metadata(JsonValue::Bool(value))) => assert!(value),
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn global_get_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::GetMetadata(None, "".to_string()));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::MetadataDoesNotExist(None, key)) => {
			assert_eq!("".to_string(), key);
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}

pub fn global_set_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::SetMetadata(None, sandbox.generate_unique_string("global"), JsonValue::String("test".to_string())));
	trans.request(Request::GetMetadata(None, sandbox.generate_unique_string("global")));

	let payload = response_from_transaction(&mut trans, 2);
	let set_item = payload.get(0).unwrap().clone();
	let get_item = payload.get(1).unwrap().clone();

	match set_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", set_item))
	};

	match get_item {
		Ok(Response::Metadata(JsonValue::String(val))) => {
			assert_eq!(val, "test".to_string());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_item))
	};
}

pub fn global_set_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::SetMetadata(None, sandbox.generate_unique_string("global-2"), JsonValue::String("test".to_string())));
	trans.request(Request::GetMetadata(None, sandbox.generate_unique_string("global-2")));

	let payload = response_from_transaction(&mut trans, 2);
	let set_item = payload.get(0).unwrap().clone();
	let get_item = payload.get(1).unwrap().clone();

	match set_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", set_item))
	};

	match get_item {
		Ok(Response::Metadata(JsonValue::String(val))) => {
			assert_eq!(val, "test".to_string());
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_item))
	};
}

pub fn global_delete_metadata_existing<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteMetadata(None, sandbox.generate_unique_string("global")));
	trans.request(Request::GetMetadata(None, sandbox.generate_unique_string("global")));

	let payload = response_from_transaction(&mut trans, 2);
	let delete_item = payload.get(0).unwrap().clone();
	let get_item = payload.get(1).unwrap().clone();

	match delete_item {
		Ok(Response::Ok) => (),
		_ => assert!(false, format!("Unexpected response: {:?}", delete_item))
	};

	match get_item {
		Err(ErrorResponse::MetadataDoesNotExist(None, key)) => {
			assert_eq!(key, sandbox.generate_unique_string("global"));
		},
		_ => assert!(false, format!("Unexpected response: {:?}", get_item))
	};
}

pub fn global_delete_metadata_nonexisting<D: Datastore<T, I>, T: Transaction<I>, I: Id>(sandbox: &mut DatastoreTestSandbox<D, T, I>) {
	let mut trans = sandbox.transaction();
	trans.request(Request::DeleteMetadata(None, sandbox.generate_unique_string("global-2")));
	let item = single_response_from_transaction(&mut trans);

	match item {
		Err(ErrorResponse::MetadataDoesNotExist(None, key)) => {
			assert_eq!(key, sandbox.generate_unique_string("global-2"));
		},
		_ => assert!(false, format!("Unexpected response: {:?}", item))
	}
}
