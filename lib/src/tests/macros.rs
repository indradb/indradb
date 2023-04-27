/// Helper macro for expecting an error result.
#[macro_export]
macro_rules! expect_err {
    ($result:expr, $err:pat) => {
        match $result {
            Err($err) => (),
            _ => assert!(false, "unexpected result: {:?}", $result),
        }
    };
}

/// Defines a unit test function.
#[macro_export]
macro_rules! define_test {
    ($name:ident, $db_constructor:expr) => {
        #[test]
        fn $name() {
            let db = $db_constructor;
            $crate::tests::$name(&db).unwrap();
        }
    };
}

/// Use this macro to enable the entire standard test suite.
#[macro_export]
macro_rules! full_test_impl {
    ($code:expr) => {
        // Sync
        define_test!(should_sync, $code);

        // Bulk insert
        define_test!(should_bulk_insert, $code);
        define_test!(should_bulk_insert_a_redundant_vertex, $code);
        define_test!(should_bulk_insert_an_invalid_edge, $code);

        // Vertices
        define_test!(should_create_vertex_from_type, $code);
        define_test!(should_get_all_vertices, $code);
        define_test!(should_get_range_vertices, $code);
        define_test!(should_get_no_vertices_with_zero_limit, $code);
        define_test!(should_get_range_vertices_out_of_range, $code);
        define_test!(should_get_no_vertices_with_type_filter, $code);
        define_test!(should_get_single_vertex, $code);
        define_test!(should_get_single_vertex_nonexisting, $code);
        define_test!(should_get_vertices, $code);
        define_test!(should_get_vertices_piped, $code);
        define_test!(should_get_a_vertex_count, $code);
        define_test!(should_delete_a_valid_outbound_vertex, $code);
        define_test!(should_delete_a_valid_inbound_vertex, $code);
        define_test!(should_not_delete_an_invalid_vertex, $code);
        define_test!(should_not_delete_on_vertex_count, $code);
        define_test!(should_not_pipe_on_vertex_count, $code);

        // Edges
        define_test!(should_get_all_edges, $code);
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
        define_test!(should_get_edges_with_no_type, $code);
        define_test!(should_get_edge_range, $code);
        define_test!(should_get_edges, $code);
        define_test!(should_get_edges_piped, $code);
        define_test!(should_delete_indexed_edge_with_property_value, $code);

        // Include queries
        define_test!(should_get_nested_include_query, $code);
        define_test!(should_get_unnested_include_query, $code);
        define_test!(should_include_with_property_presence, $code);

        // Indexing
        define_test!(should_not_query_unindexed_vertex_property, $code);
        define_test!(should_not_query_unindexed_edge_property, $code);
        define_test!(should_index_existing_vertex_property, $code);
        define_test!(should_index_existing_edge_property, $code);
        define_test!(should_delete_indexed_vertex_property, $code);
        define_test!(should_delete_indexed_edge_property, $code);
        define_test!(should_update_indexed_vertex_property, $code);
        define_test!(should_update_indexed_edge_property, $code);
        define_test!(should_query_indexed_vertex_property_empty, $code);
        define_test!(should_query_indexed_edge_property_empty, $code);
        define_test!(should_get_vertex_with_property_value_empty, $code);
        define_test!(should_pipe_not_indexed_errors, $code);

        // Properties
        define_test!(should_handle_vertex_properties, $code);
        define_test!(should_not_set_invalid_vertex_properties, $code);
        define_test!(should_not_delete_invalid_vertex_properties, $code);
        define_test!(should_get_all_vertex_properties, $code);
        define_test!(should_handle_edge_properties, $code);
        define_test!(should_not_set_invalid_edge_properties, $code);
        define_test!(should_not_delete_invalid_edge_properties, $code);
        define_test!(should_get_all_edge_properties, $code);
        define_test!(should_get_an_edge_properties_count, $code);
        define_test!(should_get_a_vertex_properties_count, $code);
        define_test!(should_not_set_properties_on_count, $code);
        define_test!(should_not_pipe_properties_on_vertex_count, $code);
        define_test!(should_not_pipe_property_presence_on_vertex_count, $code);
    };
}
