export RUST_BACKTRACE=1
export INDRADB_SCRIPT_ROOT=test_scripts

.PHONY: test bench

test:
	cd lib && cargo test --features=test-suite,rocksdb-datastore $(TEST_NAME)
	cd bin && cargo test --features=test-suite $(TEST_NAME)

bench:
	cd lib && cargo +nightly bench --features=bench-suite,rocksdb-datastore $(TEST_NAME)
	cd bin && cargo +nightly bench --features=bench-suite $(TEST_NAME)
