/// Instead of copy-pasta'ing all of the test functions for each frontend
/// implementation, frontends can just use this macro to enable the standard
/// test suite. This has the added benefit of preventing digressions between
/// datastores.
///
/// This is very similar to the macro used for the standard datastore test
/// suite, but removes the tests for features that aren't exposed by frontends.
#[macro_export]
macro_rules! test_frontend_impl {
	($code:expr) => (
		// Vertex updates
		define_test!(should_update_a_valid_vertex, ::update_a_valid_vertex, $code);
		define_test!(should_not_update_an_invalid_vertex, ::not_update_an_invalid_vertex, $code);
		define_test!(should_delete_a_valid_vertex, ::delete_a_valid_vertex, $code);
		define_test!(should_not_delete_an_invalid_vertex, ::not_delete_an_invalid_vertex, $code);
		define_test!(should_not_delete_an_unowned_vertex, ::not_delete_an_unowned_vertex, $code);

		// Edges
		define_test!(should_get_a_valid_edge, ::get_a_valid_edge, $code);
		define_test!(should_not_get_an_invalid_edge, ::not_get_an_invalid_edge, $code);
		define_test!(should_update_a_valid_edge, ::update_a_valid_edge, $code);
		define_test!(should_not_update_an_invalid_edge, ::not_update_an_invalid_edge, $code);
		define_test!(should_not_set_an_edge_with_bad_permissions, ::not_set_an_edge_with_bad_permissions, $code);
		define_test!(should_delete_a_valid_edge, ::delete_a_valid_edge, $code);
		define_test!(should_not_delete_an_invalid_edge, ::not_delete_an_invalid_edge, $code);
		define_test!(should_not_delete_an_edge_with_bad_permissions, ::not_delete_an_edge_with_bad_permissions, $code);

		// Edge ranges
		define_test!(should_get_an_edge_count, ::get_an_edge_count, $code);
		define_test!(should_get_an_edge_count_with_no_type, ::get_an_edge_count_with_no_type, $code);
		define_test!(should_get_an_edge_count_for_an_invalid_edge, ::get_an_edge_count_for_an_invalid_edge, $code);
		define_test!(should_get_an_edge_range, ::get_an_edge_range, $code);
		define_test!(should_get_edge_range_with_no_type, ::get_edge_range_with_no_type, $code);
		define_test!(should_get_no_edges_for_an_invalid_range, ::get_no_edges_for_an_invalid_range, $code);
		define_test!(should_get_edge_range_with_no_high, ::get_edge_range_with_no_high, $code);
		define_test!(should_get_edge_range_with_no_low, ::get_edge_range_with_no_low, $code);
		define_test!(should_get_edge_range_with_no_time, ::get_edge_range_with_no_time, $code);
		define_test!(should_get_no_edges_for_reversed_time, ::get_no_edges_for_reversed_time, $code);

		// Reversed edge ranges
		define_test!(should_get_a_reversed_edge_count, ::get_a_reversed_edge_count, $code);
		define_test!(should_get_a_reversed_edge_count_with_no_type, ::get_a_reversed_edge_count_with_no_type, $code);
		define_test!(should_get_a_reversed_edge_count_for_an_invalid_edge, ::get_a_reversed_edge_count_for_an_invalid_edge, $code);
		define_test!(should_get_a_reversed_edge_range, ::get_a_reversed_edge_range, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_type, ::get_a_reversed_edge_range_with_no_type, $code);
		define_test!(should_get_no_reversed_edges_for_an_invalid_range, ::get_no_reversed_edges_for_an_invalid_range, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_high, ::get_a_reversed_edge_range_with_no_high, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_low, ::get_a_reversed_edge_range_with_no_low, $code);
		define_test!(should_get_a_reversed_edge_range_with_no_time, ::get_a_reversed_edge_range_with_no_time, $code);
		define_test!(should_get_no_reversed_edges_for_reversed_time, ::get_no_reversed_edges_for_reversed_time, $code);
	)
}
