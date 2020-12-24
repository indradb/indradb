export RUST_BACKTRACE=1

.PHONY: test bench fuzz fmt check

test:
	cd lib && cargo test --features=test-suite,rocksdb-datastore,sled-datastore $(TEST_NAME)
	cd bin && cargo test --features=test-suite $(TEST_NAME)

bench:
	cd lib && cargo +nightly bench --features=bench-suite,rocksdb-datastore,sled-datastore $(TEST_NAME)
	cd bin && cargo +nightly bench --features=bench-suite $(TEST_NAME)

fuzz:
	cd lib && cargo +nightly fuzz run compare

check:
	cd lib && cargo check --all-features
	cd lib/fuzz && cargo check
	cd bin && cargo check --all-features
	cargo clippy

fmt:
	cargo fmt
