/// Isntead of copy-pasta'ing all of the test functions for each frontend
/// implementation, frontends can just use this macro to enable the standard
/// test suite. This has the added benefit of preventing digressions between
/// datastores.
///
/// This is very similar to the macro used for the standard datastore test
/// suite, but only runs the subset of functions that are exposed in
/// frontends.
#[macro_export]
macro_rules! test_frontend_impl {
	($code:block) => (
		// Various transaction tests
		define_test!(should_get_a_vertex_range, $code);
		define_test!(should_get_a_valid_vertex, $code);
		define_test!(should_not_get_an_invalid_vertex, $code);
		define_test!(should_update_a_valid_vertex, $code);
		define_test!(should_not_update_an_invalid_vertex, $code);
		define_test!(should_delete_a_valid_vertex, $code);
		define_test!(should_not_delete_an_invalid_vertex, $code);
		define_test!(should_not_delete_an_unowned_vertex, $code);

		define_test!(should_get_a_valid_edge, $code);
		define_test!(should_not_get_an_invalid_edge, $code);
		define_test!(should_update_a_valid_edge, $code);
		define_test!(should_not_update_an_invalid_edge, $code);
		define_test!(should_not_set_an_edge_with_bad_permissions, $code);
		define_test!(should_delete_a_valid_edge, $code);
		define_test!(should_not_delete_an_invalid_edge, $code);
		define_test!(should_not_delete_an_edge_with_bad_permissions, $code);

		define_test!(should_get_an_edge_count, $code);
		define_test!(should_get_an_edge_count_with_no_type, $code);
		define_test!(should_get_an_edge_count_for_an_invalid_edge, $code);
		define_test!(should_get_an_edge_range, $code);
		define_test!(should_get_edge_range_with_no_type, $code);
		define_test!(should_get_no_edges_for_an_invalid_range, $code);
		define_test!(should_get_edge_range_with_no_high, $code);
		define_test!(should_get_edge_range_with_no_low, $code);
		define_test!(should_get_edge_range_with_no_time, $code);
		define_test!(should_get_no_edges_for_reversed_time, $code);

		define_test!(should_get_a_reversed_edge_count, $code);
		define_test!(should_get_a_reversed_edge_count_with_no_type, $code);
		define_test!(should_get_a_reversed_edge_count_for_an_invalid_edge, $code);
		define_test!(should_get_a_reversed_edge_range, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_type, $code);
		define_test!(should_get_no_reversed_edges_for_an_invalid_range, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_high, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_low, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_time, $code);
		define_test!(should_get_no_reversed_edges_for_reversed_time, $code);
	)
}
