pub use test::Bencher;

/// Defines a benchmark function.
#[macro_export]
macro_rules! define_bench {
    ($name:ident, $datastore_constructor:expr) => {
        #[bench]
        fn $name(b: &mut $crate::benches::Bencher) {
            let mut datastore = $datastore_constructor;
            $crate::benches::$name(b, &mut datastore).unwrap();
        }
    };
}

/// Use this macro to enable the standard benchmarking suite for datastores.
#[macro_export]
macro_rules! full_bench_impl {
    ($code:expr) => {
        define_bench!(bench_create_vertex, $code);
        define_bench!(bench_get_vertices, $code);
        define_bench!(bench_create_edge, $code);
        define_bench!(bench_get_edges, $code);
        define_bench!(bench_get_edge_count, $code);
        define_bench!(bench_bulk_insert, $code);
    };
}
