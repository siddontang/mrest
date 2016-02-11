.PHONY: all

all: format build test

build:
	cargo build

run:
	cargo run

test:
	RUST_LOG=mrest=DEBUG RUST_BACKTRACE=1 cargo test -- --nocapture

format:
	rustfmt --write-mode overwrite src/lib.rs tests/lib.rs
