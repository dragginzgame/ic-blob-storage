SHELL := /bin/bash
.DEFAULT_GOAL := help

# Keep all builds in this repository, including calls from another workspace.
export CARGO_TARGET_DIR := $(CURDIR)/target
# Explicit path prevents PocketIC from downloading a server during tests.
export POCKET_IC_BIN ?= $(CURDIR)/.tools/ic/bin/pocket-ic
IC_TOOL_PINS ?= ci/ic-tools.tsv
HOST_TOOL_VERSIONS ?= ci/tool-versions.env
include make/tools.mk
export BLOB_AUTHORITY_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_authority_probe.wasm
export BLOB_ADMISSION_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_admission_probe.wasm
export BLOB_STORAGE_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_storage_probe.wasm
export BLOB_CONSUMER_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_consumer_probe.wasm
export BLOB_GATEWAY_SOURCE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_gateway_source.wasm
export BLOB_FUNDING_PROBE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/blob_funding_probe.wasm
export BLOB_STANDALONE_WASM := $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm
export BLOB_CLI_BIN := $(CARGO_TARGET_DIR)/debug/blob-storage
export BLOB_EXPECTED_HOST_RELEASE := $(shell perl scripts/release/release-data.pl version)
export BLOB_BROWSER_NODE ?= node
export BLOB_BROWSER_NPM ?= npm
BLOB_SDK_INPUTS_BYTES ?= 10485760
VERSION ?=
RELEASE := bash scripts/release/release.sh
RELEASE_REMOTE ?= origin
RELEASE_BRANCH ?= main
export RELEASE_KIND RELEASE_PREVIOUS RELEASE_VERSION RELEASE_DATE RELEASE_SOURCE RELEASE_COMMIT RELEASE_REMOTE RELEASE_BRANCH
SHELLCHECK ?= shellcheck
CI_TARGETS := shared-tooling-check tools-check dependency-pins-check documentation-links-check shared-tooling-tests deps shell-check release-check hooks-check fmt-check check clippy probe-check docs-check test wasm-check package

.PHONY: help version deps cloc shared-tooling-check dependency-pins-check fmt fmt-check check clippy docs-check test test-native test-pocketic test-browser test-browser-store test-browser-transport test-browser-standalone test-sdk-probe test-sdk-inputs test-fixture test-standalone build-standalone test-admission-resources test-read-resources test-funding-receipt-resources wasm-check \
	build package clean shell-check release-check probe-check ci validate release-verify test-browser-publication test-browser-bootstrap test-browser-launcher test-browser-native \
	release-plan ensure-clean release-patch release-minor release-major release-resume \
	release-version release-preflight release-prepare-version release-prepared-check release-files \
	release-commit-check release-committed-check release-tagged-check release-push-check \
	release-tag-check publish publish-dry-run install-hooks format-tools-check release-tools-check hooks-check evidence-check
.PHONY: test-hard-cut test-native-host pocketic-alignment-check install-tools tools-check install-host-tools host-tools-check install-ic-tools ic-tools-check
.PHONY: documentation-links-check release-commands-check shared-tooling-tests

ifneq ($(word 2,$(filter release-patch release-minor release-major release-resume,$(MAKECMDGOALS))),)
$(error Select exactly one release target)
endif

