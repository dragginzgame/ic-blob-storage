.DEFAULT_GOAL := help
SHELL := /bin/bash
.SHELLFLAGS := -o pipefail -c
LOCAL_TOOL_INSTALL_TARGETS += install-test-server install-browser-tools
LOCAL_TOOL_CHECK_TARGETS += test-server-check browser-tools-check
FORMAT_RUST_ARGS := --extra-workspace qualification

TOKO_TOOLING_RELEASE := 1
RELEASE_BRANCH := main
RELEASE_REMOTE := origin
RELEASE_DELIVERY_POLICY := scripts/release/direct.txt
CARGO_PUBLISH_PACKAGES := ic-blob-storage ic-blob-storage-reference-app ic-blob-storage-canister ic-blob-storage-cli
CARGO_PUBLISH_REGISTRY := crates-io
CARGO_PUBLISH_VERIFY_TARGETS := toko-tooling-check tooling-binding-check publication-admission

include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help deps update clippy clippy-qualification test-references test-caffeine test-local test-cli example-local test-example-local docs-check docs-qualification-check documentation-links-check ci qualification-tooling-check install-test-server test-server-check clippy-host build-host test-standalone test-capacity blob build-cli test-management build-reply-probe test-endpoints test-signed build-reference-app test-application build-enrollment-probe test-enrollment
.PHONY: build-callback-probe test-callbacks test-browser test-browser-runtime test-browser-application install-browser-tools browser-tools-check
.PHONY: dependency-pins-check

help:
	@printf '%s\n' \
	  'make install-tools / tools-check  Prepare / admit the selected development tools' \
	  'make toko-tooling-check           Admit the locked tooling snapshot' \
	  'make dependency-pins-check       Check Cargo inheritance, lockfiles and action pins' \
	  'make fmt / fmt-check              Format / check workspace source' \
	  'make clippy / clippy-qualification  Strict product / qualification lint checks' \
	  'make test-references             Bounded object-reference behavior tests' \
	  'make test-caffeine               Caffeine stream and manifest integrity tests' \
	  'make test-browser               Minimal transfer client (Node 24+, local HTTP only)' \
	  'make test-browser-runtime       Chromium transport (BROWSER=/absolute/path)' \
	  'make test-browser-application   Chromium with the installed application catalog' \
	  'make install-browser-tools      Install locked npm dependencies without scripts' \
	  'make test-local                  Local grant, reference and accounting tests' \
	  'make test-cli                    Provider reply and offline enrollment checks' \
	  'make blob                        Build and install blob into Cargo bin (PATH)' \
	  'make install-test-server         Explicit authenticated local simulator setup' \
	  'make test-standalone              Build and test the installed host locally' \
	  'make test-capacity                Measure full-history host resources and cleanup' \
	  'make test-endpoints               Test installed frame admission and reply rollback' \
	  'make test-signed                  Test signed application and uploader calls locally' \
	  'make test-application             Test durable catalog and cleanup between canisters' \
	  'make test-enrollment              Test the single-use owner enrollment probe locally' \
	  'make test-callbacks               Test the installed candidate callback profile locally' \
	  'make test-management              Test native inspection and maintenance locally' \
	  'make example-local               Run the offline application walkthrough' \
	  'make test-example-local          Check catalog and read-completion boundaries' \
	  'make docs-check / docs-qualification-check  Product / qualification Rustdoc checks' \
	  'make documentation-links-check   Check repository Markdown links' \
	  'make deps                        Fetch the locked dependencies' \
	  'make deps-list                   List workspace package versions and groups' \
	  'make update / deps-update        Update Cargo locks within manifest requirements' \
	  'make toko-tooling-update          Install the Cargo-selected tooling snapshot' \
	  'make release-patch/minor/major    Validate, version and deliver to GitHub' \
	  'make publish-check / publish     Check / publish the tagged product crates' \
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

dependency-pins-check: qualification-tooling-check
	bash vendor/toko-tooling/scripts/ci/check-dependency-pins.sh --consumer "$(CURDIR)" --cargo-inheritance

test-references:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib references::tests

test-caffeine:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib caffeine::tests

test-cli:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage-cli --bin blob

