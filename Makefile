.PHONY: dep test all

dep:
	cargo install cargo-strip
	cargo install diesel_cli --no-default-features --features postgres

all:
	cargo build

test:
	cargo test
