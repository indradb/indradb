pub use test::Bencher;

/// Defines a benchmark function.
#[macro_export]
macro_rules! indradb_define_bench {
    ($name:ident, $datastore_constructor:expr) => (
        #[bench]
        fn $name(b: &mut $crate::benches::Bencher) {
            let mut datastore = $datastore_constructor;
            $crate::benches::$name(b, &mut datastore);
        }
    )
}

/// Use this macro to enable the standard benchmarking suite for transactions.
#[macro_export]
macro_rules! indradb_full_bench_impl {
    ($code:expr) => (
        indradb_bench!(bench_create_vertex, $code);
        indradb_bench!(bench_get_vertices, $code);
        indradb_bench!(bench_create_edge, $code);
        indradb_bench!(bench_get_edges, $code);
        indradb_bench!(bench_get_edge_count, $code);
    )
}