test-browser:
	node --test --test-timeout=10000 clients/browser/tests/*.test.js

install-browser-tools: toko-tooling-check
	npm ci --ignore-scripts --prefix clients/browser

browser-tools-check:
	npm ls --prefix clients/browser --all

test-browser-runtime: toko-tooling-check
	BROWSER="$(BROWSER)" node clients/browser/tests/runtime.js

test-browser-application: build-callback-probe build-cli build-reference-app
	@test -n "$(BROWSER)" || { printf '%s\n' 'Set BROWSER to an absolute Chromium executable path.' >&2; exit 1; }
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test callbacks -- -D warnings
	@probe="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_callback_probe") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one callback probe artifact") end' target/blob-callback-probe-artifacts.jsonl)" && \
	  cli="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "blob" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one Blob CLI artifact") end' target/blob-cli-artifacts.jsonl)" && \
	  app="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_reference_app") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one application Wasm artifact") end' target/blob-reference-app-artifacts.jsonl)" && \
	  BROWSER="$(BROWSER)" BLOB_CALLBACK_PROBE="$$probe" BLOB_CLI="$$cli" BLOB_REFERENCE_APP="$$app" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test callbacks browser_publication_and_late_read_use_the_real_catalog -- --ignored

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

test-application: build-host build-reference-app build-cli qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test application -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  app="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_reference_app") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one application Wasm artifact") end' target/blob-reference-app-artifacts.jsonl)" && \
	  cli="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "blob" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one Blob CLI artifact") end' target/blob-cli-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_REFERENCE_APP="$$app" BLOB_CLI="$$cli" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test application

blob: build-cli
	CARGO_BUILD_WARNINGS=deny cargo install --offline --locked --debug --path crates/ic-blob-storage-cli --bin blob

build-cli: toko-tooling-check tooling-binding-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage-cli --bin blob -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked -p ic-blob-storage-cli --bin blob --message-format=json > target/blob-cli-artifacts.jsonl

build-reply-probe: qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-reply-probe --target wasm32-unknown-unknown --release -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-reply-probe --target wasm32-unknown-unknown --release --message-format=json > target/blob-reply-probe-artifacts.jsonl

build-callback-probe: qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-callback-probe --target wasm32-unknown-unknown --release -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-callback-probe --target wasm32-unknown-unknown --release --message-format=json > target/blob-callback-probe-artifacts.jsonl

test-callbacks: build-callback-probe build-cli build-reference-app
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked -p ic-blob-storage --lib --tests -- -D warnings
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --lib local::persistence::callbacks
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test callbacks -- -D warnings
	@probe="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_callback_probe") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one callback probe artifact") end' target/blob-callback-probe-artifacts.jsonl)" && \
	  cli="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "blob" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one Blob CLI artifact") end' target/blob-cli-artifacts.jsonl)" && \
	  app="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_reference_app") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one application Wasm artifact") end' target/blob-reference-app-artifacts.jsonl)" && \
	  BLOB_CALLBACK_PROBE="$$probe" BLOB_CLI="$$cli" BLOB_REFERENCE_APP="$$app" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test callbacks

build-enrollment-probe: qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-enrollment-probe --target wasm32-unknown-unknown --release -- -D warnings
	@mkdir -p target
	CARGO_BUILD_WARNINGS=deny cargo build --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-enrollment-probe --target wasm32-unknown-unknown --release --message-format=json > target/blob-enrollment-probe-artifacts.jsonl

test-enrollment: build-enrollment-probe build-host
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-enrollment-probe --lib --tests --examples -- -D warnings
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-enrollment-probe --lib --example enrollment
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test enrollment -- -D warnings
	@probe="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_enrollment_probe") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one enrollment probe artifact") end' target/blob-enrollment-probe-artifacts.jsonl)" && \
	  wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  BLOB_ENROLLMENT_PROBE="$$probe" BLOB_WASM="$$wasm" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test enrollment

test-endpoints: build-host build-reply-probe
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test endpoints -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  probe="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_reply_probe") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one reply probe artifact") end' target/blob-reply-probe-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_REPLY_PROBE="$$probe" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test endpoints

test-management: build-host build-cli qualification-tooling-check
	CARGO_BUILD_WARNINGS=deny cargo clippy --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test management -- -D warnings
	@wasm="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "ic_blob_storage_canister") | .filenames[] | select(endswith(".wasm"))] | if length == 1 then .[0] else error("expected one Blob Wasm artifact") end' target/blob-wasm-artifacts.jsonl)" && \
	  cli="$$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "blob" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one Blob CLI artifact") end' target/blob-cli-artifacts.jsonl)" && \
	  BLOB_WASM="$$wasm" BLOB_CLI="$$cli" BLOB_TEST_SERVER="$(CURDIR)/.tools/ic-testkit-server" CARGO_BUILD_WARNINGS=deny cargo test --offline --locked --manifest-path qualification/Cargo.toml -p ic-blob-storage-tests --test management

example-local:
	CARGO_BUILD_WARNINGS=deny cargo run --offline --locked -p ic-blob-storage --example local_asset

test-example-local:
	CARGO_BUILD_WARNINGS=deny cargo test --offline --locked -p ic-blob-storage --example local_asset

documentation-links-check:
	@find docs clients -name node_modules -prune -o -type f -name '*.md' -print0 | xargs -0 perl vendor/toko-tooling/scripts/ci/check-documentation-links.pl --root . *.md

.NOTPARALLEL: ci
ci: toko-tooling-check tooling-binding-check dependency-pins-check fmt-check clippy clippy-qualification test-references test-caffeine test-local test-cli test-browser example-local test-example-local docs-check docs-qualification-check documentation-links-check

# Shared release/publication engines; these hooks own only Blob's metadata and gate.
.PHONY: release-editor-setup test-release-metadata publication-admission
release-editor-setup:
	bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh --consumer "$(CURDIR)" --package ic-build --version 0.73.0 --bin ic-build-release-metadata --profile debug --features cli

test-release-metadata:
	bash scripts/release/test-metadata.sh

release-version:
	@bash scripts/release/adapter.sh version
release-files:
	@printf '%s\0' Cargo.toml Cargo.lock CHANGELOG.md qualification/Cargo.lock
release-preflight:
	@bash scripts/release/adapter.sh preflight
release-verify:
	$(MAKE) --no-print-directory tools-check test-release-metadata
	$(MAKE) --no-print-directory ci
	$(MAKE) --no-print-directory test-standalone test-capacity test-endpoints test-signed test-application test-enrollment test-callbacks test-management
release-prepare-version:
	@bash scripts/release/adapter.sh prepare
release-prepared-check release-committed-check release-tagged-check release-push-check:
	@bash scripts/release/adapter.sh check
release-commit-check:
	@bash scripts/release/adapter.sh commit-check
release-merged-preflight:
	@printf '%s\n' 'Blob supports direct release delivery only.' >&2; exit 1
publication-admission:
	@bash scripts/release/adapter.sh publication

.PHONY: tooling-binding-check
tooling-binding-check:
	@bash vendor/toko-tooling/scripts/ci/verify-file-checksum.sh sha256 "$$(cat crates/ic-blob-storage-cli/tooling.sha256)" .toko-tooling.snapshot
