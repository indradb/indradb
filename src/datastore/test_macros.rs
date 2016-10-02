#[macro_export]
macro_rules! test_datastore_impl {
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
				sandbox.setup("auth_bad_username");
				::datastore::auth_bad_username(&mut sandbox)
			}

			it "should fail auth with a bad password" {
				sandbox.setup("auth_bad_password");
				::datastore::auth_bad_password(&mut sandbox)
			}

			it "should successfully auth with good credentials" {
				sandbox.setup("auth_good");
				::datastore::auth_good(&mut sandbox)
			}

			it "should find valid accounts in `has_accounts`" {
				sandbox.setup("has_account_existing");
				::datastore::has_account_existing(&mut sandbox)
			}

			it "should fail to find invalid accounts in `has_accounts`" {
				sandbox.setup("has_account_nonexisting");
				::datastore::has_account_nonexisting(&mut sandbox)
			}

			it "should fail when attempting to delete invalid accounts" {
				sandbox.setup("delete_account_nonexisting");
				::datastore::delete_account_nonexisting(&mut sandbox)
			}

			it "should allow you to get a valid vertex" {
				sandbox.setup("get_vertex_existing");
				::datastore::get_vertex_existing(&mut sandbox)
			}

			it "should not allow you to get an invalid vertex" {
				sandbox.setup("get_vertex_nonexisting");
				::datastore::get_vertex_nonexisting(&mut sandbox)
			}

			it "should allow you to create a vertex" {
				sandbox.setup("create_vertex");
				::datastore::create_vertex(&mut sandbox)
			}

			it "should allow you to update a valid vertex" {
				sandbox.setup("set_vertex_existing");
				::datastore::set_vertex_existing(&mut sandbox)
			}

			it "should not allow you to update an invalid vertex" {
				sandbox.setup("set_vertex_nonexisting");
				::datastore::set_vertex_nonexisting(&mut sandbox)
			}

			it "should allow you to delete a valid vertex" {
				sandbox.setup("delete_vertex_existing");
				::datastore::delete_vertex_existing(&mut sandbox)
			}

			it "should not allow you to delete an invalid vertex" {
				sandbox.setup("delete_vertex_nonexisting");
				::datastore::delete_vertex_nonexisting(&mut sandbox)
			}

			it "should not allow you to delete a vertex with that you do not own" {
				sandbox.setup("delete_vertex_bad_permissions");
				::datastore::delete_vertex_bad_permissions(&mut sandbox)
			}

			it "should allow you to get a valid edge" {
				sandbox.setup("get_edge_existing");
				::datastore::get_edge_existing(&mut sandbox)
			}

			it "should not allow you to get an invalid edge" {
				sandbox.setup("get_edge_nonexisting");
				::datastore::get_edge_nonexisting(&mut sandbox)
			}

			it "should allow you to to update an existing edge" {
				sandbox.setup("set_edge_existing");
				::datastore::set_edge_existing(&mut sandbox)
			}

			it "should not allow you to update an existing edge" {
				sandbox.setup("set_edge_nonexisting");
				::datastore::set_edge_nonexisting(&mut sandbox)
			}

			it "should not allow you to set an edge with a bad weight" {
				sandbox.setup("set_edge_bad_weight");
				::datastore::set_edge_bad_weight(&mut sandbox)
			}

			it "show not allow you to set an edge with bad permissions" {
				sandbox.setup("set_edge_bad_permissions");
				::datastore::set_edge_bad_permissions(&mut sandbox)
			}

			it "should allow you to delete an existing edge" {
				sandbox.setup("delete_edge_existing");
				::datastore::delete_edge_existing(&mut sandbox)
			}

			it "should not allow you delete a non-existing edge" {
				sandbox.setup("delete_edge_nonexisting");
				::datastore::delete_edge_nonexisting(&mut sandbox)
			}

			it "should not allow you to delete an edge with bad permissions" {
				sandbox.setup("delete_edge_bad_permissions");
				::datastore::delete_edge_bad_permissions(&mut sandbox)
			}

			it "should allow you to get an edge count" {
				sandbox.setup("get_edge_count_existing");
				::datastore::get_edge_count_existing(&mut sandbox)
			}

			it "should return zero when getting the edge count for a non-existing edge" {
				sandbox.setup("get_edge_count_nonexisting");
				::datastore::get_edge_count_nonexisting(&mut sandbox)
			}

			it "should allow you to a valid edge range" {
				sandbox.setup("get_edge_range_existing");
				::datastore::get_edge_range_existing(&mut sandbox)
			}

			it "should return no edges when getting the edge range for a non-existing edge" {
				sandbox.setup("get_edge_range_nonexisting");
				::datastore::get_edge_range_nonexisting(&mut sandbox)
			}

			it "should get edges by a time range" {
				sandbox.setup("get_edge_time_range_full");
				::datastore::get_edge_time_range_full(&mut sandbox)
			}

			it "should get zero edges for a non-existing time range" {
				sandbox.setup("get_edge_time_range_empty");
				::datastore::get_edge_time_range_empty(&mut sandbox)
			}

			it "should get edges by a time range with no high" {
				sandbox.setup("get_edge_time_range_no_high");
				::datastore::get_edge_time_range_no_high(&mut sandbox)
			}

			it "should get edges by a time range with no low" {
				sandbox.setup("get_edge_time_range_no_low");
				::datastore::get_edge_time_range_no_low(&mut sandbox)
			}

			it "should get edges by a time range with no high or low" {
				sandbox.setup("get_edge_time_range_no_time");
				::datastore::get_edge_time_range_no_time(&mut sandbox)
			}

			it "should not get edges by a reversed time range" {
				sandbox.setup("get_edge_time_range_reversed_time");
				::datastore::get_edge_time_range_reversed_time(&mut sandbox)
			}

			it "should get valid local metadata" {
				sandbox.setup("local_get_metadata_existing");
				::datastore::local_get_metadata_existing(&mut sandbox)
			}

			it "should not get invalid local metadata" {
				sandbox.setup("local_get_metadata_nonexisting");
				::datastore::local_get_metadata_nonexisting(&mut sandbox)
			}

			it "should set valid local metadata" {
				sandbox.setup("local_set_metadata_existing");
				::datastore::local_set_metadata_existing(&mut sandbox)
			}

			it "should not set invalid local metadata" {
				sandbox.setup("local_set_metadata_nonexisting");
				::datastore::local_set_metadata_nonexisting(&mut sandbox)
			}

			it "should delete valid local metadata" {
				sandbox.setup("local_delete_metadata_existing");
				::datastore::local_delete_metadata_existing(&mut sandbox)
			}

			it "should not delete invalid local metadata" {
				sandbox.setup("local_delete_metadata_nonexisting");
				::datastore::local_delete_metadata_nonexisting(&mut sandbox)
			}

			it "should get valid global metadata" {
				sandbox.setup("global_get_metadata_existing");
				::datastore::global_get_metadata_existing(&mut sandbox)
			}

			it "should not get invalid global metadata" {
				sandbox.setup("global_get_metadata_nonexisting");
				::datastore::global_get_metadata_nonexisting(&mut sandbox)
			}

			it "should set valid global metadata" {
				sandbox.setup("global_set_metadata_existing");
				::datastore::global_set_metadata_existing(&mut sandbox)
			}

			it "should not set invalid global metadata" {
				sandbox.setup("global_set_metadata_nonexisting");
				::datastore::global_set_metadata_nonexisting(&mut sandbox)
			}

			it "should delete valid global metadata" {
				sandbox.setup("global_delete_metadata_existing");
				::datastore::global_delete_metadata_existing(&mut sandbox)
			}

			it "should not delete invalid global metadata" {
				sandbox.setup("global_delete_metadata_nonexisting");
				::datastore::global_delete_metadata_nonexisting(&mut sandbox)
			}
		}
	)
}
