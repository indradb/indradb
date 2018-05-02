/// Defines a unit test function.
#[macro_export]
macro_rules! define_test {
	($name:ident, $datastore_constructor:expr) => (
		#[test]
		fn $name() {
			let mut datastore = $datastore_constructor;
			$crate::tests::$name(&mut datastore);
		}
	)
}

/// Use this macro to enable the entire standard test suite.
#[macro_export]
macro_rules! full_test_impl {
	($code:expr) => (
		// Vertex queries
		define_test!(should_create_vertex_from_type, $code);
		define_test!(should_get_all_vertices, $code);
		define_test!(should_get_all_vertices_with_zero_limit, $code);
		define_test!(should_get_all_vertices_out_of_range, $code);
		define_test!(should_get_single_vertices, $code);
		define_test!(should_get_single_vertices_nonexisting, $code);
		define_test!(should_get_vertices, $code);
		define_test!(should_get_vertices_piped, $code);
		define_test!(should_get_a_vertex_count, $code);

		// Vertex updates
		define_test!(should_delete_a_valid_vertex, $code);
		define_test!(should_not_delete_an_invalid_vertex, $code);

		// Edges
		define_test!(should_get_a_valid_edge, $code);
		define_test!(should_not_get_an_invalid_edge, $code);
		define_test!(should_create_a_valid_edge, $code);
		define_test!(should_not_create_an_invalid_edge, $code);
		define_test!(should_delete_a_valid_edge, $code);
		define_test!(should_not_delete_an_invalid_edge, $code);
		define_test!(should_get_an_edge_count, $code);
		define_test!(should_get_an_edge_count_with_no_type, $code);
		define_test!(should_get_an_edge_count_for_an_invalid_edge, $code);
		define_test!(should_get_an_inbound_edge_count, $code);
		define_test!(should_get_an_edge_range, $code);
		define_test!(should_get_edges_with_no_type, $code);
		define_test!(should_get_no_edges_for_an_invalid_range, $code);
		define_test!(should_get_edges_with_no_high, $code);
		define_test!(should_get_edges_with_no_low, $code);
		define_test!(should_get_edges_with_no_time, $code);
		define_test!(should_get_no_edges_for_reversed_time, $code);
		define_test!(should_get_edges, $code);

		// Metadata
		define_test!(should_handle_vertex_metadata, $code);
		define_test!(should_not_set_invalid_vertex_metadata, $code);
		define_test!(should_not_delete_invalid_vertex_metadata, $code);
		define_test!(should_handle_edge_metadata, $code);
		define_test!(should_not_set_invalid_edge_metadata, $code);
		define_test!(should_not_delete_invalid_edge_metadata, $code);
	)
}
