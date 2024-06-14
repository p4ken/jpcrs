.PHONY: all
all: test package doc

.PHONY: test
test:
	cargo test --all-features
	cargo test --no-default-features
	cargo test --release

.PHONY: package
package:
	cargo package --allow-dirty

.PHONY: doc
doc:
	cargo +nightly doc --no-deps
