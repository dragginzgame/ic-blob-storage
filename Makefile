SHELL := /bin/bash
.DEFAULT_GOAL := help

# Keep all builds in this repository, including calls from another workspace.
export CARGO_TARGET_DIR := $(CURDIR)/target
# Explicit path prevents PocketIC from downloading a server during tests.
export POCKET_IC_BIN ?= $(CURDIR)/.tmp/tools/pocket-ic-16.0.0/pocket-ic
export BLOB_AUTHORITY_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_authority_probe.wasm
export BLOB_ADMISSION_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_admission_probe.wasm
export BLOB_STORAGE_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_storage_probe.wasm
export BLOB_CONSUMER_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_consumer_probe.wasm
export BLOB_GATEWAY_SOURCE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_gateway_source.wasm
export BLOB_FUNDING_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_funding_probe.wasm
export BLOB_STANDALONE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm
export BLOB_CANIC_PROBE_WASM := $(CURDIR)/.tmp/canic-probe/.icp/local/canisters/storage/storage.wasm
CANIC ?= canic
export BLOB_CLI_BIN := $(CARGO_TARGET_DIR)/debug/blob-storage
export BLOB_BROWSER_NODE ?= node
VERSION ?=
RELEASE := bash scripts/release/release.sh
CI_TARGETS := shell-check release-check fmt-check check clippy probe-check docs-check test wasm-check package

.PHONY: help version deps cloc fmt fmt-check check clippy docs-check test test-native test-pocketic test-browser test-sdk-probe test-fixture test-standalone build-standalone prepare-canic-probe build-canic-probe test-canic-composition test-admission-resources test-read-resources wasm-check \
	build package clean shell-check release-check probe-check ci validate release-verify \
	release-plan ensure-clean patch minor major bump-x release-patch \
	release-minor release-major release-x release-stage release-commit \
	release-tag-check release-push publish publish-dry-run

help:
	@echo "deps                         Fetch locked Rust dependencies (network)"
	@echo "cloc                         Rust runtime/test file counts under crates/"
	@echo "fmt / fmt-check              Format Rust or check formatting"
	@echo "check / clippy / test         Compile, lint, or test the workspace"
	@echo "test-native / test-pocketic   Native core tests or local IC fixtures"
	@echo "build-standalone / test-standalone   Standalone host Wasm or focused local IC tests"
	@echo "test-canic-composition        Published Canic lifecycle and shared storage fixture"
	@echo "test-browser                  Opt-in Chromium certificate/IndexedDB evidence"
	@echo "test-sdk-probe                Opt-in local SDK fault probe; BLOB_SDK_PROBE_REPORT=NEW_DIRECTORY"
	@echo "test-admission-resources     Local admission bounds and Wasm resource report"
	@echo "test-read-resources          Local read-slot and Wasm resource report"
	@echo "clean                        Explicitly remove build artifacts"
	@echo "docs-check / wasm-check       Check docs or the Wasm library build"
	@echo "probe-check                  Verify retained Caffeine probe artifacts offline"
	@echo "ci / validate                Run the current repository validation gate"
	@echo "release-check                Test release tooling without publication"
	@echo "release-plan VERSION=minor   Preview patch/minor/major or an exact version"
	@echo "patch / minor / major         Validate and update release files for review"
	@echo "bump-x VERSION=x.y.z          Prepare an exact release (including the first)"
	@echo "release-{patch,minor,major}   Maintainer: prepare, commit, tag, push"
	@echo "release-x VERSION=x.y.z       Maintainer: release an exact version"
	@echo "release-stage / release-commit / release-push   Individual release steps"
	@echo "publish / publish-dry-run     Separately publish or verify registry upload"

version:
	@$(RELEASE) version

deps:
	cargo fetch --locked

cloc:
	bash scripts/dev/cloc.sh

fmt:
	cargo fmt --all
	cargo fmt --manifest-path canisters/test/canic_probe/Cargo.toml

fmt-check:
	cargo fmt --all -- --check
	cargo fmt --manifest-path canisters/test/canic_probe/Cargo.toml -- --check

check:
	cargo check --offline --locked --workspace --all-targets --all-features

clippy:
	cargo clippy --offline --locked --workspace --all-targets --all-features -- -D warnings
	+$(MAKE) --no-print-directory prepare-canic-probe
	cargo clippy --offline --locked --manifest-path canisters/test/canic_probe/Cargo.toml --all-targets --all-features -- -D warnings

