SWITCH_URL ?= http://192.168.42.2
RUST_LOG ?= info

.PHONY: \
	all \
	rust \
	run-rust \


all: rust

rust:
	cargo build --release --manifest-path rust/Cargo.toml

test-rust:
	cargo test --manifest-path rust/Cargo.toml

run-rust: rust
	RUST_LOG=$(RUST_LOG) SWITCH_URL=$(SWITCH_URL) \
		 ./rust/target/release/procurve-cli show description
