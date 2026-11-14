.DEFAULT_GOAL := help
SHELL := /bin/bash
.SHELLFLAGS := -o pipefail -c

include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help deps update clippy test-references docs-check documentation-links-check ci

help:
	@printf '%s\n' \
	  'make install-tools / tools-check  Prepare / admit the selected development tools' \
	  'make toko-tooling-check           Admit the locked tooling snapshot' \
	  'make fmt / fmt-check              Format / check workspace source' \
	  'make clippy                      Strict native compilation and lint checks' \
	  'make test-references             Bounded object-reference behavior tests' \
	  'make docs-check                  Strict Rustdoc checks' \
	  'make documentation-links-check   Check repository Markdown links' \
	  'make deps                        Fetch the locked dependencies' \
	  'make deps-list                   List workspace package versions and groups' \
	  'make update / deps-update        Update Cargo locks within manifest requirements' \
	  'make toko-tooling-update          Install the Cargo-selected tooling snapshot' \
	  'make ci                          Aggregate tooling, format, lint, test and docs gate' \
	  'toko-cloc                        Categorized source counts (PATH launcher)'

deps:
	cargo fetch --locked

update: deps-update

clippy:
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --workspace --all-targets --all-features -- -D warnings

docs-check:
	RUSTDOCFLAGS='-D warnings' cargo doc --offline --locked --workspace --all-features --no-deps

test-references:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib references::tests

documentation-links-check:
	@find docs -type f -name '*.md' -print0 | xargs -0 perl vendor/toko-tooling/scripts/ci/check-documentation-links.pl --root . *.md

ci: toko-tooling-check fmt-check clippy test-references docs-check documentation-links-check