help:
	@echo "release-tools-check          Check ShellCheck, pinned cargo-sort and rustfmt without building"
	@echo "test-native-host             CLI/examples and local installation/restoration probe checks"
	@echo "pocketic-alignment-check      Compare the locked PocketIC client with reviewed server pins offline"
	@echo "browser-tools-check          Offline exact browser Node/npm and manifest/lock checks"
	@echo "install-tools / tools-check   Explicit local tool installation / offline verification"
	@echo "install-host-tools / host-tools-check   Pinned repo-local jq/yq/rg/cloc setup / verification"
	@echo "install-ic-tools / ic-tools-check       Pinned repo-local IC setup / verification"
	@echo "install-rust-tools / rust-tools-check   Pinned repo-local Cargo tools setup / verification"
	@echo "deps                         Fetch locked Rust dependencies (network)"
	@echo "cloc                         Offline Rust runtime/test counts for every workspace member"
	@echo "cloc-tooling                 Inventory local/shared tooling across sibling repositories"
	@echo "test-funding-receipt-resources  Measure populated receipt confirmation and restore (opt-in)"
	@echo "shared-tooling-check          Verify the reviewed shared snapshot offline"
	@echo "shared-tooling-tests          Exercise shared digest, IC installer, lockfile and metadata refusals offline"
	@echo "documentation-links-check     Check local Markdown targets without a build or network"
	@echo "release-commands-check        Check Make routing with a substitute release runner"
	@echo "dependency-pins-check         Check dependency selectors and workspace lockfiles offline"
	@echo "fmt / fmt-check              Format Rust or check formatting"
	@echo "check / clippy / test         Compile, lint, or test the workspace"
	@echo "test-native / test-pocketic   Native core tests or local IC fixtures"
	@echo "build-standalone / test-standalone   Standalone host Wasm or focused local IC tests"
	@echo "test-hard-cut                Old-ledger refusal; explicit pinned Wasm and fresh BLOB_HARD_CUT_REPORT"
	@echo "test-browser                  Opt-in Chromium certificate/IndexedDB evidence"
	@echo "test-browser-store            Opt-in Chromium journal persistence without Rust builds"
	@echo "test-browser-publication      Opt-in offline frozen-file checks; BLOB_PUBLICATION_REPORT=NEW_DIRECTORY"
	@echo "test-browser-transport        Opt-in owned TLS transport checks; BLOB_BROWSER_TRANSPORT_REPORT=NEW_DIRECTORY"
	@echo "test-browser-bootstrap        Opt-in owned worker/IndexedDB checks; BLOB_BOOTSTRAP_REPORT=NEW_DIRECTORY"
	@echo "test-browser-launcher         Opt-in native Chromium/profile/body checks; BLOB_LAUNCHER_REPORT=NEW_DIRECTORY"
	@echo "test-browser-native           Owned native pipe checks; BLOB_NATIVE_REPORT=NEW_DIRECTORY"
	@echo "test-browser-standalone       Opt-in local standalone upload/verified download rehearsal"
	@echo "test-sdk-probe                Opt-in local SDK fault probe; BLOB_SDK_PROBE_REPORT=NEW_DIRECTORY"
	@echo "test-sdk-inputs               Opt-in offline native/browser handoff; BLOB_SDK_INPUTS_REPORT=NEW_DIRECTORY"
	@echo "test-admission-resources     Local admission bounds and Wasm resource report"
	@echo "test-read-resources          Local read-slot and Wasm resource report"
	@echo "clean                        Explicitly remove build artifacts"
	@echo "docs-check / wasm-check       Check docs or the Wasm library build"
	@echo "probe-check                  Verify retained Caffeine probe artifacts offline"
	@echo "evidence-check               Check retained evidence hashes without Rust builds"
	@echo "ci / validate                Fetch locked dependencies, then validate"
	@echo "release-check                Test release tooling without publication"
	@echo "release-plan VERSION=minor   Preview patch/minor/major without effects"
	@echo "release-{patch,minor,major}   Maintainer: prepare, commit, tag, push"
	@echo "release-resume VERSION=x.y.z Maintainer: reconcile a saved release"
	@echo "install-hooks                Enable the repository-local formatting hook"
	@echo "publish / publish-dry-run     Separately publish or verify registry upload"

version:
	@$(RELEASE) version

deps:
	cargo fetch --locked

shared-tooling-check:
	bash scripts/ci/verify-shared-tooling-snapshot.sh

install-tools: install-rust-tools
tools-check: rust-tools-check

