use super::{Datastore, Transaction};
use super::test_sandbox::DatastoreTestSandbox;
use traits::Id;

pub fn run<D, T, I, C>(datastore: D, name: &str, test: C) where
	D: Datastore<T, I>,
	T: Transaction<I>,
	I: Id,
	C: FnOnce(&mut DatastoreTestSandbox<D, T, I>) -> ()
{
	let mut sandbox = DatastoreTestSandbox::new(name.to_string(), datastore);
	sandbox.setup();
    test(&mut sandbox);
	sandbox.teardown();
}

#[macro_export]
macro_rules! test_datastore_impl {
	(fn datastore() -> $typ:ty $code:block) => (
		fn datastore() -> $typ $code

		#[test]
		fn auth_bad_username() {
			::datastore::run(datastore(), "auth_bad_username", |sandbox| {
				::datastore::auth_bad_username(sandbox)
			});
		}

		#[test]
		fn auth_bad_password() {
			::datastore::run(datastore(), "auth_bad_password", |sandbox| {
				::datastore::auth_bad_password(sandbox)
			});
		}

		#[test]
		fn auth_good() {
			::datastore::run(datastore(), "auth_good", |sandbox| {
				::datastore::auth_good(sandbox)
			});
		}

		#[test]
		fn has_account_existing() {
			::datastore::run(datastore(), "has_account_existing", |sandbox| {
				::datastore::has_account_existing(sandbox)
			});
		}

		#[test]
		fn has_account_nonexisting() {
			::datastore::run(datastore(), "has_account_nonexisting", |sandbox| {
				::datastore::has_account_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_account_nonexisting() {
			::datastore::run(datastore(), "delete_account_nonexisting", |sandbox| {
				::datastore::delete_account_nonexisting(sandbox)
			});
		}

		#[test]
		fn get_vertex_existing() {
			::datastore::run(datastore(), "get_vertex_existing", |sandbox| {
				::datastore::get_vertex_existing(sandbox)
			});
		}

		#[test]
		fn get_vertex_nonexisting() {
			::datastore::run(datastore(), "get_vertex_nonexisting", |sandbox| {
				::datastore::get_vertex_nonexisting(sandbox)
			});
		}

		#[test]
		fn create_vertex() {
			::datastore::run(datastore(), "create_vertex", |sandbox| {
				::datastore::create_vertex(sandbox)
			});
		}

		#[test]
		fn set_vertex_existing() {
			::datastore::run(datastore(), "set_vertex_existing", |sandbox| {
				::datastore::set_vertex_existing(sandbox)
			});
		}

		#[test]
		fn set_vertex_nonexisting() {
			::datastore::run(datastore(), "set_vertex_nonexisting", |sandbox| {
				::datastore::set_vertex_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_vertex_existing() {
			::datastore::run(datastore(), "delete_vertex_existing", |sandbox| {
				::datastore::delete_vertex_existing(sandbox)
			});
		}

		#[test]
		fn delete_vertex_nonexisting() {
			::datastore::run(datastore(), "delete_vertex_nonexisting", |sandbox| {
				::datastore::delete_vertex_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_vertex_bad_permissions() {
			::datastore::run(datastore(), "delete_vertex_bad_permissions", |sandbox| {
				::datastore::delete_vertex_bad_permissions(sandbox)
			});
		}

		#[test]
		fn get_edge_existing() {
			::datastore::run(datastore(), "get_edge_existing", |sandbox| {
				::datastore::get_edge_existing(sandbox)
			});
		}

		#[test]
		fn get_edge_nonexisting() {
			::datastore::run(datastore(), "get_edge_nonexisting", |sandbox| {
				::datastore::get_edge_nonexisting(sandbox)
			});
		}

		#[test]
		fn set_edge_existing() {
			::datastore::run(datastore(), "set_edge_existing", |sandbox| {
				::datastore::set_edge_existing(sandbox)
			});
		}

		#[test]
		fn set_edge_nonexisting() {
			::datastore::run(datastore(), "set_edge_nonexisting", |sandbox| {
				::datastore::set_edge_nonexisting(sandbox)
			});
		}

		#[test]
		fn set_edge_bad_weight() {
			::datastore::run(datastore(), "set_edge_bad_weight", |sandbox| {
				::datastore::set_edge_bad_weight(sandbox)
			});
		}

		#[test]
		fn set_edge_bad_permissions() {
			::datastore::run(datastore(), "set_edge_bad_permissions", |sandbox| {
				::datastore::set_edge_bad_permissions(sandbox)
			});
		}

		#[test]
		fn delete_edge_existing() {
			::datastore::run(datastore(), "delete_edge_existing", |sandbox| {
				::datastore::delete_edge_existing(sandbox)
			});
		}

		#[test]
		fn delete_edge_nonexisting() {
			::datastore::run(datastore(), "delete_edge_nonexisting", |sandbox| {
				::datastore::delete_edge_nonexisting(sandbox)
			});
		}

		#[test]
		fn delete_edge_bad_permissions() {
			::datastore::run(datastore(), "delete_edge_bad_permissions", |sandbox| {
				::datastore::delete_edge_bad_permissions(sandbox)
			});
		}

		#[test]
		fn get_edge_count_existing() {
			::datastore::run(datastore(), "get_edge_count_existing", |sandbox| {
				::datastore::get_edge_count_existing(sandbox)
			});
		}

		#[test]
		fn get_edge_count_nonexisting() {
			::datastore::run(datastore(), "get_edge_count_nonexisting", |sandbox| {
				::datastore::get_edge_count_nonexisting(sandbox)
			});
		}

		#[test]
		fn get_edge_range_existing() {
			::datastore::run(datastore(), "get_edge_range_existing", |sandbox| {
				::datastore::get_edge_range_existing(sandbox)
			});
		}

		#[test]
		fn get_edge_range_nonexisting() {
			::datastore::run(datastore(), "get_edge_range_nonexisting", |sandbox| {
				::datastore::get_edge_range_nonexisting(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_full() {
			::datastore::run(datastore(), "get_edge_time_range_full", |sandbox| {
				::datastore::get_edge_time_range_full(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_empty() {
			::datastore::run(datastore(), "get_edge_time_range_empty", |sandbox| {
				::datastore::get_edge_time_range_empty(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_no_high() {
			::datastore::run(datastore(), "get_edge_time_range_no_high", |sandbox| {
				::datastore::get_edge_time_range_no_high(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_no_low() {
			::datastore::run(datastore(), "get_edge_time_range_no_low", |sandbox| {
				::datastore::get_edge_time_range_no_low(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_no_time() {
			::datastore::run(datastore(), "get_edge_time_range_no_time", |sandbox| {
				::datastore::get_edge_time_range_no_time(sandbox)
			});
		}

		#[test]
		fn get_edge_time_range_reversed_time() {
			::datastore::run(datastore(), "get_edge_time_range_reversed_time", |sandbox| {
				::datastore::get_edge_time_range_reversed_time(sandbox)
			});
		}

		#[test]
		fn local_get_metadata_existing() {
			::datastore::run(datastore(), "local_get_metadata_existing", |sandbox| {
				::datastore::local_get_metadata_existing(sandbox)
			});
		}

		#[test]
		fn local_get_metadata_nonexisting() {
			::datastore::run(datastore(), "local_get_metadata_nonexisting", |sandbox| {
				::datastore::local_get_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn local_set_metadata_existing() {
			::datastore::run(datastore(), "local_set_metadata_existing", |sandbox| {
				::datastore::local_set_metadata_existing(sandbox)
			});
		}

		#[test]
		fn local_set_metadata_nonexisting() {
			::datastore::run(datastore(), "local_set_metadata_nonexisting", |sandbox| {
				::datastore::local_set_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn local_delete_metadata_existing() {
			::datastore::run(datastore(), "local_delete_metadata_existing", |sandbox| {
				::datastore::local_delete_metadata_existing(sandbox)
			});
		}

		#[test]
		fn local_delete_metadata_nonexisting() {
			::datastore::run(datastore(), "local_delete_metadata_nonexisting", |sandbox| {
				::datastore::local_delete_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn global_get_metadata_existing() {
			::datastore::run(datastore(), "global_get_metadata_existing", |sandbox| {
				::datastore::global_get_metadata_existing(sandbox)
			});
		}

		#[test]
		fn global_get_metadata_nonexisting() {
			::datastore::run(datastore(), "global_get_metadata_nonexisting", |sandbox| {
				::datastore::global_get_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn global_set_metadata_existing() {
			::datastore::run(datastore(), "global_set_metadata_existing", |sandbox| {
				::datastore::global_set_metadata_existing(sandbox)
			});
		}

		#[test]
		fn global_set_metadata_nonexisting() {
			::datastore::run(datastore(), "global_set_metadata_nonexisting", |sandbox| {
				::datastore::global_set_metadata_nonexisting(sandbox)
			});
		}

		#[test]
		fn global_delete_metadata_existing() {
			::datastore::run(datastore(), "global_delete_metadata_existing", |sandbox| {
				::datastore::global_delete_metadata_existing(sandbox)
			});
		}

		#[test]
		fn global_delete_metadata_nonexisting() {
			::datastore::run(datastore(), "global_delete_metadata_nonexisting", |sandbox| {
				::datastore::global_delete_metadata_nonexisting(sandbox)
			});
		}
	)
}
