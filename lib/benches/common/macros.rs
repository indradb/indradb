/// Defines a benchmark function.
#[macro_export]
macro_rules! define_bench {
    ($name:ident, $datastore_constructor:expr) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let datastore = $datastore_constructor;
            ::common::$name(b, datastore);
        }
    )
}

/// Use this macro to enable the standard benchmarking suite for transactions.
#[macro_export]
macro_rules! bench_transaction_impl {
    ($code:expr) => (
        define_bench!(bench_create_vertex, $code);
        define_bench!(bench_get_vertices, $code);
        define_bench!(bench_create_edge, $code);
        define_bench!(bench_get_edges, $code);
        define_bench!(bench_get_edge_count, $code);
    )
}
