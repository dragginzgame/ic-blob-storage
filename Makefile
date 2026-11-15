.DEFAULT_GOAL := help
SHELL := /bin/bash
.SHELLFLAGS := -o pipefail -c
LOCAL_TOOL_INSTALL_TARGETS += install-test-server
LOCAL_TOOL_CHECK_TARGETS += test-server-check
FORMAT_RUST_ARGS := --extra-workspace qualification

include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help deps update clippy clippy-qualification test-references test-caffeine test-local example-local test-example-local docs-check docs-qualification-check documentation-links-check ci qualification-tooling-check install-test-server test-server-check clippy-host build-host test-standalone test-capacity build-admin test-management build-reply-probe test-endpoints test-signed build-reference-app test-application

help:
	@printf '%s\n' \
	  'make install-tools / tools-check  Prepare / admit the selected development tools' \
	  'make toko-tooling-check           Admit the locked tooling snapshot' \
	  'make fmt / fmt-check              Format / check workspace source' \
	  'make clippy / clippy-qualification  Strict product / qualification lint checks' \
	  'make test-references             Bounded object-reference behavior tests' \
	  'make test-caffeine               Caffeine stream and manifest integrity tests' \
	  'make test-local                  Local grant, reference and accounting tests' \
	  'make install-test-server         Explicit authenticated local simulator setup' \
	  'make test-standalone              Build and test the installed host locally' \
	  'make test-capacity                Measure full-history host resources and cleanup' \
	  'make test-endpoints               Test installed frame admission and reply rollback' \
	  'make test-signed                  Test signed application and uploader calls locally' \
	  'make test-application             Test durable catalog and cleanup between canisters' \
	  'make test-management              Test native inspection and maintenance locally' \
	  'make example-local               Run the offline application walkthrough' \
	  'make test-example-local          Check catalog and read-completion boundaries' \
	  'make docs-check / docs-qualification-check  Product / qualification Rustdoc checks' \
	  'make documentation-links-check   Check repository Markdown links' \
	  'make deps                        Fetch the locked dependencies' \
	  'make deps-list                   List workspace package versions and groups' \
	  'make update / deps-update        Update Cargo locks within manifest requirements' \
	  'make toko-tooling-update          Install the Cargo-selected tooling snapshot' \
	  'make ci                          Aggregate tooling, format, lint, test and docs gate' \
	  'toko-cloc                        Categorized source counts (PATH launcher)'

deps:
	cargo fetch --locked
	cargo fetch --locked --manifest-path qualification/Cargo.toml

update: deps-update

clippy:
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --workspace --all-targets --all-features -- -D warnings

clippy-qualification: qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml --workspace --all-targets --all-features -- -D warnings

docs-check:
	RUSTDOCFLAGS='-D warnings' cargo doc --offline --locked --workspace --all-features --no-deps

docs-qualification-check: qualification-tooling-check
	RUSTDOCFLAGS='-D warnings' cargo doc --offline --locked --manifest-path qualification/Cargo.toml --workspace --all-features --no-deps

qualification-tooling-check: toko-tooling-check
	@product="$$(perl vendor/toko-tooling/scripts/dev/tooling-lock.pl "$(CURDIR)")" && \
	  qualification="$$(perl vendor/toko-tooling/scripts/dev/tooling-lock.pl "$(CURDIR)/qualification")" && \
	  { test "$$product" = "$$qualification" || { printf '%s\n' 'Qualification must select the same installed Toko Tooling payload as the product workspace.' >&2; exit 1; }; }

test-references:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib references::tests

test-caffeine:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib caffeine::tests

test-local:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib local::

install-test-server: qualification-tooling-check
	@tool="$$(bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh --consumer "$(CURDIR)" --package ic-testkit --lockfile qualification/Cargo.lock --bin ic-testkit-server --profile release --features cli)" && \
	  "$$tool" setup --directory "$(CURDIR)/.tools/ic-testkit-server"

test-server-check: qualification-tooling-check
	@tool="$$(bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh --consumer "$(CURDIR)" --package ic-testkit --lockfile qualification/Cargo.lock --bin ic-testkit-server --profile release --features cli --check)" && \
	  "$$tool" check --directory "$(CURDIR)/.tools/ic-testkit-server"

clippy-host: toko-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-canister --target wasm32-unknown-unknown --release -- -D warnings

build-host: clippy-host
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked -p ic-blob-storage-canister --target wasm32-unknown-unknown --release --message-format=json > target/blob-wasm-artifacts.jsonl

test-standalone: build-host qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test standalone -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test standalone

test-capacity: build-host qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test capacity -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test capacity -- --nocapture

test-signed: build-host qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test signed -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test signed

build-reference-app: toko-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-reference-app --target wasm32-unknown-unknown --release -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked -p ic-blob-storage-reference-app --target wasm32-unknown-unknown --release --message-format=json > target/blob-reference-app-artifacts.jsonl

test-application: build-host build-reference-app qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test application -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  app="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_reference_app") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one application Wasm artifact") end' target/blob-reference-app-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_REFERENCE_APP="$$app" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test application

build-admin: toko-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-admin --bin ic-blob-storage-admin -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked -p ic-blob-storage-admin --bin ic-blob-storage-admin --message-format=json > target/blob-admin-artifacts.jsonl

build-reply-probe: qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-reply-probe --target wasm32-unknown-unknown --release -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-reply-probe --target wasm32-unknown-unknown --release --message-format=json > target/blob-reply-probe-artifacts.jsonl

test-endpoints: build-host build-reply-probe
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test endpoints -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  probe="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_reply_probe") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one reply probe artifact") end' target/blob-reply-probe-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_REPLY_PROBE="$$probe" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test endpoints

test-management: build-host build-admin qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test management -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  admin="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic-blob-storage-admin" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one Blob admin artifact") end' target/blob-admin-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_ADMIN="$$admin" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test management

example-local:
	CARGO_BUILD_WARNINGS=deny cargo run --offline --locked -p ic-blob-storage --example local_asset

test-example-local:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --example local_asset

documentation-links-check:
	@find docs -type f -name '*.md' -print0 | xargs -0 perl vendor/toko-tooling/scripts/ci/check-documentation-links.pl --root . *.md

ci: toko-tooling-check fmt-check clippy clippy-qualification test-references test-caffeine test-local example-local test-example-local docs-check docs-qualification-check documentation-links-check
