export RUST_BACKTRACE=1

.PHONY: test bench

test:
	cd lib && cargo test --features=test-suite,rocksdb-datastore $(TEST_NAME)
	ulimit -n 1024 && cd bin && cargo test --features=test-suite $(TEST_NAME)

bench:
	cd lib && cargo +nightly bench --features=bench-suite,rocksdb-datastore $(TEST_NAME)
	ulimit -n 1024 && cd bin && cargo +nightly bench --features=bench-suite $(TEST_NAME)
