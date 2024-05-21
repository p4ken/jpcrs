.PHONY: all
all:
	cargo test
	cargo test --all-features
	cargo test --no-default-features
