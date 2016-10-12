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
				::datastore::auth_bad_username(&mut sandbox)
			}

			it "should fail auth with a bad password" {
				sandbox.setup("should fail auth with a bad password");
				::datastore::auth_bad_password(&mut sandbox)
			}

			it "should successfully auth with good credentials" {
				sandbox.setup("should successfully auth with good credentials");
				::datastore::auth_good(&mut sandbox)
			}

			it "should lookup valid accounts" {
				sandbox.setup("should lookup valid accounts");
				::datastore::has_account_existing(&mut sandbox)
			}

			it "should fail to lookup invalid accounts" {
				sandbox.setup("should fail to lookup invalid accounts");
				::datastore::has_account_nonexisting(&mut sandbox)
			}

			it "should fail when attempting to delete invalid accounts" {
				sandbox.setup("should fail when attempting to delete invalid accounts");
				::datastore::delete_account_nonexisting(&mut sandbox)
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
				::datastore::get_vertex_existing(&mut sandbox)
			}

			it "should not get an invalid vertex" {
				sandbox.setup("should not get an invalid vertex");
				::datastore::get_vertex_nonexisting(&mut sandbox)
			}

			it "should create a vertex" {
				sandbox.setup("should create a vertex");
				::datastore::create_vertex(&mut sandbox)
			}

			it "should update a valid vertex" {
				sandbox.setup("should update a valid vertex");
				::datastore::set_vertex_existing(&mut sandbox)
			}

			it "should not update an invalid vertex" {
				sandbox.setup("should not update an invalid vertex");
				::datastore::set_vertex_nonexisting(&mut sandbox)
			}

			it "should delete a valid vertex" {
				sandbox.setup("should delete a valid vertex");
				::datastore::delete_vertex_existing(&mut sandbox)
			}

			it "should not delete an invalid vertex" {
				sandbox.setup("should not delete an invalid vertex");
				::datastore::delete_vertex_nonexisting(&mut sandbox)
			}

			it "should not delete an unowned vertex" {
				sandbox.setup("should not delete an unowned vertex");
				::datastore::delete_vertex_bad_permissions(&mut sandbox)
			}

			it "should get a valid edge" {
				sandbox.setup("should get a valid edge");
				::datastore::get_edge_existing(&mut sandbox)
			}

			it "should not get an invalid edge" {
				sandbox.setup("should not get an invalid edge");
				::datastore::get_edge_nonexisting(&mut sandbox)
			}

			it "should update a valid edge" {
				sandbox.setup("should update a valid edge");
				::datastore::set_edge_existing(&mut sandbox)
			}

			it "should not update an invalid edge" {
				sandbox.setup("should not update an invalid edge");
				::datastore::set_edge_nonexisting(&mut sandbox)
			}

			it "should not set an edge with a bad weight" {
				sandbox.setup("should not set an edge with a bad weight");
				::datastore::set_edge_bad_weight(&mut sandbox)
			}

			it "should not set an edge with bad permissions" {
				sandbox.setup("should not set an edge with bad permissions");
				::datastore::set_edge_bad_permissions(&mut sandbox)
			}

			it "should delete a valid edge" {
				sandbox.setup("should delete a valid edge");
				::datastore::delete_edge_existing(&mut sandbox)
			}

			it "should not delete an invalid edge" {
				sandbox.setup("should not delete an invalid edge");
				::datastore::delete_edge_nonexisting(&mut sandbox)
			}

			it "should not delete an edge with bad permissions" {
				sandbox.setup("should not delete an edge with bad permissions");
				::datastore::delete_edge_bad_permissions(&mut sandbox)
			}

			it "should get an edge count" {
				sandbox.setup("should get an edge count");
				::datastore::get_edge_count_existing(&mut sandbox)
			}

			it "should get an edge count for an invalid edge" {
				sandbox.setup("should get an edge count for an invalid edge");
				::datastore::get_edge_count_nonexisting(&mut sandbox)
			}

			it "should get an edge range" {
				sandbox.setup("should get an edge range");
				::datastore::get_edge_range_existing(&mut sandbox)
			}

			it "should get an empty edge range for an invalid edge" {
				sandbox.setup("should get an edge range for an invalid edge");
				::datastore::get_edge_range_nonexisting(&mut sandbox)
			}

			it "should not get an edge range with an invalid offset" {
				sandbox.setup("should not get an edge range with an invalid offset");
				::datastore::get_edge_range_bad_offset(&mut sandbox)
			}

			it "should not get an edge range with an invalid limit" {
				sandbox.setup("should not get an edge range with an invalid limit");
				::datastore::get_edge_range_bad_limit(&mut sandbox)
			}

			it "should get edges by a time range" {
				sandbox.setup("should get edges by a time range");
				::datastore::get_edge_time_range_full(&mut sandbox)
			}

			it "should get no edges for an invalid time range" {
				sandbox.setup("should get no edges for an invalid time range");
				::datastore::get_edge_time_range_empty(&mut sandbox)
			}

			it "should get edges by a time range with no high" {
				sandbox.setup("should get edges by a time range with no high");
				::datastore::get_edge_time_range_no_high(&mut sandbox)
			}

			it "should get edges by a time range with no low" {
				sandbox.setup("should get edges by a time range with no low");
				::datastore::get_edge_time_range_no_low(&mut sandbox)
			}

			it "should get edges by a time range with no high or low" {
				sandbox.setup("should get edges by a time range with no high or low");
				::datastore::get_edge_time_range_no_time(&mut sandbox)
			}

			it "should get no edges for a reversed time range" {
				sandbox.setup("should get no edges for a reversed time range");
				::datastore::get_edge_time_range_reversed_time(&mut sandbox)
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

			it "should get valid local metadata" {
				sandbox.setup("should get valid local metadata");
				::datastore::local_get_metadata_existing(&mut sandbox)
			}

			it "should not get invalid local metadata" {
				sandbox.setup("should not get invalid local metadata");
				::datastore::local_get_metadata_nonexisting(&mut sandbox)
			}

			it "should set valid local metadata" {
				sandbox.setup("should set valid local metadata");
				::datastore::local_set_metadata_existing(&mut sandbox)
			}

			it "should not set invalid local metadata" {
				sandbox.setup("should not set invalid local metadata");
				::datastore::local_set_metadata_nonexisting(&mut sandbox)
			}

			it "should delete valid local metadata" {
				sandbox.setup("should delete valid local metadata");
				::datastore::local_delete_metadata_existing(&mut sandbox)
			}

			it "should not delete invalid local metadata" {
				sandbox.setup("should not delete invalid local metadata");
				::datastore::local_delete_metadata_nonexisting(&mut sandbox)
			}

			it "should get valid global metadata" {
				sandbox.setup("should get valid global metadata");
				::datastore::global_get_metadata_existing(&mut sandbox)
			}

			it "should not get invalid global metadata" {
				sandbox.setup("should not get invalid global metadata");
				::datastore::global_get_metadata_nonexisting(&mut sandbox)
			}

			it "should set valid global metadata" {
				sandbox.setup("should set valid global metadata");
				::datastore::global_set_metadata_existing(&mut sandbox)
			}

			it "should not set invalid global metadata" {
				sandbox.setup("should not set invalid global metadata");
				::datastore::global_set_metadata_nonexisting(&mut sandbox)
			}

			it "should delete valid global metadata" {
				sandbox.setup("should delete valid global metadata");
				::datastore::global_delete_metadata_existing(&mut sandbox)
			}

			it "should not delete invalid global metadata" {
				sandbox.setup("should not delete invalid global metadata");
				::datastore::global_delete_metadata_nonexisting(&mut sandbox)
			}
		}
	)
}
