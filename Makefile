SWITCH_URL ?= http://192.168.42.2

.PHONY: \
	all \
	rust \
	run-rust \


all: rust

rust:
	cargo build --release --manifest-path rust/Cargo.toml

run-rust: rust
	SWITCH_URL=$(SWITCH_URL) ./rust/target/release/procurve-cli show description
