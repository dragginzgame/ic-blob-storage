SHELL := /bin/bash
.DEFAULT_GOAL := help

# Keep all builds in this repository, including calls from another workspace.
export CARGO_TARGET_DIR := $(CURDIR)/target
# Explicit path prevents PocketIC from downloading a server during tests.
export POCKET_IC_BIN ?= $(CURDIR)/.tmp/tools/pocket-ic-16.0.0/pocket-ic
export BLOB_AUTHORITY_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_authority_probe.wasm
export BLOB_GATEWAY_SOURCE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_gateway_source.wasm
VERSION ?=
RELEASE := bash scripts/release/release.sh
CI_TARGETS := shell-check release-check fmt-check check clippy docs-check test wasm-check package

.PHONY: help version deps fmt fmt-check check clippy docs-check test test-native test-pocketic test-fixture wasm-check \
	build package clean shell-check release-check ci validate release-verify \
	release-plan ensure-clean patch minor major bump-x release-patch \
	release-minor release-major release-x release-stage release-commit \
	release-tag-check release-push publish publish-dry-run

help:
	@echo "deps                         Fetch locked Rust dependencies (network)"
	@echo "fmt / fmt-check              Format Rust or check formatting"
	@echo "check / clippy / test         Compile, lint, or test the workspace"
	@echo "test-native / test-pocketic   Native core tests or local IC fixtures"
	@echo "clean                        Explicitly remove build artifacts"
	@echo "docs-check / wasm-check       Check docs or the Wasm library build"
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

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

check:
	cargo check --offline --locked --workspace --all-targets --all-features

clippy:
	cargo clippy --offline --locked --workspace --all-targets --all-features -- -D warnings

docs-check:
	RUSTDOCFLAGS="-D warnings" cargo doc --offline --locked -p ic-blob-storage --all-features --no-deps

test:
	+$(MAKE) --no-print-directory test-native
	+$(MAKE) --no-print-directory test-pocketic

test-native:
	cargo test --offline --locked -p ic-blob-storage --all-features

test-fixture:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-authority-probe -p blob-gateway-source --lib

test-pocketic:
	+$(MAKE) --no-print-directory test-fixture
	cargo test --offline --locked -p ic-blob-storage-pocketic-tests

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
	shellcheck scripts/release/*.sh
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
