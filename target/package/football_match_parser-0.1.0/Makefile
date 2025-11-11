run:
	cargo run -- $(ARGS)

parse:
	cargo run -- parse $(FILE)

credits:
	cargo run -- credits

help:
	cargo run -- --help

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

check: fmt clippy test
