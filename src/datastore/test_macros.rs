#[macro_export]
macro_rules! test_account_management_impl {
	($name:ident $code:expr) => (
		describe! $name {
			before_each {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
			}

			after_each {
				sandbox.teardown();
			}

			it "should fail auth with a bad username" {
				sandbox.setup("should fail auth with a bad username");
				::should_fail_auth_with_a_bad_username(&mut sandbox)
			}

			it "should fail auth with a bad password" {
				sandbox.setup("should fail auth with a bad password");
				::should_fail_auth_with_a_bad_password(&mut sandbox)
			}

			it "should successfully auth with good credentials" {
				sandbox.setup("should successfully auth with good credentials");
				::should_successfully_auth_with_good_credentials(&mut sandbox)
			}

			it "should lookup valid accounts" {
				sandbox.setup("should lookup valid accounts");
				::should_lookup_valid_accounts(&mut sandbox)
			}

			it "should fail to lookup invalid accounts" {
				sandbox.setup("should fail to lookup invalid accounts");
				::should_fail_to_lookup_invalid_accounts(&mut sandbox)
			}

			it "should fail when attempting to delete invalid accounts" {
				sandbox.setup("should fail when attempting to delete invalid accounts");
				::should_fail_when_attempting_to_delete_invalid_accounts(&mut sandbox)
			}
		}
	)
}

#[macro_export]
macro_rules! test_transaction_impl {
	($name:ident $code:expr) => (
		describe! $name {
			before_each {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
			}

			after_each {
				sandbox.teardown();
			}

			it "should get a valid vertex" {
				sandbox.setup("should get a valid vertex");
				::should_get_a_valid_vertex(&mut sandbox)
			}
			it "should not get an invalid vertex" {
				sandbox.setup("should not get an invalid vertex");
				::should_not_get_an_invalid_vertex(&mut sandbox)
			}
			it "should update a valid vertex" {
				sandbox.setup("should update a valid vertex");
				::should_update_a_valid_vertex(&mut sandbox)
			}
			it "should not update an invalid vertex" {
				sandbox.setup("should not update an invalid vertex");
				::should_not_update_an_invalid_vertex(&mut sandbox)
			}
			it "should delete a valid vertex" {
				sandbox.setup("should delete a valid vertex");
				::should_delete_a_valid_vertex(&mut sandbox)
			}
			it "should not delete an invalid vertex" {
				sandbox.setup("should not delete an invalid vertex");
				::should_not_delete_an_invalid_vertex(&mut sandbox)
			}
			it "should not delete an unowned vertex" {
				sandbox.setup("should not delete an unowned vertex");
				::should_not_delete_an_unowned_vertex(&mut sandbox)
			}
			it "should get a valid edge" {
				sandbox.setup("should get a valid edge");
				::should_get_a_valid_edge(&mut sandbox)
			}
			it "should not get an invalid edge" {
				sandbox.setup("should not get an invalid edge");
				::should_not_get_an_invalid_edge(&mut sandbox)
			}
			it "should update a valid edge" {
				sandbox.setup("should update a valid edge");
				::should_update_a_valid_edge(&mut sandbox)
			}
			it "should not update an invalid edge" {
				sandbox.setup("should not update an invalid edge");
				::should_not_update_an_invalid_edge(&mut sandbox)
			}
			it "should not set an edge with a bad weight" {
				sandbox.setup("should not set an edge with a bad weight");
				::should_not_set_an_edge_with_a_bad_weight(&mut sandbox)
			}
			it "should not set an edge with bad permissions" {
				sandbox.setup("should not set an edge with bad permissions");
				::should_not_set_an_edge_with_bad_permissions(&mut sandbox)
			}
			it "should delete a valid edge" {
				sandbox.setup("should delete a valid edge");
				::should_delete_a_valid_edge(&mut sandbox)
			}
			it "should not delete an invalid edge" {
				sandbox.setup("should not delete an invalid edge");
				::should_not_delete_an_invalid_edge(&mut sandbox)
			}
			it "should not delete an edge with bad permissions" {
				sandbox.setup("should not delete an edge with bad permissions");
				::should_not_delete_an_edge_with_bad_permissions(&mut sandbox)
			}
			it "should get an edge count" {
				sandbox.setup("should get an edge count");
				::should_get_an_edge_count(&mut sandbox)
			}
			it "should get an edge count for an invalid edge" {
				sandbox.setup("should get an edge count for an invalid edge");
				::should_get_an_edge_count_for_an_invalid_edge(&mut sandbox)
			}
			it "should get an empty edge range with zero limit" {
				sandbox.setup("should get an empty edge range with zero limit");
				::should_get_an_empty_edge_range_with_zero_limit(&mut sandbox)
			}
			it "should get an empty edge range" {
				sandbox.setup("should get an empty edge range");
				::should_get_an_empty_edge_range(&mut sandbox)
			}
			it "should get an edge range" {
				sandbox.setup("should get an edge range");
				::should_get_an_edge_range(&mut sandbox)
			}
			it "should get an empty edge range for an invalid edge" {
				sandbox.setup("should get an empty edge range for an invalid edge");
				::should_get_an_empty_edge_range_for_an_invalid_edge(&mut sandbox)
			}
			it "should get edges by a time range" {
				sandbox.setup("should get edges by a time range");
				::should_get_edges_by_a_time_range(&mut sandbox)
			}
			it "should get no edges for an invalid time range" {
				sandbox.setup("should get no edges for an invalid time range");
				::should_get_no_edges_for_an_invalid_time_range(&mut sandbox)
			}
			it "should get edges by a time range with no high" {
				sandbox.setup("should get edges by a time range with no high");
				::should_get_edges_by_a_time_range_with_no_high(&mut sandbox)
			}
			it "should get edges by a time range with no low" {
				sandbox.setup("should get edges by a time range with no low");
				::should_get_edges_by_a_time_range_with_no_low(&mut sandbox)
			}
			it "should get edges by a time range with no time" {
				sandbox.setup("should get edges by a time range with no time");
				::should_get_edges_by_a_time_range_with_no_time(&mut sandbox)
			}
			it "should get no edges for a reversed time range" {
				sandbox.setup("should get no edges for a reversed time range");
				::should_get_no_edges_for_a_reversed_time_range(&mut sandbox)
			}
		}
	)
}

