
all: test package doc version

test: test-d test-d-min test-r

test-d: FORCE
	cargo test

test-d-min: FORCE
	cargo test --no-default-features --all-targets  # without doc

test-r: FORCE
	cargo test --release

package: FORCE
	cargo package --allow-dirty

doc: FORCE
	cargo +nightly doc --no-deps

version: FORCE
	@grep '^version =' Cargo.toml | cut -d '"' -f 2

FORCE:
