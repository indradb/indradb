export RUST_BACKTRACE=1

.PHONY: test bench fuzz check fmt

test:
	cd lib && cargo test --features=test-suite,rocksdb-datastore $(TEST_NAME)
	cd proto && cargo test --features=test-suite $(TEST_NAME)

bench:
	cd lib && cargo +nightly bench --features=bench-suite,rocksdb-datastore,sled-datastore $(TEST_NAME)
	cd proto && cargo +nightly bench --features=bench-suite $(TEST_NAME)

fuzz:
	cd lib && cargo +nightly fuzz run compare

check:
	cargo +stable check
	cargo +nightly check --all-features
	cd lib/fuzz && cargo +stable check
	cargo +stable clippy
	cargo fmt -- --check

fmt:
	cargo fmt
