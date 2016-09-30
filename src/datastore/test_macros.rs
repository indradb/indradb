use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use traits::Id;

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
				::datastore::auth_bad_username(sandbox)
			}

			it "should fail auth with a bad password" {
				sandbox.setup("auth_bad_password");
				::datastore::auth_bad_password(sandbox)
			}

			it "should successfully auth with good credentials" {
				sandbox.setup("auth_good");
				::datastore::auth_good(sandbox)
			}

			it "should find valid accounts in `has_accounts`" {
				sandbox.setup("has_account_existing");
				::datastore::has_account_existing(sandbox)
			}

			it "should fail to find invalid accounts in `has_accounts`" {
				sandbox.setup("has_account_nonexisting");
				::datastore::has_account_nonexisting(sandbox)
			}

			it "should fail when attempting to delete invalid accounts" {
				sandbox.setup("delete_account_nonexisting");
				::datastore::delete_account_nonexisting(sandbox)
			}

			it "should allow you to get a valid vertex" {
				sandbox.setup("get_vertex_existing");
				::datastore::get_vertex_existing(sandbox)
			}

			it "should not allow you to get an invalid vertex" {
				sandbox.setup("get_vertex_nonexisting");
				::datastore::get_vertex_nonexisting(sandbox)
			}

			it "should allow you to create a vertex" {
				sandbox.setup("create_vertex");
				::datastore::create_vertex(sandbox)
			}

			it "should allow you to update a valid vertex" {
				sandbox.setup("set_vertex_existing");
				::datastore::set_vertex_existing(sandbox)
			}

			it "should not allow you to update an invalid vertex" {
				sandbox.setup("set_vertex_nonexisting");
				::datastore::set_vertex_nonexisting(sandbox)
			}

			it "should allow you to delete a valid vertex" {
				sandbox.setup("delete_vertex_existing");
				::datastore::delete_vertex_existing(sandbox)
			}

			it "should not allow you to delete an invalid vertex" {
				sandbox.setup("delete_vertex_nonexisting");
				::datastore::delete_vertex_nonexisting(sandbox)
			}

			it "should not allow you to delete a vertex with that you do not own" {
				sandbox.setup("delete_vertex_bad_permissions");
				::datastore::delete_vertex_bad_permissions(sandbox)
			}

			it "should allow you to get a valid edge" {
				sandbox.setup("get_edge_existing");
				::datastore::get_edge_existing(sandbox)
			}

			it "should not allow you to get an invalid edge" {
				sandbox.setup("get_edge_nonexisting");
				::datastore::get_edge_nonexisting(sandbox)
			}
		}
	)
}


