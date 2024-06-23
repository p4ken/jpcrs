
all: test package doc version

test: test-all test-default test-tky2jgd test-patchjgd

test-all: FORCE
	cargo test --release --all-features

test-%: FORCE
	cargo test --release --no-default-features --features $* --all-targets  # without doc

package: FORCE
	cargo package --allow-dirty

doc: FORCE
	cargo +nightly doc --no-deps

version: FORCE
	@grep '^version =' Cargo.toml | cut -d '"' -f 2

FORCE:
