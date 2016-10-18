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
				auth_bad_username(&mut sandbox)
			}

			it "should fail auth with a bad password" {
				sandbox.setup("should fail auth with a bad password");
				auth_bad_password(&mut sandbox)
			}

			it "should successfully auth with good credentials" {
				sandbox.setup("should successfully auth with good credentials");
				auth_good(&mut sandbox)
			}

			it "should lookup valid accounts" {
				sandbox.setup("should lookup valid accounts");
				has_account_existing(&mut sandbox)
			}

			it "should fail to lookup invalid accounts" {
				sandbox.setup("should fail to lookup invalid accounts");
				has_account_nonexisting(&mut sandbox)
			}

			it "should fail when attempting to delete invalid accounts" {
				sandbox.setup("should fail when attempting to delete invalid accounts");
				delete_account_nonexisting(&mut sandbox)
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
				get_vertex_existing(&mut sandbox)
			}

			it "should not get an invalid vertex" {
				sandbox.setup("should not get an invalid vertex");
				get_vertex_nonexisting(&mut sandbox)
			}

			it "should create a vertex" {
				sandbox.setup("should create a vertex");
				create_vertex(&mut sandbox)
			}

			it "should update a valid vertex" {
				sandbox.setup("should update a valid vertex");
				set_vertex_existing(&mut sandbox)
			}

			it "should not update an invalid vertex" {
				sandbox.setup("should not update an invalid vertex");
				set_vertex_nonexisting(&mut sandbox)
			}

			it "should delete a valid vertex" {
				sandbox.setup("should delete a valid vertex");
				delete_vertex_existing(&mut sandbox)
			}

			it "should not delete an invalid vertex" {
				sandbox.setup("should not delete an invalid vertex");
				delete_vertex_nonexisting(&mut sandbox)
			}

			it "should not delete an unowned vertex" {
				sandbox.setup("should not delete an unowned vertex");
				delete_vertex_bad_permissions(&mut sandbox)
			}

			it "should get a valid edge" {
				sandbox.setup("should get a valid edge");
				get_edge_existing(&mut sandbox)
			}

			it "should not get an invalid edge" {
				sandbox.setup("should not get an invalid edge");
				get_edge_nonexisting(&mut sandbox)
			}

			it "should update a valid edge" {
				sandbox.setup("should update a valid edge");
				set_edge_existing(&mut sandbox)
			}

			it "should not update an invalid edge" {
				sandbox.setup("should not update an invalid edge");
				set_edge_nonexisting(&mut sandbox)
			}

			it "should not set an edge with a bad weight" {
				sandbox.setup("should not set an edge with a bad weight");
				set_edge_bad_weight(&mut sandbox)
			}

			it "should not set an edge with bad permissions" {
				sandbox.setup("should not set an edge with bad permissions");
				set_edge_bad_permissions(&mut sandbox)
			}

			it "should delete a valid edge" {
				sandbox.setup("should delete a valid edge");
				delete_edge_existing(&mut sandbox)
			}

			it "should not delete an invalid edge" {
				sandbox.setup("should not delete an invalid edge");
				delete_edge_nonexisting(&mut sandbox)
			}

			it "should not delete an edge with bad permissions" {
				sandbox.setup("should not delete an edge with bad permissions");
				delete_edge_bad_permissions(&mut sandbox)
			}

			it "should get an edge count" {
				sandbox.setup("should get an edge count");
				get_edge_count_existing(&mut sandbox)
			}

			it "should get an edge count for an invalid edge" {
				sandbox.setup("should get an edge count for an invalid edge");
				get_edge_count_nonexisting(&mut sandbox)
			}

			it "should get an edge range" {
				sandbox.setup("should get an edge range");
				get_edge_range_existing(&mut sandbox)
			}

			it "should get an empty edge range for an invalid edge" {
				sandbox.setup("should get an edge range for an invalid edge");
				get_edge_range_nonexisting(&mut sandbox)
			}

			it "should not get an edge range with an invalid offset" {
				sandbox.setup("should not get an edge range with an invalid offset");
				get_edge_range_bad_offset(&mut sandbox)
			}

			it "should not get an edge range with an invalid limit" {
				sandbox.setup("should not get an edge range with an invalid limit");
				get_edge_range_bad_limit(&mut sandbox)
			}

			it "should get edges by a time range" {
				sandbox.setup("should get edges by a time range");
				get_edge_time_range_full(&mut sandbox)
			}

			it "should get no edges for an invalid time range" {
				sandbox.setup("should get no edges for an invalid time range");
				get_edge_time_range_empty(&mut sandbox)
			}

			it "should get edges by a time range with no high" {
				sandbox.setup("should get edges by a time range with no high");
				get_edge_time_range_no_high(&mut sandbox)
			}

			it "should get edges by a time range with no low" {
				sandbox.setup("should get edges by a time range with no low");
				get_edge_time_range_no_low(&mut sandbox)
			}

			it "should get edges by a time range with no high or low" {
				sandbox.setup("should get edges by a time range with no high or low");
				get_edge_time_range_no_time(&mut sandbox)
			}

			it "should get no edges for a reversed time range" {
				sandbox.setup("should get no edges for a reversed time range");
				get_edge_time_range_reversed_time(&mut sandbox)
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

			it "should handle local metadata" {
				sandbox.setup("should handle local metadata");
				local_metadata(&mut sandbox)
			}

			it "should handle global metadata" {
				sandbox.setup("should handle global metadata");
				global_metadata(&mut sandbox)
			}
		}
	)
}
