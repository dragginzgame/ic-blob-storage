.DEFAULT_GOAL := help
SHELL := /bin/bash
.SHELLFLAGS := -o pipefail -c
LOCAL_TOOL_INSTALL_TARGETS += install-test-server
LOCAL_TOOL_CHECK_TARGETS += test-server-check

include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help deps update clippy test-references test-caffeine test-local example-local test-example-local docs-check documentation-links-check ci install-test-server test-server-check clippy-host build-host test-standalone build-admin test-management

help:
	@printf '%s\n' \
	  'make install-tools / tools-check  Prepare / admit the selected development tools' \
	  'make toko-tooling-check           Admit the locked tooling snapshot' \
	  'make fmt / fmt-check              Format / check workspace source' \
	  'make clippy                      Strict native compilation and lint checks' \
	  'make test-references             Bounded object-reference behavior tests' \
	  'make test-caffeine               Caffeine stream and manifest integrity tests' \
	  'make test-local                  Local grant, reference and accounting tests' \
	  'make install-test-server         Explicit authenticated local simulator setup' \
	  'make test-standalone              Build and test the installed host locally' \
	  'make test-management              Test native inspection and maintenance locally' \
	  'make example-local               Run the offline application walkthrough' \
	  'make test-example-local          Check catalog and read-completion boundaries' \
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

test-caffeine:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib caffeine::tests

test-local:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib local::

install-test-server: toko-tooling-check
	@tool="$$(bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh --consumer "$(CURDIR)" --package ic-testkit --lockfile Cargo.lock --bin ic-testkit-server --profile release --features cli)" && \
	  "$$tool" setup --directory "$(CURDIR)/.tools/ic-testkit-server"

test-server-check:
	@tool="$$(bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh --consumer "$(CURDIR)" --package ic-testkit --lockfile Cargo.lock --bin ic-testkit-server --profile release --features cli --check)" && \
	  "$$tool" check --directory "$(CURDIR)/.tools/ic-testkit-server"

clippy-host: toko-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-canister --target wasm32-unknown-unknown --release -- -D warnings

build-host: clippy-host
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked -p ic-blob-storage-canister --target wasm32-unknown-unknown --release --message-format=json > target/blob-wasm-artifacts.jsonl

test-standalone: build-host
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-tests --test standalone -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage-tests --test standalone

build-admin: toko-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-admin --bin ic-blob-storage-admin -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked -p ic-blob-storage-admin --bin ic-blob-storage-admin --message-format=json > target/blob-admin-artifacts.jsonl

test-management: build-host build-admin
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-tests --test management -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  admin="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic-blob-storage-admin" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one Blob admin artifact") end' target/blob-admin-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_ADMIN="$$admin" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage-tests --test management

example-local:
	CARGO_BUILD_WARNINGS=deny cargo run --offline --locked -p ic-blob-storage --example local_asset

test-example-local:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --example local_asset

documentation-links-check:
	@find docs -type f -name '*.md' -print0 | xargs -0 perl vendor/toko-tooling/scripts/ci/check-documentation-links.pl --root . *.md

ci: toko-tooling-check fmt-check clippy test-references test-caffeine test-local example-local test-example-local docs-check documentation-links-check
