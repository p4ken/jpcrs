
all: test package doc version

test: test-min test-default

test-min: FORCE
	cargo test --release --no-default-features --all-targets  # without doc

test-default: FORCE
	cargo test --release

package: FORCE
	cargo package --allow-dirty

doc: FORCE
	cargo +nightly doc --no-deps

version: FORCE
	@grep '^version =' Cargo.toml | cut -d '"' -f 2

FORCE:
