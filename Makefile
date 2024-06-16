.PHONY: all test test-d test-d-min test-r package doc

all: test package doc

test: test-d test-d-min test-r

test-d:
	cargo test

test-d-min:
	cargo test --no-default-features --all-targets  # without doc

test-r:
	cargo test --release

package:
	cargo package --allow-dirty

doc:
	cargo +nightly doc --no-deps
