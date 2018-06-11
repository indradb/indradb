/// Defines a unit test function.
#[macro_export]
macro_rules! indradb_test {
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
macro_rules! indradb_full_test_impl {
	($code:expr) => (
		// Vertex queries
		indradb_test!(should_create_vertex_from_type, $code);
		indradb_test!(should_get_all_vertices, $code);
		indradb_test!(should_get_all_vertices_with_zero_limit, $code);
		indradb_test!(should_get_all_vertices_out_of_range, $code);
		indradb_test!(should_get_single_vertices, $code);
		indradb_test!(should_get_single_vertices_nonexisting, $code);
		indradb_test!(should_get_vertices, $code);
		indradb_test!(should_get_vertices_piped, $code);
		indradb_test!(should_get_a_vertex_count, $code);

		// Vertex updates
		indradb_test!(should_delete_a_valid_vertex, $code);
		indradb_test!(should_not_delete_an_invalid_vertex, $code);

		// Edges
		indradb_test!(should_get_a_valid_edge, $code);
		indradb_test!(should_not_get_an_invalid_edge, $code);
		indradb_test!(should_create_a_valid_edge, $code);
		indradb_test!(should_not_create_an_invalid_edge, $code);
		indradb_test!(should_delete_a_valid_edge, $code);
		indradb_test!(should_not_delete_an_invalid_edge, $code);
		indradb_test!(should_get_an_edge_count, $code);
		indradb_test!(should_get_an_edge_count_with_no_type, $code);
		indradb_test!(should_get_an_edge_count_for_an_invalid_edge, $code);
		indradb_test!(should_get_an_inbound_edge_count, $code);
		indradb_test!(should_get_an_edge_range, $code);
		indradb_test!(should_get_edges_with_no_type, $code);
		indradb_test!(should_get_no_edges_for_an_invalid_range, $code);
		indradb_test!(should_get_edges_with_no_high, $code);
		indradb_test!(should_get_edges_with_no_low, $code);
		indradb_test!(should_get_edges_with_no_time, $code);
		indradb_test!(should_get_no_edges_for_reversed_time, $code);
		indradb_test!(should_get_edges, $code);

		// Metadata
		indradb_test!(should_handle_vertex_metadata, $code);
		indradb_test!(should_not_set_invalid_vertex_metadata, $code);
		indradb_test!(should_not_delete_invalid_vertex_metadata, $code);
		indradb_test!(should_handle_edge_metadata, $code);
		indradb_test!(should_not_set_invalid_edge_metadata, $code);
		indradb_test!(should_not_delete_invalid_edge_metadata, $code);
	)
}