shared-tooling-tests:
	bash scripts/ci/test-format-tools.sh
	bash scripts/ci/check-validation-logging.sh
	bash scripts/ci/test-file-digests.sh
	bash scripts/ci/test-ic-tools.sh
	bash scripts/ci/test-pocketic-checks.sh
	perl scripts/ci/test-local-lock-versions.pl
	bash scripts/ci/test-cargo-metadata.sh
	bash scripts/ci/test-snapshot-distribution.sh
	bash scripts/ci/test-host-tools.sh
	bash scripts/ci/test-rust-tools.sh
	bash scripts/ci/test-tool-commands.sh
	bash scripts/ci/test-cloc-tooling.sh
	bash scripts/ci/test-cloc-siblings.sh
	bash scripts/ci/test-cloc.sh

documentation-links-check:
	@set -o pipefail; find docs audits rules -type f -name '*.md' -print0 | \
		xargs -0 perl scripts/ci/check-documentation-links.pl --root . *.md

release-commands-check:
	bash scripts/ci/check-release-commands.sh "$(CURDIR)" Cargo.toml scripts/release/release-data.pl make/tools.mk

dependency-pins-check:
	bash scripts/ci/check-dependency-pins.sh --cargo-inheritance

format-tools-check:
	@source ci/tool-versions.env; bash scripts/ci/check-format-tools.sh "$$SHARED_TOOLING_CARGO_SORT_VERSION"

release-tools-check: format-tools-check
	@command -v "$(SHELLCHECK)" >/dev/null 2>&1 && "$(SHELLCHECK)" --version >/dev/null 2>&1 || { echo "Prepare ShellCheck: sudo apt-get install shellcheck (Debian/Ubuntu) or brew install shellcheck (macOS)." >&2; echo "Select an existing executable with SHELLCHECK=/absolute/path/to/shellcheck." >&2; exit 1; }

install-hooks:
	bash scripts/dev/install-git-hooks.sh

hooks-check:
	bash scripts/ci/check-format-hooks.sh

fmt: format-tools-check
	cargo sort --workspace
	cargo fmt --all

fmt-check: format-tools-check
	cargo sort --workspace --check
	cargo fmt --all -- --check

check:
	cargo check --offline --locked --workspace --all-targets --all-features

clippy:
	cargo clippy --offline --locked --workspace --all-targets --all-features -- -D warnings

