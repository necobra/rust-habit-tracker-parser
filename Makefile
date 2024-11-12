build:
    cargo build

run:
    cargo run

test:
    cargo test

clean:
    cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

all: clean fmt clippy test build