#[macro_export]
macro_rules! bench_transaction_impl {
	($name:ident $code:expr) => (
		describe! $name {
			bench "benchmark get_vertex" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark get_vertex");
				::bench_get_vertex(b, &mut sandbox);
				sandbox.teardown();
			}

			bench "benchmark create_vertex" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark create_vertex");
				::bench_create_vertex(b, &mut sandbox);
				sandbox.teardown();
			}

			bench "benchmark set_vertex" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark set_vertex");
				::bench_set_vertex(b, &mut sandbox);
				sandbox.teardown();
			}

			bench "benchmark get_edge" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark get_edge");
				::bench_get_edge(b, &mut sandbox);
				sandbox.teardown();
			}

			bench "benchmark set_edge" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark set_edge");
				::bench_set_edge(b, &mut sandbox);
				sandbox.teardown();
			}

			bench "benchmark get_edge_count" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark get_edge_count");
				::bench_get_edge_count(b, &mut sandbox);
				sandbox.teardown();
			}

			bench "benchmark get_edge_range" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark get_edge_range");
				::bench_get_edge_range(b, &mut sandbox);
				sandbox.teardown();
			}

			bench "benchmark get_edge_time_range" (b) {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
				sandbox.setup("benchmark get_edge_time_range");
				::bench_get_edge_time_range(b, &mut sandbox);
				sandbox.teardown();
			}
		}
	)
}


#[macro_export]
macro_rules! test_metadata_impl {
	($name:ident $code:expr) => (
		describe! $name {
			before_each {
				let datastore = $code;
				let mut sandbox = DatastoreTestSandbox::new(datastore);
			}

			after_each {
				sandbox.teardown();
			}

			it "should handle global metadata" {
				sandbox.setup("should handle global metadata");
				::should_handle_global_metadata(&mut sandbox)
			}

			it "should handle account metadata" {
				sandbox.setup("should handle account metadata");
				::should_handle_account_metadata(&mut sandbox)
			}

			it "should handle vertex metadata" {
				sandbox.setup("should handle vertex metadata");
				::should_handle_vertex_metadata(&mut sandbox)
			}

			it "should handle edge metadata" {
				sandbox.setup("should handle edge metadata");
				::should_handle_edge_metadata(&mut sandbox)
			}
		}
	)
}