probe-check:
	cargo build --offline --locked -p ic-blob-storage-cli --bin caffeine-probe
	@for run in docs/evidence/caffeine-probes/runs/*; do \
		"$(CARGO_TARGET_DIR)/debug/caffeine-probe" verify "$$run" || exit $$?; \
	done
	+$(MAKE) --no-print-directory evidence-check

evidence-check:
	bash scripts/ci/test-evidence-checksums.sh
	bash scripts/ci/verify-evidence-checksums.sh docs/evidence/caffeine-probes/local/SHA256SUMS docs/evidence/caffeine-probes/deployed/SHA256SUMS

docs-check:
	RUSTDOCFLAGS="-D warnings" cargo doc --offline --locked -p ic-blob-storage -p ic-blob-storage-canister --all-features --no-deps

test:
	+$(MAKE) --no-print-directory test-native
	+$(MAKE) --no-print-directory test-pocketic

test-native:
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage -p blob-consumer-probe -p ic-blob-storage-canister -p ic-blob-storage-cli --all-features

pocketic-alignment-check:
	bash scripts/ci/check-pocketic-alignment.sh --manifest Cargo.toml --pins "$(IC_TOOL_PINS)"

test-native-host: pocketic-alignment-check
	+$(MAKE) --no-print-directory build-standalone
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe --lib
	cargo build --offline --locked -p ic-blob-storage-cli
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-cli --bins
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage --examples
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone standalone_installation_cli:: -- --test-threads=1
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage storage_resources::reads::restoration_reads_are_attributed_only_to_the_operator_reopen_window -- --exact --test-threads=1

test-fixture:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-authority-probe -p blob-admission-probe -p blob-storage-probe -p blob-consumer-probe -p blob-gateway-source -p blob-funding-probe --lib

test-pocketic: pocketic-alignment-check
	+$(MAKE) --no-print-directory test-fixture
	+$(MAKE) --no-print-directory build-standalone
	cargo build --offline --locked -p ic-blob-storage-cli
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests

build-standalone:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p ic-blob-storage-canister --lib

test-hard-cut:
	@test -n "$(BLOB_PRE_CUT_STANDALONE_WASM)" -a -n "$(BLOB_HARD_CUT_REPORT)" || { echo 'Set BLOB_PRE_CUT_STANDALONE_WASM and a fresh BLOB_HARD_CUT_REPORT'; exit 1; }
	+$(MAKE) --no-print-directory build-standalone
	BLOB_PRE_CUT_STANDALONE_WASM="$(BLOB_PRE_CUT_STANDALONE_WASM)" BLOB_HARD_CUT_REPORT="$(BLOB_HARD_CUT_REPORT)" \
		bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone standalone_hard_cut::older_allocation_ledger_upgrade_preserves_bytes_and_obligations_on_refusal -- --ignored --exact --test-threads=1

test-standalone: pocketic-alignment-check
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p ic-blob-storage-canister -p blob-gateway-source -p blob-storage-probe -p blob-consumer-probe --lib
	cargo build --offline --locked -p ic-blob-storage-cli
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone -- --test-threads=1

# Browser tooling is explicitly provisioned; this target performs no downloads.
.PHONY: browser-tools-check
browser-tools-check:
	bash scripts/ci/check-browser-tools.sh

test-sdk-probe test-sdk-inputs test-browser-transport test-browser-publication \
test-browser-store test-browser-bootstrap test-browser-launcher test-browser-native \
test-browser-standalone test-browser: browser-tools-check

test-sdk-probe:
	@test -n "$(BLOB_SDK_PROBE_REPORT)" || { echo 'Set BLOB_SDK_PROBE_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	cargo build --offline --locked -p ic-blob-storage --example verify_download
	$(BLOB_BROWSER_NODE) .tmp/browser/sdk-probe.mjs "$(BLOB_SDK_PROBE_REPORT)" "$(CARGO_TARGET_DIR)/debug/examples/verify_download"

test-sdk-inputs:
	@test -n "$(BLOB_SDK_INPUTS_REPORT)" || { echo 'Set BLOB_SDK_INPUTS_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage
	$(BLOB_BROWSER_NODE) .tmp/browser/native-inputs.mjs "$(BLOB_SDK_INPUTS_REPORT)" "$(CARGO_TARGET_DIR)/debug/blob-storage" "$(BLOB_SDK_INPUTS_BYTES)"

test-browser-transport:
	@test -n "$(BLOB_BROWSER_TRANSPORT_REPORT)" || { echo 'Set BLOB_BROWSER_TRANSPORT_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/transport.mjs "$(BLOB_BROWSER_TRANSPORT_REPORT)"

test-browser-publication:
	@test -n "$(BLOB_PUBLICATION_REPORT)" || { echo 'Set BLOB_PUBLICATION_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) .tmp/browser/publication.mjs "$(BLOB_PUBLICATION_REPORT)"

test-browser-store:
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/store.mjs

test-browser-bootstrap:
	@test -n "$(BLOB_BOOTSTRAP_REPORT)" || { echo 'Set BLOB_BOOTSTRAP_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/bootstrap.mjs "$(BLOB_BOOTSTRAP_REPORT)"

test-browser-launcher:
	@test -n "$(BLOB_LAUNCHER_REPORT)" || { echo 'Set BLOB_LAUNCHER_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/launcher.mjs "$(BLOB_LAUNCHER_REPORT)"

test-browser-native:
	@test -n "$(BLOB_NATIVE_REPORT)" || { echo 'Set BLOB_NATIVE_REPORT to a new directory beneath an existing parent'; exit 1; }
	$(BLOB_BROWSER_NODE) tests/browser/native.mjs "$(BLOB_NATIVE_REPORT)"

test-browser-standalone:
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p ic-blob-storage-canister --lib
	cargo build --offline --locked -p ic-blob-storage-cli --bin blob-storage
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone chromium_standalone_trial -- --ignored --test-threads=1

test-browser:
	$(BLOB_BROWSER_NODE) tests/browser/build.mjs
	$(BLOB_BROWSER_NODE) tests/browser/store.mjs
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe -p blob-consumer-probe --lib
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage chromium_certificate_intent -- --ignored --test-threads=1

test-admission-resources:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-admission-probe --lib
	@mkdir -p .tmp
	BLOB_ADMISSION_RESOURCE_REPORT="$(CURDIR)/.tmp/admission-resources.json" \
	BLOB_ADMISSION_HISTORY_REPORT="$(CURDIR)/.tmp/admission-history.json" \
	BLOB_RELEASE_HISTORY_REPORT="$(CURDIR)/.tmp/release-history.json" \
	BLOB_REFERENCE_HISTORY_REPORT="$(CURDIR)/.tmp/reference-history.json" \
	BLOB_DESCRIPTOR_RESOURCE_REPORT="$(CURDIR)/.tmp/descriptor-resources.json" \
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test admission admission_resources -- --test-threads=2
	@echo "Local resource reports in .tmp/: admission-resources.json, admission-history.json, release-history.json, reference-history.json, descriptor-resources.json (not provider pricing)"

test-funding-receipt-resources:
	@test -n "$(BLOB_FUNDING_RECEIPT_PROFILE)" || { echo "Set BLOB_FUNDING_RECEIPT_PROFILE to a fresh report directory." >&2; exit 1; }
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-storage-probe --lib
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage storage_resources::credits::receipt_populated_confirmation_and_restoration_profile -- --ignored --exact --test-threads=1

test-read-resources:
	cargo build --offline --locked --release --target wasm32-unknown-unknown -p blob-authority-probe -p blob-gateway-source --lib
	@mkdir -p .tmp
	BLOB_READ_RESOURCE_REPORT="$(CURDIR)/.tmp/read-resources.json" \
	bash scripts/ci/run-nonempty-cargo-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test journey readback -- --test-threads=2
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
	@for script in scripts/release/*.sh scripts/dev/*.sh scripts/ci/*.sh .githooks/pre-commit; do bash -n "$$script" || exit $$?; done
	"$(SHELLCHECK)" scripts/release/*.sh scripts/dev/*.sh scripts/ci/*.sh .githooks/pre-commit
	perl -c scripts/release/release-data.pl
	perl -c scripts/ci/check-documentation-links.pl

release-check: release-commands-check
	bash scripts/ci/test-release-runner.sh
	bash scripts/release/test-release.sh

ci:
	+@set -e; for target in $(CI_TARGETS); do \
		$(MAKE) --no-print-directory "$$target" || exit $$?; \
	done

validate: ci

release-verify:
	+VALIDATION_FAILURE_LOG_DIR="$$(git rev-parse --git-path release-state)/validation-failures" \
		bash scripts/ci/run-validation-targets.sh ci

release-plan:
	@$(RELEASE) plan "$(if $(VERSION),$(VERSION),patch)"

ensure-clean:
	@$(RELEASE) ensure-clean

release-patch release-minor release-major:
	+@bash scripts/ci/run-release.sh "$(@:release-%=%)" "$(RELEASE_REMOTE)" "$(RELEASE_BRANCH)"

release-resume:
	+@bash scripts/ci/run-release.sh resume "$(VERSION)" "$(RELEASE_REMOTE)" "$(RELEASE_BRANCH)"

release-version:
	@$(RELEASE) version

release-preflight:
	@$(RELEASE) preflight

release-prepare-version:
	@$(RELEASE) prepare

release-prepared-check:
	@$(RELEASE) prepared-check

release-files:
	@$(RELEASE) files

release-commit-check:
	@$(RELEASE) commit-check

release-committed-check:
	@$(RELEASE) committed-check

release-tagged-check release-push-check:
	@$(RELEASE) tagged-check

release-tag-check:
	$(RELEASE) tag-check

publish:
	$(RELEASE) publish

publish-dry-run:
	$(RELEASE) publish --dry-run
