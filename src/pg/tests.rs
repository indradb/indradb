#![cfg(test)]

use super::datastore::PostgresDatastore;
use datastore_test;
use std::env;

fn datastore() -> PostgresDatastore {
	let connection_string = env::var("DATABASE_URL").expect("Expected a DATABASE_URL");
	let secret = "OME88YorohonzPNWEFsi0dIsouXWqeO$".to_string();
	PostgresDatastore::new(Some(1), connection_string, secret)
}

#[test]
fn auth_bad_username() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::auth_bad_username(sandbox)
	});
}

#[test]
fn auth_bad_password() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::auth_bad_password(sandbox)
	});
}

#[test]
fn auth_good() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::auth_good(sandbox)
	});
}

#[test]
fn create_account() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::create_account(sandbox)
	});
}

#[test]
fn get_account_id_nonexistent() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_account_id_nonexistent(sandbox)
	});
}

#[test]
fn get_account_id_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_account_id_existing(sandbox)
	});
}

#[test]
fn delete_account_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_account_existing(sandbox)
	});
}

#[test]
fn delete_account_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_account_nonexisting(sandbox)
	});
}

#[test]
fn get_vertex_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_vertex_existing(sandbox)
	});
}

#[test]
fn get_vertex_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_vertex_nonexisting(sandbox);
	});
}

#[test]
fn create_vertex() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::create_vertex(sandbox);
	});
}

#[test]
fn set_vertex_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::set_vertex_existing(sandbox);
	});
}

#[test]
fn set_vertex_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::set_vertex_nonexisting(sandbox);
	});
}

#[test]
fn delete_vertex_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_vertex_existing(sandbox);
	});
}

#[test]
fn delete_vertex_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_vertex_nonexisting(sandbox);
	});
}

#[test]
fn delete_vertex_bad_permissions() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_vertex_bad_permissions(sandbox);
	});
}

#[test]
fn get_edge_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_existing(sandbox);
	});
}

#[test]
fn get_edge_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_nonexisting(sandbox);
	});
}

#[test]
fn set_edge_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::set_edge_existing(sandbox);
	});
}

#[test]
fn set_edge_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::set_edge_nonexisting(sandbox);
	});
}

#[test]
fn set_edge_bad_weight() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::set_edge_bad_weight(sandbox);
	});
}

#[test]
fn set_edge_bad_permissions() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::set_edge_bad_permissions(sandbox);
	});
}

#[test]
fn delete_edge_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_edge_existing(sandbox);
	});
}

#[test]
fn delete_edge_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_edge_nonexisting(sandbox);
	});
}

#[test]
fn delete_edge_bad_permissions() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::delete_edge_bad_permissions(sandbox);
	});
}

#[test]
fn get_edge_count_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_count_existing(sandbox);
	});
}

#[test]
fn get_edge_count_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_count_nonexisting(sandbox);
	});
}

#[test]
fn get_edge_range_existing() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_range_existing(sandbox);
	});
}

#[test]
fn get_edge_range_nonexisting() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_range_nonexisting(sandbox);
	});
}

#[test]
fn get_edge_time_range_full() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_time_range_full(sandbox);
	});
}

#[test]
fn get_edge_time_range_empty() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_time_range_empty(sandbox);
	});
}

#[test]
fn get_edge_time_range_no_high() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_time_range_no_high(sandbox);
	});
}

#[test]
fn get_edge_time_range_no_low() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_time_range_no_low(sandbox);
	});
}

#[test]
fn get_edge_time_range_no_time() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_time_range_no_time(sandbox);
	});
}

#[test]
fn get_edge_time_range_reversed_time() {
	datastore_test::run(datastore(), |sandbox| {
		datastore_test::get_edge_time_range_reversed_time(sandbox);
	});
}