probe-check:
	cargo build --offline --locked -p ic-blob-storage-cli --bin caffeine-probe
	@for run in docs/evidence/caffeine-probes/runs/*; do \
		"$(CARGO_TARGET_DIR)/debug/caffeine-probe" verify "$$run" || exit $$?; \
	done
	sha256sum --check --strict --quiet docs/evidence/caffeine-probes/local/SHA256SUMS

docs-check:
	RUSTDOCFLAGS="-D warnings" cargo doc --offline --locked -p ic-blob-storage -p ic-blob-storage-canister -p ic-blob-storage-canic --all-features --no-deps

test:
	+$(MAKE) --no-print-directory test-native
	+$(MAKE) --no-print-directory test-pocketic

test-native:
	cargo test --offline --locked -p ic-blob-storage -p blob-consumer-probe -p ic-blob-storage-canister -p ic-blob-storage-cli -p ic-blob-storage-canic --all-features

test-fixture:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-authority-probe -p blob-admission-probe -p blob-storage-probe -p blob-consumer-probe -p blob-gateway-source -p blob-funding-probe --lib
	+$(MAKE) --no-print-directory build-canic-probe

test-pocketic:
	+$(MAKE) --no-print-directory test-fixture
	+$(MAKE) --no-print-directory build-standalone
	cargo build --offline --locked -p ic-blob-storage-cli
	cargo test --offline --locked -p ic-blob-storage-pocketic-tests

build-standalone:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p ic-blob-storage-canister --lib

prepare-canic-probe:
	cp Cargo.lock canisters/test/canic_probe/Cargo.lock
	cargo metadata --offline --manifest-path canisters/test/canic_probe/Cargo.toml --format-version 1 > /dev/null

build-canic-probe:
	+$(MAKE) --no-print-directory prepare-canic-probe
	mkdir -p .tmp/canic-probe
	CARGO_NET_OFFLINE=true RUSTC_WRAPPER="$${RUSTC_WRAPPER-}" $(CANIC) build blob_canic_probe storage --workspace "$(CURDIR)/canisters/test/canic_probe" --icp-root "$(CURDIR)/.tmp/canic-probe" --config "$(CURDIR)/canisters/test/canic_probe/canic.toml" --profile release

test-canic-composition:
	+$(MAKE) --no-print-directory build-canic-probe
	cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test canic_composition -- --test-threads=1

test-standalone:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p ic-blob-storage-canister -p blob-gateway-source -p blob-storage-probe -p blob-consumer-probe --lib
	cargo build --offline --locked -p ic-blob-storage-cli
	cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test standalone -- --test-threads=1

# Browser tooling is explicitly provisioned; this target performs no downloads.
test-sdk-probe:
	@test -n "$(BLOB_SDK_PROBE_REPORT)" || { echo 'Set BLOB_SDK_PROBE_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	cargo build --offline --locked -p ic-blob-storage --example verify_download
	$(BLOB_BROWSER_NODE) .tmp/browser/sdk-probe.mjs "$(BLOB_SDK_PROBE_REPORT)" "$(CARGO_TARGET_DIR)/debug/examples/verify_download"

test-browser:
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe -p blob-consumer-probe --lib
	cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test storage chromium_certificate_intent -- --ignored --test-threads=1

test-admission-resources:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-admission-probe --lib
	@mkdir -p .tmp
	BLOB_ADMISSION_RESOURCE_REPORT="$(CURDIR)/.tmp/admission-resources.json" \
	BLOB_ADMISSION_HISTORY_REPORT="$(CURDIR)/.tmp/admission-history.json" \
	BLOB_RELEASE_HISTORY_REPORT="$(CURDIR)/.tmp/release-history.json" \
	BLOB_REFERENCE_HISTORY_REPORT="$(CURDIR)/.tmp/reference-history.json" \
	BLOB_DESCRIPTOR_RESOURCE_REPORT="$(CURDIR)/.tmp/descriptor-resources.json" \
	cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test admission admission_resources -- --test-threads=2
	@echo "Local resource reports in .tmp/: admission-resources.json, admission-history.json, release-history.json, reference-history.json, descriptor-resources.json (not provider pricing)"

test-read-resources:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-authority-probe -p blob-gateway-source --lib
	@mkdir -p .tmp
	BLOB_READ_RESOURCE_REPORT="$(CURDIR)/.tmp/read-resources.json" \
	cargo test --offline --locked -p ic-blob-storage-pocketic-tests --test journey readback -- --test-threads=2
	@echo "Local read report: .tmp/read-resources.json (not a production read protocol or provider pricing)"

wasm-check:
	cargo check --offline --locked --workspace --all-features --target wasm32-unknown-unknown

build:
	cargo build --offline --locked -p ic-blob-storage --all-features

package:
	cargo package --offline --locked --allow-dirty -p ic-blob-storage

clean:
	cargo clean

shell-check:
	bash -n scripts/release/*.sh
	bash -n scripts/dev/cloc.sh
	shellcheck scripts/release/*.sh scripts/dev/cloc.sh
	perl -c scripts/release/release-data.pl

release-check:
	bash scripts/release/test-release.sh

ci:
	+@set -e; for target in $(CI_TARGETS); do \
		$(MAKE) --no-print-directory "$$target"; \
	done

validate: ci

release-verify: ci

release-plan:
	@$(RELEASE) plan "$(if $(VERSION),$(VERSION),patch)"

ensure-clean:
	@$(RELEASE) ensure-clean

patch:
	$(RELEASE) bump patch

minor:
	$(RELEASE) bump minor

major:
	$(RELEASE) bump major

bump-x:
	$(RELEASE) bump "$(VERSION)"

release-patch:
	$(RELEASE) release patch

release-minor:
	$(RELEASE) release minor

release-major:
	$(RELEASE) release major

release-x:
	$(RELEASE) release "$(VERSION)"

release-stage:
	$(RELEASE) stage

release-commit:
	$(RELEASE) commit

release-tag-check:
	$(RELEASE) tag-check

release-push:
	$(RELEASE) push

publish:
	$(RELEASE) publish

publish-dry-run:
	$(RELEASE) publish --dry-run
