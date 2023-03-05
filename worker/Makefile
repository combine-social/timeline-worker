.PHONY: dep test all

dep:
	cargo install sqlx-cli cargo-strip

all: dep	
	cargo build

test: dep
	cargo test