// #[test]
// fn set_edge_existing() {
// 	::datastore::run(datastore(), "set_edge_existing", |sandbox| {
// 		::datastore::set_edge_existing(sandbox)
// 	});
// }
//
// #[test]
// fn set_edge_nonexisting() {
// 	::datastore::run(datastore(), "set_edge_nonexisting", |sandbox| {
// 		::datastore::set_edge_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn set_edge_bad_weight() {
// 	::datastore::run(datastore(), "set_edge_bad_weight", |sandbox| {
// 		::datastore::set_edge_bad_weight(sandbox)
// 	});
// }
//
// #[test]
// fn set_edge_bad_permissions() {
// 	::datastore::run(datastore(), "set_edge_bad_permissions", |sandbox| {
// 		::datastore::set_edge_bad_permissions(sandbox)
// 	});
// }
//
// #[test]
// fn delete_edge_existing() {
// 	::datastore::run(datastore(), "delete_edge_existing", |sandbox| {
// 		::datastore::delete_edge_existing(sandbox)
// 	});
// }
//
// #[test]
// fn delete_edge_nonexisting() {
// 	::datastore::run(datastore(), "delete_edge_nonexisting", |sandbox| {
// 		::datastore::delete_edge_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn delete_edge_bad_permissions() {
// 	::datastore::run(datastore(), "delete_edge_bad_permissions", |sandbox| {
// 		::datastore::delete_edge_bad_permissions(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_count_existing() {
// 	::datastore::run(datastore(), "get_edge_count_existing", |sandbox| {
// 		::datastore::get_edge_count_existing(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_count_nonexisting() {
// 	::datastore::run(datastore(), "get_edge_count_nonexisting", |sandbox| {
// 		::datastore::get_edge_count_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_range_existing() {
// 	::datastore::run(datastore(), "get_edge_range_existing", |sandbox| {
// 		::datastore::get_edge_range_existing(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_range_nonexisting() {
// 	::datastore::run(datastore(), "get_edge_range_nonexisting", |sandbox| {
// 		::datastore::get_edge_range_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_time_range_full() {
// 	::datastore::run(datastore(), "get_edge_time_range_full", |sandbox| {
// 		::datastore::get_edge_time_range_full(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_time_range_empty() {
// 	::datastore::run(datastore(), "get_edge_time_range_empty", |sandbox| {
// 		::datastore::get_edge_time_range_empty(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_time_range_no_high() {
// 	::datastore::run(datastore(), "get_edge_time_range_no_high", |sandbox| {
// 		::datastore::get_edge_time_range_no_high(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_time_range_no_low() {
// 	::datastore::run(datastore(), "get_edge_time_range_no_low", |sandbox| {
// 		::datastore::get_edge_time_range_no_low(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_time_range_no_time() {
// 	::datastore::run(datastore(), "get_edge_time_range_no_time", |sandbox| {
// 		::datastore::get_edge_time_range_no_time(sandbox)
// 	});
// }
//
// #[test]
// fn get_edge_time_range_reversed_time() {
// 	::datastore::run(datastore(), "get_edge_time_range_reversed_time", |sandbox| {
// 		::datastore::get_edge_time_range_reversed_time(sandbox)
// 	});
// }
//
// #[test]
// fn local_get_metadata_existing() {
// 	::datastore::run(datastore(), "local_get_metadata_existing", |sandbox| {
// 		::datastore::local_get_metadata_existing(sandbox)
// 	});
// }
//
// #[test]
// fn local_get_metadata_nonexisting() {
// 	::datastore::run(datastore(), "local_get_metadata_nonexisting", |sandbox| {
// 		::datastore::local_get_metadata_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn local_set_metadata_existing() {
// 	::datastore::run(datastore(), "local_set_metadata_existing", |sandbox| {
// 		::datastore::local_set_metadata_existing(sandbox)
// 	});
// }
//
// #[test]
// fn local_set_metadata_nonexisting() {
// 	::datastore::run(datastore(), "local_set_metadata_nonexisting", |sandbox| {
// 		::datastore::local_set_metadata_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn local_delete_metadata_existing() {
// 	::datastore::run(datastore(), "local_delete_metadata_existing", |sandbox| {
// 		::datastore::local_delete_metadata_existing(sandbox)
// 	});
// }
//
// #[test]
// fn local_delete_metadata_nonexisting() {
// 	::datastore::run(datastore(), "local_delete_metadata_nonexisting", |sandbox| {
// 		::datastore::local_delete_metadata_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn global_get_metadata_existing() {
// 	::datastore::run(datastore(), "global_get_metadata_existing", |sandbox| {
// 		::datastore::global_get_metadata_existing(sandbox)
// 	});
// }
//
// #[test]
// fn global_get_metadata_nonexisting() {
// 	::datastore::run(datastore(), "global_get_metadata_nonexisting", |sandbox| {
// 		::datastore::global_get_metadata_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn global_set_metadata_existing() {
// 	::datastore::run(datastore(), "global_set_metadata_existing", |sandbox| {
// 		::datastore::global_set_metadata_existing(sandbox)
// 	});
// }
//
// #[test]
// fn global_set_metadata_nonexisting() {
// 	::datastore::run(datastore(), "global_set_metadata_nonexisting", |sandbox| {
// 		::datastore::global_set_metadata_nonexisting(sandbox)
// 	});
// }
//
// #[test]
// fn global_delete_metadata_existing() {
// 	::datastore::run(datastore(), "global_delete_metadata_existing", |sandbox| {
// 		::datastore::global_delete_metadata_existing(sandbox)
// 	});
// }
//
// #[test]
// fn global_delete_metadata_nonexisting() {
// 	::datastore::run(datastore(), "global_delete_metadata_nonexisting", |sandbox| {
// 		::datastore::global_delete_metadata_nonexisting(sandbox)
// 	});
// }
