export RUST_BACKTRACE=1

.PHONY: test bench fuzz clippy fmt

test:
	cd lib && cargo test --features=test-suite,rocksdb-datastore $(TEST_NAME)
	cd bin && cargo test --features=test-suite $(TEST_NAME)

bench:
	cd lib && cargo +nightly bench --features=bench-suite,rocksdb-datastore $(TEST_NAME)
	cd bin && cargo +nightly bench --features=bench-suite $(TEST_NAME)

fuzz:
	cd lib && cargo +nightly fuzz run compare

clippy:
	cargo clippy

fmt:
	cargo fmt
